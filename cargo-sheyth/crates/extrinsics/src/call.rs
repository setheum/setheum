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
    pallet_contracts_primitives::ContractExecResult,
    state_call,
    submit_extrinsic,
    ContractMessageTranscoder,
    ErrorVariant,
};
use crate::{
    check_env_types,
    extrinsic_calls::Call,
    extrinsic_opts::ExtrinsicOpts,
};

use anyhow::{
    anyhow,
    Result,
};
use ink_env::Environment;
use scale::Encode;
use sp_weights::Weight;

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

/// A builder for the call command.
pub struct CallCommandBuilder<C: Config, E: Environment, Signer: Clone> {
    contract: C::AccountId,
    message: String,
    args: Vec<String>,
    extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
    gas_limit: Option<u64>,
    proof_size: Option<u64>,
    value: E::Balance,
}

impl<C: Config, E: Environment, Signer> CallCommandBuilder<C, E, Signer>
where
    E::Balance: Default,
    Signer: tx::Signer<C> + Clone,
{
    /// Returns a clean builder for [`CallExec`].
    pub fn new(
        contract: C::AccountId,
        message: &str,
        extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
    ) -> CallCommandBuilder<C, E, Signer> {
        CallCommandBuilder {
            contract,
            message: message.to_string(),
            args: Vec::new(),
            extrinsic_opts,
            gas_limit: None,
            proof_size: None,
            value: Default::default(),
        }
    }

    /// Sets the arguments of the contract message to call.
    pub fn args<T: ToString>(self, args: Vec<T>) -> Self {
        let mut this = self;
        this.args = args.into_iter().map(|arg| arg.to_string()).collect();
        this
    }

    /// Sets the maximum amount of gas to be used for this command.
    pub fn gas_limit(self, gas_limit: Option<u64>) -> Self {
        let mut this = self;
        this.gas_limit = gas_limit;
        this
    }

    /// Sets the maximum proof size for this call.
    pub fn proof_size(self, proof_size: Option<u64>) -> Self {
        let mut this = self;
        this.proof_size = proof_size;
        this
    }

    /// Sets the value to be transferred as part of the call.
    pub fn value(self, value: E::Balance) -> Self {
        let mut this = self;
        this.value = value;
        this
    }

    /// Preprocesses contract artifacts and options for subsequent contract calls.
    ///
    /// This function prepares the necessary data for making a contract call based on the
    /// provided contract artifacts, message, arguments, and options. It ensures that the
    /// required contract code and message data are available, sets up the client,
    /// and other relevant parameters, preparing for the contract call operation.
    ///
    /// Returns the `CallExec` containing the preprocessed data for the contract call,
    /// or an error in case of failure.
    pub async fn done(self) -> Result<CallExec<C, E, Signer>> {
        let artifacts = self.extrinsic_opts.contract_artifacts()?;
        let transcoder = artifacts.contract_transcoder()?;

        let call_data = transcoder.encode(&self.message, &self.args)?;
        tracing::debug!("Message data: {:?}", hex::encode(&call_data));

        let url = self.extrinsic_opts.url();
        let rpc = RpcClient::from_url(&url).await?;
        let client = OnlineClient::from_rpc_client(rpc.clone()).await?;
        let rpc = LegacyRpcMethods::new(rpc);
        check_env_types(&client, &transcoder, self.extrinsic_opts.verbosity())?;

        Ok(CallExec {
            contract: self.contract,
            message: self.message.clone(),
            args: self.args.clone(),
            opts: self.extrinsic_opts,
            gas_limit: self.gas_limit,
            proof_size: self.proof_size,
            value: self.value,
            rpc,
            client,
            transcoder,
            call_data,
        })
    }
}

pub struct CallExec<C: Config, E: Environment, Signer: Clone> {
    contract: C::AccountId,
    message: String,
    args: Vec<String>,
    opts: ExtrinsicOpts<C, E, Signer>,
    gas_limit: Option<u64>,
    proof_size: Option<u64>,
    value: E::Balance,
    rpc: LegacyRpcMethods<C>,
    client: OnlineClient<C>,
    transcoder: ContractMessageTranscoder,
    call_data: Vec<u8>,
}

