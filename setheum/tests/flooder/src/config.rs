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

use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version = "1.0")]
pub struct Config {
/// URL address(es) of the nodes (in the same network) to send transactions to
    #[clap(long, default_value = "ws://127.0.0.1:9944")]
    pub nodes: Vec<String>,

/// How many transactions to send in the interval
    #[clap(long)]
    pub transactions_in_interval: u64,

/// How long the interval is, in secs
    #[clap(long, default_value = "1")]
    pub interval_duration: u64,

/// For how many intervals should the flood last
    #[clap(long, default_value = "180")]
    pub intervals: u64,

/// Secret phrase : a path to a file or passed on stdin
    #[clap(long)]
    pub phrase: Option<String>,

/// Secret seed of the account keypair passed on stdin
    #[clap(long, conflicts_with_all = &["phrase"])]
    pub seed: Option<String>,

/// Allows to skip accounts
    #[clap(long)]
    pub skip_initialization: bool,

/// Beginning of the integer range used to derive accounts
    #[clap(long, default_value = "0")]
    pub first_account_in_range: u64,

/// Changes the awaited status of every transaction from `SubmitOnly` to `Ready`
    #[clap(long)]
    pub wait_for_ready: bool,

/// Flooder will pause sending transactions to the node, if there are more than
/// `pool_limit` transactions in the tx pool of the node. Should
/// be smaller than `--pool-limit` parameter of nodes.
    #[clap(long, default_value = "6144")]
    pub pool_limit: u64,
}

pub fn read_phrase(phrase: String) -> String {
    let file = PathBuf::from(&phrase);
    if file.is_file() {
        fs::read_to_string(phrase).unwrap().trim_end().to_owned()
    } else {
        phrase
    }
}
