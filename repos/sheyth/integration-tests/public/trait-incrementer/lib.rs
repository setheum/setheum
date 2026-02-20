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
pub mod incrementer {
    use traits::{
        Increment,
        Reset,
    };

    /// A concrete incrementer smart contract.
    #[ink(storage)]
    pub struct Incrementer {
        value: u64,
    }

    impl Incrementer {
        /// Creates a new incrementer smart contract initialized with zero.
        #[ink(constructor)]
        pub fn new(init_value: u64) -> Self {
            Self { value: init_value }
        }

        /// Increases the value of the incrementer by an amount.
        #[ink(message)]
        pub fn inc_by(&mut self, delta: u64) {
            self.value = self.value.checked_add(delta).unwrap();
        }
    }

    impl Increment for Incrementer {
        #[ink(message)]
        fn inc(&mut self) {
            self.inc_by(1)
        }

        #[ink(message)]
        fn get(&self) -> u64 {
            self.value
        }
    }

    impl Reset for Incrementer {
        #[ink(message)]
        fn reset(&mut self) {
            self.value = 0;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let incrementer = Incrementer::new(0);
            assert_eq!(incrementer.get(), 0);
        }

        #[test]
        fn it_works() {
            let mut incrementer = Incrementer::new(0);
            // Can call using universal call syntax using the trait.
            assert_eq!(<Incrementer as Increment>::get(&incrementer), 0);
            <Incrementer as Increment>::inc(&mut incrementer);
            // Normal call syntax possible to as long as the trait is in scope.
            assert_eq!(incrementer.get(), 1);
        }
    }
}
