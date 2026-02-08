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

#[ink::contract]
pub mod strict_balance_equality {
    #[ink(storage)]
    pub struct StrictBalanceEquality {}

    impl StrictBalanceEquality {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        // Return value tainted with balance
        fn get_balance_1(&self) -> Balance {
            self.env().balance()
        }
        fn get_balance_2(&self) -> Balance {
            let tmp = self.env().balance();
            tmp
        }
        fn get_balance_3(&self) -> Balance {
            let tmp = self.env().balance();
            tmp + 42
        }
        fn get_balance_recursive(&self, acc: &Balance) -> Balance {
            if acc < &10_u128 {
                self.get_balance_recursive(&(acc + 1))
            } else {
                self.env().balance()
            }
        }

        // Return the result of non-strict comparison with balance
        fn cmp_balance_1(&self, value: &Balance) -> bool {
            *value < self.env().balance()
        }
        fn cmp_balance_2(&self, value: &Balance, threshold: &Balance) -> bool {
            value > threshold
        }
        fn cmp_balance_3(&self, value: Balance, threshold: Balance) -> bool {
            value >= threshold
        }

        // `&mut` input argument gets the balance value
        fn get_balance_arg_1(&self, value: &mut Balance) {
            *value = self.env().balance();
        }
        fn get_balance_arg_indirect(&self, value: &mut Balance) {
            self.get_balance_arg_1(value)
        }

        #[ink(message)]
        pub fn do_nothing(&mut self) {
            let threshold: Balance = 100;
            let value: Balance = self.env().balance();

            // Good: Non-strict equality with balance
            if self.env().balance() < 10 { /* ... */ }
            if value > 11 { /* ... */ }
            if self.env().balance() < threshold { /* ... */ }

            // Good: Non-strict equality in function call: return value
            if self.get_balance_1() < 10 { /* ... */ }
            if self.get_balance_2() > 10 { /* ... */ }
            if self.get_balance_3() >= 10 { /* ... */ }
            if self.get_balance_recursive(&10) <= 10 { /* ... */ }

            // Good: Non-strict equality in function call: return value contains the
            // result of comparison
            if self.cmp_balance_1(&10) { /* ... */ }
            if self.cmp_balance_2(&self.env().balance(), &threshold) { /* ... */ }
            if self.cmp_balance_3(self.env().balance(), threshold) { /* ... */ }

            // Good: Non-strict equality in function: tainted arguments
            let mut res_1 = 0_u128;
            self.get_balance_arg_1(&mut res_1);
            if res_1 < 10 { /* ... */ }
            let mut res_2 = 0_u128;
            self.get_balance_arg_indirect(&mut res_2);
            if res_2 > 10 { /* ... */ }

            // Good: warning is suppressed
            #[cfg_attr(dylint_lib = "ink_linting", allow(strict_balance_equality))]
            if self.env().balance() == 10 { /* ... */ }
        }
    }
}

fn main() {}
