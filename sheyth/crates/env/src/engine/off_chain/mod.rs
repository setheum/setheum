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

mod call_data;
mod impls;
pub mod test_api;
mod types;

#[cfg(test)]
mod tests;

use super::OnInstance;
use crate::Error;

use derive_more::From;
use ink_engine::ext::Engine;

/// The off-chain environment.
pub struct EnvInstance {
    engine: Engine,
}

impl OnInstance for EnvInstance {
    fn on_instance<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        use core::cell::RefCell;
        thread_local!(
            static INSTANCE: RefCell<EnvInstance> = RefCell::new(
                EnvInstance {
                    engine: Engine::new()
                }
            )
        );
        INSTANCE.with(|instance| f(&mut instance.borrow_mut()))
    }
}

#[derive(Debug, From, PartialEq, Eq)]
pub enum OffChainError {
    Account(AccountError),
    #[from(ignore)]
    UninitializedBlocks,
    #[from(ignore)]
    UninitializedExecutionContext,
    #[from(ignore)]
    UnregisteredChainExtension,
}

/// Errors encountered upon interacting with the accounts database.
#[derive(Debug, From, PartialEq, Eq)]
pub enum AccountError {
    Decoding(scale::Error),
    #[from(ignore)]
    UnexpectedUserAccount,
    #[from(ignore)]
    NoAccountForId(Vec<u8>),
}
