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

use codec::Encode;
use subxt::rpc_params;

use crate::{
    api,
    api::runtime_types::{
        module_aleph::pallet::Call::set_emergency_finalizer, primitives::app::Public,
        sp_core::ed25519::Public as EdPublic,
    },
    connections::TxInfo,
    module_aleph::pallet::Call::schedule_finality_version_change,
    sp_core::Bytes,
    AccountId, AlephKeyPair, BlockHash, BlockNumber,
    Call::Aleph,
    ConnectionApi, Pair, RootConnection, SessionIndex, SudoCall, TxStatus, Version,
};

// TODO replace docs with link to pallet aleph docs, once they are published
/// Pallet aleph API which does not require sudo.
#[async_trait::async_trait]
pub trait AlephApi {
/// Gets the current finality version.
    async fn finality_version(&self, at: Option<BlockHash>) -> Version;
/// Gets the finality version for the next session.
    async fn next_session_finality_version(&self, at: Option<BlockHash>) -> Version;
/// Gets the emergency finalizer
    async fn emergency_finalizer(&self, at: Option<BlockHash>) -> Option<[u8; 32]>;
}

/// Pallet aleph API that requires sudo.
#[async_trait::async_trait]
pub trait AlephSudoApi {
/// Sets the emergency finalization key.
/// * `finalizer` - a new finalizer key
/// * `status` - a [`TxStatus`] of a tx to wait for
/// # Returns
/// Block hash of block where transaction was put or error
    async fn set_emergency_finalizer(
        &self,
        finalizer: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// Schedules a finality version change for a future session.
/// * `version` - next version of the finalizer
/// * `session` - from which session the next version applies
/// * `status` - a [`TxStatus`] of a tx to wait for
/// # Returns
/// Block hash of block where transaction was put or error
    async fn schedule_finality_version_change(
        &self,
        version: u32,
        session: SessionIndex,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Pallet aleph RPC api.
#[async_trait::async_trait]
pub trait AlephRpc {
/// Finalize the block with given hash and number using attached signature.
/// # Returns
/// Block hash of block where transaction was put or error
    async fn emergency_finalize(
        &self,
        number: BlockNumber,
        hash: BlockHash,
        key_pair: AlephKeyPair,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> AlephApi for C {
    async fn finality_version(&self, at: Option<BlockHash>) -> Version {
        let addrs = api::storage().aleph().finality_version();

        self.get_storage_entry(&addrs, at).await
    }

    async fn next_session_finality_version(&self, hash: Option<BlockHash>) -> Version {
        let method = "state_call";
        let api_method = "AlephSessionApi_next_session_finality_version";
        let params = rpc_params![api_method, "0x", hash];

        self.rpc_call(method.to_string(), params).await.unwrap()
    }

    async fn emergency_finalizer(&self, at: Option<BlockHash>) -> Option<[u8; 32]> {
        let addrs = api::storage().aleph().emergency_finalizer();

        self.get_storage_entry_maybe(&addrs, at)
            .await
            .map(|public| public.0 .0)
    }
}

#[async_trait::async_trait]
impl AlephSudoApi for RootConnection {
    async fn set_emergency_finalizer(
        &self,
        finalizer: AccountId,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let call = Aleph(set_emergency_finalizer {
            emergency_finalizer: Public(EdPublic(finalizer.into())),
        });
        self.sudo_unchecked(call, status).await
    }

    async fn schedule_finality_version_change(
        &self,
        version: u32,
        session: SessionIndex,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let call = Aleph(schedule_finality_version_change {
            version_incoming: version,
            session,
        });

        self.sudo_unchecked(call, status).await
    }
}

#[async_trait::async_trait]
impl<C: ConnectionApi> AlephRpc for C {
    async fn emergency_finalize(
        &self,
        number: BlockNumber,
        hash: BlockHash,
        key_pair: AlephKeyPair,
    ) -> anyhow::Result<()> {
        let method = "setheumNode_emergencyFinalize";
        let signature = key_pair.sign(&hash.encode());
        let raw_signature = Bytes::from(signature.0.to_vec());
        let params = rpc_params![raw_signature, hash, number];

        let _: () = self.rpc_call_no_return(method.to_string(), params).await?;

        Ok(())
    }
}
