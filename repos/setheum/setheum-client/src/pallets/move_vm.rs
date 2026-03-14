// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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
use serde::{Deserialize, Serialize};
use subxt::rpc_params;

use crate::{
	api,
	api::runtime_types::pallet_move::api::ModuleAbi,
	AccountId, Balance, ConnectionApi, SignedConnectionApi, TxInfo, TxStatus, BlockHash,
};

/// Gas estimation information.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Estimation {
	/// Gas used.
	pub gas_used: u64,
	/// Status code for the MoveVM execution.
	pub vm_status_code: u64,
	/// Substrate weight required for the complete extrinsic cost.
	pub total_weight_including_gas_used: crate::sp_weights::weight_v2::Weight,
}

/// MoveVM Pallet read-only api.
#[async_trait::async_trait]
pub trait MoveModuleApi {
	/// Estimate gas for publishing module.
	async fn estimate_gas_publish_module(&self, account: AccountId, bytecode: Vec<u8>) -> anyhow::Result<Estimation>;
	/// Estimate gas for publishing bundle.
	async fn estimate_gas_publish_bundle(&self, account: AccountId, bytecode: Vec<u8>) -> anyhow::Result<Estimation>;
	/// Estimate gas for executing Move script.
	async fn estimate_gas_execute_script(&self, transaction: Vec<u8>) -> anyhow::Result<Estimation>;
	/// Returns the resource for the given account and tag.
	async fn get_resource(&self, account: AccountId, tag: Vec<u8>) -> anyhow::Result<Option<Vec<u8>>>;
	/// Returns the module for the given account and name.
	async fn get_module(&self, address: AccountId, name: String) -> anyhow::Result<Option<Vec<u8>>>;
	/// Returns the module ABI for the given account and name.
	async fn get_module_abi(&self, address: AccountId, name: String) -> anyhow::Result<Option<ModuleAbi>>;
}

/// MoveVM Pallet user api.
#[async_trait::async_trait]
pub trait MoveModuleUserApi {
	/// API for `execute` call.
	async fn execute(
		&self,
		transaction_bc: Vec<u8>,
		gas_limit: u32,
		cheque_limit: Balance,
		status: TxStatus,
	) -> anyhow::Result<TxInfo>;

	/// API for `publish_module` call.
	async fn publish_module(
		&self,
		bytecode: Vec<u8>,
		gas_limit: u32,
		status: TxStatus,
	) -> anyhow::Result<TxInfo>;

	/// API for `publish_module_bundle` call.
	async fn publish_module_bundle(
		&self,
		bundle: Vec<u8>,
		gas_limit: u32,
		status: TxStatus,
	) -> anyhow::Result<TxInfo>;

	/// API for `update_stdlib_bundle` call (Sudo).
	async fn update_stdlib_bundle(
		&self,
		stdlib: Vec<u8>,
		status: TxStatus,
	) -> anyhow::Result<TxInfo>;
}

#[async_trait::async_trait]
impl<C: ConnectionApi> MoveModuleApi for C {
	async fn estimate_gas_publish_module(&self, account: AccountId, bytecode: Vec<u8>) -> anyhow::Result<Estimation> {
		let params = rpc_params!["mvm_estimateGasPublishModule", account, bytecode];
		self.rpc_call("mvm_estimateGasPublishModule".to_string(), params).await
	}

	async fn estimate_gas_publish_bundle(&self, account: AccountId, bytecode: Vec<u8>) -> anyhow::Result<Estimation> {
		let params = rpc_params!["mvm_estimateGasPublishBundle", account, bytecode];
		self.rpc_call("mvm_estimateGasPublishBundle".to_string(), params).await
	}

	async fn estimate_gas_execute_script(&self, transaction: Vec<u8>) -> anyhow::Result<Estimation> {
		let params = rpc_params!["mvm_estimateGasExecuteScript", transaction];
		self.rpc_call("mvm_estimateGasExecuteScript".to_string(), params).await
	}

	async fn get_resource(&self, account: AccountId, tag: Vec<u8>) -> anyhow::Result<Option<Vec<u8>>> {
		let params = rpc_params!["mvm_getResource", account, tag];
		self.rpc_call("mvm_getResource".to_string(), params).await
	}

	async fn get_module(&self, address: AccountId, name: String) -> anyhow::Result<Option<Vec<u8>>> {
		let params = rpc_params!["mvm_getModule", address, name];
		self.rpc_call("mvm_getModule".to_string(), params).await
	}

	async fn get_module_abi(&self, address: AccountId, name: String) -> anyhow::Result<Option<ModuleAbi>> {
		let params = rpc_params!["mvm_getModuleABI", address, name];
		self.rpc_call("mvm_getModuleABI".to_string(), params).await
	}
}

#[async_trait::async_trait]
impl<S: SignedConnectionApi> MoveModuleUserApi for S {
	async fn execute(
		&self,
		transaction_bc: Vec<u8>,
		gas_limit: u32,
		cheque_limit: Balance,
		status: TxStatus,
	) -> anyhow::Result<TxInfo> {
		let tx = api::tx()
			.move_module()
			.execute(transaction_bc, gas_limit, cheque_limit);

		self.send_tx(tx, status).await
	}

	async fn publish_module(
		&self,
		bytecode: Vec<u8>,
		gas_limit: u32,
		status: TxStatus,
	) -> anyhow::Result<TxInfo> {
		let tx = api::tx()
			.move_module()
			.publish_module(bytecode, gas_limit);

		self.send_tx(tx, status).await
	}

	async fn publish_module_bundle(
		&self,
		bundle: Vec<u8>,
		gas_limit: u32,
		status: TxStatus,
	) -> anyhow::Result<TxInfo> {
		let tx = api::tx()
			.move_module()
			.publish_module_bundle(bundle, gas_limit);

		self.send_tx(tx, status).await
	}

	async fn update_stdlib_bundle(
		&self,
		stdlib: Vec<u8>,
		status: TxStatus,
	) -> anyhow::Result<TxInfo> {
		let tx = api::tx()
			.move_module()
			.update_stdlib_bundle(stdlib);

		self.send_tx(tx, status).await
	}
}
