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

#![deny(unused_crate_dependencies)]

mod cmd;

use self::cmd::{
    BuildCommand,
    CallCommand,
    CheckCommand,
    DecodeCommand,
    ErrorVariant,
    GenerateSchemaCommand,
    InfoCommand,
    InstantiateCommand,
    RemoveCommand,
    RpcCommand,
    StorageCommand,
    UploadCommand,
    VerifyCommand,
    VerifySchemaCommand,
};
use anyhow::{
    anyhow,
    Error,
    Result,
};
use clap::{
    Args,
    Parser,
    Subcommand,
};
use cmd::encode::EncodeCommand;
use colored::Colorize;
use contract_build::{
    util::DEFAULT_KEY_COL_WIDTH,
    OutputType,
};
use contract_extrinsics::InstantiateExec;
use sp_weights::Weight;
use std::{
    fmt::Debug,
    path::PathBuf,
    str::FromStr,
};
use tokio::runtime::Runtime;
// These crates are only used when we run integration tests `--features
// integration-tests`. However since we can't have optional `dev-dependencies` we pretend
// to use them during normal test runs in order to satisfy the `unused_crate_dependencies`
// lint.
#[cfg(test)]
use assert_cmd as _;
#[cfg(test)]
use predicates as _;
#[cfg(test)]
use regex as _;
#[cfg(test)]
use tempfile as _;

// Only used on windows.
use which as _;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
#[clap(version = env!("CARGO_CONTRACT_CLI_IMPL_VERSION"))]
pub(crate) enum Opts {
    /// Utilities to develop Wasm smart contracts.
    #[clap(name = "contract")]
    #[clap(version = env!("CARGO_CONTRACT_CLI_IMPL_VERSION"))]
    #[clap(action = ArgAction::DeriveDisplayOrder)]
    Contract(ContractArgs),
}

#[derive(Debug, Args)]
pub(crate) struct ContractArgs {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct HexData(pub Vec<u8>);

impl FromStr for HexData {
    type Err = hex::FromHexError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        hex::decode(input).map(HexData)
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Setup and create a new smart contract project
    #[clap(name = "new")]
    New {
        /// The name of the newly created smart contract
        name: String,
        /// The optional target directory for the contract project
        #[clap(short, long, value_parser)]
        target_dir: Option<PathBuf>,
    },
    /// Compiles the contract, generates metadata, bundles both together in a
    /// `<name>.contract` file
    #[clap(name = "build")]
    Build(BuildCommand),
    /// Check that the code builds as Wasm; does not output any `<name>.contract`
    /// artifact to the `target/` directory
    #[clap(name = "check")]
    Check(CheckCommand),
    /// Upload contract code
    #[clap(name = "upload")]
    Upload(UploadCommand),
    /// Instantiate a contract
    #[clap(name = "instantiate")]
    Instantiate(InstantiateCommand),
    /// Call a contract
    #[clap(name = "call")]
    Call(CallCommand),
    /// Encodes a contracts input calls and their arguments
    #[clap(name = "encode")]
    Encode(EncodeCommand),
    /// Decodes a contracts input or output data (supplied in hex-encoding)
    #[clap(name = "decode")]
    Decode(DecodeCommand),
    /// Remove contract code
    #[clap(name = "remove")]
    Remove(RemoveCommand),
    /// Display information about a contract
    #[clap(name = "info")]
    Info(InfoCommand),
    /// Inspect the on-chain storage of a contract.
    #[clap(name = "storage")]
    Storage(StorageCommand),
    /// Verifies that a given contract binary matches the build result of the specified
    /// workspace.
    #[clap(name = "verify")]
    Verify(VerifyCommand),
    /// Generates schema from the current metadata specification.
    #[clap(name = "generate-schema")]
    GenerateSchema(GenerateSchemaCommand),
    /// Verify schema from the current metadata specification.
    #[clap(name = "verify-schema")]
    VerifySchema(VerifySchemaCommand),
    /// Make a raw RPC call.
    #[clap(name = "rpc")]
    Rpc(RpcCommand),
}

fn main() {
    tracing_subscriber::fmt::init();

    let Opts::Contract(args) = Opts::parse();

    match exec(args.cmd) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err:?}");
            std::process::exit(1);
        }
    }
}

fn exec(cmd: Command) -> Result<()> {
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    match &cmd {
        Command::New { name, target_dir } => {
            contract_build::new_contract_project(name, target_dir.as_ref())?;
            println!("Created contract {name}");
            Ok(())
        }
        Command::Build(build) => {
            let result = build.exec().map_err(format_err)?;

            if matches!(result.output_type, OutputType::Json) {
                println!("{}", result.serialize_json()?)
            } else if result.verbosity.is_verbose() {
                println!("{}", result.display())
            }
            Ok(())
        }
        Command::Check(check) => {
            let res = check.exec().map_err(format_err)?;
            assert!(
                res.dest_wasm.is_none(),
                "no dest_wasm must be on the generation result"
            );
            Ok(())
        }
        Command::Upload(upload) => {
            runtime.block_on(async {
                upload
                    .handle()
                    .await
                    .map_err(|err| map_extrinsic_err(err, upload.output_json()))
            })
        }
        Command::Instantiate(instantiate) => {
            runtime.block_on(async {
                instantiate
                    .handle()
                    .await
                    .map_err(|err| map_extrinsic_err(err, instantiate.output_json()))
            })
        }
        Command::Call(call) => {
            runtime.block_on(async {
                call.handle()
                    .await
                    .map_err(|err| map_extrinsic_err(err, call.output_json()))
            })
        }
        Command::Encode(encode) => encode.run().map_err(format_err),
        Command::Decode(decode) => decode.run().map_err(format_err),
        Command::Remove(remove) => {
            runtime.block_on(async {
                remove
                    .handle()
                    .await
                    .map_err(|err| map_extrinsic_err(err, remove.output_json()))
            })
        }
        Command::Info(info) => {
            runtime.block_on(async { info.handle().await.map_err(format_err) })
        }
        Command::Storage(storage) => {
            runtime.block_on(async { storage.handle().await.map_err(format_err) })
        }
        Command::Verify(verify) => {
            let result = verify.run().map_err(format_err)?;

            if result.output_json {
                println!("{}", result.serialize_json()?)
            } else if result.verbosity.is_verbose() {
                println!("{}", result.display())
            }
            Ok(())
        }
        Command::GenerateSchema(generate) => {
            let result = generate.run().map_err(format_err)?;
            println!("{}", result);
            Ok(())
        }
        Command::VerifySchema(verify) => {
            let result = verify.run().map_err(format_err)?;

            if result.output_json {
                println!("{}", result.serialize_json()?)
            } else if result.verbosity.is_verbose() {
                println!("{}", result.display())
            }
            Ok(())
        }
        Command::Rpc(rpc) => {
            runtime.block_on(async { rpc.run().await.map_err(format_err) })
        }
    }
}

fn map_extrinsic_err(err: ErrorVariant, is_json: bool) -> Error {
    if is_json {
        anyhow!(
            "{}",
            serde_json::to_string_pretty(&err)
                .expect("error serialization is infallible; qed")
        )
    } else {
        format_err(err)
    }
}

fn format_err<E: Debug>(err: E) -> Error {
    anyhow!(
        "{} {}",
        "ERROR:".bright_red().bold(),
        format!("{err:?}").bright_red()
    )
}
