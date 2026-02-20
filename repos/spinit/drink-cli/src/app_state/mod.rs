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

use std::{env, path::PathBuf};

pub use contracts::{Contract, ContractIndex, ContractRegistry};
use drink::{minimal::MinimalSandbox, session::Session, AccountId32, Sandbox, Weight};
pub use user_input::UserInput;

use crate::app_state::output::Output;

mod contracts;
mod output;
pub mod print;
mod user_input;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ChainInfo {
    pub block_height: u32,
    pub actor: AccountId32,
    pub gas_limit: Weight,
}

impl Default for ChainInfo {
    fn default() -> Self {
        Self {
            block_height: 0,
            actor: MinimalSandbox::default_actor(),
            gas_limit: MinimalSandbox::default_gas_limit(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Mode {
    #[default]
    Managing,
    Drinking,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UiState {
    pub cwd: PathBuf,
    pub mode: Mode,

    pub user_input: UserInput,
    pub output: Output,

    pub show_help: bool,
}

impl UiState {
    pub fn new(cwd_override: Option<PathBuf>) -> Self {
        let cwd = cwd_override
            .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

        UiState {
            cwd,
            mode: Default::default(),
            user_input: Default::default(),
            output: Default::default(),
            show_help: false,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        UiState::new(None)
    }
}

pub struct AppState {
    pub session: Session<MinimalSandbox>,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
    pub contracts: ContractRegistry,
}

impl AppState {
    pub fn new(cwd_override: Option<PathBuf>) -> Self {
        AppState {
            session: Session::default(),
            chain_info: Default::default(),
            ui_state: UiState::new(cwd_override),
            contracts: Default::default(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(None)
    }
}
