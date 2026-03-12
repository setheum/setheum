// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use fake_runtime_api::fake_runtime::RuntimeApi;
use finality_setbft::{
	build_network, get_setheum_block_import, run_validator_node, BlockImporter, BuildNetworkOutput, ChannelProvider,
	FavouriteSelectChainProvider, Justification, JustificationTranslator, MillisecsPerBlock, RateLimiterConfig,
	RedirectingBlockImport, SessionPeriod, SetheumNodeConfig, SubstrateChainStatus, SyncOracle, ValidatorAddressCache,
};
use log::warn;
use module_setbft_runtime_api::SetBftApi;
use primitives::{Block, DEFAULT_BACKUP_FOLDER, MAX_BLOCK_SIZE};
use sc_basic_authorship::ProposerFactory;
use sc_client_api::HeaderBackend;
use sc_consensus::{ImportQueue, Link};
use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_consensus_slots::BackoffAuthoringBlocksStrategy;
use sc_service::{error::Error as ServiceError, Configuration, KeystoreContainer, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_api::ProvideRuntimeApi;
use sp_arithmetic::traits::BaseArithmetic;
use sp_consensus::DisableProofRecording;
use sp_consensus_aura::{sr25519::AuthorityPair as AuraPair, Slot};

use crate::{
	executor::executor,
	rpc::{create_full as create_full_rpc, FullDeps as RpcFullDeps},
	setheum_cli::SetheumCli,
};

type Executor = executor::Executor;
type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullPool = sc_transaction_pool::FullPool<Block, FullClient>;
type FullImportQueue = sc_consensus::DefaultImportQueue<Block>;
type FullProposerFactory = ProposerFactory<FullPool, FullClient, DisableProofRecording>;
pub struct ServiceComponents {
	pub client: Arc<FullClient>,
	pub backend: Arc<FullBackend>,
	pub task_manager: TaskManager,
	pub select_chain_provider: FavouriteSelectChainProvider<Block>,
	pub import_queue: FullImportQueue,
	pub transaction_pool: Arc<FullPool>,
	pub keystore_container: KeystoreContainer,
	pub justification_channel_provider: ChannelProvider<Justification>,
	pub telemetry: Option<Telemetry>,
}
struct LimitNonfinalized(u32);

impl<N: BaseArithmetic> BackoffAuthoringBlocksStrategy<N> for LimitNonfinalized {
	fn should_backoff(
		&self,
		chain_head_number: N,
		_chain_head_slot: Slot,
		finalized_number: N,
		_slow_now: Slot,
		_logging_target: &str,
	) -> bool {
		let nonfinalized_blocks: u32 = chain_head_number.saturating_sub(finalized_number).unique_saturated_into();
		match nonfinalized_blocks >= self.0 {
			true => {
				warn!(
					"We have {} nonfinalized blocks, with the limit being {}, delaying block production.",
					nonfinalized_blocks, self.0
				);
				true
			},
			false => false,
		}
	}
}

fn backup_path(setheum_config: &SetheumCli, base_path: &Path) -> Option<PathBuf> {
	if setheum_config.no_backup() {
		return None;
	}
	if let Some(path) = setheum_config.backup_path() {
		Some(path)
	} else {
		let path = base_path.join(DEFAULT_BACKUP_FOLDER);
		eprintln!("No backup path provided, using default path: {path:?} for SetBFT backups. Please do not remove this folder");
		Some(path)
	}
}

pub fn new_partial(config: &Configuration) -> Result<ServiceComponents, ServiceError> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = executor::get_executor(config);

	let (client, backend, keystore_container, task_manager) = sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
		config,
		telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
		executor,
	)?;

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let client: Arc<TFullClient<_, _, _>> = Arc::new(client);

	let select_chain_provider = FavouriteSelectChainProvider::default();

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);
	let justification_translator = JustificationTranslator::new(
		SubstrateChainStatus::new(backend.clone())
			.map_err(|e| ServiceError::Other(format!("failed to set up chain status: {e}")))?,
	);
	let justification_channel_provider = ChannelProvider::new();
	let setheum_block_import = get_setheum_block_import(
		client.clone(),
		justification_channel_provider.get_sender(),
		justification_translator,
		select_chain_provider.select_chain(),
	);

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

	// DO NOT change Aura parameters without updating the finality-setbft sync accordingly,
	// in particular the code responsible for verifying incoming Headers, as it is supposed
	// to duplicate parts of Aura internal logic
	let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(ImportQueueParams {
		block_import: setheum_block_import.clone(),
		justification_import: Some(Box::new(setheum_block_import)),
		client: client.clone(),
		create_inherent_data_providers: move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);

			Ok((slot, timestamp))
		},
		spawner: &task_manager.spawn_essential_handle(),
		registry: config.prometheus_registry(),
		check_for_equivocation: Default::default(),
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		compatibility_mode: Default::default(),
	})?;

	Ok(ServiceComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain_provider,
		transaction_pool,
		justification_channel_provider,
		telemetry,
	})
}

