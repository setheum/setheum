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

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
};

pub use build::{
    network as build_network, NetworkOutput as BuildNetworkOutput, SubstrateNetworkConfig,
};
use network_clique::{AddressingInformation, NetworkIdentity, PeerId};
use parity_scale_codec::Codec;
pub use substrate::{PeerId as SubstratePeerId, ProtocolNetwork};

pub mod address_cache;
mod base_protocol;
mod build;
pub mod data;
#[cfg(test)]
pub mod mock;
pub mod session;
mod substrate;
pub mod tcp;

const LOG_TARGET: &str = "setbft-network";

/// A basic alias for properties we expect basic data to satisfy.
pub trait Data: Clone + Codec + Send + Sync + 'static {}

impl<D: Clone + Codec + Send + Sync + 'static> Data for D {}

#[async_trait::async_trait]
/// Interface for the gossip network. This represents a P2P network and a lot of the properties of
/// this interface result from that. In particular we might know the ID of a given peer, but not be
/// connected to them directly.
pub trait GossipNetwork<D: Data>: Send + 'static {
    type Error: Display + Send;
    type PeerId: Clone + Debug + Eq + Hash + Send + 'static;

    /// Attempt to send data to a peer. Might silently fail if we are not connected to them.
    fn send_to(&mut self, data: D, peer_id: Self::PeerId) -> Result<(), Self::Error>;

    /// Send data to a random peer, preferably from a list. It should send the data to a randomly
    /// chosen peer from the provided list, but if it cannot (e.g. because it's not connected) it
    /// will send to a random available peer. No guarantees any peer gets it even if no errors are
    /// returned, retry appropriately.
    fn send_to_random(
        &mut self,
        data: D,
        peer_ids: HashSet<Self::PeerId>,
    ) -> Result<(), Self::Error>;

    /// Broadcast data to all directly connected peers. Network-wide broadcasts have to be
    /// implemented on top of this abstraction. Note that there might be no currently connected
    /// peers, so there are no guarantees any single call sends anything even if no errors are
    /// returned, retry appropriately.
    fn broadcast(&mut self, data: D) -> Result<(), Self::Error>;

    /// Receive some data from the network, including information about who sent it.
    /// This method's implementation must be cancellation safe.
    async fn next(&mut self) -> Result<(D, Self::PeerId), Self::Error>;
}
