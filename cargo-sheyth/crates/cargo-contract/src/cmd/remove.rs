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
    call_with_config,
    ErrorVariant,
};
use std::{
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use super::{
    config::SignerConfig,
    parse_balance,
    parse_code_hash,
    CLIExtrinsicOpts,
};
use anyhow::Result;
use contract_build::name_value_println;
use contract_extrinsics::{
    DisplayEvents,
    ExtrinsicOptsBuilder,
    RemoveCommandBuilder,
    RemoveExec,
    TokenMetadata,
};
use ink_env::Environment;
use serde::Serialize;
use subxt::{
    config::{
        DefaultExtrinsicParams,
        ExtrinsicParams,
    },
    ext::{
        scale_decode::IntoVisitor,
        scale_encode::EncodeAsType,
    },
    Config,
};

#[derive(Debug, clap::Args)]
#[clap(name = "remove", about = "Remove a contract's code")]
pub struct RemoveCommand {
    /// The hash of the smart contract code already uploaded to the chain.
    #[clap(long)]
    code_hash: Option<String>,
    #[clap(flatten)]
    extrinsic_cli_opts: CLIExtrinsicOpts,
    /// Export the call output as JSON.
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
}

impl RemoveCommand {
    /// Returns whether to export the call output in JSON format.
    pub fn output_json(&self) -> bool {
        self.output_json
    }

    pub async fn handle(&self) -> Result<(), ErrorVariant> {
        call_with_config!(
            self,
            run,
            self.extrinsic_cli_opts.chain_cli_opts.chain().config()
        )
    }

    async fn run<C: Config + Environment + SignerConfig<C>>(
        &self,
    ) -> Result<(), ErrorVariant>
    where
        <C as Config>::AccountId: IntoVisitor + FromStr + EncodeAsType,
        <<C as Config>::AccountId as FromStr>::Err: Display,
        C::Balance: Into<u128>
            + From<u128>
            + Display
            + Default
            + FromStr
            + Serialize
            + Debug
            + IntoVisitor,
        <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
            From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
        <C as Config>::Hash: IntoVisitor + EncodeAsType + From<[u8; 32]>,
    {
        let signer = C::Signer::from_str(&self.extrinsic_cli_opts.suri)
            .map_err(|_| anyhow::anyhow!("Failed to parse suri option"))?;
        let chain = self.extrinsic_cli_opts.chain_cli_opts.chain();
        let token_metadata = TokenMetadata::query::<C>(&chain.url()).await?;
        let storage_deposit_limit = self
            .extrinsic_cli_opts
            .storage_deposit_limit
            .clone()
            .map(|b| parse_balance(&b, &token_metadata))
            .transpose()
            .map_err(|e| {
                anyhow::anyhow!("Failed to parse storage_deposit_limit option: {}", e)
            })?;
        let code_hash = self
            .code_hash
            .clone()
            .map(|h| parse_code_hash(&h))
            .transpose()
            .map_err(|e| anyhow::anyhow!("Failed to parse code_hash option: {}", e))?;
        let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
            .file(self.extrinsic_cli_opts.file.clone())
            .manifest_path(self.extrinsic_cli_opts.manifest_path.clone())
            .url(chain.url())
            .storage_deposit_limit(storage_deposit_limit)
            .done();

        let remove_exec: RemoveExec<C, C, _> = RemoveCommandBuilder::new(extrinsic_opts)
            .code_hash(code_hash)
            .done()
            .await?;
        let remove_result = remove_exec.remove_code().await?;
        let display_events = DisplayEvents::from_events::<C, C>(
            &remove_result.events,
            Some(remove_exec.transcoder()),
            &remove_exec.client().metadata(),
        )?;

        let output_events = if self.output_json() {
            display_events.to_json()?
        } else {
            display_events.display_events::<C>(
                self.extrinsic_cli_opts.verbosity().unwrap(),
                &token_metadata,
            )?
        };
        if let Some(code_removed) = remove_result.code_removed {
            let remove_result: <C as Config>::Hash = code_removed.code_hash;

            if self.output_json() {
                // Create a JSON object with the events and the removed code hash.
                let json_object = serde_json::json!({
                    "events": serde_json::from_str::<serde_json::Value>(&output_events)?,
                    "code_hash": remove_result,
                });
                let json_object = serde_json::to_string_pretty(&json_object)?;
                println!("{}", json_object);
            } else {
                println!("{}", output_events);
                name_value_println!("Code hash", format!("{remove_result:?}"));
            }
            Result::<(), ErrorVariant>::Ok(())
        } else {
            let error_code_hash = hex::encode(remove_exec.final_code_hash());
            Err(anyhow::anyhow!(
                "Error removing the code for the supplied code hash: {}",
                error_code_hash
            )
            .into())
        }
    }
}
