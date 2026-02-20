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

use contract::Contract;
#[ink::contract]
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    #[ink::trait_definition]
    pub trait Messages {
        #[ink(message)]
        fn message_0(&self);

        #[ink(message, selector = 1)]
        fn message_1(&self);
    }

    impl Messages for Contract {
        #[ink(message)]
        fn message_0(&self) {}

        #[ink(message, selector = 1)]
        fn message_1(&self) {}
    }

    impl Contract {
        #[ink(constructor)]
        pub fn constructor() -> Self {
            Self {}
        }

        #[ink(message, selector = 0xC0DE_CAFE)]
        pub fn message_2(&self) {}

        #[ink(message)]
        pub fn message_3(&self) {}
    }

    #[ink::trait_definition]
    pub trait Messages2 {
        #[ink(message, selector = 0x12345678)]
        fn message_4(&self);
    }

    impl Messages2 for Contract {
        #[ink(message, selector = 0x12345678)]
        fn message_4(&self) {}
    }
}

fn main() {
    const TRAIT_ID: u32 = ::ink::selector_id!("Messages::message_0");
    assert_eq!(
        <Contract as ::ink::reflect::DispatchableMessageInfo<TRAIT_ID>>::SELECTOR,
        [0xFB, 0xAB, 0x03, 0xCE],
    );
    assert_eq!(
        <Contract as ::ink::reflect::DispatchableMessageInfo<1_u32>>::SELECTOR,
        1_u32.to_be_bytes(),
    );
    assert_eq!(
        <Contract as ::ink::reflect::DispatchableMessageInfo<0xC0DE_CAFE_u32>>::SELECTOR,
        0xC0DE_CAFE_u32.to_be_bytes(),
    );
    const INHERENT_ID: u32 = ::ink::selector_id!("message_3");
    assert_eq!(
        <Contract as ::ink::reflect::DispatchableMessageInfo<INHERENT_ID>>::SELECTOR,
        [0xB6, 0xC3, 0x27, 0x49],
    );
    assert_eq!(
        <Contract as ::ink::reflect::DispatchableMessageInfo<0x12345678_u32>>::SELECTOR,
        0x12345678_u32.to_be_bytes(),
    );
}
