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

use subxt::utils::{MultiAddress, Static};

use crate::{
    api, connections::TxInfo, module_vesting::vesting_info::VestingInfo, AccountId, Balance,
    BlockHash, BlockNumber, ConnectionApi, SignedConnectionApi, TxStatus,
};

/// Read only pallet vesting API.
#[async_trait::async_trait]
pub trait VestingApi {
/// Returns [`VestingInfo`] of the given account.
/// * `who` - an account id
/// * `at` - optional hash of a block to query state from
    async fn get_vesting(
        &self,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> Vec<VestingInfo<Balance, BlockNumber>>;
}

/// Pallet vesting api.
#[async_trait::async_trait]
pub trait VestingUserApi {
/// API for [`vest`](https://paritytech.github.io/substrate/master/module_vesting/pallet/enum.Call.html#variant.vest) call.
    async fn vest(&self, status: TxStatus) -> anyhow::Result<TxInfo>;

/// API for [`vest_other`](https://paritytech.github.io/substrate/master/module_vesting/pallet/enum.Call.html#variant.vest_other) call.
    async fn vest_other(&self, status: TxStatus, other: AccountId) -> anyhow::Result<TxInfo>;

/// API for [`vested_transfer`](https://paritytech.github.io/substrate/master/module_vesting/pallet/enum.Call.html#variant.vested_transfer) call.
    async fn vested_transfer(
        &self,
        receiver: AccountId,
        schedule: VestingInfo<Balance, BlockNumber>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`merge_schedules`](https://paritytech.github.io/substrate/master/module_vesting/pallet/enum.Call.html#variant.merge_schedules) call.
    async fn merge_schedules(
        &self,
        idx1: u32,
        idx2: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> VestingApi for C {
    async fn get_vesting(
        &self,
        who: AccountId,
        at: Option<BlockHash>,
    ) -> Vec<VestingInfo<Balance, BlockNumber>> {
        let addrs = api::storage().vesting().vesting(Static(who));

        self.get_storage_entry(&addrs, at).await.0
    }
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> VestingUserApi for S {
    async fn vest(&self, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().vesting().vest();

        self.send_tx(tx, status).await
    }

    async fn vest_other(&self, status: TxStatus, other: AccountId) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .vesting()
            .vest_other(MultiAddress::Id(other.into()));

        self.send_tx(tx, status).await
    }

    async fn vested_transfer(
        &self,
        receiver: AccountId,
        schedule: VestingInfo<Balance, BlockNumber>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .vesting()
            .vested_transfer(MultiAddress::Id(receiver.into()), schedule);

        self.send_tx(tx, status).await
    }

    async fn merge_schedules(
        &self,
        idx1: u32,
        idx2: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx().vesting().merge_schedules(idx1, idx2);

        self.send_tx(tx, status).await
    }
}
