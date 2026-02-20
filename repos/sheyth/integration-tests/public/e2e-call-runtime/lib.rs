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
pub mod e2e_call_runtime {
    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

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

        #[ink_e2e::test]
        async fn call_runtime_works<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("e2e_call_runtime", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Contract>();

            let transfer_amount = 100_000_000_000u128;

            // when
            let call_data = vec![
                // A value representing a `MultiAddress<AccountId32, _>`. We want the
                // "Id" variant, and that will ultimately contain the
                // bytes for our destination address
                Value::unnamed_variant("Id", [Value::from_bytes(&contract.account_id)]),
                // A value representing the amount we'd like to transfer.
                Value::u128(transfer_amount),
            ];

            let get_balance = call_builder.get_contract_balance();
            let pre_balance = client
                .call(&ink_e2e::alice(), &get_balance)
                .dry_run()
                .await?
                .return_value();

            // Send funds from Alice to the contract using Balances::transfer
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
            let get_balance = call_builder.get_contract_balance();
            let get_balance_res = client
                .call(&ink_e2e::alice(), &get_balance)
                .dry_run()
                .await?;

            assert_eq!(
                get_balance_res.return_value(),
                pre_balance + transfer_amount
            );

            Ok(())
        }
    }
}
