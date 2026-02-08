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
    api, api::runtime_types::setheum_runtime::SessionKeys, connections::TxInfo, AccountId, BlockHash,
    ConnectionApi, SessionIndex, SignedConnectionApi, TxStatus,
};

/// Pallet session read-only api.
#[async_trait::async_trait]
pub trait SessionApi {
/// API for [`next_keys`](https://paritytech.github.io/substrate/master/pallet_session/pallet/type.NextKeys.html) call.
    async fn get_next_session_keys(
        &self,
        account: AccountId,
        at: Option<BlockHash>,
    ) -> Option<SessionKeys>;

/// API for [`current_index`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.current_index) call.
    async fn get_session(&self, at: Option<BlockHash>) -> SessionIndex;

/// API for [`validators`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.validators) call.
    async fn get_validators(&self, at: Option<BlockHash>) -> Vec<AccountId>;
}

/// any object that implements pallet session api
#[async_trait::async_trait]
pub trait SessionUserApi {
/// API for [`set_keys`](https://paritytech.github.io/substrate/master/pallet_session/pallet/struct.Pallet.html#method.set_keys) call.
    async fn set_keys(&self, new_keys: SessionKeys, status: TxStatus) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> SessionApi for C {
    async fn get_next_session_keys(
        &self,
        account: AccountId,
        at: Option<BlockHash>,
    ) -> Option<SessionKeys> {
        let addrs = api::storage().session().next_keys(Static::from(account));

        self.get_storage_entry_maybe(&addrs, at).await
    }

    async fn get_session(&self, at: Option<BlockHash>) -> SessionIndex {
        let addrs = api::storage().session().current_index();

        self.get_storage_entry_maybe(&addrs, at)
            .await
            .unwrap_or_default()
    }

    async fn get_validators(&self, at: Option<BlockHash>) -> Vec<AccountId> {
        let addrs = api::storage().session().validators();

        self.get_storage_entry(&addrs, at)
            .await
            .into_iter()
            .map(|x| x.0)
            .collect()
    }
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> SessionUserApi for S {
    async fn set_keys(&self, new_keys: SessionKeys, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().session().set_keys(new_keys, vec![]);

        self.send_tx(tx, status).await
    }
}
