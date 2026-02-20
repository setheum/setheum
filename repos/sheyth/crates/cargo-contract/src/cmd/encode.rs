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

use std::path::PathBuf;

use crate::DEFAULT_KEY_COL_WIDTH;
use anyhow::Result;
use colored::Colorize as _;
use contract_extrinsics::ContractArtifacts;

#[derive(Debug, Clone, clap::Args)]
#[clap(
    name = "encode",
    about = "Encodes a contracts input calls and their arguments"
)]
pub struct EncodeCommand {
    /// Path to a contract build artifact file: a raw `.wasm` file, a `.contract` bundle,
    /// or a `.json` metadata file.
    #[clap(value_parser, conflicts_with = "manifest_path")]
    file: Option<PathBuf>,
    /// Path to the `Cargo.toml` of the contract.
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// The name of the contract message to encode.
    #[clap(long, short)]
    message: String,
    /// The arguments to encode
    #[clap(long, num_args = 0..)]
    args: Vec<String>,
}

impl EncodeCommand {
    pub fn run(&self) -> Result<()> {
        let artifacts = ContractArtifacts::from_manifest_or_file(
            self.manifest_path.as_ref(),
            self.file.as_ref(),
        )?;
        let transcoder = artifacts.contract_transcoder()?;

        let call_data = transcoder.encode(&self.message, &self.args)?;
        let call_data_encoded = hex::encode_upper(call_data);

        println!(
            "{:>width$} {}",
            "Encoded data:".bright_green().bold(),
            call_data_encoded,
            width = DEFAULT_KEY_COL_WIDTH
        );

        Ok(())
    }
}
