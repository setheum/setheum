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
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn message_0_ref(&self) {}

        #[ink(message)]
        pub fn message_0_mut(&mut self) {}

        #[ink(message)]
        pub fn message_1_ref(&self, _input_1: i8) {}

        #[ink(message)]
        pub fn message_1_mut(&mut self, _input_1: i8) {}

        #[ink(message)]
        pub fn message_2_ref(&self, _input_1: i8, _input_2: i16) {}

        #[ink(message)]
        pub fn message_2_mut(&mut self, _input_1: i8, _input_2: i16) {}

        #[ink(message)]
        pub fn message_3_ref(&self, _input_1: i8, _input_2: i16, _input_3: i32) {}

        #[ink(message)]
        pub fn message_3_mut(&mut self, _input_1: i8, _input_2: i16, _input_3: i32) {}

        #[ink(message)]
        pub fn message_4_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
        ) {
        }

        #[ink(message)]
        pub fn message_4_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
        ) {
        }

        #[ink(message)]
        pub fn message_5_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
        ) {
        }

        #[ink(message)]
        pub fn message_5_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
        ) {
        }

        #[ink(message)]
        pub fn message_6_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
        ) {
        }

        #[ink(message)]
        pub fn message_6_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
        ) {
        }

        #[ink(message)]
        pub fn message_7_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
        ) {
        }

        #[ink(message)]
        pub fn message_7_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
        ) {
        }

        #[ink(message)]
        pub fn message_8_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
        ) {
        }

        #[ink(message)]
        pub fn message_8_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
        ) {
        }

        #[ink(message)]
        pub fn message_9_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
            _input_9: u64,
        ) {
        }

        #[ink(message)]
        pub fn message_9_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
            _input_9: u64,
        ) {
        }

        #[ink(message)]
        pub fn message_10_ref(
            &self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
            _input_9: u64,
            _input_10: u128,
        ) {
        }

        #[ink(message)]
        pub fn message_10_mut(
            &mut self,
            _input_1: i8,
            _input_2: i16,
            _input_3: i32,
            _input_4: i64,
            _input_5: i128,
            _input_6: u8,
            _input_7: u16,
            _input_8: u32,
            _input_9: u64,
            _input_10: u128,
        ) {
        }
    }
}

fn main() {}
