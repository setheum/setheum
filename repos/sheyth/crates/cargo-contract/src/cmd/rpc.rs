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

use contract_build::name_value_println;
use contract_extrinsics::{
    ErrorVariant,
    RawParams,
    RpcRequest,
};
use subxt::ext::scale_value;

use super::{
    CLIChainOpts,
    MAX_KEY_COL_WIDTH,
};

#[derive(Debug, clap::Args)]
#[clap(name = "rpc", about = "Make a raw RPC call")]
pub struct RpcCommand {
    /// The name of the method to call.
    method: String,
    /// The arguments of the method to call.
    #[clap(num_args = 0..)]
    params: Vec<String>,
    /// Export the call output in JSON format.
    #[clap(long)]
    output_json: bool,
    /// Arguments required for communicating with a Substrate node.
    #[clap(flatten)]
    chain_cli_opts: CLIChainOpts,
}

impl RpcCommand {
    pub async fn run(&self) -> Result<(), ErrorVariant> {
        let request = RpcRequest::new(&self.chain_cli_opts.chain().url()).await?;
        let params = RawParams::new(&self.params)?;

        let result = request.raw_call(&self.method, params).await;

        match (result, self.output_json) {
            (Err(err), false) => Err(anyhow::anyhow!("Method call failed: {}", err))?,
            (Err(err), true) => {
                Err(anyhow::anyhow!(serde_json::to_string_pretty(
                    &ErrorVariant::from(err)
                )?))?
            }
            (Ok(res), false) => {
                let output: scale_value::Value = serde_json::from_str(res.get())?;
                name_value_println!("Result", output, MAX_KEY_COL_WIDTH);
                Ok(())
            }
            (Ok(res), true) => {
                let json: serde_json::Value = serde_json::from_str(res.get())?;
                println!("{}", serde_json::to_string_pretty(&json)?);
                Ok(())
            }
        }
    }
}
