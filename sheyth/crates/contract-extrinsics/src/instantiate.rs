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

use super::{
    events::{
        CodeStored,
        ContractInstantiated,
    },
    pallet_contracts_primitives::{
        ContractInstantiateResult,
        StorageDeposit,
    },
    state_call,
    submit_extrinsic,
    ContractMessageTranscoder,
    ErrorVariant,
};
use crate::{
    check_env_types,
    extrinsic_calls::{
        Instantiate,
        InstantiateWithCode,
    },
    extrinsic_opts::ExtrinsicOpts,
};
use anyhow::{
    anyhow,
    Context,
    Result,
};
use contract_transcode::Value;
use ink_env::Environment;
use serde::Serialize;

use scale::{
    Decode,
    Encode,
};
use sp_core::Bytes;
use sp_weights::Weight;
use std::fmt::Display;
use subxt::{
    backend::{
        legacy::LegacyRpcMethods,
        rpc::RpcClient,
    },
    blocks::ExtrinsicEvents,
    config::{
        DefaultExtrinsicParams,
        ExtrinsicParams,
    },
    ext::{
        scale_decode::IntoVisitor,
        scale_encode::EncodeAsType,
    },
    tx,
    Config,
    OnlineClient,
};

/// A builder for the instantiate command.
pub struct InstantiateCommandBuilder<C: Config, E: Environment, Signer: Clone> {
    constructor: String,
    args: Vec<String>,
    extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
    value: E::Balance,
    gas_limit: Option<u64>,
    proof_size: Option<u64>,
    salt: Option<Bytes>,
}

impl<C: Config, E: Environment, Signer> InstantiateCommandBuilder<C, E, Signer>
where
    E::Balance: Default,
    Signer: tx::Signer<C> + Clone,
    C::Hash: From<[u8; 32]>,
{
    /// Returns a clean builder for [`InstantiateExec`].
    pub fn new(
        extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
    ) -> InstantiateCommandBuilder<C, E, Signer> {
        InstantiateCommandBuilder {
            constructor: String::from("new"),
            args: Vec::new(),
            extrinsic_opts,
            value: Default::default(),
            gas_limit: None,
            proof_size: None,
            salt: None,
        }
    }

    /// Sets the name of the contract constructor to call.
    pub fn constructor<T: Into<String>>(self, constructor: T) -> Self {
        let mut this = self;
        this.constructor = constructor.into();
        this
    }

    /// Sets the constructor arguments.
    pub fn args<T: ToString>(self, args: Vec<T>) -> Self {
        let mut this = self;
        this.args = args.into_iter().map(|arg| arg.to_string()).collect();
        this
    }

    /// Sets the initial balance to transfer to the instantiated contract.
    pub fn value(self, value: E::Balance) -> Self {
        let mut this = self;
        this.value = value;
        this
    }

    /// Sets the maximum amount of gas to be used for this command.
    pub fn gas_limit(self, gas_limit: Option<u64>) -> Self {
        let mut this = self;
        this.gas_limit = gas_limit;
        this
    }

    /// Sets the maximum proof size for this instantiation.
    pub fn proof_size(self, proof_size: Option<u64>) -> Self {
        let mut this = self;
        this.proof_size = proof_size;
        this
    }

    /// Sets the salt used in the address derivation of the new contract.
    pub fn salt(self, salt: Option<Bytes>) -> Self {
        let mut this = self;
        this.salt = salt;
        this
    }

    /// Preprocesses contract artifacts and options for instantiation.
    ///
    /// This function prepares the required data for instantiating a contract based on the
    /// provided contract artifacts and options. It ensures that the necessary contract
    /// code is available, sets up the client, signer, and other relevant parameters,
    /// preparing for the instantiation process.
    ///
    /// Returns the [`InstantiateExec`] containing the preprocessed data for the
    /// instantiation, or an error in case of failure.
    pub async fn done(self) -> Result<InstantiateExec<C, E, Signer>> {
        let artifacts = self.extrinsic_opts.contract_artifacts()?;
        let transcoder = artifacts.contract_transcoder()?;
        let data = transcoder.encode(&self.constructor, &self.args)?;
        let url = self.extrinsic_opts.url();
        let code = if let Some(code) = artifacts.code {
            Code::Upload(code.0)
        } else {
            let code_hash = artifacts.code_hash()?;
            Code::Existing(code_hash.into())
        };
        let salt = self.salt.clone().map(|s| s.0).unwrap_or_default();

        let rpc_cli = RpcClient::from_url(&url).await?;
        let client = OnlineClient::from_rpc_client(rpc_cli.clone()).await?;
        check_env_types(&client, &transcoder, self.extrinsic_opts.verbosity())?;
        let rpc = LegacyRpcMethods::new(rpc_cli);

        let args = InstantiateArgs {
            constructor: self.constructor.clone(),
            raw_args: self.args.clone(),
            value: self.value,
            gas_limit: self.gas_limit,
            proof_size: self.proof_size,
            storage_deposit_limit: self.extrinsic_opts.storage_deposit_limit(),
            code,
            data,
            salt,
        };

        Ok(InstantiateExec {
            args,
            opts: self.extrinsic_opts,
            rpc,
            client,
            transcoder,
        })
    }
}

