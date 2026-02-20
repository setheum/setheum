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
#![allow(clippy::new_without_default)]

#[ink::contract]
pub mod just_terminates {
    /// No storage is needed for this simple contract.
    #[ink(storage)]
    pub struct JustTerminate {}

    impl JustTerminate {
        /// Creates a new instance of this contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Terminates with the caller as beneficiary.
        #[ink(message)]
        pub fn terminate_me(&mut self) {
            self.env().terminate_contract(self.env().caller());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn terminating_works() {
            // given
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
                contract_id,
                100,
            );
            let mut contract = JustTerminate::new();

            // when
            let should_terminate = move || contract.terminate_me();

            // then
            ink::env::test::assert_contract_termination::<ink::env::DefaultEnvironment, _>(
                should_terminate,
                accounts.alice,
                100,
            );
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_contract_terminates<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = JustTerminateRef::new();
            let contract = client
                .instantiate("contract_terminate", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<JustTerminate>();

            // when
            let terminate_me = call_builder.terminate_me();
            let call_res = client
                .call(&ink_e2e::alice(), &terminate_me)
                .submit()
                .await
                .expect("terminate_me messages failed");

            assert!(
                call_res.return_data().is_empty(),
                "Terminated contract never returns"
            );

            // then
            assert!(call_res.contains_event("System", "KilledAccount"));
            assert!(call_res.contains_event("Balances", "Withdraw"));
            assert!(call_res.contains_event("Contracts", "Terminated"));

            Ok(())
        }
    }
}
