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

use codec::{Compact, Encode};
use pallet_contracts_primitives::ContractExecResult;
use subxt::{ext::sp_core::Bytes, rpc_params, utils::Static};

use crate::{
    api,
    api::runtime_types,
    pallet_contracts::wasm::{CodeInfo, Determinism},
    sp_weights::weight_v2::Weight,
    AccountId, Balance, BlockHash, CodeHash, ConnectionApi, SignedConnectionApi, TxInfo, TxStatus,
};

/// The Event that was emitted during execution of calls.
pub type EventRecord =
    runtime_types::frame_system::EventRecord<runtime_types::setheum_runtime::RuntimeEvent, BlockHash>;

/// Arguments to [`ContractRpc::call_and_get`].
#[derive(Encode)]
pub struct ContractCallArgs {
/// Who is singing a tx.
    pub origin: AccountId,
/// Address of the contract to call.
    pub dest: AccountId,
/// The balance to transfer from the `origin` to `dest`.
    pub value: Balance,
/// The gas limit enforced when executing the constructor.
    pub gas_limit: Option<Weight>,
/// The maximum amount of balance that can be charged from the caller to pay for the storage consumed.
    pub storage_deposit_limit: Option<Balance>,
/// The input data to pass to the contract.
    pub input_data: Vec<u8>,
}

/// Pallet contracts read-only api.
#[async_trait::async_trait]
pub trait ContractsApi {
/// Returns `contracts.code_info` storage for a given code hash.
/// * `code_hash` - a code hash
/// * `at` - optional hash of a block to query state from
    async fn get_code_info(&self, code_hash: CodeHash, at: Option<BlockHash>) -> Option<CodeInfo>;
}

/// Pallet contracts api.
#[async_trait::async_trait]
pub trait ContractsUserApi {
/// API for [`upload_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.upload_code) call.
    async fn upload_code(
        &self,
        code: Vec<u8>,
        storage_limit: Option<Compact<Balance>>,
        determinism: Determinism,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`instantiate`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.instantiate) call.
    #[allow(clippy::too_many_arguments)]
    async fn instantiate(
        &self,
        code_hash: CodeHash,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`instantiate_with_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.instantiate_with_code) call.
    #[allow(clippy::too_many_arguments)]
    async fn instantiate_with_code(
        &self,
        code: Vec<u8>,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`call`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.call) call.
    async fn call(
        &self,
        destination: AccountId,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

/// API for [`remove_code`](https://paritytech.github.io/substrate/master/pallet_contracts/pallet/struct.Pallet.html#method.remove_code) call.
    async fn remove_code(&self, code_hash: BlockHash, status: TxStatus) -> anyhow::Result<TxInfo>;
}

/// RPC for runtime ContractsApi
#[async_trait::async_trait]
pub trait ContractRpc {
/// API for [`call`](https://paritytech.github.io/substrate/master/pallet_contracts/trait.ContractsApi.html#method.call) call.
    async fn call_and_get(
        &self,
        args: ContractCallArgs,
    ) -> anyhow::Result<ContractExecResult<Balance, EventRecord>>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> ContractsApi for C {
    async fn get_code_info(&self, code_hash: CodeHash, at: Option<BlockHash>) -> Option<CodeInfo> {
        let addrs = api::storage().contracts().code_info_of(code_hash);

        self.get_storage_entry_maybe(&addrs, at).await
    }
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> ContractsUserApi for S {
    async fn upload_code(
        &self,
        code: Vec<u8>,
        storage_limit: Option<Compact<Balance>>,
        determinism: Determinism,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx()
            .contracts()
            .upload_code(code, storage_limit, determinism);

        self.send_tx(tx, status).await
    }

    async fn instantiate(
        &self,
        code_hash: CodeHash,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx().contracts().instantiate(
            balance,
            gas_limit,
            storage_limit,
            code_hash,
            data,
            salt,
        );

        self.send_tx(tx, status).await
    }

    async fn instantiate_with_code(
        &self,
        code: Vec<u8>,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        salt: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx().contracts().instantiate_with_code(
            balance,
            gas_limit,
            storage_limit,
            code,
            data,
            salt,
        );

        self.send_tx(tx, status).await
    }

    async fn call(
        &self,
        destination: AccountId,
        balance: Balance,
        gas_limit: Weight,
        storage_limit: Option<Compact<Balance>>,
        data: Vec<u8>,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo> {
        let tx = api::tx().contracts().call(
            Static::from(destination).into(),
            balance,
            gas_limit,
            storage_limit,
            data,
        );
        self.send_tx(tx, status).await
    }

    async fn remove_code(&self, code_hash: BlockHash, status: TxStatus) -> anyhow::Result<TxInfo> {
        let tx = api::tx().contracts().remove_code(code_hash);

        self.send_tx(tx, status).await
    }
}

#[async_trait::async_trait]
impl<C: ConnectionApi> ContractRpc for C {
    async fn call_and_get(
        &self,
        args: ContractCallArgs,
    ) -> anyhow::Result<ContractExecResult<Balance, EventRecord>> {
        let params = rpc_params!["ContractsApi_call", Bytes(args.encode())];
        self.rpc_call("state_call".to_string(), params).await
    }
}