pub struct InstantiateArgs<C: Config, E: Environment> {
    constructor: String,
    raw_args: Vec<String>,
    value: E::Balance,
    gas_limit: Option<u64>,
    proof_size: Option<u64>,
    storage_deposit_limit: Option<E::Balance>,
    code: Code<C::Hash>,
    data: Vec<u8>,
    salt: Vec<u8>,
}

impl<C: Config, E: Environment> InstantiateArgs<C, E> {
    /// Returns the constructor name.
    pub fn constructor(&self) -> &str {
        &self.constructor
    }

    /// Returns the constructor raw arguments.
    pub fn raw_args(&self) -> &[String] {
        &self.raw_args
    }

    /// Returns the value to transfer to the instantiated contract.
    pub fn value(&self) -> E::Balance {
        self.value
    }

    /// Returns the maximum amount of gas to be used for this command.
    pub fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    /// Returns the maximum proof size for this instantiation.
    pub fn proof_size(&self) -> Option<u64> {
        self.proof_size
    }

    /// Returns the storage deposit limit for this instantiation.
    pub fn storage_deposit_limit_compact(&self) -> Option<scale::Compact<E::Balance>> {
        self.storage_deposit_limit.map(Into::into)
    }

    pub fn code(&self) -> &Code<C::Hash> {
        &self.code
    }

    /// Returns the input data for the contract constructor.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the salt used in the address derivation of the new contract.
    pub fn salt(&self) -> &[u8] {
        &self.salt
    }
}

pub struct InstantiateExec<C: Config, E: Environment, Signer: Clone> {
    opts: ExtrinsicOpts<C, E, Signer>,
    args: InstantiateArgs<C, E>,
    rpc: LegacyRpcMethods<C>,
    client: OnlineClient<C>,
    transcoder: ContractMessageTranscoder,
}

