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
    config::DelaySchedule,
    network::UnitMessageTo,
    units::{UncheckedSignedUnit, Validator},
    Data, Hasher, Keychain, MultiKeychain, NodeIndex, Receiver, Round, Sender, Signable, Signature,
    UncheckedSigned,
};
use codec::{Decode, Encode};
use futures::{channel::oneshot, Future};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher as _},
};

mod service;

pub use service::{Collection, IO};

const LOG_TARGET: &str = "AlephBFT-collection";

/// Salt uniquely identifying an initial unit collection instance.
pub type Salt = u64;

fn generate_salt() -> Salt {
    let mut hasher = DefaultHasher::new();
    std::time::Instant::now().hash(&mut hasher);
    hasher.finish()
}

/// A response to the request for the newest unit.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Decode, Encode)]
pub struct NewestUnitResponse<H: Hasher, D: Data, S: Signature> {
    requester: NodeIndex,
    responder: NodeIndex,
    unit: Option<UncheckedSignedUnit<H, D, S>>,
    salt: Salt,
}

impl<H: Hasher, D: Data, S: Signature> Signable for NewestUnitResponse<H, D, S> {
    type Hash = Vec<u8>;

    fn hash(&self) -> Self::Hash {
        self.encode()
    }
}

impl<H: Hasher, D: Data, S: Signature> crate::Index for NewestUnitResponse<H, D, S> {
    fn index(&self) -> NodeIndex {
        self.responder
    }
}

impl<H: Hasher, D: Data, S: Signature> NewestUnitResponse<H, D, S> {
    /// Create a newest unit response.
    pub fn new(
        requester: NodeIndex,
        responder: NodeIndex,
        unit: Option<UncheckedSignedUnit<H, D, S>>,
        salt: Salt,
    ) -> Self {
        NewestUnitResponse {
            requester,
            responder,
            unit,
            salt,
        }
    }

    /// The data included in this message, i.e. contents of the unit if any.
    pub fn included_data(&self) -> Vec<D> {
        match &self.unit {
            Some(u) => u.as_signable().included_data(),
            None => Vec::new(),
        }
    }
}

pub type CollectionResponse<H, D, MK> = UncheckedSigned<
    NewestUnitResponse<H, D, <MK as Keychain>::Signature>,
    <MK as Keychain>::Signature,
>;

#[cfg(feature = "initial_unit_collection")]
pub fn initial_unit_collection<'a, H: Hasher, D: Data, MK: MultiKeychain>(
    keychain: &'a MK,
    validator: &'a Validator<MK>,
    messages_for_network: Sender<UnitMessageTo<H, D, MK::Signature>>,
    starting_round_sender: oneshot::Sender<Option<Round>>,
    starting_round_from_backup: Round,
    responses_from_network: Receiver<CollectionResponse<H, D, MK>>,
    request_delay: DelaySchedule,
) -> Result<impl Future<Output = ()> + 'a, ()> {
    let collection = Collection::new(keychain, validator);

    let collection = IO::new(
        starting_round_sender,
        starting_round_from_backup,
        responses_from_network,
        messages_for_network,
        collection,
        request_delay,
    );
    Ok(collection.run())
}

/// A trivial start that doesn't actually perform the initial unit collection.
#[cfg(not(feature = "initial_unit_collection"))]
pub fn initial_unit_collection(
    _keychain: &'a MK,
    _validator: &'a Validator<MK>,
    _messages_for_network: Sender<UnitMessageTo<H, D, MK::Signature>>,
    starting_round_sender: oneshot::Sender<Option<Round>>,
    starting_round_from_backup: Round,
    _responses_from_network: Receiver<CollectionResponse<H, D, MK>>,
    _request_delay: DelaySchedule,
) -> Result<impl Future<Output = ()>, ()> {
    if let Err(e) = starting_round_sender.send(Some(starting_round_from_backup)) {
        error!(target: LOG_TARGET, "Unable to send the starting round: {}", e);
        return Err(());
    }
    Ok(async {})
}
