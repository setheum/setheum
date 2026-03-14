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

//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::{collections::HashMap, sync::Arc};

pub use fc_rpc::{Eth, EthApiServer, Net, NetApiServer, Web3, Web3ApiServer};
pub use fp_rpc::EthereumRuntimeRPCApi;
use finality_setbft::{
	BlockId, Justification, JustificationTranslator, SetheumJustification, ValidatorAddressCache,
	ValidatorAddressingInfo,
};
use futures::channel::mpsc;
use jsonrpsee::{
	core::{error::Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
	RpcModule,
};
use parity_scale_codec::Decode;
use primitives::{AccountId, Balance, Block, BlockHash, BlockNumber, Nonce, Signature};
use sc_client_api::StorageProvider;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use setheum_runtime::{CurrencyId, DataProviderId, TimeStampedPrice};
use sp_api::ProvideRuntimeApi;
use sp_arithmetic::traits::Zero;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SyncOracle;
use sp_consensus_aura::digests::CompatibleDigestItem;
use sp_core::{twox_128, Bytes};
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT},
	DigestItem,
};

/// Full client dependencies.
pub struct FullDeps<C, P, BE, SO> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	pub import_justification_tx: mpsc::UnboundedSender<Justification>,
	pub justification_translator: JustificationTranslator,
	pub sync_oracle: SO,
	pub validator_address_cache: Option<ValidatorAddressCache>,
	/// Frontier backend.
	pub frontier_backend: Arc<fc_db::Backend<Block, BE>>,
	/// Eth filter pool.
	pub filter_pool: Option<fc_rpc::FilterPool>,
	/// Graph pool.
	pub graph: Arc<P::Analyzer>,
	/// Maximum number of logs in a filter.
	pub max_past_logs: u32,
	/// Fee history limit.
	pub fee_history_limit: u32,
	/// Fee history cache.
	pub fee_history_cache: fc_rpc::FeeHistoryCache,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, BE, SO>(
	deps: FullDeps<C, P, SO>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ StorageProvider<Block, BE>
		+ Send
		+ Sync
		+ 'static,
	BE: sc_client_api::Backend<Block> + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ module_oracle_rpc::OracleRuntimeApi<Block, DataProviderId, CurrencyId, TimeStampedPrice>
		+ module_move_rpc::MoveRuntimeApi<Block, AccountId>
		+ EthereumRuntimeRPCApi<Block>
		+ BlockBuilder<Block>,
	P: TransactionPool + 'static,
	SO: SyncOracle + Send + Sync + 'static,
{
	use module_oracle_rpc::{Oracle, OracleApiServer};
	use module_move_rpc::{MoveApiServer, MovePallet};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		deny_unsafe,
		import_justification_tx,
		justification_translator,
		sync_oracle,
		validator_address_cache,
		frontier_backend,
		filter_pool,
		graph,
		max_past_logs,
		fee_history_limit,
		fee_history_cache,
	} = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;

	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	module.merge(Oracle::new(client.clone()).into_rpc())?;
	module.merge(MovePallet::new(client.clone()).into_rpc())?;

	module.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph,
			None, // Sync service - optional
			filter_pool,
			frontier_backend,
			max_past_logs,
			fee_history_limit,
			fee_history_cache,
			Default::default(), // Forced gas price
		)
		.into_rpc(),
	)?;

	module.merge(Net::new(client.clone(), pool.clone(), true).into_rpc())?;
	module.merge(Web3::new(client.clone()).into_rpc())?;

	module.merge(
		SetheumNode::new(
			import_justification_tx,
			justification_translator,
			client,
			sync_oracle,
			validator_address_cache,
		)
		.into_rpc(),
	)?;

	Ok(module)
}

/// System RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Justification argument is malformed.
	#[error("{0}")]
	MalformedJustificationArg(String),
	/// Provided block range couldn't be resolved to a list of blocks.
	#[error("Node is not fully functional: {}", .0)]
	FailedJustificationSend(String),
	/// Justification argument is malformed.
	#[error("Failed to translate justification into an internal one: {}", .0)]
	FailedJustificationTranslation(String),
	/// Block doesn't have any Aura pre-runtime digest item.
	#[error("Block doesn't have any Aura pre-runtime digest item.")]
	BlockWithoutDigest,
	/// Failed to get storage item.
	#[error("Failed to get storage item {0}/{1} at block {2}.")]
	StorageItemNotAvailable(&'static str, &'static str, String),
	/// Failed to read storage.
	#[error("Failed to read {0}/{1} at the block {2}: {3:?}.")]
	FailedStorageRead(&'static str, &'static str, String, sp_blockchain::Error),
	/// Failed to decode storage item.
	#[error("Failed to decode storage item: {0}/{1} at the block {2}: {3:?}.")]
	FailedStorageDecoding(&'static str, &'static str, String, parity_scale_codec::Error),
	/// Failed to decode header.
	#[error("Failed to decode header of a block {0}: {1:?}.")]
	FailedHeaderDecoding(String, sp_blockchain::Error),
	/// Failed to find a block with provided hash.
	#[error("Failed to find a block with hash {0}.")]
	UnknownHash(String),
	/// Network info caching is not enabled.
	#[error("Unable to get any data, because network info caching is not enabled.")]
	NetworkInfoCachingNotEnabled,
}

