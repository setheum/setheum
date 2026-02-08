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

use ink_env::{
    DefaultEnvironment,
    Environment,
};

#[derive(Clone)]
pub struct EnvironmentMoreTopics;

impl ink_env::Environment for EnvironmentMoreTopics {
    const MAX_EVENT_TOPICS: usize = 10; // Default is 4.

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = ();
}

#[ink::contract(env = super::EnvironmentMoreTopics)]
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    #[ink(event, anonymous)]
    pub struct EventWithManyTopics {
        #[ink(topic)]
        arg_1: i8,
        #[ink(topic)]
        arg_2: i16,
        #[ink(topic)]
        arg_3: i32,
        #[ink(topic)]
        arg_4: i64,
        #[ink(topic)]
        arg_5: i128,
        #[ink(topic)]
        arg_6: u8,
        #[ink(topic)]
        arg_7: u16,
        #[ink(topic)]
        arg_8: u32,
        #[ink(topic)]
        arg_9: u64,
        #[ink(topic)]
        arg_10: u128,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Self {
            Self::env().emit_event(EventWithManyTopics {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
                arg_5: 5,
                arg_6: 6,
                arg_7: 7,
                arg_8: 8,
                arg_9: 9,
                arg_10: 10,
            });
            Self {}
        }

        #[ink(message)]
        pub fn message(&self) {
            self.env().emit_event(EventWithManyTopics {
                arg_1: 1,
                arg_2: 2,
                arg_3: 3,
                arg_4: 4,
                arg_5: 5,
                arg_6: 6,
                arg_7: 7,
                arg_8: 8,
                arg_9: 9,
                arg_10: 10,
            });
        }
    }
}

fn main() {}
