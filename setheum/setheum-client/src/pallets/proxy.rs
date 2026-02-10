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

use subxt::utils::MultiAddress;

use crate::{
    setheum_runtime::{ProxyType, RuntimeCall},
    api, AccountId, SignedConnectionApi, TxInfo, TxStatus,
};

/// any object that implements pallet proxy api
#[async_trait::async_trait]
pub trait ProxyUserApi {
/// API for [`proxy`](https://paritytech.github.io/polkadot-sdk/master/pallet_proxy/pallet/struct.Pallet.html#method.proxy) call.
    async fn proxy(
        &self,
        real: AccountId,
        call: RuntimeCall,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`add_proxy`](https://paritytech.github.io/polkadot-sdk/master/pallet_proxy/pallet/struct.Pallet.html#method.add_proxy) call.
    async fn add_proxy(
        &self,
        delegate: AccountId,
        proxy_type: ProxyType,
        delay: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`remove_proxy`](https://paritytech.github.io/polkadot-sdk/master/pallet_proxy/pallet/struct.Pallet.html#method.remove_proxy) call.
    async fn remove_proxy(
        &self,
        delegate: AccountId,
        proxy_type: ProxyType,
        delay: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> ProxyUserApi for S {
    async fn proxy(
        &self,
        real: AccountId,
        call: RuntimeCall,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .proxy()
            .proxy(MultiAddress::Id(real.into()), None, call);

        self.send_tx(tx, status).await
    }
    async fn add_proxy(
        &self,
        delegate: AccountId,
        proxy_type: ProxyType,
        delay: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .proxy()
            .add_proxy(MultiAddress::Id(delegate.into()), proxy_type, delay);

        self.send_tx(tx, status).await
    }
    async fn remove_proxy(
        &self,
        delegate: AccountId,
        proxy_type: ProxyType,
        delay: u32,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx =
            api::tx()
                .proxy()
                .remove_proxy(MultiAddress::Id(delegate.into()), proxy_type, delay);

        self.send_tx(tx, status).await
    }
}