// Base code for all system errors.
const BASE_ERROR: i32 = 2000;
// Justification argument is malformatted.
const MALFORMATTED_JUSTIFICATION_ARG_ERROR: i32 = BASE_ERROR + 1;
// SetheumNodeApiServer is failed to send translated justification.
const FAILED_JUSTIFICATION_SEND_ERROR: i32 = BASE_ERROR + 2;
// SetheumNodeApiServer failed to translate justification into internal representation.
const FAILED_JUSTIFICATION_TRANSLATION_ERROR: i32 = BASE_ERROR + 3;
// Block doesn't have any Aura pre-runtime digest item.
const BLOCK_WITHOUT_DIGEST_ERROR: i32 = BASE_ERROR + 4;
// Failed to get storage item.
const STORAGE_ITEM_NOT_AVAILABLE_ERROR: i32 = BASE_ERROR + 5;
/// Failed to read storage.
const FAILED_STORAGE_READ_ERROR: i32 = BASE_ERROR + 6;
/// Failed to decode storage item.
const FAILED_STORAGE_DECODING_ERROR: i32 = BASE_ERROR + 7;
/// Failed to decode header.
const FAILED_HEADER_DECODING_ERROR: i32 = BASE_ERROR + 8;
/// Failed to find a block with provided hash.
const UNKNOWN_HASH_ERROR: i32 = BASE_ERROR + 9;
/// Network info caching is not enabled.
const NETWORK_INFO_CACHING_NOT_ENABLED_ERROR: i32 = BASE_ERROR + 10;

impl From<Error> for JsonRpseeError {
	fn from(e: Error) -> Self {
		match e {
			Error::FailedJustificationSend(e) => {
				CallError::Custom(ErrorObject::owned(FAILED_JUSTIFICATION_SEND_ERROR, e, None::<()>))
			},
			Error::MalformedJustificationArg(e) => {
				CallError::Custom(ErrorObject::owned(MALFORMATTED_JUSTIFICATION_ARG_ERROR, e, None::<()>))
			},
			Error::FailedJustificationTranslation(e) => {
				CallError::Custom(ErrorObject::owned(FAILED_JUSTIFICATION_TRANSLATION_ERROR, e, None::<()>))
			},
			Error::BlockWithoutDigest => CallError::Custom(ErrorObject::owned(
				BLOCK_WITHOUT_DIGEST_ERROR,
				"Block doesn't have any Aura pre-runtime digest item.",
				None::<()>,
			)),
			Error::StorageItemNotAvailable(pallet, key, hash) => CallError::Custom(ErrorObject::owned(
				STORAGE_ITEM_NOT_AVAILABLE_ERROR,
				format!("Failed to get storage item {pallet}/{key} at the block {hash}."),
				None::<()>,
			)),
			Error::FailedStorageRead(pallet, key, hash, err) => CallError::Custom(ErrorObject::owned(
				FAILED_STORAGE_READ_ERROR,
				format!("Failed to read {pallet}/{key} at the block {hash}: {err:?}."),
				None::<()>,
			)),
			Error::FailedStorageDecoding(pallet, key, hash, err) => CallError::Custom(ErrorObject::owned(
				FAILED_STORAGE_DECODING_ERROR,
				format!("Failed to decode {pallet}/{key} at the block {hash}: {err:?}.",),
				None::<()>,
			)),
			Error::FailedHeaderDecoding(hash, err) => CallError::Custom(ErrorObject::owned(
				FAILED_HEADER_DECODING_ERROR,
				format!("Failed to decode header of a block {hash}: {err:?}.",),
				None::<()>,
			)),
			Error::UnknownHash(hash) => CallError::Custom(ErrorObject::owned(
				UNKNOWN_HASH_ERROR,
				format!("Failed to find a block with hash {hash}.",),
				None::<()>,
			)),
			Error::NetworkInfoCachingNotEnabled => CallError::Custom(ErrorObject::owned(
				NETWORK_INFO_CACHING_NOT_ENABLED_ERROR,
				"Unable to get any data, because network info caching is not enabled.",
				None::<()>,
			)),
		}
		.into()
	}
}

/// Setheum Node RPC API
#[rpc(client, server, namespace = "setheumNode")]
pub trait SetheumNodeApi<BE> {
	/// Finalize the block with given hash and number using attached signature. Returns the empty string or an error.
	#[method(name = "emergencyFinalize")]
	fn emergency_finalize(&self, justification: Bytes, hash: BlockHash, number: BlockNumber) -> RpcResult<()>;