impl<C: Config, E: Environment, Signer> InstantiateExec<C, E, Signer>
where
    C::AccountId: Decode,
    <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    C::Hash: IntoVisitor + EncodeAsType,
    C::AccountId: IntoVisitor + Display,
    E::Balance: Serialize + EncodeAsType,
    Signer: tx::Signer<C> + Clone,
{
    /// Decodes the result of a simulated contract instantiation.
    ///
    /// This function decodes the result of a simulated contract instantiation dry run.
    /// It processes the returned data, including the constructor's return value, contract
    /// address, gas consumption, and storage deposit, and packages them into an
    /// [`InstantiateDryRunResult`].
    ///
    /// Returns the decoded dry run result, or an error in case of failure.
    pub async fn decode_instantiate_dry_run(
        &self,
        result: &ContractInstantiateResult<C::AccountId, E::Balance>,
    ) -> Result<InstantiateDryRunResult<E::Balance>, ErrorVariant> {
        tracing::debug!("instantiate data {:?}", self.args.data);
        match result.result {
            Ok(ref ret_val) => {
                let value = self
                    .transcoder
                    .decode_constructor_return(
                        &self.args.constructor,
                        &mut &ret_val.result.data[..],
                    )
                    .context(format!("Failed to decode return value {:?}", &ret_val))?;
                let dry_run_result = InstantiateDryRunResult {
                    result: value,
                    contract: ret_val.account_id.to_string(),
                    reverted: ret_val.result.did_revert(),
                    gas_consumed: result.gas_consumed,
                    gas_required: result.gas_required,
                    storage_deposit: result.storage_deposit.clone(),
                };
                Ok(dry_run_result)
            }
            Err(ref err) => {
                let metadata = self.client.metadata();
                Err(ErrorVariant::from_dispatch_error(err, &metadata)?)
            }
        }
    }

    /// Simulates a contract instantiation without modifying the blockchain.
    ///
    /// This function performs a dry run simulation of a contract instantiation, capturing
    /// essential information such as the contract address, gas consumption, and storage
    /// deposit. The simulation is executed without actually executing the
    /// instantiation on the blockchain.
    ///
    /// Returns the dry run simulation result, or an error in case of failure.
    pub async fn instantiate_dry_run(
        &self,
    ) -> Result<ContractInstantiateResult<C::AccountId, E::Balance>> {
        let storage_deposit_limit = self.args.storage_deposit_limit;
        let call_request = InstantiateRequest::<C, E> {
            origin: self.opts.signer().account_id(),
            value: self.args.value,
            gas_limit: None,
            storage_deposit_limit,
            code: self.args.code.clone(),
            data: self.args.data.clone(),
            salt: self.args.salt.clone(),
        };
        state_call(&self.rpc, "ContractsApi_instantiate", &call_request).await
    }

    async fn instantiate_with_code(
        &self,
        code: Vec<u8>,
        gas_limit: Weight,
    ) -> Result<InstantiateExecResult<C>, ErrorVariant> {
        let call = InstantiateWithCode::new(
            self.args.value,
            gas_limit,
            self.args.storage_deposit_limit,
            code,
            self.args.data.clone(),
            self.args.salt.clone(),
        )
        .build();

        let events =
            submit_extrinsic(&self.client, &self.rpc, &call, self.opts.signer()).await?;

        // The CodeStored event is only raised if the contract has not already been
        // uploaded.
        let code_hash = events
            .find_first::<CodeStored<C::Hash>>()?
            .map(|code_stored| code_stored.code_hash);

        let instantiated = events
            .find_last::<ContractInstantiated<C::AccountId>>()?
            .ok_or_else(|| anyhow!("Failed to find Instantiated event"))?;

        Ok(InstantiateExecResult {
            events,
            code_hash,
            contract_address: instantiated.contract,
        })
    }

    async fn instantiate_with_code_hash(
        &self,
        code_hash: C::Hash,
        gas_limit: Weight,
    ) -> Result<InstantiateExecResult<C>, ErrorVariant> {
        let call = Instantiate::<C::Hash, E::Balance>::new(
            self.args.value,
            gas_limit,
            self.args.storage_deposit_limit,
            code_hash,
            self.args.data.clone(),
            self.args.salt.clone(),
        )
        .build();

        let events =
            submit_extrinsic(&self.client, &self.rpc, &call, self.opts.signer()).await?;

        let instantiated = events
            .find_first::<ContractInstantiated<C::AccountId>>()?
            .ok_or_else(|| anyhow!("Failed to find Instantiated event"))?;

        Ok(InstantiateExecResult {
            events,
            code_hash: None,
            contract_address: instantiated.contract,
        })
    }

    /// Initiates the deployment of a smart contract on the blockchain.
    ///
    /// This function can be used to deploy a contract using either its source code or an
    /// existing code hash. It triggers the instantiation process by submitting an
    /// extrinsic with the specified gas limit, storage deposit, code or code hash,
    /// input data, and salt.
    ///
    /// The deployment result provides essential information about the instantiation,
    /// encapsulated in an [`InstantiateExecResult`] object, including the contract's
    /// result, contract address, and token metadata.
    pub async fn instantiate(
        &self,
        gas_limit: Option<Weight>,
    ) -> Result<InstantiateExecResult<C>, ErrorVariant> {
        // use user specified values where provided, otherwise estimate
        let gas_limit = match gas_limit {
            Some(gas_limit) => gas_limit,
            None => self.estimate_gas().await?,
        };
        match self.args.code.clone() {
            Code::Upload(code) => self.instantiate_with_code(code, gas_limit).await,
            Code::Existing(code_hash) => {
                self.instantiate_with_code_hash(code_hash, gas_limit).await
            }
        }
    }

    /// Estimates the gas required for the contract instantiation process without
    /// modifying the blockchain.
    ///
    /// This function provides a gas estimation for contract instantiation, considering
    /// the user-specified values or using estimates based on a dry run.
    ///
    /// Returns the estimated gas weight of type [`Weight`] for contract instantiation, or
    /// an error.
    pub async fn estimate_gas(&self) -> Result<Weight> {
        match (self.args.gas_limit, self.args.proof_size) {
            (Some(ref_time), Some(proof_size)) => {
                Ok(Weight::from_parts(ref_time, proof_size))
            }
            _ => {
                let instantiate_result = self.instantiate_dry_run().await?;
                match instantiate_result.result {
                    Ok(_) => {
                        // use user specified values where provided, otherwise use the
                        // estimates
                        let ref_time = self.args.gas_limit.unwrap_or_else(|| {
                            instantiate_result.gas_required.ref_time()
                        });
                        let proof_size = self.args.proof_size.unwrap_or_else(|| {
                            instantiate_result.gas_required.proof_size()
                        });
                        Ok(Weight::from_parts(ref_time, proof_size))
                    }
                    Err(ref err) => {
                        let object = ErrorVariant::from_dispatch_error(
                            err,
                            &self.client.metadata(),
                        )?;
                        Err(anyhow!("Pre-submission dry-run failed. Error: {}", object))
                    }
                }
            }
        }
    }

    /// Returns the extrinsic options.
    pub fn opts(&self) -> &ExtrinsicOpts<C, E, Signer> {
        &self.opts
    }

    /// Returns the instantiate arguments.
    pub fn args(&self) -> &InstantiateArgs<C, E> {
        &self.args
    }

    /// Returns the client.
    pub fn client(&self) -> &OnlineClient<C> {
        &self.client
    }

    /// Returns the contract message transcoder.
    pub fn transcoder(&self) -> &ContractMessageTranscoder {
        &self.transcoder
    }
}

