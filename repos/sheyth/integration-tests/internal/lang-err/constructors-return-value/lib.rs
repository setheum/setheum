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

pub use self::constructors_return_value::{
    ConstructorError,
    ConstructorsReturnValue,
    ConstructorsReturnValueRef,
};

#[ink::contract]
pub mod constructors_return_value {
    #[ink(storage)]
    pub struct ConstructorsReturnValue {
        value: bool,
    }

    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct ConstructorError;

    impl ConstructorsReturnValue {
        /// Infallible constructor
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Fallible constructor
        #[ink(constructor)]
        pub fn try_new(succeed: bool) -> Result<Self, ConstructorError> {
            if succeed {
                Ok(Self::new(true))
            } else {
                Err(ConstructorError)
            }
        }

        /// A constructor which reverts and fills the output buffer with an erroneously
        /// encoded return value.
        #[ink(constructor)]
        pub fn revert_new(_init_value: bool) -> Self {
            ink::env::return_value::<ink::ConstructorResult<AccountId>>(
                ink::env::ReturnFlags::REVERT,
                &Ok(AccountId::from([0u8; 32])),
            )
        }

        /// A constructor which reverts and fills the output buffer with an erroneously
        /// encoded return value.
        #[ink(constructor)]
        pub fn try_revert_new(init_value: bool) -> Result<Self, ConstructorError> {
            let value = if init_value {
                Ok(Ok(AccountId::from([0u8; 32])))
            } else {
                Err(ink::LangError::CouldNotReadInput)
            };

            ink::env::return_value::<
                ink::ConstructorResult<Result<AccountId, ConstructorError>>,
            >(ink::env::ReturnFlags::REVERT, &value)
        }

        /// Returns the current value of the contract storage.
        #[ink(message)]
        pub fn get_value(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::ConstructorsReturnValue as Contract;
        use std::any::TypeId;

        #[test]
        #[allow(clippy::assertions_on_constants)]
        fn infallible_constructor_reflection() {
            const ID: u32 = ::ink::selector_id!("new");

            assert!(
                !<Contract as ::ink::reflect::DispatchableConstructorInfo<ID>>::IS_RESULT,
            );
            assert_eq!(
                TypeId::of::<
                    <Contract as ::ink::reflect::DispatchableConstructorInfo<ID>>::Error,
                >(),
                TypeId::of::<&()>(),
            )
        }

        #[test]
        #[allow(clippy::assertions_on_constants)]
        fn fallible_constructor_reflection() {
            const ID: u32 = ::ink::selector_id!("try_new");

            assert!(
                <Contract as ::ink::reflect::DispatchableConstructorInfo<ID>>::IS_RESULT,
            );
            assert_eq!(
                TypeId::of::<
                    <Contract as ::ink::reflect::DispatchableConstructorInfo<ID>>::Error,
                >(),
                TypeId::of::<super::ConstructorError>(),
            )
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_infallible_constructor<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let mut constructor = ConstructorsReturnValueRef::new(true);
            let infallible_constructor_result = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .dry_run()
                .await?;

            let decoded_result = infallible_constructor_result.constructor_result::<()>();
            assert!(
                decoded_result.is_ok(),
                "Constructor dispatch should have succeeded"
            );

            let mut constructor = ConstructorsReturnValueRef::new(true);
            let success = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .is_ok();

            assert!(success, "Contract created successfully");

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_fallible_constructor_succeed<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let mut constructor = ConstructorsReturnValueRef::try_new(true);
            let result = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::bob(),
                    &mut constructor,
                )
                .dry_run()
                .await?;

            let decoded_result =
                result.constructor_result::<Result<(), ConstructorError>>();

            assert!(
                decoded_result.is_ok(),
                "Constructor dispatch should have succeeded"
            );

            assert!(
                decoded_result.unwrap().is_ok(),
                "Fallible constructor should have succeeded"
            );

            let mut constructor = ConstructorsReturnValueRef::try_new(true);
            let contract = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::bob(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<ConstructorsReturnValue>();

            let get = call_builder.get_value();
            let value = client
                .call(&ink_e2e::bob(), &get)
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                true, value,
                "Contract success should write to contract storage"
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_fallible_constructor_fails<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let mut constructor = ConstructorsReturnValueRef::try_new(false);

            let result = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::charlie(),
                    &mut constructor,
                )
                .dry_run()
                .await?;

            let decoded_result =
                result.constructor_result::<Result<(), ConstructorError>>();

            assert!(
                decoded_result.is_ok(),
                "Constructor dispatch should have succeeded"
            );

            assert!(
                decoded_result.unwrap().is_err(),
                "Fallible constructor should have failed"
            );

            let mut constructor = ConstructorsReturnValueRef::try_new(false);
            let result = client
                .instantiate(
                    "constructors_return_value",
                    &ink_e2e::charlie(),
                    &mut constructor,
                )
                .submit()
                .await;

            assert!(
                matches!(result, Err(ink_e2e::Error::InstantiateExtrinsic(_))),
                "Constructor should fail"
            );

            Ok(())
        }
    }
}
