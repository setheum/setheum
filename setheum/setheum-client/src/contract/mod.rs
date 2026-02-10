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

mod convertible_value;
pub mod event;

use std::fmt::{Debug, Formatter};

use anyhow::{anyhow, Context, Result};
use contract_transcode::ContractMessageTranscoder;
pub use convertible_value::ConvertibleValue;
use log::info;
use pallet_contracts_primitives::ContractExecResult;

use crate::{
    connections::TxInfo,
    contract_transcode::Value,
    pallets::contract::{ContractCallArgs, ContractRpc, ContractsUserApi, EventRecord},
    sp_weights::weight_v2::Weight,
    AccountId, Balance, ConnectionApi, SignedConnectionApi, TxStatus,
};

/// Default gas limit, which allows up to 25% of block time (62.5% of the actual block capacity).
pub const DEFAULT_MAX_GAS: u64 = 250_000_000_000u64;
/// Default proof size limit, which allows up to 25% of block time (62.5% of the actual block
/// capacity).
pub const DEFAULT_MAX_PROOF_SIZE: u64 = 250_000_000_000u64;

/// Represents a contract instantiated on the chain.
pub struct ContractInstance {
    address: AccountId,
    transcoder: ContractMessageTranscoder,
    max_gas_override: Option<u64>,
    max_proof_size_override: Option<u64>,
}

impl ContractInstance {
/// Creates a new contract instance under `address` with metadata read from `metadata_path`.
    pub fn new(address: AccountId, metadata_path: &str) -> Result<Self> {
        Ok(Self {
            address,
            transcoder: ContractMessageTranscoder::load(metadata_path)?,
            max_gas_override: None,
            max_proof_size_override: None,
        })
    }

/// From now on, the contract instance will use `limit_override` as the gas limit for all
/// contract calls. If `limit_override` is `None`, then [DEFAULT_MAX_GAS] will be used.
    pub fn override_gas_limit(&mut self, limit_override: Option<u64>) {
        self.max_gas_override = limit_override;
    }

/// From now on, the contract instance will use `limit_override` as the proof size limit for all
/// contract calls. If `limit_override` is `None`, then [DEFAULT_MAX_PROOF_SIZE] will be used.
    pub fn override_proof_size_limit(&mut self, limit_override: Option<u64>) {
        self.max_proof_size_override = limit_override;
    }

/// The address of this contract instance.
    pub fn address(&self) -> &AccountId {
        &self.address
    }

/// Reads the value of a read-only, 0-argument call via RPC.
    pub async fn contract_read0<
        T: TryFrom<ConvertibleValue, Error = anyhow::Error>,
        C: ConnectionApi,
    >(
        &self,
        conn: &C,
        message: &str,
    ) -> Result<T> {
        self.contract_read::<String, T, C>(conn, message, &[]).await
    }

/// Reads the value of a read-only call via RPC.
    pub async fn contract_read<
        S: AsRef<str> + Debug,
        T: TryFrom<ConvertibleValue, Error = anyhow::Error>,
        C: ConnectionApi,
    >(
        &self,
        conn: &C,
        message: &str,
        args: &[S],
    ) -> Result<T> {
        self.contract_read_as(conn, message, args, self.address.clone())
            .await
    }

/// Reads the value of a contract call via RPC as if it was executed by `sender`.
    pub async fn contract_read_as<
        S: AsRef<str> + Debug,
        T: TryFrom<ConvertibleValue, Error = anyhow::Error>,
        C: ConnectionApi,
    >(
        &self,
        conn: &C,
        message: &str,
        args: &[S],
        sender: AccountId,
    ) -> Result<T> {
        let result = self
            .dry_run(conn, message, args, sender, 0)
            .await?
            .result
            .map_err(|e| anyhow!("Contract exec failed {:?}", e))?;

        let decoded = self.decode(message, result.data)?;
        ConvertibleValue(decoded).try_into()?
    }

/// Executes a 0-argument contract call.
    pub async fn contract_exec0<C: SignedConnectionApi>(
        &self,
        conn: &C,
        message: &str,
    ) -> Result<TxInfo> {
        self.contract_exec::<C, String>(conn, message, &[]).await
    }

/// Executes a contract call.
    pub async fn contract_exec<C: SignedConnectionApi, S: AsRef<str> + Debug>(
        &self,
        conn: &C,
        message: &str,
        args: &[S],
    ) -> Result<TxInfo> {
        self.contract_exec_value::<C, S>(conn, message, args, 0)
            .await
    }

/// Executes a 0-argument contract call sending the given amount of value with it.
    pub async fn contract_exec_value0<C: SignedConnectionApi>(
        &self,
        conn: &C,
        message: &str,
        value: Balance,
    ) -> Result<TxInfo> {
        self.contract_exec_value::<C, String>(conn, message, &[], value)
            .await
    }

/// Executes a contract call sending the given amount of value with it.
    pub async fn contract_exec_value<C: SignedConnectionApi, S: AsRef<str> + Debug>(
        &self,
        conn: &C,
        message: &str,
        args: &[S],
        value: Balance,
    ) -> Result<TxInfo> {
        let dry_run_result = self
            .dry_run(conn, message, args, conn.account_id().clone(), value)
            .await?;

        let data = self.encode(message, args)?;
        conn.call(
            self.address.clone(),
            value,
            Weight {
                ref_time: dry_run_result.gas_required.ref_time(),
                proof_size: dry_run_result.gas_required.proof_size(),
            },
            None,
            data,
            TxStatus::Finalized,
        )
        .await
    }

    fn encode<S: AsRef<str> + Debug>(&self, message: &str, args: &[S]) -> Result<Vec<u8>> {
        self.transcoder.encode(message, args)
    }

    fn decode(&self, message: &str, data: Vec<u8>) -> Result<Value> {
        self.transcoder.decode_return(message, &mut data.as_slice())
    }

    async fn dry_run<S: AsRef<str> + Debug, C: ConnectionApi>(
        &self,
        conn: &C,
        message: &str,
        args: &[S],
        sender: AccountId,
        value: Balance,
    ) -> Result<ContractExecResult<Balance, EventRecord>> {
        let payload = self.encode(message, args)?;
        let args = ContractCallArgs {
            origin: sender,
            dest: self.address.clone(),
            value,
            gas_limit: None,
            input_data: payload,
            storage_deposit_limit: None,
        };

        let contract_read_result = conn
            .call_and_get(args)
            .await
            .context("RPC request error - there may be more info in node logs.")?;

        if !contract_read_result.debug_message.is_empty() {
            info!(
                target: "setheum_client::contract",
                "Dry-run debug messages: {:?}",
                core::str::from_utf8(&contract_read_result.debug_message)
                    .unwrap_or("<Invalid UTF8>")
                    .split('\n')
                    .filter(|m| !m.is_empty())
                    .collect::<Vec<_>>()
            );
        }

// For dry run, failed transactions don't return `Err` but `Ok(_)`
// and we have to inspect flags manually.
        if let Ok(res) = &contract_read_result.result {
            if res.did_revert() {
                return Err(anyhow!(
                    "Dry-run call reverted, decoded result: {:?}",
                    self.decode(message, res.data.clone())
                ));
            }
        }

        Ok(contract_read_result)
    }
}

impl Debug for ContractInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContractInstance")
            .field("address", &self.address)
            .finish()
    }
}
