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

//! Mocking utilities for contract calls.

mod contract;
mod error;
mod extension;
use std::collections::BTreeMap;

pub use contract::{mock_message, ContractMock, MessageMock, Selector};
use error::MockingError;
use ink_sandbox::pallet_revive::evm::H160;

/// Untyped result of a mocked call.
pub type MockedCallResult = Result<Vec<u8>, MockingError>;

/// A registry of mocked contracts.
pub(crate) struct MockRegistry {
    mocked_contracts: BTreeMap<H160, ContractMock>,
    nonce: u8,
}

impl MockRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            mocked_contracts: BTreeMap::new(),
            nonce: 0u8,
        }
    }

    /// Returns the salt for the next contract.
    pub fn salt(&mut self) -> [u8; 32] {
        self.nonce += 1;
        [self.nonce; 32]
    }

    /// Registers `mock` for `address`. Returns the previous mock, if any.
    pub fn register(&mut self, address: H160, mock: ContractMock) -> Option<ContractMock> {
        self.mocked_contracts.insert(address, mock)
    }

    /// Returns the mock for `address`, if any.
    #[allow(dead_code)] // FIXME: Remove when mocking extension is replaced
    pub fn get(&self, address: &H160) -> Option<&ContractMock> {
        self.mocked_contracts.get(address)
    }
}