/// A struct representing the result of an instantiate command execution.
pub struct InstantiateExecResult<C: Config> {
    pub events: ExtrinsicEvents<C>,
    pub code_hash: Option<C::Hash>,
    pub contract_address: C::AccountId,
}

/// Result of the contract call
#[derive(serde::Serialize)]
pub struct InstantiateDryRunResult<Balance: Serialize> {
    /// The decoded result returned from the constructor
    pub result: Value,
    /// contract address
    pub contract: String,
    /// Was the operation reverted
    pub reverted: bool,
    pub gas_consumed: Weight,
    pub gas_required: Weight,
    /// Storage deposit after the operation
    pub storage_deposit: StorageDeposit<Balance>,
}

impl<Balance: Serialize> InstantiateDryRunResult<Balance> {
    /// Returns a result in json format
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// A struct that encodes RPC parameters required to instantiate a new smart contract.
#[derive(Encode)]
struct InstantiateRequest<C: Config, E: Environment> {
    origin: C::AccountId,
    value: E::Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<E::Balance>,
    code: Code<C::Hash>,
    data: Vec<u8>,
    salt: Vec<u8>,
}

/// Reference to an existing code hash or a new Wasm module.
#[derive(Clone, Encode)]
pub enum Code<Hash>
where
    Hash: Clone,
{
    /// A Wasm module as raw bytes.
    Upload(Vec<u8>),
    /// The code hash of an on-chain Wasm blob.
    Existing(Hash),
}
