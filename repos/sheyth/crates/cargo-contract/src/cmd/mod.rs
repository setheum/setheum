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

mod config;
mod prod_chains;

pub mod build;
pub mod call;
pub mod decode;
pub mod encode;
pub mod info;
pub mod instantiate;
pub mod remove;
pub mod rpc;
pub mod schema;
pub mod storage;
pub mod upload;
pub mod verify;

pub(crate) use self::{
    build::{
        BuildCommand,
        CheckCommand,
    },
    call::CallCommand,
    decode::DecodeCommand,
    info::{
        ExtendedContractInfo,
        InfoCommand,
    },
    instantiate::InstantiateCommand,
    prod_chains::ProductionChain,
    remove::RemoveCommand,
    rpc::RpcCommand,
    schema::{
        GenerateSchemaCommand,
        VerifySchemaCommand,
    },
    storage::StorageCommand,
    upload::UploadCommand,
    verify::VerifyCommand,
};

use crate::{
    anyhow,
    PathBuf,
    Weight,
};
use anyhow::{
    Context,
    Result,
};
use colored::Colorize;
use contract_build::{
    name_value_println,
    Verbosity,
    VerbosityFlags,
    DEFAULT_KEY_COL_WIDTH,
};
pub(crate) use contract_extrinsics::ErrorVariant;
use contract_extrinsics::{
    pallet_contracts_primitives::ContractResult,
    BalanceVariant,
    TokenMetadata,
};

use std::{
    fmt::{
        Debug,
        Display,
    },
    io::{
        self,
        Write,
    },
    str::FromStr,
};

/// Arguments required for creating and sending an extrinsic to a Substrate node.
#[derive(Clone, Debug, clap::Args)]
pub struct CLIExtrinsicOpts {
    /// Path to a contract build artifact file: a raw `.wasm` file, a `.contract` bundle,
    /// or a `.json` metadata file.
    #[clap(value_parser, conflicts_with = "manifest_path")]
    file: Option<PathBuf>,
    /// Path to the `Cargo.toml` of the contract.
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Secret key URI for the account deploying the contract.
    ///
    /// e.g.
    /// - for a dev account "//Alice"
    /// - with a password "//Alice///SECRET_PASSWORD"
    #[clap(name = "suri", long, short)]
    suri: String,
    #[clap(flatten)]
    verbosity: VerbosityFlags,
    /// Submit the extrinsic for on-chain execution.
    #[clap(short('x'), long)]
    execute: bool,
    /// The maximum amount of balance that can be charged from the caller to pay for the
    /// storage. consumed.
    #[clap(long)]
    storage_deposit_limit: Option<String>,
    /// Before submitting a transaction, do not dry-run it via RPC first.
    #[clap(long)]
    skip_dry_run: bool,
    /// Before submitting a transaction, do not ask the user for confirmation.
    #[clap(short('y'), long)]
    skip_confirm: bool,
    /// Arguments required for communicating with a Substrate node.
    #[clap(flatten)]
    chain_cli_opts: CLIChainOpts,
}

impl CLIExtrinsicOpts {
    /// Returns the verbosity
    pub fn verbosity(&self) -> Result<Verbosity> {
        TryFrom::try_from(&self.verbosity)
    }
}

/// Arguments required for communicating with a Substrate node.
#[derive(Clone, Debug, clap::Args)]
pub struct CLIChainOpts {
    /// Websockets url of a Substrate node.
    #[clap(
        name = "url",
        long,
        value_parser,
        default_value = "ws://localhost:9944"
    )]
    url: url::Url,
    /// Chain config to be used as part of the call.
    #[clap(name = "config", long, default_value = "Polkadot")]
    config: String,
    /// Name of a production chain to be communicated with.
    #[clap(name = "chain", long, conflicts_with_all = ["url", "config"])]
    chain: Option<ProductionChain>,
}

impl CLIChainOpts {
    pub fn chain(&self) -> Chain {
        if let Some(chain) = &self.chain {
            Chain::Production(chain.clone())
        } else if let Some(prod) = ProductionChain::from_parts(&self.url, &self.config) {
            Chain::Production(prod)
        } else {
            Chain::Custom(self.url.clone(), self.config.clone())
        }
    }
}