struct SetheumRuntimeVars {
	pub session_period: SessionPeriod,
	pub millisecs_per_block: MillisecsPerBlock,
	pub score_submission_period: u32,
}

fn get_setheum_runtime_vars(client: &Arc<FullClient>) -> SetheumRuntimeVars {
	let finalized = client.info().finalized_hash;

	let session_period =
		SessionPeriod(client.runtime_api().session_period(finalized).expect("should always be available"));

	let millisecs_per_block =
		MillisecsPerBlock(client.runtime_api().millisecs_per_block(finalized).expect("should always be available"));

	let score_submission_period = client.runtime_api().score_submission_period(finalized).unwrap_or(u32::MAX);

	SetheumRuntimeVars { session_period, millisecs_per_block, score_submission_period }
}

fn get_validator_address_cache(setheum_config: &SetheumCli) -> Option<ValidatorAddressCache> {
	setheum_config.collect_validator_network_data().then(ValidatorAddressCache::new)
}

fn get_proposer_factory(service_components: &ServiceComponents, config: &Configuration) -> FullProposerFactory {
	let mut proposer_factory = FullProposerFactory::new(
		service_components.task_manager.spawn_handle(),
		service_components.client.clone(),
		service_components.transaction_pool.clone(),
		config.prometheus_registry().cloned().as_ref(),
		None,
	);
	proposer_factory.set_default_block_size_limit(MAX_BLOCK_SIZE as usize);

	proposer_factory
}

fn get_rate_limit_config(setheum_config: &SetheumCli) -> RateLimiterConfig {
	RateLimiterConfig {
		setbft_network_bit_rate: setheum_config.setbft_network_bit_rate(),
		substrate_network_bit_rate: setheum_config.substrate_network_bit_rate(),
	}
}

struct NoopLink;

impl Link<Block> for NoopLink {}

