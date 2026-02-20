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

    #[ink(event)]
    pub struct Event0 {}

    #[ink(event)]
    pub struct Event1 {
        #[ink(topic)]
        arg_1: i8,
    }

    #[ink(event)]
    pub struct Event2 {
        #[ink(topic)]
        arg_1: i8,
        #[ink(topic)]
        arg_2: i16,
    }

    #[ink(event)]
    pub struct Event3 {
        #[ink(topic)]
        arg_1: i8,
        #[ink(topic)]
        arg_2: i16,
        #[ink(topic)]
        arg_3: i32,
    }

    #[ink(event)]
    pub struct Event4 {
        #[ink(topic)]
        arg_1: i8,
        #[ink(topic)]
        arg_2: i16,
        #[ink(topic)]
        arg_3: i32,
        #[ink(topic)]
        arg_4: i64,
    }

    #[ink(event)]
    pub struct Event5 {
        #[ink(topic)]
        arg_1: i8,
        #[ink(topic)]
        arg_2: i16,
        #[ink(topic)]
        arg_3: i32,
        #[ink(topic)]
        arg_4: i64,
        // #[ink(topic)] <- Cannot have more than 4 topics by default.
        arg_5: i128,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Self {
            Self::env().emit_event(Event0 {});
            Self::env().emit_event(Event1 { arg_1: 1 });
            Self::env().emit_event(Event2 { arg_1: 1, arg_2: 2 });
            Self::env().emit_event(Event3 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
            });
            Self::env().emit_event(Event4 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
            });
            Self::env().emit_event(Event5 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
                arg_5: 5,
            });
            Self {}
        }

        #[ink(message)]
        pub fn message(&self) {
            self.env().emit_event(Event0 {});
            self.env().emit_event(Event1 { arg_1: 1 });
            self.env().emit_event(Event2 { arg_1: 1, arg_2: 2 });
            self.env().emit_event(Event3 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
            });
            self.env().emit_event(Event4 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
            });
            self.env().emit_event(Event5 {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
                arg_5: 5,
            });
        }
    }
}

fn main() {}
