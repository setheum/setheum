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
    use ink::{
        env::{
            call::{
                ExecutionInput,
                Selector,
            },
            DefaultEnvironment,
        },
        prelude::{
            format,
            string::{
                String,
                ToString,
            },
        },
    };

    #[ink(storage)]
    #[derive(Default)]
    pub struct CallBuilderReturnValue {
        /// Since we're going to `DelegateCall` into the `incrementer` contract, we need
        /// to make sure our storage layout matches.
        value: i32,
    }

    impl CallBuilderReturnValue {
        #[ink(constructor)]
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        /// Delegate a call to the given contract/selector and return the result.
        #[ink(message)]
        pub fn delegate_call(&mut self, code_hash: Hash, selector: [u8; 4]) -> i32 {
            use ink::env::call::build_call;

            build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<i32>()
                .invoke()
        }

        /// Delegate call to the given contract/selector and attempt to decode the return
        /// value into an `i8`.
        #[ink(message)]
        pub fn delegate_call_short_return_type(
            &mut self,
            code_hash: Hash,
            selector: [u8; 4],
        ) -> Result<i8, String> {
            use ink::env::call::build_call;

            let result = build_call::<DefaultEnvironment>()
                .delegate(code_hash)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<i8>()
                .try_invoke();

            match result {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(err)) => Err(format!("LangError: {:?}", err)),
                Err(ink::env::Error::Decode(_)) => Err("Decode Error".to_string()),
                Err(err) => Err(format!("Env Error: {:?}", err)),
            }
        }

        /// Forward a call to the given contract/selector and return the result.
        #[ink(message)]
        pub fn forward_call(&mut self, address: AccountId, selector: [u8; 4]) -> i32 {
            use ink::env::call::build_call;

            build_call::<DefaultEnvironment>()
                .call(address)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<i32>()
                .invoke()
        }

        /// Forward call to the given contract/selector and attempt to decode the return
        /// value into an `i8`.
        #[ink(message)]
        pub fn forward_call_short_return_type(
            &mut self,
            address: AccountId,
            selector: [u8; 4],
        ) -> Result<i8, String> {
            use ink::env::call::build_call;

            let result = build_call::<DefaultEnvironment>()
                .call(address)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<i8>()
                .try_invoke();

            match result {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(err)) => Err(format!("LangError: {:?}", err)),
                Err(ink::env::Error::Decode(_)) => Err("Decode Error".to_string()),
                Err(err) => Err(format!("Env Error: {:?}", err)),
            }
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use incrementer::IncrementerRef;
        use ink_e2e::{
            ChainBackend,
            ContractsBackend,
        };

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_delegate_call_return_value_returns_correct_value<
            Client: E2EBackend,
        >(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
                .await;

            let expected_value = 42;
            let mut constructor = CallBuilderReturnValueRef::new(expected_value);
            let contract = client
                .instantiate("call_builder_return_value", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderReturnValue>();

            let code_hash = client
                .upload("incrementer", &origin)
                .submit()
                .await
                .expect("upload `incrementer` failed")
                .code_hash;

            let selector = ink::selector_bytes!("get");
            let call = call_builder.delegate_call(code_hash, selector);
            let call_result = client
                .call(&origin, &call)
                .submit()
                .await
                .expect("Client failed to call `call_builder::invoke`.")
                .return_value();

            assert_eq!(
                call_result, expected_value,
                "Decoded an unexpected value from the call."
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_delegate_call_return_value_errors_if_return_data_too_long<
            Client: E2EBackend,
        >(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
                .await;

            let mut constructor = CallBuilderReturnValueRef::new(42);
            let contract = client
                .instantiate("call_builder_return_value", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderReturnValue>();

            let code_hash = client
                .upload("incrementer", &origin)
                .submit()
                .await
                .expect("upload `incrementer` failed")
                .code_hash;

            let selector = ink::selector_bytes!("get");
            let call = call_builder.delegate_call_short_return_type(code_hash, selector);
            let call_result: Result<i8, String> =
                client.call(&origin, &call).dry_run().await?.return_value();

            assert!(
                call_result.is_err(),
                "Should fail of decoding an `i32` into an `i8`"
            );
            assert_eq!(
                "Decode Error".to_string(),
                call_result.unwrap_err(),
                "Should fail to decode short type if bytes remain from return data."
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_forward_call_return_value_returns_correct_value<
            Client: E2EBackend,
        >(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
                .await;

            let mut constructor = CallBuilderReturnValueRef::new(0);
            let contract = client
                .instantiate("call_builder_return_value", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderReturnValue>();

            let expected_value = 42;
            let mut incrementer_constructor = IncrementerRef::new(expected_value);
            let incrementer = client
                .instantiate("incrementer", &origin, &mut incrementer_constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let selector = ink::selector_bytes!("get");
            let call = call_builder.forward_call(incrementer.account_id, selector);
            let call_result = client
                .call(&origin, &call)
                .submit()
                .await
                .expect("Client failed to call `call_builder::invoke`.")
                .return_value();

            assert_eq!(
                call_result, expected_value,
                "Decoded an unexpected value from the call."
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_forward_call_return_value_errors_if_return_data_too_long<
            Client: E2EBackend,
        >(
            mut client: Client,
        ) -> E2EResult<()> {
            let origin = client
                .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
                .await;

            let mut constructor = CallBuilderReturnValueRef::new(0);
            let contract = client
                .instantiate("call_builder_return_value", &origin, &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CallBuilderReturnValue>();

            let expected_value = 42;
            let mut incrementer_constructor = IncrementerRef::new(expected_value);
            let incrementer = client
                .instantiate("incrementer", &origin, &mut incrementer_constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let selector = ink::selector_bytes!("get");
            let call = call_builder
                .forward_call_short_return_type(incrementer.account_id, selector);
            let call_result: Result<i8, String> =
                client.call(&origin, &call).dry_run().await?.return_value();

            assert!(
                call_result.is_err(),
                "Should fail of decoding an `i32` into an `i8`"
            );
            assert_eq!(
                "Decode Error".to_string(),
                call_result.unwrap_err(),
                "Should fail to decode short type if bytes remain from return data."
            );

            Ok(())
        }
    }
}