#[derive(Debug)]
pub enum Chain {
    Production(ProductionChain),
    Custom(url::Url, String),
}

impl Chain {
    pub fn url(&self) -> url::Url {
        match self {
            Chain::Production(prod) => prod.url(),
            Chain::Custom(url, _) => url.clone(),
        }
    }

    pub fn config(&self) -> &str {
        match self {
            Chain::Production(prod) => prod.config(),
            Chain::Custom(_, config) => config,
        }
    }

    pub fn production(&self) -> Option<&ProductionChain> {
        if let Chain::Production(prod) = self {
            return Some(prod)
        }
        None
    }
}

const STORAGE_DEPOSIT_KEY: &str = "Storage Total Deposit";
pub const MAX_KEY_COL_WIDTH: usize = STORAGE_DEPOSIT_KEY.len() + 1;

/// Print to stdout the fields of the result of a `instantiate` or `call` dry-run via RPC.
pub fn display_contract_exec_result<R, const WIDTH: usize, Balance>(
    result: &ContractResult<R, Balance>,
) -> Result<()>
where
    Balance: Debug,
{
    let mut debug_message_lines = std::str::from_utf8(&result.debug_message)
        .context("Error decoding UTF8 debug message bytes")?
        .lines();
    name_value_println!("Gas Consumed", format!("{:?}", result.gas_consumed), WIDTH);
    name_value_println!("Gas Required", format!("{:?}", result.gas_required), WIDTH);
    name_value_println!(
        STORAGE_DEPOSIT_KEY,
        format!("{:?}", result.storage_deposit),
        WIDTH
    );

    // print debug messages aligned, only first line has key
    if let Some(debug_message) = debug_message_lines.next() {
        name_value_println!("Debug Message", format!("{debug_message}"), WIDTH);
    }

    for debug_message in debug_message_lines {
        name_value_println!("", format!("{debug_message}"), WIDTH);
    }
    Ok(())
}

pub fn display_contract_exec_result_debug<R, const WIDTH: usize, Balance>(
    result: &ContractResult<R, Balance>,
) -> Result<()> {
    let mut debug_message_lines = std::str::from_utf8(&result.debug_message)
        .context("Error decoding UTF8 debug message bytes")?
        .lines();
    if let Some(debug_message) = debug_message_lines.next() {
        name_value_println!("Debug Message", format!("{debug_message}"), WIDTH);
    }

    for debug_message in debug_message_lines {
        name_value_println!("", format!("{debug_message}"), WIDTH);
    }
    Ok(())
}

pub fn display_dry_run_result_warning(command: &str) {
    println!("Your {} call {} been executed.", command, "has not".bold());
    println!(
            "To submit the transaction and execute the call on chain, add {} flag to the command.",
            "-x/--execute".bold()
        );
}

/// Prompt the user to confirm transaction submission.
pub fn prompt_confirm_tx<F: FnOnce()>(show_details: F) -> Result<()> {
    println!(
        "{} (skip with --skip-confirm or -y)",
        "Confirm transaction details:".bright_white().bold()
    );
    show_details();
    print!(
        "{} ({}/n): ",
        "Submit?".bright_white().bold(),
        "Y".bright_white().bold()
    );

    let mut buf = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    match buf.trim().to_lowercase().as_str() {
        // default is 'y'
        "y" | "" => Ok(()),
        "n" => Err(anyhow!("Transaction not submitted")),
        c => Err(anyhow!("Expected either 'y' or 'n', got '{}'", c)),
    }
}

pub fn print_dry_running_status(msg: &str) {
    println!(
        "{:>width$} {} (skip with --skip-dry-run)",
        "Dry-running".green().bold(),
        msg.bright_white().bold(),
        width = DEFAULT_KEY_COL_WIDTH
    );
}

pub fn print_gas_required_success(gas: Weight) {
    println!(
        "{:>width$} Gas required estimated at {}",
        "Success!".green().bold(),
        gas.to_string().bright_white(),
        width = DEFAULT_KEY_COL_WIDTH
    );
}

