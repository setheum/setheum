// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

use std::sync::Arc;

use contract_transcode::{ContractMessageTranscoder, Value};
use parity_scale_codec::{Decode, Encode};

use crate::{
    errors::MessageResult,
    session::error::SessionError,
};

/// Data structure storing the results of contract interaction during a session.
#[derive(Default)]
pub struct Record {
    /// The return values of contract instantiation (i.e. the addresses of the newly instantiated
    /// contracts).
    deploy_returns: Vec<[u8; 32]>,

    /// The return values of contract calls (in the SCALE-encoded form).
    call_returns: Vec<Vec<u8>>,
}

impl Record {
    pub(super) fn push_deploy_return(&mut self, return_value: [u8; 32]) {
        self.deploy_returns.push(return_value);
    }

    pub(super) fn push_call_return(&mut self, return_value: Vec<u8>) {
        self.call_returns.push(return_value);
    }
}

impl Record {
    /// Returns all the return values of contract instantiations that happened during the session.
    pub fn deploy_returns(&self) -> &[[u8; 32]] {
        &self.deploy_returns
    }

    /// Returns the last return value of contract instantiation that happened during the session.
    /// Panics if there were no contract instantiations.
    pub fn last_deploy_return(&self) -> &[u8; 32] {
        self.deploy_returns.last().expect("No deploy returns")
    }

    /// Returns all the (encoded) return values of contract calls that happened during the session.
    pub fn call_returns(&self) -> &[Vec<u8>] {
        &self.call_returns
    }

    /// Returns the last (encoded) return value of contract call that happened during the session.
    /// Panics if there were no contract calls.
    pub fn last_call_return(&self) -> &[u8] {
        self.call_returns.last().expect("No call returns")
    }

    /// Returns the last (decoded) return value of contract call that happened during the session.
    /// Panics if there were no contract calls.
    pub fn last_call_return_decoded<T: Decode>(&self) -> Result<MessageResult<T>, SessionError> {
        let mut raw = self.last_call_return();
        MessageResult::decode(&mut raw).map_err(|err| {
            SessionError::Decoding(format!(
                "Failed to decode the result of calling a contract: {err:?}"
            ))
        })
    }
}
