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

use std::{path::PathBuf, sync::Arc};

use contract_transcode::ContractMessageTranscoder;
use drink::pallet_revive::evm::H160;
use ContractIndex::NoContracts;

use crate::app_state::ContractIndex::CurrentContract;

pub struct Contract {
    pub name: String,
    pub address: H160,
    pub base_path: PathBuf,
    #[allow(dead_code)]
    pub transcoder: Arc<ContractMessageTranscoder>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum ContractIndex {
    #[default]
    NoContracts,
    CurrentContract(usize),
}

#[derive(Default)]
pub struct ContractRegistry {
    contracts: Vec<Contract>,
    index: ContractIndex,
}

impl ContractRegistry {
    pub fn add(&mut self, contract: Contract) {
        self.contracts.push(contract);
        self.index = CurrentContract(self.contracts.len() - 1);
    }

    pub fn current_index(&self) -> ContractIndex {
        self.index
    }

    pub fn current_contract(&self) -> Option<&Contract> {
        match self.index {
            NoContracts => None,
            CurrentContract(idx) => Some(&self.contracts[idx]),
        }
    }

    pub fn get_all(&self) -> &[Contract] {
        &self.contracts
    }

    pub fn next(&mut self) -> Option<&Contract> {
        let CurrentContract(old_index) = self.index else {
            return None;
        };

        self.index = CurrentContract((old_index + 1) % self.contracts.len());
        self.current_contract()
    }

    pub fn count(&self) -> usize {
        self.contracts.len()
    }
}
