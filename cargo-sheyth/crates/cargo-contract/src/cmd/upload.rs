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
    display_dry_run_result_warning,
    parse_balance,
    prompt_confirm_unverifiable_upload,
    CLIExtrinsicOpts,
};
use anyhow::Result;
use contract_build::name_value_println;
use contract_extrinsics::{
    DisplayEvents,
    ExtrinsicOptsBuilder,
    TokenMetadata,
    UploadCommandBuilder,
    UploadExec,
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
#[clap(name = "upload", about = "Upload a contract's code")]
pub struct UploadCommand {
    #[clap(flatten)]
    extrinsic_cli_opts: CLIExtrinsicOpts,
    /// Export the call output in JSON format.
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
}

impl UploadCommand {
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
            + EncodeAsType,
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
        let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
            .file(self.extrinsic_cli_opts.file.clone())
            .manifest_path(self.extrinsic_cli_opts.manifest_path.clone())
            .url(chain.url())
            .storage_deposit_limit(storage_deposit_limit)
            .done();

        let upload_exec: UploadExec<C, C, _> =
            UploadCommandBuilder::new(extrinsic_opts).done().await?;
        let code_hash = upload_exec.code().code_hash();
        let metadata = upload_exec.client().metadata();

        if !self.extrinsic_cli_opts.execute {
            match upload_exec.upload_code_rpc().await? {
                Ok(result) => {
                    let upload_result = UploadDryRunResult {
                        result: String::from("Success!"),
                        code_hash: format!("{:?}", result.code_hash),
                        deposit: result.deposit,
                    };
                    if self.output_json() {
                        println!("{}", upload_result.to_json()?);
                    } else {
                        upload_result.print();
                        display_dry_run_result_warning("upload");
                    }
                }
                Err(err) => {
                    let err = ErrorVariant::from_dispatch_error(&err, &metadata)?;
                    if self.output_json() {
                        return Err(err)
                    } else {
                        name_value_println!("Result", err);
                    }
                }
            }
        } else {
            if let Some(chain) = chain.production() {
                if !upload_exec.opts().contract_artifacts()?.is_verifiable() {
                    prompt_confirm_unverifiable_upload(&chain.to_string())?
                }
            }
            let upload_result = upload_exec.upload_code().await?;
            let display_events = DisplayEvents::from_events::<C, C>(
                &upload_result.events,
                None,
                &metadata,
            )?;
            let output_events = if self.output_json() {
                display_events.to_json()?
            } else {
                display_events.display_events::<C>(
                    self.extrinsic_cli_opts.verbosity()?,
                    &token_metadata,
                )?
            };
            if let Some(code_stored) = upload_result.code_stored {
                let code_hash: <C as Config>::Hash = code_stored.code_hash;
                if self.output_json() {
                    // Create a JSON object with the events and the code hash.
                    let json_object = serde_json::json!({
                        "events": serde_json::from_str::<serde_json::Value>(&output_events)?,
                        "code_hash": code_hash,
                    });
                    println!("{}", serde_json::to_string_pretty(&json_object)?);
                } else {
                    println!("{}", output_events);
                    name_value_println!("Code hash", format!("{:?}", code_hash));
                }
            } else {
                let code_hash = hex::encode(code_hash);
                return Err(anyhow::anyhow!(
                    "This contract has already been uploaded with code hash: 0x{code_hash}"
                )
                .into())
            }
        }
        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct UploadDryRunResult<Balance> {
    pub result: String,
    pub code_hash: String,
    pub deposit: Balance,
}

impl<Balance> UploadDryRunResult<Balance>
where
    Balance: Debug + Serialize,
{
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn print(&self) {
        name_value_println!("Result", self.result);
        name_value_println!("Code hash", format!("{:?}", self.code_hash));
        name_value_println!("Deposit", format!("{:?}", self.deposit));
    }
}
