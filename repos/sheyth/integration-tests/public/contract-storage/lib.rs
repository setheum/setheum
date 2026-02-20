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

#[ink::contract]
mod contract_storage {
    use ink::prelude::{
        format,
        string::String,
    };

    /// A contract for testing reading and writing contract storage.
    #[ink(storage)]
    #[derive(Default)]
    pub struct ContractStorage;

    impl ContractStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Read from the contract storage slot, consuming all the data from the buffer.
        #[ink(message)]
        pub fn set_and_get_storage_all_data_consumed(&self) -> Result<(), String> {
            let key = 0u32;
            let value = [0x42; 32];
            ink::env::set_contract_storage(&key, &value);
            let loaded_value = ink::env::get_contract_storage(&key)
                .map_err(|e| format!("get_contract_storage failed: {:?}", e))?;
            assert_eq!(loaded_value, Some(value));
            Ok(())
        }

        /// Read from the contract storage slot, only partially consuming data from the
        /// buffer.
        #[ink(message)]
        pub fn set_and_get_storage_partial_data_consumed(&self) -> Result<(), String> {
            let key = 0u32;
            let value = [0x42; 32];
            ink::env::set_contract_storage(&key, &value);
            // Only attempt to read the first byte (the `u8`) of the storage value data
            let _loaded_value: Option<u8> = ink::env::get_contract_storage(&key)
                .map_err(|e| format!("get_contract_storage failed: {:?}", e))?;
            Ok(())
        }

        /// Read from the contract storage slot, consuming all the data from the buffer.
        #[ink(message)]
        pub fn set_and_take_storage_all_data_consumed(&self) -> Result<(), String> {
            let key = 0u32;
            let value = [0x42; 32];
            ink::env::set_contract_storage(&key, &value);
            let loaded_value = ink::env::take_contract_storage(&key)
                .map_err(|e| format!("get_contract_storage failed: {:?}", e))?;
            assert_eq!(loaded_value, Some(value));
            Ok(())
        }

        /// Read from the contract storage slot, only partially consuming data from the
        /// buffer.
        #[ink(message)]
        pub fn set_and_take_storage_partial_data_consumed(&self) -> Result<(), String> {
            let key = 0u32;
            let value = [0x42; 32];
            ink::env::set_contract_storage(&key, &value);
            // Only attempt to read the first byte (the `u8`) of the storage value data
            let _loaded_value: Option<u8> = ink::env::take_contract_storage(&key)
                .map_err(|e| format!("get_contract_storage failed: {:?}", e))?;
            Ok(())
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
