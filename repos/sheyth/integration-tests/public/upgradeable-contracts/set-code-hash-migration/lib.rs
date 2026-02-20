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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! Demonstrates how to use [`set_code_hash`](https://docs.rs/ink_env/latest/ink_env/fn.set_code_hash.html)
//! to swap out the `code_hash` of an on-chain contract.
//!
//! We will swap the code of our `Incrementer` contract with that of the `Incrementer`
//! found in the `updated_incrementer` folder.
//!
//! See the included End-to-End tests an example update workflow.

#[ink::contract]
pub mod incrementer {
    /// Track a counter in storage.
    ///
    /// # Note
    ///
    /// Is is important to realize that after the call to `set_code_hash` the contract's
    /// storage remains the same.
    ///
    /// If you change the storage layout in your storage struct you may introduce
    /// undefined behavior to your contract!
    #[ink(storage)]
    #[derive(Default)]
    pub struct Incrementer {
        count: u32,
    }

    impl Incrementer {
        /// Creates a new counter smart contract initialized with the given base value.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Increments the counter value which is stored in the contract's storage.
        #[ink(message)]
        pub fn inc(&mut self) {
            self.count = self.count.checked_add(1).unwrap();
            ink::env::debug_println!(
                "The new count is {}, it was modified using the original contract code.",
                self.count
            );
        }

        /// Returns the counter value which is stored in this contract's storage.
        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.count
        }

        /// Modifies the code which is used to execute calls to this contract address
        /// (`AccountId`).
        ///
        /// We use this to upgrade the contract logic. We don't do any authorization here,
        /// any caller can execute this method.
        ///
        /// In a production contract you would do some authorization here!
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: Hash) {
            self.env().set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!("Failed to `set_code_hash` to {code_hash:?} due to {err:?}")
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
