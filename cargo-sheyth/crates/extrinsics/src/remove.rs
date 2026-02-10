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
    events::CodeRemoved,
    submit_extrinsic,
    ContractMessageTranscoder,
    ErrorVariant,
};
use crate::{
    extrinsic_calls::RemoveCode,
    extrinsic_opts::ExtrinsicOpts,
};

use anyhow::Result;
use ink_env::Environment;
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

/// A builder for the remove command.
pub struct RemoveCommandBuilder<C: Config, E: Environment, Signer: Clone> {
    code_hash: Option<C::Hash>,
    extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
}

impl<C: Config, E: Environment, Signer> RemoveCommandBuilder<C, E, Signer>
where
    Signer: tx::Signer<C> + Clone,
{
    /// Returns a clean builder for [`RemoveExec`].
    pub fn new(
        extrinsic_opts: ExtrinsicOpts<C, E, Signer>,
    ) -> RemoveCommandBuilder<C, E, Signer> {
        RemoveCommandBuilder {
            code_hash: None,
            extrinsic_opts,
        }
    }

    /// Sets the hash of the smart contract code already uploaded to the chain.
    pub fn code_hash(self, code_hash: Option<C::Hash>) -> Self {
        let mut this = self;
        this.code_hash = code_hash;
        this
    }
}

impl<C: Config, E: Environment, Signer> RemoveCommandBuilder<C, E, Signer>
where
    C::Hash: From<[u8; 32]>,
    Signer: tx::Signer<C> + Clone,
{
    /// Preprocesses contract artifacts and options for subsequent removal of contract
    /// code.
    ///
    /// This function prepares the necessary data for removing contract code based on the
    /// provided contract artifacts and options. It ensures that the required code hash is
    /// available and sets up the client, signer, and other relevant parameters for the
    /// contract code removal operation.
    ///
    /// Returns the `RemoveExec` containing the preprocessed data for the contract code
    /// removal, or an error in case of failure.
    pub async fn done(self) -> Result<RemoveExec<C, E, Signer>> {
        let artifacts = self.extrinsic_opts.contract_artifacts()?;
        let transcoder = artifacts.contract_transcoder()?;

        let artifacts_path = artifacts.artifact_path().to_path_buf();

        let final_code_hash = match (self.code_hash.as_ref(), artifacts.code.as_ref()) {
            (Some(code_h), _) => Ok(*code_h),
            (None, Some(_)) => artifacts.code_hash().map(|h| h.into() ),
            (None, None) => Err(anyhow::anyhow!(
                "No code_hash was provided or contract code was not found from artifact \
                file {}. Please provide a code hash with --code-hash argument or specify the \
                path for artifacts files with --manifest-path",
                artifacts_path.display()
            )),
        }?;

        let url = self.extrinsic_opts.url();
        let rpc_cli = RpcClient::from_url(&url).await?;
        let client = OnlineClient::<C>::from_rpc_client(rpc_cli.clone()).await?;
        let rpc = LegacyRpcMethods::<C>::new(rpc_cli);

        Ok(RemoveExec {
            final_code_hash,
            opts: self.extrinsic_opts,
            rpc,
            client,
            transcoder,
        })
    }
}

pub struct RemoveExec<C: Config, E: Environment, Signer: Clone> {
    final_code_hash: C::Hash,
    opts: ExtrinsicOpts<C, E, Signer>,
    rpc: LegacyRpcMethods<C>,
    client: OnlineClient<C>,
    transcoder: ContractMessageTranscoder,
}

impl<C: Config, E: Environment, Signer> RemoveExec<C, E, Signer>
where
    C::Hash: IntoVisitor + EncodeAsType,
    C::AccountId: IntoVisitor,
    <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    Signer: tx::Signer<C> + Clone,
{
    /// Removes a contract code from the blockchain.
    ///
    /// This function removes a contract code with the specified code hash from the
    /// blockchain, ensuring that it's no longer available for instantiation or
    /// execution. It interacts with the blockchain's runtime API to execute the
    /// removal operation and provides the resulting events from the removal.
    ///
    /// Returns the `RemoveResult` containing the events generated from the contract
    /// code removal, or an error in case of failure.
    pub async fn remove_code(&self) -> Result<RemoveResult<C, E>, ErrorVariant>
    where
        E::Balance: IntoVisitor + Into<u128>,
    {
        let code_hash = self.final_code_hash;

        let call = RemoveCode::new(code_hash).build();

        let events =
            submit_extrinsic(&self.client, &self.rpc, &call, self.opts.signer()).await?;

        let code_removed =
            events.find_first::<CodeRemoved<C::Hash, C::AccountId, E::Balance>>()?;
        Ok(RemoveResult {
            code_removed,
            events,
        })
    }

    /// Returns the final code hash.
    pub fn final_code_hash(&self) -> C::Hash {
        self.final_code_hash
    }

    /// Returns the extrinsic options.
    pub fn opts(&self) -> &ExtrinsicOpts<C, E, Signer> {
        &self.opts
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

/// A struct representing the result of an remove command execution.
pub struct RemoveResult<C: Config, E: Environment> {
    pub code_removed: Option<CodeRemoved<C::Hash, C::AccountId, E::Balance>>,
    pub events: ExtrinsicEvents<C>,
}
