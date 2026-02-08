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

use crate::{
    units::{UncheckedSignedUnit, Unit},
    Data, Hasher, Index, Keychain, MultiKeychain, Multisigned, NodeIndex, PartialMultisignature,
    Signable, Signature, UncheckedSigned,
};
use aleph_bft_rmc::Message as RmcMessage;
use codec::{Decode, Encode};
use derivative::Derivative;
use parking_lot::RwLock;
use std::ops::Deref;

mod handler;
mod service;

pub use handler::Handler;
pub use service::{Service, IO};

pub type ForkProof<H, D, S> = (UncheckedSignedUnit<H, D, S>, UncheckedSignedUnit<H, D, S>);

pub type NetworkMessage<H, D, MK> =
    AlertMessage<H, D, <MK as Keychain>::Signature, <MK as MultiKeychain>::PartialMultisignature>;

#[derive(Debug, Decode, Derivative, Encode)]
#[derivative(Eq, PartialEq, Hash)]
pub struct Alert<H: Hasher, D: Data, S: Signature> {
    sender: NodeIndex,
    proof: ForkProof<H, D, S>,
    legit_units: Vec<UncheckedSignedUnit<H, D, S>>,
    #[codec(skip)]
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    hash: RwLock<Option<H::Hash>>,
}

impl<H: Hasher, D: Data, S: Signature> Clone for Alert<H, D, S> {
    fn clone(&self) -> Self {
        let hash = match self.hash.try_read() {
            None => None,
            Some(guard) => *guard.deref(),
        };
        Alert {
            sender: self.sender,
            proof: self.proof.clone(),
            legit_units: self.legit_units.clone(),
            hash: RwLock::new(hash),
        }
    }
}

impl<H: Hasher, D: Data, S: Signature> Alert<H, D, S> {
    pub fn new(
        sender: NodeIndex,
        proof: ForkProof<H, D, S>,
        legit_units: Vec<UncheckedSignedUnit<H, D, S>>,
    ) -> Alert<H, D, S> {
        Alert {
            sender,
            proof,
            legit_units,
            hash: RwLock::new(None),
        }
    }

    fn hash(&self) -> H::Hash {
        let hash = *self.hash.read();
        match hash {
            Some(hash) => hash,
            None => {
                let hash = self.using_encoded(H::hash);
                *self.hash.write() = Some(hash);
                hash
            }
        }
    }

    /// Simplified forker check, should only be called for alerts that have already been checked to
    /// contain valid proofs.
    pub fn forker(&self) -> NodeIndex {
        self.proof.0.as_signable().creator()
    }

    pub fn included_data(&self) -> Vec<D> {
        // Only legit units might end up in the DAG, we can ignore the fork proof.
        self.legit_units
            .iter()
            .filter_map(|uu| uu.as_signable().data().clone())
            .collect()
    }
}

impl<H: Hasher, D: Data, S: Signature> Index for Alert<H, D, S> {
    fn index(&self) -> NodeIndex {
        self.sender
    }
}

impl<H: Hasher, D: Data, S: Signature> Signable for Alert<H, D, S> {
    type Hash = H::Hash;
    fn hash(&self) -> Self::Hash {
        self.hash()
    }
}

/// A message concerning alerts.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Decode, Encode)]
pub enum AlertMessage<H: Hasher, D: Data, S: Signature, MS: PartialMultisignature> {
    /// Alert regarding forks, signed by the person claiming misconduct.
    ForkAlert(UncheckedSigned<Alert<H, D, S>, S>),
    /// An internal RMC message, together with the id of the sender.
    RmcMessage(NodeIndex, RmcMessage<H::Hash, S, MS>),
    /// A request by a node for a fork alert identified by the given hash.
    AlertRequest(NodeIndex, H::Hash),
}

impl<H: Hasher, D: Data, S: Signature, MS: PartialMultisignature> AlertMessage<H, D, S, MS> {
    pub fn included_data(&self) -> Vec<D> {
        match self {
            Self::ForkAlert(unchecked_alert) => unchecked_alert.as_signable().included_data(),
            Self::RmcMessage(_, _) => Vec::new(),
            Self::AlertRequest(_, _) => Vec::new(),
        }
    }
}

// Notifications being sent to consensus, so that it can learn about proven forkers and receive
// legitimized units.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Decode, Encode)]
pub enum ForkingNotification<H: Hasher, D: Data, S: Signature> {
    Forker(ForkProof<H, D, S>),
    Units(Vec<UncheckedSignedUnit<H, D, S>>),
}

#[derive(Clone, Debug, Decode, Encode, PartialEq)]
pub enum AlertData<H: Hasher, D: Data, MK: MultiKeychain> {
    OwnAlert(Alert<H, D, MK::Signature>),
    NetworkAlert(Alert<H, D, MK::Signature>),
    MultisignedHash(Multisigned<H::Hash, MK>),
}
