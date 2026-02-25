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

use std::iter;

use array_bytes::bytes2hex;
use sc_network::{
    config::{NonDefaultSetConfig, NonReservedPeerMode, NotificationHandshake, Role, SetConfig},
    NotificationService,
};
use sc_network_common::sync::message::BlockAnnouncesHandshake;
use sp_runtime::traits::{Block, Header};

use crate::{BlockHash, BlockNumber};

// NOTE: `set_config` will be ignored by `protocol.rs` as the base
// protocol is still hardcoded into the peerset.
const DUMMY_SET_CONFIG: SetConfig = SetConfig {
    in_peers: 0,
    out_peers: 0,
    reserved_nodes: Vec::new(),
    non_reserved_mode: NonReservedPeerMode::Deny,
};
// Setting the message size too low makes it impossible to establish notification streams,
// which is expected by the base protocol to inform other protocols about peers.
// Other than that we send no messages.
// This value provides a wide margin, I tested it works with just 1024, but 4KB is not a problem.
const MAX_MESSAGE_SIZE: u64 = 4 * 1024;

/// Generate a config for the base protocol and the notification service that should be passed to its service.
pub fn setup<B>(genesis_hash: B::Hash) -> (NonDefaultSetConfig, Box<dyn NotificationService>)
where
    B: Block<Hash = BlockHash>,
    B::Header: Header<Number = BlockNumber>,
{
    // used for backwards compatibility with older nodes, should be safe to remove after update 14
    let legacy_block_announces_protocol =
        format!("/{}/block-announces/1", bytes2hex("", genesis_hash));
    let base_protocol_name = format!("/{}/base-protocol/1", bytes2hex("", genesis_hash));

    NonDefaultSetConfig::new(
        base_protocol_name.into(),
        iter::once(legacy_block_announces_protocol.into()).collect(),
        MAX_MESSAGE_SIZE,
        Some(NotificationHandshake::new(
            BlockAnnouncesHandshake::<B>::build(
                // All nodes are full nodes.
                (&Role::Full).into(),
                // We always pretend the genesis block is our best block
                0,
                genesis_hash,
                genesis_hash,
            ),
        )),
        DUMMY_SET_CONFIG,
    )
}
