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

use anyhow::Result;
use colored::Colorize;
use comfy_table::{
    ContentArrangement,
    Table,
};
use contract_extrinsics::{
    ContractArtifacts,
    ContractStorage,
    ContractStorageLayout,
    ContractStorageRpc,
    ErrorVariant,
};
use ink_env::Environment;
use serde::Serialize;
use std::{
    fmt::Display,
    path::PathBuf,
    str::FromStr,
};
use subxt::{
    ext::scale_decode::IntoVisitor,
    Config,
};

use crate::call_with_config;

use super::{
    parse_account,
    CLIChainOpts,
};

#[derive(Debug, clap::Args)]
#[clap(name = "storage", about = "Inspect contract storage")]
pub struct StorageCommand {
    /// The address of the contract to inspect storage of.
    #[clap(
        name = "contract",
        long,
        env = "CONTRACT",
        required_unless_present = "version"
    )]
    contract: Option<String>,
    /// Fetch the "raw" storage keys and values for the contract.
    #[clap(long)]
    raw: bool,
    /// Export the instantiate output in JSON format.
    #[clap(name = "output-json", long, conflicts_with = "raw")]
    output_json: bool,
    /// Path to a contract build artifact file: a raw `.wasm` file, a `.contract` bundle,
    /// or a `.json` metadata file.
    #[clap(value_parser, conflicts_with = "manifest_path")]
    file: Option<PathBuf>,
    /// Path to the `Cargo.toml` of the contract.
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Fetch the storage version of the pallet contracts (state query:
    /// contracts::palletVersion()).
    #[clap(long, short)]
    version: bool,
    /// Arguments required for communicating with a Substrate node.
    #[clap(flatten)]
    chain_cli_opts: CLIChainOpts,
}

impl StorageCommand {
    pub async fn handle(&self) -> Result<(), ErrorVariant> {
        call_with_config!(self, run, self.chain_cli_opts.chain().config())
    }

    pub async fn run<C: Config + Environment>(&self) -> Result<(), ErrorVariant>
    where
        <C as Config>::AccountId: Display + IntoVisitor + AsRef<[u8]> + FromStr,
        <<C as Config>::AccountId as FromStr>::Err:
            Into<Box<(dyn std::error::Error)>> + Display,
        C::Balance: Serialize + IntoVisitor,
        <C as Config>::Hash: IntoVisitor,
    {
        let rpc =
            ContractStorageRpc::<C>::new(&self.chain_cli_opts.chain().url()).await?;
        let storage_layout = ContractStorage::<C, C>::new(rpc);
        if self.version {
            println!("{}", storage_layout.version().await?);
            return Ok(())
        }

        // Contract arg shall be always present in this case, it is enforced by
        // clap configuration
        let contract = self
            .contract
            .as_ref()
            .map(|c| parse_account(c))
            .transpose()?
            .expect("Contract argument shall be present");

        if self.raw {
            let storage_data =
                storage_layout.load_contract_storage_data(&contract).await?;
            println!(
                "{json}",
                json = serde_json::to_string_pretty(&storage_data)?
            );
            return Ok(())
        }

        let contract_artifacts = ContractArtifacts::from_manifest_or_file(
            self.manifest_path.as_ref(),
            self.file.as_ref(),
        );

        match contract_artifacts {
            Ok(contract_artifacts) => {
                let transcoder = contract_artifacts.contract_transcoder()?;
                let contract_storage = storage_layout
                    .load_contract_storage_with_layout(&contract, &transcoder)
                    .await?;
                if self.output_json {
                    println!(
                        "{json}",
                        json = serde_json::to_string_pretty(&contract_storage)?
                    );
                } else {
                    let table = StorageDisplayTable::new(&contract_storage);
                    table.display();
                }
            }
            Err(_) => {
                eprintln!(
                    "{} Displaying raw storage: no valid contract metadata artifacts found",
                    "Info:".cyan().bold(),
                );
                let storage_data =
                    storage_layout.load_contract_storage_data(&contract).await?;
                println!(
                    "{json}",
                    json = serde_json::to_string_pretty(&storage_data)?
                );
            }
        }

        Ok(())
    }
}

struct StorageDisplayTable(Table);

impl StorageDisplayTable {
    const INDEX_LABEL: &'static str = "Index";
    const KEY_LABEL: &'static str = "Root Key";
    const PARENT_LABEL: &'static str = "Parent";
    const VALUE_LABEL: &'static str = "Value";

    fn new(storage_layout: &ContractStorageLayout) -> Self {
        let mut table = Table::new();
        Self::table_add_header(&mut table);
        Self::table_add_rows(&mut table, storage_layout);
        Self(table)
    }

    fn table_add_header(table: &mut Table) {
        table.set_content_arrangement(ContentArrangement::Dynamic);

        let header = vec![
            Self::INDEX_LABEL,
            Self::KEY_LABEL,
            Self::PARENT_LABEL,
            Self::VALUE_LABEL,
        ];
        table.set_header(header);
    }

    fn table_add_rows(table: &mut Table, storage_layout: &ContractStorageLayout) {
        for (index, cell) in storage_layout.iter().enumerate() {
            let formatted_cell = format!("{cell}");
            let values = formatted_cell.split('\n');
            for (i, v) in values.enumerate() {
                table.add_row(vec![
                    (index + i).to_string().as_str(),
                    cell.root_key().as_str(),
                    cell.parent().as_str(),
                    v,
                ]);
            }
        }
    }

    fn display(&self) {
        println!("{}", self.0);
    }
}
