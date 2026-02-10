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

use primitives::{staking::era_payout, MILLISECS_PER_BLOCK};
use subxt::utils::{MultiAddress, Static};

use crate::{
    api,
    connections::{AsConnection, TxInfo},
    frame_support::PalletId,
    pallet_treasury::pallet::Call::{approve_proposal, reject_proposal},
    pallets::{committee_management::CommitteeManagementApi, staking::StakingApi},
    sp_core::TypeId,
    sp_runtime::traits::AccountIdConversion,
    AccountId, Balance, BlockHash,
    Call::Treasury,
    ConnectionApi, RootConnection, SignedConnectionApi, SudoCall, TxStatus,
};

// Copied from `frame_support`.
impl TypeId for PalletId {
    const TYPE_ID: [u8; 4] = *b"modl";
}

/// Pallet treasury read-only api.
#[async_trait::async_trait]
pub trait TreasuryApi {
/// Returns an unique account id for all treasury transfers.
    async fn treasury_account(&self) -> AccountId;

/// Returns storage `proposals_count`.
/// * `at` - an optional block hash to query state from
    async fn proposals_count(&self, at: Option<BlockHash>) -> Option<u32>;

/// Returns storage `approvals`.
/// * `at` - an optional block hash to query state from
    async fn approvals(&self, at: Option<BlockHash>) -> Vec<u32>;
}

/// Pallet treasury api.
#[async_trait::async_trait]
pub trait TreasuryUserApi {
/// API for [`propose_spend`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.propose_spend) call.
    async fn propose_spend(
        &self,
        amount: Balance,
        beneficiary: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`approve_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.approve_proposal) call.
    async fn approve(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;

/// API for [`reject_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.reject_proposal) call.
    async fn reject(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;
}

/// Pallet treasury funcionality that is not directly related to any pallet call.
#[async_trait::async_trait]
pub trait TreasureApiExt {
/// When `staking.payout_stakers` is done, what amount of SEE is transferred to.
/// the treasury
    async fn possible_treasury_payout(&self) -> anyhow::Result<Balance>;
}

/// Pallet treasury api issued by the sudo account.
#[async_trait::async_trait]
pub trait TreasurySudoApi {
/// API for [`approve_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.approve_proposal) call.
/// wrapped  in [`sudo_unchecked_weight`](https://paritytech.github.io/substrate/master/pallet_sudo/pallet/struct.Pallet.html#method.sudo_unchecked_weight)
    async fn approve(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;

/// API for [`reject_proposal`](https://paritytech.github.io/substrate/master/pallet_treasury/pallet/struct.Pallet.html#method.reject_proposal) call.
/// wrapped [`sudo_unchecked_weight`](https://paritytech.github.io/substrate/master/pallet_sudo/pallet/struct.Pallet.html#method.sudo_unchecked_weight)
    async fn reject(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> TreasuryApi for C {
    async fn treasury_account(&self) -> AccountId {
        PalletId(*b"set/trsry").into_account_truncating()
    }

    async fn proposals_count(&self, at: Option<BlockHash>) -> Option<u32> {
        let addrs = api::storage().treasury().proposal_count();

        self.get_storage_entry_maybe(&addrs, at).await
    }

    async fn approvals(&self, at: Option<BlockHash>) -> Vec<u32> {
        let addrs = api::storage().treasury().approvals();

        self.get_storage_entry(&addrs, at).await.0
    }
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> TreasuryUserApi for S {
    async fn propose_spend(
        &self,
        amount: Balance,
        beneficiary: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .treasury()
            .propose_spend(amount, MultiAddress::Id(Static::from(beneficiary)));

        self.send_tx(tx, status).await
    }

    async fn approve(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().treasury().approve_proposal(proposal_id);

        self.send_tx(tx, status).await
    }

    async fn reject(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().treasury().reject_proposal(proposal_id);

        self.send_tx(tx, status).await
    }
}

#[async_trait::async_trait]
impl TreasurySudoApi for RootConnection {
    async fn approve(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = Treasury(approve_proposal { proposal_id });

        self.sudo_unchecked(call, status).await
    }

    async fn reject(&self, proposal_id: u32, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = Treasury(reject_proposal { proposal_id });

        self.sudo_unchecked(call, status).await
    }
}

#[async_trait::async_trait]
impl<C: AsConnection + Sync> TreasureApiExt for C {
    async fn possible_treasury_payout(&self) -> anyhow::Result<Balance> {
        let session_period = self.get_session_period().await?;
        let sessions_per_era = self.get_session_per_era().await?;
        let millisecs_per_era =
            MILLISECS_PER_BLOCK * session_period as u64 * sessions_per_era as u64;
        Ok(era_payout(millisecs_per_era).1)
    }
}
