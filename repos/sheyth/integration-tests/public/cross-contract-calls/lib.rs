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
mod cross_contract_calls {
    use ink::codegen::TraitCallBuilder;
    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct CrossContractCalls {
        other_contract: OtherContractRef,
    }

    impl CrossContractCalls {
        /// Initializes the contract by instantiating the code at the given code hash via
        /// `instantiate_v2` host function with the supplied weight and storage
        /// limits.
        #[ink(constructor)]
        pub fn new_v2_with_limits(
            other_contract_code_hash: Hash,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) -> Self {
            let other_contract = OtherContractRef::new(true)
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .instantiate();

            Self { other_contract }
        }

        /// Initializes the contract by instantiating the code at the given code hash via
        /// the `instantiate_v2` host function with no weight or storage limits.
        #[ink(constructor)]
        pub fn new_v2_no_limits(other_contract_code_hash: Hash) -> Self {
            let other_contract = OtherContractRef::new(true)
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_contract }
        }

        /// Initializes the contract by instantiating the code at the given code hash via
        /// the original `instantiate` host function.
        #[ink(constructor)]
        pub fn new_v1(other_contract_code_hash: Hash) -> Self {
            let other_contract = OtherContractRef::new(true)
                .instantiate_v1()
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { other_contract }
        }

        /// Basic invocation of the other contract via the contract reference.
        ///
        /// *Note* this will invoke the original `call` (V1) host function, which will be
        /// deprecated in the future.
        #[ink(message)]
        pub fn flip_and_get_v1(&mut self) -> bool {
            let call_builder = self.other_contract.call_mut();

            call_builder.flip().call_v1().invoke();
            call_builder.get().call_v1().invoke()
        }

        /// Use the new `call_v2` host function via the call builder to forward calls to
        /// the other contract, initially calling `flip` and then `get` to return the
        /// result.
        ///
        /// This demonstrates how to set the new weight and storage limit parameters via
        /// the call builder API.
        #[ink(message)]
        pub fn flip_and_get_invoke_v2_with_limits(
            &mut self,
            ref_time_limit: u64,
            proof_size_limit: u64,
            storage_deposit_limit: Balance,
        ) -> bool {
            let call_builder = self.other_contract.call_mut();

            call_builder
                .flip()
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .invoke();

            call_builder
                .get()
                .ref_time_limit(ref_time_limit)
                .proof_size_limit(proof_size_limit)
                .storage_deposit_limit(storage_deposit_limit)
                .invoke()
        }

        /// Demonstrate that the `call_v2` succeeds without having specified the weight
        /// and storage limit parameters
        #[ink(message)]
        pub fn flip_and_get_invoke_v2_no_weight_limit(&mut self) -> bool {
            self.other_contract.flip();
            self.other_contract.get()
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;
