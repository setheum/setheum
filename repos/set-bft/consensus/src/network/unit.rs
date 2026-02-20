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
    collection::NewestUnitResponse,
    units::{UncheckedSignedUnit, UnitCoord},
    Data, Hasher, NodeIndex, Signature, UncheckedSigned,
};
use codec::{Decode, Encode};

/// A message concerning units, either about new units or some requests for them.
#[derive(Clone, Eq, PartialEq, Debug, Decode, Encode)]
pub enum UnitMessage<H: Hasher, D: Data, S: Signature> {
    /// For disseminating newly created units.
    Unit(UncheckedSignedUnit<H, D, S>),
    /// Request for a unit by its coord.
    CoordRequest(NodeIndex, UnitCoord),
    /// Request for the full list of parents of a unit.
    ParentsRequest(NodeIndex, H::Hash),
    /// Response to a request for a full list of parents.
    ParentsResponse(H::Hash, Vec<UncheckedSignedUnit<H, D, S>>),
    /// Request by a node for the newest unit created by them, together with a u64 salt
    NewestRequest(NodeIndex, u64),
    /// Response to RequestNewest: (our index, maybe unit, salt) signed by us
    NewestResponse(UncheckedSigned<NewestUnitResponse<H, D, S>, S>),
}

impl<H: Hasher, D: Data, S: Signature> UnitMessage<H, D, S> {
    pub fn included_data(&self) -> Vec<D> {
        use UnitMessage::*;
        match self {
            Unit(uu) => uu.as_signable().included_data(),
            ParentsResponse(_, units) => units
                .iter()
                .flat_map(|uu| uu.as_signable().included_data())
                .collect(),
            NewestResponse(response) => response.as_signable().included_data(),
            NewestRequest(_, _) | CoordRequest(_, _) | ParentsRequest(_, _) => Vec::new(),
        }
    }
}
