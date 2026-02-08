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

use ink_primitives::AccountId;

pub type TyAlias1 = AccountId;
pub type TyAlias2 = TyAlias1;

#[ink::contract]
pub mod non_fallible_api {
    use crate::TyAlias2;
    use ink::{
        prelude::string::String,
        storage::{
            Lazy,
            Mapping,
            StorageVec,
        },
    };

    #[ink(storage)]
    pub struct NonFallibleAPI {
        map_1: Mapping<AccountId, AccountId>,
        map_2: Mapping<AccountId, [AccountId; 1]>,
        map_3: Mapping<AccountId, (AccountId, AccountId)>,
        lazy_1: Lazy<AccountId>,
        lazy_2: Lazy<TyAlias2>,
        lazy_3: Lazy<String>,
        vec_1: StorageVec<AccountId>,
    }

    impl NonFallibleAPI {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                map_1: Mapping::new(),
                map_2: Mapping::new(),
                map_3: Mapping::new(),
                lazy_1: Lazy::new(),
                lazy_2: Lazy::new(),
                lazy_3: Lazy::new(),
                vec_1: StorageVec::new(),
            }
        }

        // Don't generate warnings when using the fallible API
        #[ink(message)]
        pub fn fallible(&mut self, a: AccountId, b: AccountId) {
            // Mapping
            let _ = self.map_1.try_insert(a, &b);
            let _ = self.map_1.try_get(a);
            let _ = self.map_1.try_take(a);

            // Lazy
            let _ = self.lazy_1.try_get();
            let _ = self.lazy_1.try_set(&a);

            // StorageVec
            let _ = self.vec_1.try_peek();
            let _ = self.vec_1.try_get(0);
            let _ = self.vec_1.try_set(0, &a);
            let _ = self.vec_1.try_pop();
            let _ = self.vec_1.try_push(&a);
        }

        // Don't raise warnings when using non-fallible API with argument which encoded
        // size is statically known.
        #[ink(message)]
        pub fn non_fallible_statically_known(&mut self, a: AccountId, b: AccountId) {
            // Mapping
            let _ = self.map_1.insert(a, &b);
            let _ = self.map_1.get(a);
            let _ = self.map_1.take(a);
            let _ = self.map_2.insert(a, &[b; 1]);
            let _ = self.map_3.insert(a, &(b, b));

            // Lazy
            let _ = self.lazy_1.get();
            self.lazy_1.set(&a);
            let _ = self.lazy_2.get();
            self.lazy_2.set(&a);

            // StorageVec
            let _ = self.vec_1.peek();
            let _ = self.vec_1.get(0);
            self.vec_1.set(0, &a);
            let _ = self.vec_1.pop();
            self.vec_1.push(&a);
        }

        // Check if local suppressions work
        #[ink(message)]
        pub fn suppressions(&mut self, a: String) {
            #[cfg_attr(dylint_lib = "ink_linting", allow(non_fallible_api))]
            self.lazy_3.set(&a);
        }
    }
}

fn main() {}
