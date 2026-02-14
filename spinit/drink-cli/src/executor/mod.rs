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

mod contract;
mod error;

use std::env;

use anyhow::Result;
use clap::Parser;
use drink::{sandbox_api::prelude::*, AccountId32, Weight};

use crate::{app_state::AppState, cli::CliCommand};

fn vec_u8_to_array_32(bytes: Vec<u8>) -> [u8; 32] {
    let mut array = [0u8; 32];
    let len = bytes.len().min(32);
    array[..len].copy_from_slice(&bytes[..len]);
    array
}

pub fn execute(app_state: &mut AppState) -> Result<()> {
    let command = app_state.ui_state.user_input.current_input().to_string();
    app_state.print_command(&command);

    let command = command
        .split_ascii_whitespace()
        .map(|a| a.trim())
        .collect::<Vec<_>>();
    let cli_command = match CliCommand::try_parse_from([vec![""], command].concat()) {
        Ok(cli_command) => cli_command,
        Err(_) => {
            app_state.print_error("Invalid command");
            return Ok(());
        }
    };

    match cli_command {
        CliCommand::Clear => app_state.ui_state.output.clear(),
        CliCommand::ChangeDir { path } => {
            let target_dir = app_state.ui_state.cwd.join(path);
            match env::set_current_dir(target_dir) {
                Ok(_) => {
                    app_state.ui_state.cwd =
                        env::current_dir().expect("Failed to get current directory");
                    app_state.print("Directory changed");
                }
                Err(err) => app_state.print_error(&err.to_string()),
            }
        }

        CliCommand::NextBlock { count } => build_blocks(app_state, count),
        CliCommand::AddTokens { recipient, value } => add_tokens(app_state, recipient, value)?,
        CliCommand::SetActor { actor } => {
            app_state.chain_info.actor = actor;
            app_state.print("Actor was set");
        }
        CliCommand::SetGasLimit {
            ref_time,
            proof_size,
        } => {
            app_state.chain_info.gas_limit = Weight::from_parts(ref_time, proof_size);
            app_state.print("Gas limit was set");
        }
        CliCommand::Build => contract::build(app_state),
        CliCommand::Deploy {
            constructor,
            args,
            salt,
        } => {
            contract::deploy(app_state, constructor, args, Some(vec_u8_to_array_32(salt)));
        }
        CliCommand::Call { message, args } => contract::call(app_state, message, args),
    }

    Ok(())
}

fn build_blocks(app_state: &mut AppState, count: u32) {
    app_state.chain_info.block_height = app_state.session.sandbox().build_blocks(count);
    app_state.print(&format!("{count} blocks built"));
}

fn add_tokens(app_state: &mut AppState, recipient: AccountId32, value: u128) -> Result<()> {
    app_state
        .session
        .sandbox()
        .mint_into(&recipient, value)
        .map_err(|err| anyhow::format_err!("Failed to add token: {err:?}"))?;
    app_state.print(&format!("{value} tokens added to {recipient}",));
    Ok(())
}
