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

use clap::Parser;
use drink::{AccountId32, Ss58Codec};

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "c")]
    Clear,
    #[clap(alias = "cd")]
    ChangeDir {
        path: String,
    },

    #[clap(alias = "nb")]
    NextBlock {
        #[clap(default_value = "1")]
        count: u32,
    },
    AddTokens {
        #[clap(value_parser = AccountId32::from_ss58check)]
        recipient: AccountId32,
        value: u128,
    },
    SetActor {
        #[clap(value_parser = AccountId32::from_ss58check)]
        actor: AccountId32,
    },
    SetGasLimit {
        ref_time: u64,
        proof_size: u64,
    },

    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy {
        #[clap(long, default_value = "new")]
        constructor: String,
        args: Vec<String>,
        #[clap(long, default_values_t = Vec::<u8>::new(), value_delimiter = ',')]
        salt: Vec<u8>,
    },
    Call {
        message: String,
        args: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliCommand::command().debug_assert()
    }
}
