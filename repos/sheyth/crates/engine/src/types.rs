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

use derive_more::From;

/// Same type as the `DefaultEnvironment::BlockNumber` type.
pub type BlockNumber = u32;

/// Same type as the `DefaultEnvironment::BlockTimestamp` type.
pub type BlockTimestamp = u64;

/// Same type as the `DefaultEnvironment::Balance` type.
pub type Balance = u128;

/// The Account Id type used by this crate.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AccountId(Vec<u8>);

impl AccountId {
    /// Creates a new `AccountId` from the given raw bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Returns the `AccountId` as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

/// Key into the database.
///
/// Used to identify contract storage cells for read and write operations.
#[derive(Default, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Key(Vec<u8>);

impl Key {
    /// Creates a new `Key` from the given raw bytes.
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

/// Errors encountered upon interacting with accounts.
#[derive(Clone, Debug, From, PartialEq, Eq)]
pub enum AccountError {
    Decoding(scale::Error),
    #[from(ignore)]
    UnexpectedUserAccount,
    #[from(ignore)]
    NoAccountForId(Vec<u8>),
}
