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

use crate::call_with_config;

use super::{
    basic_display_format_extended_contract_info,
    display_all_contracts,
    parse_account,
    CLIChainOpts,
};
use anyhow::Result;
use contract_analyze::determine_language;
use contract_extrinsics::{
    fetch_all_contracts,
    fetch_contract_info,
    fetch_wasm_code,
    url_to_string,
    ContractInfo,
    ErrorVariant,
    TrieId,
};
use ink_env::Environment;
use serde::Serialize;
use std::{
    fmt::{
        Debug,
        Display,
    },
    io::Write,
    str::FromStr,
};
use subxt::{
    backend::{
        legacy::LegacyRpcMethods,
        rpc::RpcClient,
    },
    ext::{
        codec::Decode,
        scale_decode::IntoVisitor,
    },
    Config,
    OnlineClient,
};

#[derive(Debug, clap::Args)]
#[clap(name = "info", about = "Get infos from a contract")]
pub struct InfoCommand {
    /// The address of the contract to display info of.
    #[clap(
        name = "contract",
        long,
        env = "CONTRACT",
        required_unless_present = "all"
    )]
    contract: Option<String>,
    /// Export the instantiate output in JSON format.
    #[clap(name = "output-json", long)]
    output_json: bool,
    /// Display the contract's Wasm bytecode.
    #[clap(name = "binary", long, conflicts_with = "all")]
    binary: bool,
    /// Display all contracts addresses
    #[clap(name = "all", long)]
    all: bool,
    /// Arguments required for communicating with a Substrate node.
    #[clap(flatten)]
    chain_cli_opts: CLIChainOpts,
}

impl InfoCommand {
    pub async fn handle(&self) -> Result<(), ErrorVariant> {
        call_with_config!(self, run, self.chain_cli_opts.chain().config())
    }

    pub async fn run<C: Config + Environment>(&self) -> Result<(), ErrorVariant>
    where
        <C as Config>::AccountId:
            Serialize + Display + IntoVisitor + Decode + AsRef<[u8]> + FromStr,
        <C as Config>::Hash: IntoVisitor + Display,
        <C as Environment>::Balance: Serialize + Debug + IntoVisitor,
        <<C as Config>::AccountId as FromStr>::Err:
            Into<Box<(dyn std::error::Error)>> + Display,
    {
        let rpc_cli =
            RpcClient::from_url(url_to_string(&self.chain_cli_opts.chain().url()))
                .await?;
        let client = OnlineClient::<C>::from_rpc_client(rpc_cli.clone()).await?;
        let rpc = LegacyRpcMethods::<C>::new(rpc_cli.clone());

        // All flag applied
        if self.all {
            let contracts = fetch_all_contracts(&client, &rpc).await?;

            if self.output_json {
                let contracts_json = serde_json::json!({
                    "contracts": contracts
                });
                println!("{}", serde_json::to_string_pretty(&contracts_json)?);
            } else {
                display_all_contracts(&contracts)
            }
            Ok(())
        } else {
            // Contract arg shall be always present in this case, it is enforced by
            // clap configuration
            let contract = self
                .contract
                .as_ref()
                .map(|c| parse_account(c))
                .transpose()?
                .expect("Contract argument shall be present");

            let info_to_json =
                fetch_contract_info::<C, C>(&contract, &rpc, &client).await?;

            let wasm_code =
                fetch_wasm_code(&client, &rpc, info_to_json.code_hash()).await?;
            // Binary flag applied
            if self.binary {
                if self.output_json {
                    let wasm = serde_json::json!({
                        "wasm": format!("0x{}", hex::encode(wasm_code))
                    });
                    println!("{}", serde_json::to_string_pretty(&wasm)?);
                } else {
                    std::io::stdout()
                        .write_all(&wasm_code)
                        .expect("Writing to stdout failed")
                }
            } else if self.output_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ExtendedContractInfo::<
                        <C as Config>::Hash,
                        C::Balance,
                    >::new(
                        info_to_json, &wasm_code
                    ))?
                )
            } else {
                basic_display_format_extended_contract_info(&ExtendedContractInfo::<
                    <C as Config>::Hash,
                    C::Balance,
                >::new(
                    info_to_json, &wasm_code
                ))
            }
            Ok(())
        }
    }
}

#[derive(serde::Serialize)]
pub struct ExtendedContractInfo<Hash, Balance> {
    pub trie_id: TrieId,
    pub code_hash: Hash,
    pub storage_items: u32,
    pub storage_items_deposit: Balance,
    pub storage_total_deposit: Balance,
    pub source_language: String,
}

impl<Hash, Balance> ExtendedContractInfo<Hash, Balance>
where
    Hash: serde::Serialize + Copy,
    Balance: serde::Serialize + Copy,
{
    pub fn new(contract_info: ContractInfo<Hash, Balance>, code: &[u8]) -> Self {
        let language = match determine_language(code).ok() {
            Some(lang) => lang.to_string(),
            None => "Unknown".to_string(),
        };
        ExtendedContractInfo {
            trie_id: contract_info.trie_id().clone(),
            code_hash: *contract_info.code_hash(),
            storage_items: contract_info.storage_items(),
            storage_items_deposit: contract_info.storage_items_deposit(),
            storage_total_deposit: contract_info.storage_total_deposit(),
            source_language: language,
        }
    }
}