impl<C: Config, E: Environment, Signer> CallExec<C, E, Signer>
where
    <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    C::AccountId: EncodeAsType + IntoVisitor,
    E::Balance: EncodeAsType,
    Signer: tx::Signer<C> + Clone,
{
    /// Simulates a contract call without modifying the blockchain.
    ///
    /// This function performs a dry run simulation of a contract call, capturing
    /// essential information such as the contract address, gas consumption, and
    /// storage deposit. The simulation is executed without actually executing the
    /// call on the blockchain.
    ///
    /// Returns the dry run simulation result of type [`ContractExecResult`], which
    /// includes information about the simulated call, or an error in case of failure.
    pub async fn call_dry_run(&self) -> Result<ContractExecResult<E::Balance>> {
        let storage_deposit_limit = self.opts.storage_deposit_limit();
        let call_request = CallRequest {
            origin: self.opts.signer().account_id(),
            dest: self.contract.clone(),
            value: self.value,
            gas_limit: None,
            storage_deposit_limit,
            input_data: self.call_data.clone(),
        };
        state_call(&self.rpc, "ContractsApi_call", call_request).await
    }

    /// Calls a contract on the blockchain with a specified gas limit.
    ///
    /// This function facilitates the process of invoking a contract, specifying the gas
    /// limit for the operation. It interacts with the blockchain's runtime API to
    /// execute the contract call and provides the resulting events from the call.
    ///
    /// Returns the events generated from the contract call, or an error in case of
    /// failure.
    pub async fn call(
        &self,
        gas_limit: Option<Weight>,
    ) -> Result<ExtrinsicEvents<C>, ErrorVariant> {
        if !self
            .transcoder()
            .metadata()
            .spec()
            .messages()
            .iter()
            .find(|msg| msg.label() == &self.message)
            .expect("message exist after calling CallExec::done()")
            .mutates()
        {
            let inner = anyhow!(
                "Tried to execute a call on the immutable contract message '{}'. Please do a dry-run instead.",
                &self.message
            );
            return Err(inner.into())
        }

        // use user specified values where provided, otherwise estimate
        let gas_limit = match gas_limit {
            Some(gas_limit) => gas_limit,
            None => self.estimate_gas().await?,
        };
        tracing::debug!("calling contract {:?}", self.contract);
        let storage_deposit_limit = self.opts.storage_deposit_limit();

        let call = Call::new(
            self.contract.clone().into(),
            self.value,
            gas_limit,
            storage_deposit_limit,
            self.call_data.clone(),
        )
        .build();

        let result =
            submit_extrinsic(&self.client, &self.rpc, &call, self.opts.signer()).await?;

        Ok(result)
    }

    /// Estimates the gas required for a contract call without modifying the blockchain.
    ///
    /// This function provides a gas estimation for contract calls, considering the
    /// user-specified values or using estimates based on a dry run. The estimated gas
    /// weight is returned, or an error is reported if the estimation fails.
    ///
    /// Returns the estimated gas weight of type [`Weight`] for contract calls, or an
    /// error.
    pub async fn estimate_gas(&self) -> Result<Weight> {
        match (self.gas_limit, self.proof_size) {
            (Some(ref_time), Some(proof_size)) => {
                Ok(Weight::from_parts(ref_time, proof_size))
            }
            _ => {
                let call_result = self.call_dry_run().await?;
                match call_result.result {
                    Ok(_) => {
                        // use user specified values where provided, otherwise use the
                        // estimates
                        let ref_time = self
                            .gas_limit
                            .unwrap_or_else(|| call_result.gas_required.ref_time());
                        let proof_size = self
                            .proof_size
                            .unwrap_or_else(|| call_result.gas_required.proof_size());
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

    /// Returns the address of the the contract to call.
    pub fn contract(&self) -> &C::AccountId {
        &self.contract
    }

    /// Returns the name of the contract message to call.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the arguments of the contract message to call.
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    /// Returns the extrinsic options.
    pub fn opts(&self) -> &ExtrinsicOpts<C, E, Signer> {
        &self.opts
    }

    /// Returns the maximum amount of gas to be used for this command.
    pub fn gas_limit(&self) -> Option<u64> {
        self.gas_limit
    }

    /// Returns the maximum proof size for this call.
    pub fn proof_size(&self) -> Option<u64> {
        self.proof_size
    }

    /// Returns the value to be transferred as part of the call.
    pub fn value(&self) -> &E::Balance {
        &self.value
    }

    /// Returns the client.
    pub fn client(&self) -> &OnlineClient<C> {
        &self.client
    }

    /// Returns the contract message transcoder.
    pub fn transcoder(&self) -> &ContractMessageTranscoder {
        &self.transcoder
    }

    /// Returns the call data.
    pub fn call_data(&self) -> &Vec<u8> {
        &self.call_data
    }
}

/// A struct that encodes RPC parameters required for a call to a smart contract.
///
/// Copied from `pallet-contracts-rpc-runtime-api`.
#[derive(Encode)]
struct CallRequest<AccountId, Balance> {
    origin: AccountId,
    dest: AccountId,
    value: Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<Balance>,
    input_data: Vec<u8>,
}
