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

use std::collections::BTreeMap;

use parity_scale_codec::{Decode, Encode};

use crate::{
    errors::LangError,
    session::mock::{error::MockingError, MockedCallResult},
};

/// Alias for a 4-byte selector.
pub type Selector = [u8; 4];
/// An untyped message mock.
///
/// Notice that in the end, we cannot operate on specific argument/return types. Rust won't let us
/// have a collection of differently typed closures. Fortunately, we can assume that all types are
/// en/decodable, so we can use `Vec<u8>` as a common denominator.
pub type MessageMock = Box<dyn Fn(Vec<u8>) -> MockedCallResult + Send + Sync>;

/// A contract mock.
pub struct ContractMock {
    messages: BTreeMap<Selector, MessageMock>,
}

impl ContractMock {
    /// Creates a new mock without any message.
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
        }
    }

    /// Adds a message mock.
    pub fn with_message(mut self, selector: Selector, message: MessageMock) -> Self {
        self.messages.insert(selector, message);
        self
    }

    /// Try to call a message mock. Returns an error if there is no message mock for `selector`.
    pub fn call(&self, selector: Selector, input: Vec<u8>) -> MockedCallResult {
        match self.messages.get(&selector) {
            None => Err(MockingError::MessageNotFound(selector)),
            Some(message) => message(input),
        }
    }
}

impl Default for ContractMock {
    fn default() -> Self {
        Self::new()
    }
}

/// A helper function to create a message mock out of a typed closure.
///
/// In particular, it takes care of decoding the input and encoding the output. Also, wraps the
/// return value in a `Result`, which is normally done implicitly by ink!.
pub fn mock_message<Args: Decode, Ret: Encode, Body: Fn(Args) -> Ret + Send + Sync + 'static>(
    body: Body,
) -> MessageMock {
    Box::new(move |encoded_input| {
        let input = Decode::decode(&mut &*encoded_input).map_err(MockingError::ArgumentDecoding)?;
        Ok(Ok::<Ret, LangError>(body(input)).encode())
    })
}
