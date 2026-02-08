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

#[ink::contract]
mod contract_callee {
    #[ink(storage)]
    pub struct Callee {}

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        Foo,
    }

    impl Callee {
        #[ink(constructor)]
        pub fn new_self() -> Self {
            Self {}
        }

        #[ink(constructor)]
        pub fn new_storage_name() -> Callee {
            Callee {}
        }

        #[ink(constructor)]
        pub fn new_result_self() -> Result<Self, Error> {
            Ok(Self {})
        }

        #[ink(constructor)]
        pub fn new_result_storage_name() -> Result<Callee, Error> {
            Ok(Callee {})
        }

        #[ink(message)]
        pub fn message(&self) {}
    }
}

fn main() {
    use contract_callee::{
        CalleeRef,
        Error,
    };

    // fn new_self() -> Self
    let _: fn() -> CalleeRef = || {
        CalleeRef::new_self()
            .code_hash(ink_primitives::Clear::CLEAR_HASH)
            .endowment(25)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate()
    };

    // fn new_storage_name() -> Callee
    let _: fn() -> CalleeRef = || {
        CalleeRef::new_storage_name()
            .code_hash(ink_primitives::Clear::CLEAR_HASH)
            .endowment(25)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate()
    };

    // fn new_result_self() -> Result<Self, Error>
    let _: fn() -> Result<CalleeRef, Error> = || {
        CalleeRef::new_result_self()
            .code_hash(ink_primitives::Clear::CLEAR_HASH)
            .endowment(25)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate()
    };

    // fn new_result_storage_name() -> Result<Callee, Error>
    let _: fn() -> Result<CalleeRef, Error> = || {
        CalleeRef::new_result_self()
            .code_hash(ink_primitives::Clear::CLEAR_HASH)
            .endowment(25)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate()
    };
}
