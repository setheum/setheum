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

use crate::DEFAULT_KEY_COL_WIDTH;
use anyhow::{
    Context,
    Result,
};
use clap::{
    Args,
    Subcommand,
};
use colored::Colorize as _;
use contract_build::{
    util,
    CrateMetadata,
};
use contract_transcode::ContractMessageTranscoder;

#[derive(Debug, Args)]
pub struct DecodeCommand {
    #[clap(subcommand)]
    commands: DecodeCommands,
}

#[derive(Debug, Subcommand)]
pub enum DecodeCommands {
    // Decode a message as input
    #[clap(name = "message")]
    Message(DecodeMessage),
    /// Upload contract code
    #[clap(name = "constructor")]
    Constructor(DecodeConstructor),
    /// Decode an event as input
    #[clap(name = "event")]
    Event(DecodeEvent),
}

#[derive(Debug, Clone, Args)]
pub struct DecodeMessage {
    /// The data to decode; this has to be a hex value starting with `0x`.
    #[clap(short, long)]
    data: String,
}

#[derive(Debug, Clone, Args)]
pub struct DecodeConstructor {
    /// The data to decode; this has to be a hex value starting with `0x`.
    #[clap(short, long)]
    data: String,
}

#[derive(Debug, Clone, Args)]
pub struct DecodeEvent {
    /// The signature topic of the event to be decoded; this has to be a hex value
    /// starting with `0x`.
    #[clap(short, long)]
    signature_topic: String,
    /// The data to decode; this has to be a hex value starting with `0x`.
    #[clap(short, long)]
    data: String,
}

impl DecodeCommand {
    pub fn run(&self) -> Result<()> {
        let crate_metadata =
            CrateMetadata::from_manifest_path(None, contract_build::Target::Wasm)?;
        let transcoder = ContractMessageTranscoder::load(crate_metadata.metadata_path())?;

        const ERR_MSG: &str = "Failed to decode specified data as a hex value";
        let decoded_data = match &self.commands {
            DecodeCommands::Event(event) => {
                let signature_topic_data =
                    util::decode_hex(&event.signature_topic).context(ERR_MSG)?;
                let signature_topic =
                    primitive_types::H256::from_slice(&signature_topic_data);
                transcoder.decode_contract_event(
                    &signature_topic,
                    &mut &util::decode_hex(&event.data).context(ERR_MSG)?[..],
                )?
            }
            DecodeCommands::Message(message) => {
                transcoder.decode_contract_message(
                    &mut &util::decode_hex(&message.data).context(ERR_MSG)?[..],
                )?
            }
            DecodeCommands::Constructor(constructor) => {
                transcoder.decode_contract_constructor(
                    &mut &util::decode_hex(&constructor.data).context(ERR_MSG)?[..],
                )?
            }
        };

        println!(
            "{:>width$} {}",
            "Decoded data:".bright_green().bold(),
            decoded_data,
            width = DEFAULT_KEY_COL_WIDTH
        );

        Ok(())
    }
}
