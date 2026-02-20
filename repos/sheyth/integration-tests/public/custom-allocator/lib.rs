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

// Here we set `dlmalloc` to be the global memory allocator.
//
// The [`GlobalAlloc`](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) trait is
// important to understand if you're swapping our your allocator.
#[cfg(not(feature = "std"))]
#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[ink::contract]
mod custom_allocator {
    use ink::prelude::{
        vec,
        vec::Vec,
    };

    #[ink(storage)]
    pub struct CustomAllocator {
        /// Stores a single `bool` value on the storage.
        ///
        /// # Note
        ///
        /// We're using a `Vec` here as it allocates its elements onto the heap, as
        /// opposed to the stack. This allows us to demonstrate that our new
        /// allocator actually works.
        value: Vec<bool>,
    }

    impl CustomAllocator {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: vec![init_value],
            }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value[0] = !self.value[0];
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value[0]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let custom_allocator = CustomAllocator::default();
            assert!(!custom_allocator.get());
        }

        #[ink::test]
        fn it_works() {
            let mut custom_allocator = CustomAllocator::new(false);
            assert!(!custom_allocator.get());
            custom_allocator.flip();
            assert!(custom_allocator.get());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default
        /// constructor.
        #[ink_e2e::test]
        async fn default_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // Given
            let mut constructor = CustomAllocatorRef::default();

            // When
            let contract = client
                .instantiate("custom_allocator", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<CustomAllocator>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract
        /// contract.
        #[ink_e2e::test]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // Given
            let mut constructor = CustomAllocatorRef::new(false);
            let contract = client
                .instantiate("custom_allocator", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<CustomAllocator>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
