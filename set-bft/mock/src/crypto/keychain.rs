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
    PartialMultisignature as PartialMultisignatureT, SignatureSet,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Keychain {
    count: NodeCount,
    index: NodeIndex,
}

impl Keychain {
    pub fn new(count: NodeCount, index: NodeIndex) -> Self {
        Keychain { count, index }
    }

    pub fn new_vec(node_count: NodeCount) -> Vec<Self> {
        (0..node_count.0)
            .map(|i| Self::new(node_count, i.into()))
            .collect()
    }
}

impl Index for Keychain {
    fn index(&self) -> NodeIndex {
        self.index
    }
}

impl KeychainT for Keychain {
    type Signature = Signature;

    fn node_count(&self) -> NodeCount {
        self.count
    }

    fn sign(&self, msg: &[u8]) -> Self::Signature {
        Signature::new(msg.to_vec(), self.index)
    }

    fn verify(&self, msg: &[u8], sgn: &Self::Signature, index: NodeIndex) -> bool {
        index == sgn.index() && msg == sgn.msg()
    }
}

impl MultiKeychainT for Keychain {
    type PartialMultisignature = PartialMultisignature;

    fn bootstrap_multi(
        &self,
        signature: &Self::Signature,
        index: NodeIndex,
    ) -> Self::PartialMultisignature {
        SignatureSet::add_signature(SignatureSet::with_size(self.node_count()), signature, index)
    }

    fn is_complete(&self, msg: &[u8], partial: &Self::PartialMultisignature) -> bool {
        let signature_count = partial.iter().count();
        if signature_count < self.node_count().consensus_threshold().0 {
            return false;
        }
        partial.iter().all(|(i, sgn)| self.verify(msg, sgn, i))
    }
}
