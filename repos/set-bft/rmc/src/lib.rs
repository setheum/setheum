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

pub use aleph_bft_crypto::{
    Indexed, MultiKeychain, Multisigned, NodeCount, PartialMultisignature, PartiallyMultisigned,
    Signable, Signature, Signed, UncheckedSigned,
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use std::hash::Hash;

mod handler;
mod scheduler;
mod service;

pub use handler::Handler;
pub use scheduler::DoublingDelayScheduler;
pub use service::Service;

/// An RMC message consisting of either a signed (indexed) hash, or a multisigned hash.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, Hash)]
pub enum Message<H: Signable, S: Signature, M: PartialMultisignature> {
    SignedHash(UncheckedSigned<Indexed<H>, S>),
    MultisignedHash(UncheckedSigned<H, M>),
}

impl<H: Signable, S: Signature, M: PartialMultisignature> Message<H, S, M> {
    pub fn hash(&self) -> &H {
        match self {
            Message::SignedHash(unchecked) => unchecked.as_signable_strip_index(),
            Message::MultisignedHash(unchecked) => unchecked.as_signable(),
        }
    }
    pub fn is_complete(&self) -> bool {
        matches!(self, Message::MultisignedHash(_))
    }
}
