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

pub type TyAlias1 = ink::prelude::vec::Vec<i32>;
pub type TyAlias2 = TyAlias1;

#[ink::contract]
pub mod non_fallible_api {
    use crate::TyAlias2;
    use ink::{
        prelude::{
            string::String,
            vec::Vec,
        },
        storage::{
            Lazy,
            Mapping,
            StorageVec,
        },
    };

    #[ink(storage)]
    pub struct NonFallibleAPI {
        map_1: Mapping<String, String>,
        map_2: Mapping<i32, TyAlias2>,
        lazy_1: Lazy<String>,
        lazy_2: Lazy<(String, String)>,
        vec_1: StorageVec<String>,
    }

    impl NonFallibleAPI {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                map_1: Mapping::new(),
                map_2: Mapping::new(),
                lazy_1: Lazy::new(),
                lazy_2: Lazy::new(),
                vec_1: StorageVec::new(),
            }
        }

        // Raise warnings when using non-fallible API with argument which encoded size is
        // statically unknown.
        #[ink(message)]
        pub fn non_fallible_not_statically_known(&mut self, a: String, b: String) {
            // Mapping
            let _ = self.map_1.insert(a.clone(), &b);
            let _ = self.map_1.get(a.clone());
            let _ = self.map_1.take(a.clone());
            let mut v = Vec::new();
            v.push(42);
            let _ = self.map_2.insert(42, &v);

            // Lazy
            let _ = self.lazy_1.get();
            self.lazy_1.set(&a);
            self.lazy_2.set(&(a.clone(), a.clone()));

            // StorageVec
            let _ = self.vec_1.peek();
            let _ = self.vec_1.get(0);
            self.vec_1.set(0, &a.clone());
            let _ = self.vec_1.pop();
            self.vec_1.push(&a.clone());
        }
    }
}

fn main() {}