/// Builds a new service for a full client.
pub fn new_authority(config: Configuration, setheum_config: SetheumCli) -> Result<TaskManager, ServiceError> {
	if setheum_config.external_addresses().is_empty() {
		panic!("Cannot run a validator node without external addresses, stopping.");
	}

	let mut service_components = new_partial(&config)?;

	let backup_path = backup_path(&setheum_config, config.base_path.path());

	let backoff_authoring_blocks = Some(LimitNonfinalized(setheum_config.max_nonfinalized_blocks()));
	let prometheus_registry = config.prometheus_registry().cloned();
	let sync_oracle = SyncOracle::new();
	let proposer_factory = get_proposer_factory(&service_components, &config);
	let slot_duration = sc_consensus_aura::slot_duration(&*service_components.client)?;
	let (block_import, block_rx) = RedirectingBlockImport::new(service_components.client.clone());

	let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _>(StartAuraParams {
		slot_duration,
		client: service_components.client.clone(),
		select_chain: service_components.select_chain_provider.select_chain(),
		block_import,
		proposer_factory,
		create_inherent_data_providers: move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);

			Ok((slot, timestamp))
		},
		force_authoring: config.force_authoring,
		backoff_authoring_blocks,
		keystore: service_components.keystore_container.local_keystore(),
		sync_oracle: sync_oracle.clone(),
		justification_sync_link: (),
		block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
		max_block_proposal_slot_portion: None,
		telemetry: service_components.telemetry.as_ref().map(|x| x.handle()),
		compatibility_mode: Default::default(),
	})?;

	let import_queue_handle = BlockImporter::new(service_components.import_queue.service());
	let rate_limiter_config = get_rate_limit_config(&setheum_config);
	let network_config = finality_setbft::SubstrateNetworkConfig {
		substrate_network_bit_rate: rate_limiter_config.substrate_network_bit_rate,
		network_config: config.network.clone(),
	};

	let BuildNetworkOutput {
		network,
		authentication_network,
		block_sync_network,
		sync_service,
		tx_handler_controller,
		system_rpc_tx,
	} = build_network(
		network_config,
		config.protocol_id(),
		service_components.client.clone(),
		sync_oracle.underlying_atomic(),
		service_components.transaction_pool.clone(),
		&service_components.task_manager.spawn_handle(),
		config.prometheus_config.as_ref().map(|config| config.registry.clone()),
	)?;

	let chain_status = SubstrateChainStatus::new(service_components.backend.clone())
		.map_err(|e| ServiceError::Other(format!("failed to set up chain status: {e}")))?;
	let validator_address_cache = get_validator_address_cache(&setheum_config);
	let rpc_builder = {
		let client = service_components.client.clone();
		let pool = service_components.transaction_pool.clone();
		let sync_oracle = sync_oracle.clone();
		let validator_address_cache = validator_address_cache.clone();
		let import_justification_tx = service_components.justification_channel_provider.get_sender();
		let chain_status = chain_status.clone();
		Box::new(move |deny_unsafe, _| {
			let deps = RpcFullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				import_justification_tx: import_justification_tx.clone(),
				justification_translator: JustificationTranslator::new(chain_status.clone()),
				sync_oracle: sync_oracle.clone(),
				validator_address_cache: validator_address_cache.clone(),
			};

			Ok(create_full_rpc(deps)?)
		})
	};

	service_components.task_manager.spawn_handle().spawn(
		"import-queue",
		None,
		service_components.import_queue.run(Box::new(NoopLink)),
	);

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network,
		sync_service,
		client: service_components.client.clone(),
		keystore: service_components.keystore_container.local_keystore(),
		task_manager: &mut service_components.task_manager,
		transaction_pool: service_components.transaction_pool.clone(),
		rpc_builder,
		backend: service_components.backend,
		system_rpc_tx,
		tx_handler_controller,
		config,
		telemetry: service_components.telemetry.as_mut(),
	})?;

	service_components.task_manager.spawn_essential_handle().spawn_blocking("aura", None, aura);

	let SetheumRuntimeVars { millisecs_per_block, session_period, score_submission_period } =
		get_setheum_runtime_vars(&service_components.client);

	let setheum_config = SetheumNodeConfig {
		authentication_network,
		block_sync_network,
		client: service_components.client,
		chain_status,
		import_queue_handle,
		select_chain_provider: service_components.select_chain_provider,
		session_period,
		millisecs_per_block,
		score_submission_period,
		spawn_handle: service_components.task_manager.spawn_handle().into(),
		keystore: service_components.keystore_container.local_keystore(),
		justification_channel_provider: service_components.justification_channel_provider,
		block_rx,
		registry: prometheus_registry,
		unit_creation_delay: setheum_config.unit_creation_delay(),
		backup_saving_path: backup_path,
		external_addresses: setheum_config.external_addresses(),
		validator_port: setheum_config.validator_port(),
		rate_limiter_config,
		sync_oracle,
		validator_address_cache,
		transaction_pool: service_components.transaction_pool,
	};

	service_components.task_manager.spawn_essential_handle().spawn_blocking(
		"setheum",
		None,
		run_validator_node(setheum_config),
	);

	Ok(service_components.task_manager)
}
