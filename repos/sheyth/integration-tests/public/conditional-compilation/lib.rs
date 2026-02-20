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

#[ink::trait_definition]
pub trait Flip {
    /// Flips the current value of the Flipper's boolean.
    #[ink(message)]
    fn flip(&mut self);

    /// Returns the current value of the Flipper's boolean.
    #[ink(message)]
    fn get(&self) -> bool;

    #[cfg(feature = "foo")]
    #[ink(message)]
    fn push_foo(&mut self, value: bool);
}

#[ink::contract]
pub mod conditional_compilation {
    use super::Flip;

    /// Feature gated event
    #[cfg(feature = "foo")]
    #[ink(event)]
    pub struct Changes {
        // attributing event field with `cfg` is not allowed
        new_value: bool,
        #[ink(topic)]
        by: AccountId,
    }

    /// Feature gated event
    #[cfg(feature = "bar")]
    #[ink(event)]
    pub struct ChangesDated {
        // attributing event field with `cfg` is not allowed
        new_value: bool,
        #[ink(topic)]
        by: AccountId,
        when: BlockNumber,
    }

    #[ink(storage)]
    pub struct ConditionalCompilation {
        value: bool,
    }

    impl ConditionalCompilation {
        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: Default::default(),
            }
        }

        /// Constructor that is included when `foo` is enabled
        #[cfg(feature = "foo")]
        #[ink(constructor)]
        pub fn new_foo(value: bool) -> Self {
            Self { value }
        }

        /// Constructor that is included when `bar` is enabled
        #[cfg(feature = "bar")]
        #[ink(constructor)]
        pub fn new_bar(value: bool) -> Self {
            Self { value }
        }

        /// Constructor that is included with either `foo` or `bar` features enabled
        #[cfg(feature = "foo")]
        #[cfg(feature = "bar")]
        #[ink(constructor)]
        pub fn new_foo_bar(value: bool) -> Self {
            Self { value }
        }

        #[cfg(feature = "foo")]
        #[ink(message)]
        pub fn inherent_flip_foo(&mut self) {
            self.value = !self.value;
            let caller = Self::env().caller();
            Self::env().emit_event(Changes {
                new_value: self.value,
                by: caller,
            });
        }

        #[cfg(feature = "bar")]
        #[ink(message)]
        pub fn inherent_flip_bar(&mut self) {
            let caller = Self::env().caller();
            let block_number = Self::env().block_number();
            self.value = !self.value;
            Self::env().emit_event(ChangesDated {
                new_value: self.value,
                by: caller,
                when: block_number,
            });
        }
    }

    impl Flip for ConditionalCompilation {
        #[ink(message)]
        fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        fn get(&self) -> bool {
            self.value
        }

        /// Feature gated mutating message
        #[cfg(feature = "foo")]
        #[ink(message)]
        fn push_foo(&mut self, value: bool) {
            let caller = Self::env().caller();
            Self::env().emit_event(Changes {
                new_value: value,
                by: caller,
            });
            self.value = value;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let flipper = ConditionalCompilation::new();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = ConditionalCompilation::new();
            // Can call using universal call syntax using the trait.
            assert!(!<ConditionalCompilation as Flip>::get(&flipper));
            <ConditionalCompilation as Flip>::flip(&mut flipper);
            // Normal call syntax possible to as long as the trait is in scope.
            assert!(flipper.get());
        }

        #[cfg(feature = "foo")]
        #[ink::test]
        fn foo_works() {
            let mut flipper = ConditionalCompilation::new_foo(false);

            flipper.inherent_flip_foo();
            assert!(flipper.get());

            <ConditionalCompilation as Flip>::push_foo(&mut flipper, false);
            assert!(!flipper.get())
        }
    }
}
