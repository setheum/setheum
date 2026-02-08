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

pub use self::integration_flipper::{
    Flipper,
    FlipperRef,
};

#[ink::contract]
pub mod integration_flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    #[derive(Debug)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct FlipperError;

    impl Flipper {
        /// Creates a new integration_flipper smart contract initialized with the given
        /// value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new integration_flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Attempts to create a new integration_flipper smart contract initialized with
        /// the given value.
        #[ink(constructor)]
        pub fn try_new(succeed: bool) -> Result<Self, FlipperError> {
            if succeed {
                Ok(Self::new(true))
            } else {
                Err(FlipperError)
            }
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

        /// Flips the current value of the Flipper's boolean.
        ///
        /// We should see the state being reverted here, no write should occur.
        #[ink(message)]
        #[allow(clippy::result_unit_err)]
        pub fn err_flip(&mut self) -> Result<(), ()> {
            self.flip();
            Err(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_can_flip_correctly<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let mut constructor = FlipperRef::new_default();
            let flipper = client
                .instantiate("integration_flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("Instantiate `integration_flipper` failed");
            let mut call_builder = flipper.call_builder::<Flipper>();

            let get = call_builder.get();
            let initial_value = client
                .call(&ink_e2e::alice(), &get)
                .dry_run()
                .await?
                .return_value();

            let flip = call_builder.flip();
            let flip_call_result = client
                .call(&ink_e2e::alice(), &flip)
                .submit()
                .await
                .expect("Calling `flip` failed");
            assert!(
                flip_call_result.message_result().is_ok(),
                "Messages now return a `Result`, which should be `Ok` here."
            );

            let flipped_value = client
                .call(&ink_e2e::alice(), &get)
                .dry_run()
                .await?
                .return_value();
            assert!(flipped_value != initial_value);

            Ok(())
        }

        #[ink_e2e::test]
        async fn e2e_message_error_reverts_state<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            let mut constructor = FlipperRef::new_default();
            let flipper = client
                .instantiate("integration_flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = flipper.call_builder::<Flipper>();

            let get = call_builder.get();
            let initial_value = client
                .call(&ink_e2e::bob(), &get)
                .dry_run()
                .await?
                .return_value();

            let err_flip = call_builder.err_flip();
            let err_flip_call_result =
                client.call(&ink_e2e::bob(), &err_flip).submit().await;

            assert!(matches!(
                err_flip_call_result,
                Err(ink_e2e::Error::CallExtrinsic(_))
            ));

            let flipped_value = client
                .call(&ink_e2e::bob(), &get)
                .dry_run()
                .await?
                .return_value();
            assert!(flipped_value == initial_value);

            Ok(())
        }
    }
}
