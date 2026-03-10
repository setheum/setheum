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

//! Module to glue legacy and current version of the aggregator;

use std::{hash::Hash as StdHash, marker::PhantomData};

use setbft_aggregator::NetworkError as CurrentNetworkError;
use setbft_aggregator::NetworkError as LegacyNetworkError;
use parity_scale_codec::{Decode, Encode};

use crate::{
    sbft::SignatureSet,
    setbft_primitives::Hash,
    crypto::Signature,
    network::{
        data::{Network, SendError},
        Data,
    },
    Keychain,
};

/// Either a block hash or a performance report hash. They should never overlap. We assume that
/// BlockHash and Hash are the same, it has always been this way, but if it ever changes this place
/// will cause trouble.
#[derive(PartialEq, Eq, StdHash, Copy, Clone, Debug, Encode, Decode)]
pub enum SignableTypedHash {
    /// The hash corresponds to a block.
    Block(Hash),
    /// The hash corresponds to an SBFT performance report.
    Performance(Hash),
}

impl AsRef<[u8]> for SignableTypedHash {
    fn as_ref(&self) -> &[u8] {
        use SignableTypedHash::*;
        match self {
            Block(hash) => hash.as_ref(),
            Performance(hash) => hash.as_ref(),
        }
    }
}

pub type RmcNetworkData =
    setbft_aggregator::RmcNetworkData<SignableTypedHash, Signature, SignatureSet<Signature>>;

pub type AggregatorIO<N> = setbft_aggregator::IO<
    SignableTypedHash,
    NetworkWrapper<RmcNetworkData, N>,
    Keychain,
>;

/// Wrapper on the aggregator
pub struct Aggregator<N>
where
    N: Network<RmcNetworkData>,
{
    agg: AggregatorIO<N>,
}

impl<N> Aggregator<N>
where
    N: Network<RmcNetworkData>,
{
    pub fn new(multikeychain: &Keychain, rmc_network: N) -> Self {
        let scheduler = set_bft_rmc::DoublingDelayScheduler::new(
            tokio::time::Duration::from_millis(500),
        );
        let rmc_handler = set_bft_rmc::Handler::new(multikeychain.clone());
        let rmc_service = set_bft_rmc::Service::new(scheduler, rmc_handler);
        let aggregator = setbft_aggregator::HashSignatureAggregator::new();
        let aggregator_io =
            AggregatorIO::<N>::new(NetworkWrapper::new(rmc_network), rmc_service, aggregator);

        Self {
            agg: aggregator_io,
        }
    }

    pub async fn start_aggregation(&mut self, h: SignableTypedHash) {
        self.agg.start_aggregation(h).await
    }

    pub async fn next_multisigned_hash(
        &mut self,
    ) -> Option<(SignableTypedHash, SignatureSet<Signature>)> {
         self.agg.next_multisigned_hash().await
    }

    pub fn status_report(&self) {
        self.agg.status_report()
    }
}

pub struct NetworkWrapper<D: Data, N: Network<D>>(N, PhantomData<D>);

impl<D: Data, N: Network<D>> NetworkWrapper<D, N> {
    pub fn new(network: N) -> Self {
        Self(network, PhantomData)
    }
}

#[async_trait::async_trait]
impl<T, D> setbft_aggregator::ProtocolSink<D> for NetworkWrapper<D, T>
where
    T: Network<D>,
    D: Data,
{
    async fn next(&mut self) -> Option<D> {
        self.0.next().await
    }

    fn send(
        &self,
        data: D,
        recipient: set_bft::Recipient,
    ) -> Result<(), CurrentNetworkError> {
        self.0.send(data, recipient.into()).map_err(|e| match e {
            SendError::SendFailed => CurrentNetworkError::SendFail,
        })
    }
}