	/// Get the author of the block with given hash.
	#[method(name = "getBlockAuthor")]
	fn block_author(&self, hash: BlockHash) -> RpcResult<Option<AccountId>>;

	/// Whether the node is ready for operation.
	#[method(name = "ready")]
	fn ready(&self) -> RpcResult<bool>;

	#[method(name = "unstable_validatorNetworkInfo")]
	fn validator_network_info(&self) -> RpcResult<HashMap<AccountId, ValidatorAddressingInfo>>;
}

/// Setheum Node API implementation
pub struct SetheumNode<Client, SO> {
	import_justification_tx: mpsc::UnboundedSender<Justification>,
	justification_translator: JustificationTranslator,
	client: Arc<Client>,
	sync_oracle: SO,
	validator_address_cache: Option<ValidatorAddressCache>,
}

impl<Client, SO> SetheumNode<Client, SO>
where
	SO: SyncOracle,
{
	pub fn new(
		import_justification_tx: mpsc::UnboundedSender<Justification>,
		justification_translator: JustificationTranslator,
		client: Arc<Client>,
		sync_oracle: SO,
		validator_address_cache: Option<ValidatorAddressCache>,
	) -> Self {
		SetheumNode { import_justification_tx, justification_translator, client, sync_oracle, validator_address_cache }
	}
}

impl<Client, BE, SO> SetheumNodeApiServer<BE> for SetheumNode<Client, SO>
where
	BE: sc_client_api::Backend<Block> + 'static,
	Client: HeaderBackend<Block> + StorageProvider<Block, BE> + 'static,
	SO: SyncOracle + Send + Sync + 'static,
{
	fn emergency_finalize(&self, justification: Bytes, hash: BlockHash, number: BlockNumber) -> RpcResult<()> {
		let justification: SetheumJustification =
			SetheumJustification::EmergencySignature(justification.0.try_into().map_err(|_| {
				Error::MalformedJustificationArg("Provided justification cannot be converted into correct type".into())
			})?);
		let justification = self
			.justification_translator
			.translate(justification, BlockId::new(hash, number))
			.map_err(|e| Error::FailedJustificationTranslation(format!("{e}")))?;
		self.import_justification_tx.unbounded_send(justification).map_err(|_| {
			Error::FailedJustificationSend(
				"SetheumNodeApiServer failed to send JustifictionNotification via its channel".into(),
			)
		})?;
		Ok(())
	}

	fn block_author(&self, hash: BlockHash) -> RpcResult<Option<AccountId>> {
		let header = self
			.client
			.header(hash)
			.map_err(|e| Error::FailedHeaderDecoding(hash.to_string(), e))?
			.ok_or(Error::UnknownHash(hash.to_string()))?;
		if header.number().is_zero() {
			return Ok(None);
		}

		let slot = header
			.digest()
			.logs()
			.iter()
			.find_map(<DigestItem as CompatibleDigestItem<Signature>>::as_aura_pre_digest)
			.ok_or(Error::BlockWithoutDigest)?;

		let parent = header.parent_hash();
		let block_producers_at_parent: Vec<AccountId> = read_storage("Session", "Validators", &self.client, *parent)?;

		Ok(Some(block_producers_at_parent[(u64::from(slot) as usize) % block_producers_at_parent.len()].clone()))
	}

	fn ready(&self) -> RpcResult<bool> {
		Ok(!self.sync_oracle.is_offline() && !self.sync_oracle.is_major_syncing())
	}

	fn validator_network_info(&self) -> RpcResult<HashMap<AccountId, ValidatorAddressingInfo>> {
		self.validator_address_cache
			.as_ref()
			.map(|c| c.snapshot())
			.ok_or(Error::NetworkInfoCachingNotEnabled.into())
	}
}

fn read_storage<
	T: Decode,
	Block: BlockT,
	Backend: sc_client_api::Backend<Block>,
	SP: StorageProvider<Block, Backend>,
>(
	pallet: &'static str,
	pallet_item: &'static str,
	storage_provider: &Arc<SP>,
	block_hash: Block::Hash,
) -> RpcResult<T> {
	let storage_key = [twox_128(pallet.as_bytes()), twox_128(pallet_item.as_bytes())].concat();

	let item_encoded = match storage_provider.storage(block_hash, &sc_client_api::StorageKey(storage_key)) {
		Ok(Some(bytes)) => bytes,
		Ok(None) => return Err(Error::StorageItemNotAvailable(pallet, pallet_item, block_hash.to_string()).into()),
		Err(e) => return Err(Error::FailedStorageRead(pallet, pallet_item, block_hash.to_string(), e).into()),
	};

	T::decode(&mut item_encoded.0.as_ref())
		.map_err(|e| Error::FailedStorageDecoding(pallet, pallet_item, block_hash.to_string(), e).into())
}
