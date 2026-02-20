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
pub type MapAlias<K, V> = ink::storage::Mapping<K, V>;

#[ink::contract]
pub mod storage_never_freed {
    use crate::MapAlias;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct StorageNeverFreed {
        vec_1: Vec<AccountId>,
        map_1: Mapping<AccountId, AccountId>,
        map_2: MapAlias<AccountId, AccountId>,
        #[cfg_attr(dylint_lib = "ink_linting", allow(storage_never_freed))]
        map_field_suppressed: Mapping<AccountId, AccountId>,

        // Vec which buffer was used unsafe operations with their raw pointers are not
        // reported
        vec_field_mut_pointer: Vec<AccountId>,
    }

    impl StorageNeverFreed {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                vec_1: Vec::new(),
                map_1: Mapping::new(),
                map_2: Mapping::new(),
                map_field_suppressed: Mapping::new(),
                vec_field_mut_pointer: Vec::new(),
            }
        }

        #[ink(message)]
        pub fn add_to_fields(&mut self, v: AccountId) {
            self.vec_1.push(v);
            self.map_1.insert(v, &v);
            self.map_2.insert(v, &v);
            self.map_field_suppressed.insert(v, &v);

            // Should not be reported, since elements may be removed using the pointer
            self.vec_field_mut_pointer[0] = v;
            unsafe {
                let ptr = self.vec_field_mut_pointer.as_mut_ptr();
                let new_len = self.vec_field_mut_pointer.len() - 1;
                std::ptr::copy(ptr.offset(1), ptr, new_len);
                self.vec_field_mut_pointer.set_len(new_len);
            }
        }

        #[ink(message)]
        pub fn remove_from_fields(&mut self, v: AccountId) {
            self.vec_1.pop();
            self.map_1.remove(v);
            self.map_2.remove(v);
        }
    }
}

fn main() {}
