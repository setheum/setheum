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

use crate::{
    api, module_feature_control::Feature, BlockHash, ConnectionApi, RootConnection,
    SignedConnectionApi, TxInfo, TxStatus,
};

/// Read only pallet feature control API.
#[async_trait::async_trait]
pub trait FeatureControlApi {
/// Check if a feature is active.
    async fn is_feature_active(&self, feature: Feature, at: Option<BlockHash>) -> bool;
}

/// Pallet feature control API that requires sudo.
#[async_trait::async_trait]
pub trait FeatureControlSudoApi {
/// Enable a feature.
    async fn enable_feature(&self, feature: Feature, status: TxStatus) -> anyhow::Result<TxInfo>;
/// Disable a feature.
    async fn disable_feature(&self, feature: Feature, status: TxStatus) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> FeatureControlApi for C {
    async fn is_feature_active(&self, feature: Feature, at: Option<BlockHash>) -> bool {
        let addrs = api::storage().feature_control().active_features(feature);
        self.get_storage_entry_maybe(&addrs, at).await.is_some()
    }
}

#[async_trait::async_trait]
impl FeatureControlSudoApi for RootConnection {
    async fn enable_feature(&self, feature: Feature, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().feature_control().enable(feature);
        self.send_tx(tx, status).await
    }

    async fn disable_feature(&self, feature: Feature, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().feature_control().disable(feature);
        self.send_tx(tx, status).await
    }
}
