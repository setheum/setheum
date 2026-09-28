// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

use std::sync::Arc;

use sc_client_api::Backend;
use sc_network::{
	config::{
		FullNetworkConfiguration, NetworkConfiguration, NonDefaultSetConfig,
		Params as NetworkParams, ProtocolId, Role,
	},
	error::Error as NetworkError,
	NetworkBackend, NetworkService, NetworkWorker, NotificationMetrics,
};
use sc_network_light::light_client_requests::handler::LightClientRequestHandler;
use sc_network_sync::state_request_handler::StateRequestHandler;
use sc_network_transactions::TransactionsHandlerPrototype;
use sc_service::SpawnTaskHandle;
use sp_runtime::traits::{Block, Header};
use substrate_prometheus_endpoint::Registry;

use crate::{
	network::build::{
		own_protocols::Networks, transactions::build_transactions_prototype, SPAWN_CATEGORY,
	},
	BlockHash, BlockNumber, ClientForSetBFT,
};

/// The networking backend used for the substrate (libp2p) network.
pub type SetBFTNetworkBackend<B> = NetworkWorker<B, BlockHash>;

fn spawn_state_request_handler<B, BE, C>(
	full_network_config: &mut FullNetworkConfiguration<B, BlockHash, SetBFTNetworkBackend<B>>,
	protocol_id: &ProtocolId,
	client: Arc<C>,
	spawn_handle: &SpawnTaskHandle,
) where
	B: Block<Hash = BlockHash>,
	BE: Backend<B>,
	C: ClientForSetBFT<B, BE>,
{
	let num_peer_hint = full_network_config
		.network_config
		.default_peers_set_num_full as usize
		+ full_network_config
			.network_config
			.default_peers_set
			.reserved_nodes
			.len();
	let (service, protocol_config) = StateRequestHandler::<B, C>::new::<SetBFTNetworkBackend<B>>(
		// The None is the fork id, which we don't have.
		protocol_id,
		None,
		client,
		num_peer_hint,
	);
	spawn_handle.spawn("state-request-handler", SPAWN_CATEGORY, service.run());
	full_network_config.add_request_response_protocol(protocol_config);
}

fn spawn_light_client_request_handler<B, BE, C>(
	full_network_config: &mut FullNetworkConfiguration<B, BlockHash, SetBFTNetworkBackend<B>>,
	protocol_id: &ProtocolId,
	client: Arc<C>,
	spawn_handle: &SpawnTaskHandle,
) where
	B: Block<Hash = BlockHash>,
	BE: Backend<B>,
	C: ClientForSetBFT<B, BE>,
{
	let (handler, protocol_config) = LightClientRequestHandler::<B, C>::new::<SetBFTNetworkBackend<B>>(
		// The None is the fork id, which we don't have.
		protocol_id,
		None,
		client.clone(),
	);
	spawn_handle.spawn(
		"light-client-request-handler",
		SPAWN_CATEGORY,
		handler.run(),
	);
	full_network_config.add_request_response_protocol(protocol_config);
}

type BaseNetworkOutput<B> = (
	Arc<NetworkService<B, <B as Block>::Hash>>,
	Networks,
	TransactionsHandlerPrototype,
);

/// Create a base network with all the protocols already included. Also spawn (almost) all the necessary services.
pub fn network<B, BE, C>(
	network_config: &NetworkConfiguration,
	protocol_id: ProtocolId,
	client: Arc<C>,
	spawn_handle: &SpawnTaskHandle,
	base_protocol_config: NonDefaultSetConfig,
	metrics_registry: Option<Registry>,
) -> Result<BaseNetworkOutput<B>, NetworkError>
where
	B: Block<Hash = BlockHash>,
	B::Header: Header<Number = BlockNumber>,
	BE: Backend<B>,
	C: ClientForSetBFT<B, BE>,
{
	let mut full_network_config: FullNetworkConfiguration<
		B,
		BlockHash,
		SetBFTNetworkBackend<B>,
	> = FullNetworkConfiguration::new(network_config, metrics_registry.clone());
	let genesis_hash = client
		.hash(0)
		.ok()
		.flatten()
		.expect("Genesis block exists.");
	let networks = Networks::new::<B, BlockHash, SetBFTNetworkBackend<B>>(
		&mut full_network_config,
		&genesis_hash,
	);

	spawn_state_request_handler(
		&mut full_network_config,
		&protocol_id,
		client.clone(),
		spawn_handle,
	);
	spawn_light_client_request_handler(
		&mut full_network_config,
		&protocol_id,
		client.clone(),
		spawn_handle,
	);
	let notification_metrics = NotificationMetrics::new(metrics_registry.as_ref());
	let transactions_prototype = build_transactions_prototype::<B, SetBFTNetworkBackend<B>>(
		&mut full_network_config,
		&protocol_id,
		genesis_hash,
		notification_metrics,
	);

	// `PeerStore` is created when the `FullNetworkConfiguration` is initialized, it has to be
	// started (and started only once) by the user of the configuration.
	let peer_store_service = full_network_config.take_peer_store();
	spawn_handle.spawn("peer-store", SPAWN_CATEGORY, peer_store_service.run());

	let network_params = NetworkParams::<B, BlockHash, SetBFTNetworkBackend<B>> {
		role: Role::Full,
		executor: {
			let spawn_handle = spawn_handle.clone();
			Box::new(move |fut| {
				spawn_handle.spawn("libp2p-node", SPAWN_CATEGORY, fut);
			})
		},
		network_config: full_network_config,
		genesis_hash,
		protocol_id: protocol_id.clone(),
		fork_id: None,
		metrics_registry: metrics_registry.clone(),
		// The names are silly, but that's substrate's fault.
		block_announce_config: base_protocol_config,
		bitswap_config: None,
		notification_metrics: NotificationMetrics::new(metrics_registry.as_ref()),
	};

	let network_service = <SetBFTNetworkBackend<B> as NetworkBackend<B, BlockHash>>::new(network_params)?;
	let network = network_service.service().clone();
	spawn_handle.spawn_blocking("network-worker", SPAWN_CATEGORY, network_service.run());
	Ok((network, networks, transactions_prototype))
}
