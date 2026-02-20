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

use ink::{
    reflect::{
        ContractConstructorDecoder,
        ContractMessageDecoder,
        DecodeDispatch,
        DispatchError,
    },
    selector_bytes,
};
use scale::Encode;

#[ink::contract]
pub mod contract {
    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn constructor(_input_1: bool, _input_2: i32) -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn message(&self, _input_1: bool, _input_2: i32) {}
    }
}

use contract::Contract;

fn main() {
    constructor_decoder_works();
    message_decoder_works();
}

fn constructor_decoder_works() {
    // Valid call to `constructor`:
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("constructor"));
        input_bytes.extend(true.encode());
        input_bytes.extend(42i32.encode());
        assert!(
            <<Contract as ContractConstructorDecoder>::Type as DecodeDispatch>::decode_dispatch(
                &mut &input_bytes[..]).is_ok()
        );
    }
    // Invalid call with invalid selector (or empty input).
    {
        let input_bytes = Vec::new();
        assert_eq!(
            <<Contract as ContractConstructorDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::InvalidSelector,
        );
    }
    // Invalid call to `message` with unknown selector.
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("unknown_selector"));
        assert_eq!(
            <<Contract as ContractConstructorDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::UnknownSelector,
        );
    }
    // Invalid call to `message` with invalid (or missing) parameters.
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("constructor"));
        assert_eq!(
            <<Contract as ContractConstructorDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::InvalidParameters,
        );
    }
}

fn message_decoder_works() {
    // Valid call to `message`:
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("message"));
        input_bytes.extend(true.encode());
        input_bytes.extend(42i32.encode());
        assert!(
            <<Contract as ContractMessageDecoder>::Type as DecodeDispatch>::decode_dispatch(
                &mut &input_bytes[..]).is_ok()
        );
    }
    // Invalid call with invalid selector (or empty input).
    {
        let input_bytes = Vec::new();
        assert_eq!(
            <<Contract as ContractMessageDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::InvalidSelector,
        );
    }
    // Invalid call to `message` with unknown selector.
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("unknown_selector"));
        assert_eq!(
            <<Contract as ContractMessageDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::UnknownSelector,
        );
    }
    // Invalid call to `message` with invalid (or missing) parameters.
    {
        let mut input_bytes = Vec::new();
        input_bytes.extend(selector_bytes!("message"));
        assert_eq!(
            <<Contract as ContractMessageDecoder>::Type
                as DecodeDispatch>::decode_dispatch(&mut &input_bytes[..])
                .map(|_| ())
                .unwrap_err(),
            DispatchError::InvalidParameters,
        );
    }
}