/// Display contract information in a formatted way
pub fn basic_display_format_extended_contract_info<Hash, Balance>(
    info: &ExtendedContractInfo<Hash, Balance>,
) where
    Hash: Debug,
    Balance: Debug,
{
    name_value_println!("TrieId", info.trie_id, MAX_KEY_COL_WIDTH);
    name_value_println!(
        "Code Hash",
        format!("{:?}", info.code_hash),
        MAX_KEY_COL_WIDTH
    );
    name_value_println!(
        "Storage Items",
        format!("{:?}", info.storage_items),
        MAX_KEY_COL_WIDTH
    );
    name_value_println!(
        "Storage Items Deposit",
        format!("{:?}", info.storage_items_deposit),
        MAX_KEY_COL_WIDTH
    );
    name_value_println!(
        STORAGE_DEPOSIT_KEY,
        format!("{:?}", info.storage_total_deposit),
        MAX_KEY_COL_WIDTH
    );
    name_value_println!(
        "Source Language",
        format!("{}", info.source_language),
        MAX_KEY_COL_WIDTH
    );
}

/// Display all contracts addresses in a formatted way
pub fn display_all_contracts<AccountId>(contracts: &[AccountId])
where
    AccountId: Display,
{
    contracts.iter().for_each(|e: &AccountId| println!("{}", e))
}

/// Parse a balance from string format
pub fn parse_balance<Balance: FromStr + From<u128> + Clone>(
    balance: &str,
    token_metadata: &TokenMetadata,
) -> Result<Balance> {
    BalanceVariant::from_str(balance)
        .map_err(|e| anyhow!("Balance parsing failed: {e}"))
        .and_then(|bv| bv.denominate_balance(token_metadata))
}

/// Parse a account from string format
pub fn parse_account<AccountId: FromStr>(account: &str) -> Result<AccountId>
where
    <AccountId as FromStr>::Err: Display,
{
    AccountId::from_str(account)
        .map_err(|e| anyhow::anyhow!("Account address parsing failed: {e}"))
}

/// Parse a hex encoded 32 byte hash. Returns error if not exactly 32 bytes.
pub fn parse_code_hash<Hash>(input: &str) -> Result<Hash>
where
    Hash: From<[u8; 32]>,
{
    let bytes = contract_build::util::decode_hex(input)?;
    if bytes.len() != 32 {
        anyhow::bail!("Code hash should be 32 bytes in length")
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr.into())
}

/// Prompt the user to confirm the upload of unverifiable code to the production chain.
pub fn prompt_confirm_unverifiable_upload(chain: &str) -> Result<()> {
    println!("{}", "Confirm upload:".bright_white().bold());
    let warning = format!(
        "Warning: You are about to upload unverifiable code to {} mainnet.\n\
        A third party won't be able to confirm that your uploaded contract Wasm blob \
        matches a particular contract source code.\n\n\
        You can use `cargo contract build --verifiable` to make the contract verifiable.\n\
        See https://use.ink/basics/contract-verification for more info.",
        chain
    )
    .bold()
    .yellow();
    print!("{}", warning);
    println!(
        "{} ({}): ",
        "\nContinue?".bright_white().bold(),
        "y/N".bright_white().bold()
    );

    let mut buf = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut buf)?;
    match buf.trim().to_lowercase().as_str() {
        // default is 'n'
        "y" => Ok(()),
        "n" | "" => Err(anyhow!("Upload cancelled!")),
        c => Err(anyhow!("Expected either 'y' or 'n', got '{}'", c)),
    }
}

#[cfg(test)]
mod tests {
    use subxt::{
        Config,
        SubstrateConfig,
    };

    use super::*;

    #[test]
    fn parse_code_hash_works() {
        // with 0x prefix
        assert!(parse_code_hash::<<SubstrateConfig as Config>::Hash>(
            "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        )
        .is_ok());
        // without 0x prefix
        assert!(parse_code_hash::<<SubstrateConfig as Config>::Hash>(
            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        )
        .is_ok())
    }

    #[test]
    fn parse_incorrect_len_code_hash_fails() {
        // with len not equal to 32
        assert!(parse_code_hash::<<SubstrateConfig as Config>::Hash>(
            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da2"
        )
        .is_err())
    }

    #[test]
    fn parse_bad_format_code_hash_fails() {
        // with bad format
        assert!(parse_code_hash::<<SubstrateConfig as Config>::Hash>(
            "x43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        )
        .is_err())
    }
}
