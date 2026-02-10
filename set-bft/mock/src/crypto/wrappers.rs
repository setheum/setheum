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

use crate::crypto::{PartialMultisignature, Signature};
use aleph_bft_types::{
    Index, Keychain as KeychainT, MultiKeychain as MultiKeychainT, NodeCount, NodeIndex,
};
use codec::{Decode, Encode};
use std::fmt::Debug;

pub trait MK:
    KeychainT<Signature = Signature> + MultiKeychainT<PartialMultisignature = PartialMultisignature>
{
}

impl<
        T: KeychainT<Signature = Signature>
            + MultiKeychainT<PartialMultisignature = PartialMultisignature>,
    > MK for T
{
}

/// Keychain wrapper which produces incorrect signatures
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Encode, Decode)]
pub struct BadSigning<T: MK>(T);

impl<T: MK> From<T> for BadSigning<T> {
    fn from(mk: T) -> Self {
        Self(mk)
    }
}

impl<T: MK> Index for BadSigning<T> {
    fn index(&self) -> NodeIndex {
        self.0.index()
    }
}

impl<T: MK> KeychainT for BadSigning<T> {
    type Signature = T::Signature;

    fn node_count(&self) -> NodeCount {
        self.0.node_count()
    }

    fn sign(&self, msg: &[u8]) -> Self::Signature {
        let signature = self.0.sign(msg);
        let mut msg = b"BAD".to_vec();
        msg.extend(signature.msg().clone());
        Signature::new(msg, signature.index())
    }

    fn verify(&self, msg: &[u8], sgn: &Self::Signature, index: NodeIndex) -> bool {
        self.0.verify(msg, sgn, index)
    }
}

impl<T: MK> MultiKeychainT for BadSigning<T> {
    type PartialMultisignature = T::PartialMultisignature;

    fn bootstrap_multi(
        &self,
        signature: &Self::Signature,
        index: NodeIndex,
    ) -> Self::PartialMultisignature {
        self.0.bootstrap_multi(signature, index)
    }

    fn is_complete(&self, msg: &[u8], partial: &Self::PartialMultisignature) -> bool {
        self.0.is_complete(msg, partial)
    }
}
