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
pub mod caller {
    use dyn_traits::Increment;

    /// The caller of the incrementer smart contract.
    #[ink(storage)]
    pub struct Caller {
        /// Here we accept a type which implements the `Incrementer` ink! trait.
        incrementer: ink::contract_ref!(Increment),
    }

    impl Caller {
        /// Creates a new caller smart contract around the `incrementer` account id.
        #[ink(constructor)]
        pub fn new(incrementer: AccountId) -> Self {
            Self {
                incrementer: incrementer.into(),
            }
        }
    }

    impl Increment for Caller {
        #[ink(message)]
        fn inc(&mut self) {
            self.incrementer.inc()
        }

        #[ink(message)]
        fn get(&self) -> u64 {
            self.incrementer.get()
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::caller::{
        Caller,
        CallerRef,
    };
    use dyn_traits::Increment;
    use ink_e2e::ContractsBackend;
    use trait_incrementer::incrementer::{
        Incrementer,
        IncrementerRef,
    };

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    /// A test deploys and instantiates the `trait_incrementer::Incrementer` and
    /// `trait_incrementer_caller::Caller` contracts, where the `Caller` uses the account
    /// id of the `Incrementer` for instantiation.
    ///
    /// The test verifies that we can increment the value of the `Incrementer` contract
    /// through the `Caller` contract.
    #[ink_e2e::test]
    async fn e2e_cross_contract_calls<Client: E2EBackend>(
        mut client: Client,
    ) -> E2EResult<()> {
        let _ = client
            .upload("trait-incrementer", &ink_e2e::alice())
            .submit()
            .await
            .expect("uploading `trait-incrementer` failed")
            .code_hash;

        let _ = client
            .upload("trait-incrementer-caller", &ink_e2e::alice())
            .submit()
            .await
            .expect("uploading `trait-incrementer-caller` failed")
            .code_hash;

        let mut constructor = IncrementerRef::new();

        let incrementer = client
            .instantiate("trait-incrementer", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let incrementer_call = incrementer.call_builder::<Incrementer>();

        let mut constructor = CallerRef::new(incrementer.account_id.clone());

        let caller = client
            .instantiate(
                "trait-incrementer-caller",
                &ink_e2e::alice(),
                &mut constructor,
            )
            .submit()
            .await
            .expect("instantiate failed");
        let mut caller_call = caller.call_builder::<Caller>();

        // Check through the caller that the value of the incrementer is zero
        let get = caller_call.get();
        let value = client
            .call(&ink_e2e::alice(), &get)
            .dry_run()
            .await?
            .return_value();
        assert_eq!(value, 0);

        // Increment the value of the incrementer via the caller
        let inc = caller_call.inc();
        let _ = client
            .call(&ink_e2e::alice(), &inc)
            .submit()
            .await
            .expect("calling `inc` failed");

        // Ask the `trait-increment` about a value. It should be updated by the caller.
        // Also use `contract_ref!(Increment)` instead of `IncrementerRef`
        // to check that it also works with e2e testing.
        let get = incrementer_call.get();
        let value = client
            .call(&ink_e2e::alice(), &get)
            .dry_run()
            .await?
            .return_value();
        assert_eq!(value, 1);

        Ok(())
    }
}
