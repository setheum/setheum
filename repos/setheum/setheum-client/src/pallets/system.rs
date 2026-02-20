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

use primitives::Nonce;
use subxt::utils::Static;

use crate::{
    api, connections::TxInfo, frame_system::pallet::Call::set_code, AccountId, AsConnection,
    Balance, BlockHash, Call::System, ConnectionApi, RootConnection, SudoCall, TxStatus,
};

/// Pallet system read-only api.
#[async_trait::async_trait]
pub trait SystemApi {
/// returns free balance of a given account
/// * `account` - account id
/// * `at` - optional hash of a block to query state from
///
/// it uses [`system.account`](https://paritytech.github.io/substrate/master/frame_system/pallet/struct.Pallet.html#method.account) storage
    async fn get_free_balance(&self, account: AccountId, at: Option<BlockHash>) -> Balance;

/// returns account nonce of a given account
/// * `account` - account id
    async fn account_nonce(&self, account: &AccountId) -> anyhow::Result<Nonce>;
}

/// Pallet system api.
#[async_trait::async_trait]
pub trait SystemSudoApi {
/// API for [`set_code`](https://paritytech.github.io/substrate/master/frame_system/pallet/struct.Pallet.html#method.set_code) call.
    async fn set_code(&self, code: Vec<u8>, status: TxStatus) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl SystemSudoApi for RootConnection {
    async fn set_code(&self, code: Vec<u8>, status: TxStatus) -> anyhow::Result<TxInfo> {
        let call = System(set_code { code });

        self.sudo_unchecked(call, status).await
    }
}

#[async_trait::async_trait]
impl<C: AsConnection + Sync> SystemApi for C {
    async fn get_free_balance(&self, account: AccountId, at: Option<BlockHash>) -> Balance {
        let addrs = api::storage().system().account(Static(account));

        match self.get_storage_entry_maybe(&addrs, at).await {
            None => 0,
            Some(account) => account.data.free,
        }
    }

    async fn account_nonce(&self, account: &AccountId) -> anyhow::Result<Nonce> {
        let conn = self.as_connection();
        Ok(conn.client.tx().account_nonce(account).await?.try_into()?)
    }
}
