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

use contract_build::util::DEFAULT_KEY_COL_WIDTH;
use ink_env::Environment;
use serde::Serialize;
use std::{
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use super::{
    config::SignerConfig,
    display_contract_exec_result,
    display_contract_exec_result_debug,
    display_dry_run_result_warning,
    parse_account,
    parse_balance,
    print_dry_running_status,
    print_gas_required_success,
    prompt_confirm_tx,
    CLIExtrinsicOpts,
    MAX_KEY_COL_WIDTH,
};
use anyhow::{
    anyhow,
    Context,
    Result,
};
use contract_build::name_value_println;
use contract_extrinsics::{
    pallet_contracts_primitives::StorageDeposit,
    CallCommandBuilder,
    CallExec,
    DisplayEvents,
    ExtrinsicOptsBuilder,
    TokenMetadata,
};
use contract_transcode::Value;
use sp_weights::Weight;
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
#[clap(name = "call", about = "Call a contract")]
pub struct CallCommand {
    /// The address of the the contract to call.
    #[clap(name = "contract", long, env = "CONTRACT")]
    contract: String,
    /// The name of the contract message to call.
    #[clap(long, short)]
    message: String,
    /// The arguments of the contract message to call.
    #[clap(long, num_args = 0..)]
    args: Vec<String>,
    #[clap(flatten)]
    extrinsic_cli_opts: CLIExtrinsicOpts,
    /// Maximum amount of gas (execution time) to be used for this command.
    /// If not specified will perform a dry-run to estimate the gas consumed for the
    /// call.
    #[clap(name = "gas", long)]
    gas_limit: Option<u64>,
    /// Maximum proof size for this call.
    /// If not specified will perform a dry-run to estimate the proof size required for
    /// the call.
    #[clap(long)]
    proof_size: Option<u64>,
    /// The value to be transferred as part of the call.
    #[clap(name = "value", long, default_value = "0")]
    value: String,
    /// Export the call output in JSON format.
    #[clap(long, conflicts_with = "verbose")]
    output_json: bool,
}

impl CallCommand {
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
        C::Balance:
            From<u128> + Display + Default + FromStr + Serialize + Debug + EncodeAsType,
        <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
            From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    {
        let contract = parse_account(&self.contract)
            .map_err(|e| anyhow::anyhow!("Failed to parse contract option: {}", e))?;
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
        let value = parse_balance(&self.value, &token_metadata)
            .map_err(|e| anyhow::anyhow!("Failed to parse value option: {}", e))?;
        let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
            .file(self.extrinsic_cli_opts.file.clone())
            .manifest_path(self.extrinsic_cli_opts.manifest_path.clone())
            .url(chain.url())
            .storage_deposit_limit(storage_deposit_limit)
            .verbosity(self.extrinsic_cli_opts.verbosity()?)
            .done();

        let call_exec = CallCommandBuilder::new(contract, &self.message, extrinsic_opts)
            .args(self.args.clone())
            .gas_limit(self.gas_limit)
            .proof_size(self.proof_size)
            .value(value)
            .done()
            .await?;
        let metadata = call_exec.client().metadata();

        if !self.extrinsic_cli_opts.execute {
            let result = call_exec.call_dry_run().await?;
            match result.result {
                Ok(ref ret_val) => {
                    let value = call_exec
                        .transcoder()
                        .decode_message_return(
                            call_exec.message(),
                            &mut &ret_val.data[..],
                        )
                        .context(format!(
                            "Failed to decode return value {:?}",
                            &ret_val
                        ))?;
                    let dry_run_result = CallDryRunResult {
                        reverted: ret_val.did_revert(),
                        data: value,
                        gas_consumed: result.gas_consumed,
                        gas_required: result.gas_required,
                        storage_deposit: result.storage_deposit.clone(),
                    };
                    if self.output_json() {
                        println!("{}", dry_run_result.to_json()?);
                    } else {
                        dry_run_result.print();
                        display_contract_exec_result_debug::<_, DEFAULT_KEY_COL_WIDTH, _>(
                            &result,
                        )?;
                        display_dry_run_result_warning("message");
                    };
                }
                Err(ref err) => {
                    let object = ErrorVariant::from_dispatch_error(err, &metadata)?;
                    if self.output_json() {
                        return Err(object)
                    } else {
                        name_value_println!("Result", object, MAX_KEY_COL_WIDTH);
                        display_contract_exec_result::<_, MAX_KEY_COL_WIDTH, _>(&result)?;
                    }
                }
            }
        } else {
            let gas_limit = pre_submit_dry_run_gas_estimate_call(
                &call_exec,
                self.output_json(),
                self.extrinsic_cli_opts.skip_dry_run,
            )
            .await?;
            if !self.extrinsic_cli_opts.skip_confirm {
                prompt_confirm_tx(|| {
                    name_value_println!(
                        "Message",
                        call_exec.message(),
                        DEFAULT_KEY_COL_WIDTH
                    );
                    name_value_println!(
                        "Args",
                        call_exec.args().join(" "),
                        DEFAULT_KEY_COL_WIDTH
                    );
                    name_value_println!(
                        "Gas limit",
                        gas_limit.to_string(),
                        DEFAULT_KEY_COL_WIDTH
                    );
                })?;
            }
            let events = call_exec.call(Some(gas_limit)).await?;
            let display_events =
                DisplayEvents::from_events::<C, C>(&events, None, &metadata)?;

            let output = if self.output_json() {
                display_events.to_json()?
            } else {
                display_events.display_events::<C>(
                    self.extrinsic_cli_opts.verbosity().unwrap(),
                    &token_metadata,
                )?
            };
            println!("{output}");
        }
        Ok(())
    }
}

/// A helper function to estimate the gas required for a contract call.
async fn pre_submit_dry_run_gas_estimate_call<C: Config + Environment, Signer>(
    call_exec: &CallExec<C, C, Signer>,
    output_json: bool,
    skip_dry_run: bool,
) -> Result<Weight>
where
    Signer: subxt::tx::Signer<C> + Clone,
    <C as Config>::AccountId: IntoVisitor + EncodeAsType,
    C::Balance: Debug + EncodeAsType,
    <C::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
{
    if skip_dry_run {
        return match (call_exec.gas_limit(), call_exec.proof_size()) {
            (Some(ref_time), Some(proof_size)) => Ok(Weight::from_parts(ref_time, proof_size)),
            _ => {
                Err(anyhow!(
                "Weight args `--gas` and `--proof-size` required if `--skip-dry-run` specified"
            ))
            }
        };
    }
    if !output_json {
        print_dry_running_status(call_exec.message());
    }
    let call_result = call_exec.call_dry_run().await?;
    match call_result.result {
        Ok(_) => {
            if !output_json {
                print_gas_required_success(call_result.gas_required);
            }
            // use user specified values where provided, otherwise use the estimates
            let ref_time = call_exec
                .gas_limit()
                .unwrap_or_else(|| call_result.gas_required.ref_time());
            let proof_size = call_exec
                .proof_size()
                .unwrap_or_else(|| call_result.gas_required.proof_size());
            Ok(Weight::from_parts(ref_time, proof_size))
        }
        Err(ref err) => {
            let object =
                ErrorVariant::from_dispatch_error(err, &call_exec.client().metadata())?;
            if output_json {
                Err(anyhow!("{}", serde_json::to_string_pretty(&object)?))
            } else {
                name_value_println!("Result", object, MAX_KEY_COL_WIDTH);
                display_contract_exec_result::<_, MAX_KEY_COL_WIDTH, _>(&call_result)?;

                Err(anyhow!("Pre-submission dry-run failed. Use --skip-dry-run to skip this step."))
            }
        }
    }
}

/// Result of the contract call
#[derive(serde::Serialize)]
pub struct CallDryRunResult<Balance> {
    /// Was the operation reverted
    pub reverted: bool,
    pub data: Value,
    pub gas_consumed: Weight,
    pub gas_required: Weight,
    /// Storage deposit after the operation
    pub storage_deposit: StorageDeposit<Balance>,
}

impl<Balance: Serialize> CallDryRunResult<Balance> {
    /// Returns a result in json format
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn print(&self) {
        name_value_println!("Result", format!("{}", self.data), DEFAULT_KEY_COL_WIDTH);
        name_value_println!(
            "Reverted",
            format!("{:?}", self.reverted),
            DEFAULT_KEY_COL_WIDTH
        );
    }
}
