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
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Returns the current balance of the Flipper.
        #[ink(message)]
        pub fn get_contract_balance(&self) -> Balance {
            self.env().balance()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::{
            subxt::dynamic::Value,
            ChainBackend,
            ContractsBackend,
        };

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// Tests standard flipper scenario:
        /// - deploy the flipper contract with initial value `false`
        /// - flip the flipper
        /// - get the flipper's value
        /// - assert that the value is `true`
        #[ink_e2e::test(backend(runtime_only))]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            const INITIAL_VALUE: bool = false;
            let mut constructor = FlipperRef::new(INITIAL_VALUE);

            let contract = client
                .instantiate(
                    "e2e-runtime-only-backend",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("deploy failed");

            // when
            let mut call_builder = contract.call_builder::<Flipper>();
            let _flip_res = client
                .call(&ink_e2e::bob(), &call_builder.flip())
                .submit()
                .await;

            // then
            let get_res = client
                .call(&ink_e2e::bob(), &call_builder.get())
                .dry_run()
                .await?;
            assert_eq!(get_res.return_value(), !INITIAL_VALUE);

            Ok(())
        }

        /// Tests runtime call scenario:
        /// - deploy the flipper contract
        /// - get the contract's balance
        /// - transfer some funds to the contract using runtime call
        /// - get the contract's balance again
        /// - assert that the contract's balance increased by the transferred amount
        #[ink_e2e::test(backend(runtime_only))]
        async fn runtime_call_works() -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new(false);

            let contract = client
                .instantiate(
                    "e2e-runtime-only-backend",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("deploy failed");
            let call_builder = contract.call_builder::<Flipper>();

            let old_balance = client
                .call(&ink_e2e::alice(), &call_builder.get_contract_balance())
                .submit()
                .await
                .expect("get_contract_balance failed")
                .return_value();

            const ENDOWMENT: u128 = 10;

            // when
            let call_data = vec![
                Value::unnamed_variant("Id", [Value::from_bytes(contract.account_id)]),
                Value::u128(ENDOWMENT),
            ];
            client
                .runtime_call(
                    &ink_e2e::alice(),
                    "Balances",
                    "transfer_allow_death",
                    call_data,
                )
                .await
                .expect("runtime call failed");

            // then
            let new_balance = client
                .call(&ink_e2e::alice(), &call_builder.get_contract_balance())
                .submit()
                .await
                .expect("get_contract_balance failed")
                .return_value();

            assert_eq!(old_balance + ENDOWMENT, new_balance);
            Ok(())
        }

        /// Just instantiate a contract using non-default runtime.
        #[ink_e2e::test(backend(runtime_only(sandbox = ink_e2e::DefaultSandbox)))]
        async fn custom_runtime<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            client
                .instantiate(
                    "e2e-runtime-only-backend",
                    &ink_e2e::alice(),
                    &mut FlipperRef::new(false),
                )
                .submit()
                .await
                .expect("instantiate failed");

            Ok(())
        }
    }
}
