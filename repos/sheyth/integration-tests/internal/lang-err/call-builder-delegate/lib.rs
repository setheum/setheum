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
mod call_builder {
    use ink::env::{
        call::{
            build_call,
            ExecutionInput,
            Selector,
        },
        DefaultEnvironment,
    };

    #[ink(storage)]
    #[derive(Default)]
    pub struct CallBuilderDelegateTest {
        /// Since we're going to `DelegateCall` into the `incrementer` contract, we need
        /// to make sure our storage layout matches.
        value: i32,
    }

    impl CallBuilderDelegateTest {
        #[ink(constructor)]
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        /// Call a contract using the `CallBuilder`.
        ///
        /// Since we can't use the `CallBuilder` in a test environment directly we need
        /// this wrapper to test things like crafting calls with invalid
        /// selectors.
        ///
        /// We also wrap the output in an `Option` since we can't return a `Result`
        /// directly from a contract message without erroring out ourselves.
        #[ink(message)]
        pub fn delegate(
            &mut self,
            code_hash: Hash,
            selector: [u8; 4],
        ) -> Option<ink::LangError> {
            let result = build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<bool>()
                .try_invoke()
                .expect("Error from the Contracts pallet.");

            match result {
                Ok(_) => None,
                Err(e @ ink::LangError::CouldNotReadInput) => Some(e),
                Err(_) => {
                    unimplemented!("No other `LangError` variants exist at the moment.")
                }
            }
        }

        /// Call a contract using the `CallBuilder`.
        ///
        /// Since we can't use the `CallBuilder` in a test environment directly we need
        /// this wrapper to test things like crafting calls with invalid
        /// selectors.
        ///
        /// This message does not allow the caller to handle any `LangErrors`, for that
        /// use the `call` message instead.
        #[ink(message)]
        pub fn invoke(&mut self, code_hash: Hash, selector: [u8; 4]) -> i32 {
            use ink::env::call::build_call;

            build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<i32>()
                .invoke()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::{
            ChainBackend,
            ContractsBackend,
        };

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_invalid_message_selector_can_be_handled<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::bob(), 10_000_000_000_000)
                .await;

            let mut constructor = CallBuilderDelegateTestRef::new(Default::default());
            let contract = client
                .instantiate("call_builder_delegate", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderDelegateTest>();

            let code_hash = client
                .upload("incrementer", &origin)
                .submit()
                .await
                .expect("upload `incrementer` failed")
                .code_hash;

            let selector = ink::selector_bytes!("invalid_selector");
            let call = call_builder.delegate(code_hash, selector);
            let call_result = client
                .call(&origin, &call)
                .submit()
                .await
                .expect("Calling `call_builder::delegate` failed");

            assert!(matches!(
                call_result.return_value(),
                Some(ink::LangError::CouldNotReadInput)
            ));

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_invalid_message_selector_panics_on_invoke<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::charlie(), 10_000_000_000_000)
                .await;

            let mut constructor = CallBuilderDelegateTestRef::new(Default::default());
            let contract = client
                .instantiate("call_builder_delegate", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderDelegateTest>();

            let code_hash = client
                .upload("incrementer", &origin)
                .submit()
                .await
                .expect("upload `incrementer` failed")
                .code_hash;

            // Since `LangError`s can't be handled by the `CallBuilder::invoke()` method
            // we expect this to panic.
            let selector = ink::selector_bytes!("invalid_selector");
            let call = call_builder.invoke(code_hash, selector);
            let call_result = client.call(&origin, &call).dry_run().await;

            if let Err(ink_e2e::Error::CallDryRun(dry_run)) = call_result {
                assert!(dry_run
                    .debug_message
                    .contains("Cross-contract call failed with CouldNotReadInput"));
            } else {
                panic!("Expected call to fail");
            }

            Ok(())
        }
    }
}
