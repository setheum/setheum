// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use subxt::utils::Static;

use crate::{
    api,
    api::runtime_types::primitives::{CommitteeSeats, EraValidators},
    connections::{AsConnection, TxInfo},
    module_elections::pallet::Call::{change_validators, set_elections_openness},
    primitives::ElectionOpenness,
    AccountId, BlockHash,
    Call::Elections,
    ConnectionApi, RootConnection, SudoCall, TxStatus,
};

// TODO once pallet elections docs are published, replace api docs with links to public docs
/// Pallet elections read-only api.
#[async_trait::async_trait]
pub trait ElectionsApi {
/// Returns `elections.committee_size` storage of the elections pallet.
/// * `at` - optional hash of a block to query state from
    async fn get_committee_seats(&self, at: Option<BlockHash>) -> CommitteeSeats;

/// Returns `elections.next_era_committee_seats` storage of the elections pallet.
/// * `at` - optional hash of a block to query state from
    async fn get_next_era_committee_seats(&self, at: Option<BlockHash>) -> CommitteeSeats;

/// Returns `elections.current_era_validators` storage of the elections pallet.
/// * `at` - optional hash of a block to query state from
    async fn get_current_era_validators(&self, at: Option<BlockHash>) -> EraValidators<AccountId>;

/// Returns `elections.next_era_reserved_validators` storage of the elections pallet.
/// * `at` - optional hash of a block to query state from
    async fn get_next_era_reserved_validators(&self, at: Option<BlockHash>) -> Vec<AccountId>;

/// Returns `elections.next_era_non_reserved_validators` storage of the elections pallet.
/// * `at` - optional hash of a block to query state from
    async fn get_next_era_non_reserved_validators(&self, at: Option<BlockHash>) -> Vec<AccountId>;
}

/// any object that implements pallet elections api that requires sudo
#[async_trait::async_trait]
pub trait ElectionsSudoApi {
/// Issues `elections.change_validators` that sets the committee for the next era.
/// * `new_reserved_validators` - reserved validators to be in place in the next era; optional
/// * `new_non_reserved_validators` - non reserved validators to be in place in the next era; optional
/// * `committee_size` - committee size to be in place in the next era; optional
/// * `status` - a [`TxStatus`] for a tx to wait for
    async fn change_validators(
        &self,
        new_reserved_validators: Option<Vec<AccountId>>,
        new_non_reserved_validators: Option<Vec<AccountId>>,
        committee_size: Option<CommitteeSeats>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// Set openness of the elections.
/// * `mode` - new elections openness mode
/// * `status` - a [`TxStatus`] for a tx to wait for
    async fn set_election_openness(
        &self,
        mode: ElectionOpenness,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi + AsConnection> ElectionsApi for C {
    async fn get_committee_seats(&self, at: Option<BlockHash>) -> CommitteeSeats {
        let addrs = api::storage().elections().committee_size();

        self.get_storage_entry(&addrs, at).await
    }

    async fn get_next_era_committee_seats(&self, at: Option<BlockHash>) -> CommitteeSeats {
        let addrs = api::storage().elections().next_era_committee_size();

        self.get_storage_entry(&addrs, at).await
    }

    async fn get_current_era_validators(&self, at: Option<BlockHash>) -> EraValidators<AccountId> {
        let addrs = api::storage().elections().current_era_validators();
        let era_validators_with_static_account_ids = self.get_storage_entry(&addrs, at).await;
        return EraValidators {
            reserved: era_validators_with_static_account_ids
                .reserved
                .into_iter()
                .map(|x| x.0)
                .collect(),
            non_reserved: era_validators_with_static_account_ids
                .non_reserved
                .into_iter()
                .map(|x| x.0)
                .collect(),
        };
    }

    async fn get_next_era_reserved_validators(&self, at: Option<BlockHash>) -> Vec<AccountId> {
        let addrs = api::storage().elections().next_era_reserved_validators();

        self.get_storage_entry(&addrs, at)
            .await
            .into_iter()
            .map(|x| x.0)
            .collect()
    }

    async fn get_next_era_non_reserved_validators(&self, at: Option<BlockHash>) -> Vec<AccountId> {
        let addrs = api::storage()
            .elections()
            .next_era_non_reserved_validators();

        self.get_storage_entry(&addrs, at)
            .await
            .into_iter()
            .map(|x| x.0)
            .collect()
    }
}

#[async_trait::async_trait]
impl ElectionsSudoApi for RootConnection {
    async fn change_validators(
        &self,
        new_reserved_validators: Option<Vec<AccountId>>,
        new_non_reserved_validators: Option<Vec<AccountId>>,
        committee_size: Option<CommitteeSeats>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let call = Elections(change_validators {
            reserved_validators: new_reserved_validators
                .map(|inner| inner.into_iter().map(Static).collect()),
            non_reserved_validators: new_non_reserved_validators
                .map(|inner| inner.into_iter().map(Static).collect()),
            committee_size,
        });

        self.sudo_unchecked(call, status).await
    }

    async fn set_election_openness(
        &self,
        mode: ElectionOpenness,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let call = Elections(set_elections_openness { openness: mode });

        self.sudo_unchecked(call, status).await
    }
}
