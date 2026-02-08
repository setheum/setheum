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
pub mod wildcard_selector {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct WildcardSelector {}

    impl WildcardSelector {
        /// Creates a new wildcard selector smart contract.
        #[ink(constructor)]
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }

        /// Wildcard selector handles messages with any selector.
        #[ink(message, selector = _)]
        pub fn wildcard(&mut self) {
            let (_selector, _message) =
                ink::env::decode_input::<([u8; 4], String)>().unwrap();
            ink::env::debug_println!(
                "Wildcard selector: {:?}, message: {}",
                _selector,
                _message
            );
        }

        /// Wildcard complement handles messages with a well-known reserved selector.
        #[ink(message, selector = @)]
        pub fn wildcard_complement(&mut self, _message: String) {
            ink::env::debug_println!("Wildcard complement message: {}", _message);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        use ink::env::call::utils::{
            Argument,
            ArgumentList,
            EmptyArgumentList,
        };

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        type Environment = <WildcardSelectorRef as ink::env::ContractEnv>::Env;

        fn build_message(
            account_id: AccountId,
            selector: [u8; 4],
            message: String,
        ) -> ink_e2e::CallBuilderFinal<
            Environment,
            ArgumentList<Argument<String>, EmptyArgumentList>,
            (),
        > {
            ink::env::call::build_call::<Environment>()
                .call(account_id)
                .exec_input(
                    ink::env::call::ExecutionInput::new(ink::env::call::Selector::new(
                        selector,
                    ))
                    .push_arg(message),
                )
                .returns::<()>()
        }

        #[ink_e2e::test]
        async fn arbitrary_selectors_handled_by_wildcard<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = WildcardSelectorRef::new();
            let contract_acc_id = client
                .instantiate("wildcard_selector", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .account_id;

            // when
            const ARBITRARY_SELECTOR: [u8; 4] = [0xF9, 0xF9, 0xF9, 0xF9];
            let wildcard_message = "WILDCARD_MESSAGE 1".to_string();
            let wildcard = build_message(
                contract_acc_id,
                ARBITRARY_SELECTOR,
                wildcard_message.clone(),
            );

            let result = client
                .call(&ink_e2e::bob(), &wildcard)
                .submit()
                .await
                .expect("wildcard failed");

            const ARBITRARY_SELECTOR_2: [u8; 4] = [0x01, 0x23, 0x45, 0x67];
            let wildcard_message2 = "WILDCARD_MESSAGE 2".to_string();
            let wildcard2 = build_message(
                contract_acc_id,
                ARBITRARY_SELECTOR_2,
                wildcard_message2.clone(),
            );

            let result2 = client
                .call(&ink_e2e::bob(), &wildcard2)
                .submit()
                .await
                .expect("wildcard failed");

            // then
            assert!(result.debug_message().contains(&format!(
                "Wildcard selector: {:?}, message: {}",
                ARBITRARY_SELECTOR, wildcard_message
            )));

            assert!(result2.debug_message().contains(&format!(
                "Wildcard selector: {:?}, message: {}",
                ARBITRARY_SELECTOR_2, wildcard_message2
            )));

            Ok(())
        }

        #[ink_e2e::test]
        async fn wildcard_complement_works<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = WildcardSelectorRef::new();
            let contract_acc_id = client
                .instantiate("wildcard_selector", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .account_id;

            // when
            let wildcard_complement_message = "WILDCARD COMPLEMENT MESSAGE".to_string();
            let wildcard = build_message(
                contract_acc_id,
                ink::IIP2_WILDCARD_COMPLEMENT_SELECTOR,
                wildcard_complement_message.clone(),
            );

            let result = client
                .call(&ink_e2e::bob(), &wildcard)
                .submit()
                .await
                .expect("wildcard failed");

            // then
            assert!(result.debug_message().contains(&format!(
                "Wildcard complement message: {}",
                wildcard_complement_message
            )));

            Ok(())
        }
    }
}
