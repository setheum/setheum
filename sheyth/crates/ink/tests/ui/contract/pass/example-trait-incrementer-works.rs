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

use incrementer::Incrementer;

#[ink::trait_definition]
pub trait Increment {
    #[ink(message)]
    fn inc(&mut self);

    #[ink(message)]
    fn get(&self) -> i64;
}

#[ink::trait_definition]
pub trait Reset {
    #[ink(message)]
    fn reset(&mut self);
}

#[ink::contract]
mod incrementer {
    use super::{
        Increment,
        Reset,
    };

    #[ink(storage)]
    pub struct Incrementer {
        value: i64,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i64) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn inc_by(&mut self, delta: i64) {
            self.value = self.value.checked_add(delta).unwrap();
        }
    }

    impl Increment for Incrementer {
        #[ink(message)]
        fn inc(&mut self) {
            self.inc_by(1)
        }

        #[ink(message)]
        fn get(&self) -> i64 {
            self.value
        }
    }

    impl Reset for Incrementer {
        #[ink(message)]
        fn reset(&mut self) {
            self.value = 0;
        }
    }
}

fn main() {
    let mut incrementer = Incrementer::new(0);
    assert_eq!(<Incrementer as Increment>::get(&incrementer), 0);
    incrementer.inc_by(1);
    assert_eq!(<Incrementer as Increment>::get(&incrementer), 1);
    incrementer.inc_by(-1);
    assert_eq!(<Incrementer as Increment>::get(&incrementer), 0);

    <Incrementer as Increment>::inc(&mut incrementer);
    assert_eq!(<Incrementer as Increment>::get(&incrementer), 1);
    <Incrementer as Increment>::inc(&mut incrementer);
    assert_eq!(<Incrementer as Increment>::get(&incrementer), 2);
}
