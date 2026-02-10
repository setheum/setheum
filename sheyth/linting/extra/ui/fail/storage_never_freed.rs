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

#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(dylint_lib = "ink_linting", deny(storage_never_freed))]
pub type MapAlias1<K, V> = ink::storage::Mapping<K, V>;
pub type MapAlias2<K, V> = MapAlias1<K, V>;

#[ink::contract]
pub mod storage_never_freed {
    use crate::MapAlias2;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct StorageNeverFreed {
        // All the fields generate warnings, since there are `insert` operations for
        // them, but there are no `remove` operations.
        vec_1: Vec<AccountId>,
        vec_2: Vec<bool>,
        vec_subscription: Vec<AccountId>,
        map_1: Mapping<AccountId, AccountId>,
        map_2: Mapping<AccountId, AccountId>,
        map_3: Mapping<AccountId, AccountId>,
        map_alias: MapAlias2<AccountId, AccountId>,
    }

    impl StorageNeverFreed {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                vec_1: Vec::new(),
                vec_2: Vec::new(),
                vec_subscription: Vec::new(),
                map_1: Mapping::new(),
                map_2: Mapping::new(),
                map_3: Mapping::new(),
                map_alias: Mapping::new(),
            }
        }

        fn flip(a: bool) -> bool {
            !a
        }

        #[ink(message)]
        pub fn add_to_fields(&mut self, v: AccountId) {
            self.vec_1.push(v);
            self.vec_subscription[0] = v;
            self.map_1.insert(v, &v);
            let _ = Self::flip(self.map_2.insert(v, &v).is_some());
            self.map_alias.insert(v, &v);
            self.vec_2.push(self.map_3.insert(v, &v).is_some());
        }
    }
}

fn main() {}
