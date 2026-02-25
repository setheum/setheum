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

use std::sync::Arc;

use sc_client_api::Backend;
use sc_network::{config::Role, NetworkService};
use sc_network_sync::SyncingService;
use sc_rpc::system::Request as RpcRequest;
use sc_service::{build_system_rpc_future, SpawnTaskHandle};
use sc_utils::mpsc::{tracing_unbounded, TracingUnboundedSender};
use sp_runtime::traits::Block;

use crate::{network::build::SPAWN_CATEGORY, ClientForSetBFT};

/// Spawn the RPC handling service and return the interface for submitting requests to it.
pub fn spawn_rpc_service<B: Block, BE: Backend<B>, C: ClientForSetBFT<B, BE>>(
    network: Arc<NetworkService<B, B::Hash>>,
    sync_service: Arc<SyncingService<B>>,
    client: Arc<C>,
    spawn_handle: &SpawnTaskHandle,
) -> TracingUnboundedSender<RpcRequest<B>> {
    let (rpcs_for_handling, rpcs_from_user) = tracing_unbounded("mpsc_system_rpc", 10_000);
    spawn_handle.spawn(
        "system-rpc-handler",
        SPAWN_CATEGORY,
        build_system_rpc_future(
            Role::Full,
            network,
            sync_service,
            client,
            rpcs_from_user,
            // We almost always run with bootnodes, and this impacts only one deprecated RPC call, so whatever.
            true,
        ),
    );
    rpcs_for_handling
}
