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
    collection::{NewestUnitResponse, Salt},
    dag::{DagUnit, Request},
    dissemination::DisseminationResponse,
    units::{UnitCoord, UnitStore, UnitWithParents, WrappedUnit},
    Data, Hasher, MultiKeychain, NodeIndex, Signed,
};
use std::marker::PhantomData;
use thiserror::Error;

/// A responder that is able to answer requests for data about units.
pub struct Responder<H: Hasher, D: Data, MK: MultiKeychain> {
    keychain: MK,
    _phantom: PhantomData<(H, D)>,
}

/// Ways in which it can be impossible for us to respond to a request.
#[derive(Eq, Error, Debug, PartialEq)]
pub enum Error<H: Hasher> {
    #[error("no canonical unit at {0}")]
    NoCanonicalAt(UnitCoord),
    #[error("unit with hash {0:?} not known")]
    UnknownUnit(H::Hash),
}

impl<H: Hasher, D: Data, MK: MultiKeychain> Responder<H, D, MK> {
    /// Create a new responder.
    pub fn new(keychain: MK) -> Self {
        Responder {
            keychain,
            _phantom: PhantomData,
        }
    }

    fn index(&self) -> NodeIndex {
        self.keychain.index()
    }

    fn on_request_coord(
        &self,
        coord: UnitCoord,
        units: &UnitStore<DagUnit<H, D, MK>>,
    ) -> Result<DisseminationResponse<H, D, MK::Signature>, Error<H>> {
        units
            .canonical_unit(coord)
            .map(|unit| DisseminationResponse::Coord(unit.clone().unpack().into()))
            .ok_or(Error::NoCanonicalAt(coord))
    }

    fn on_request_parents(
        &self,
        hash: H::Hash,
        units: &UnitStore<DagUnit<H, D, MK>>,
    ) -> Result<DisseminationResponse<H, D, MK::Signature>, Error<H>> {
        units
            .unit(&hash)
            .map(|unit| {
                let parents = unit
                    .parents()
                    .map(|parent_hash| {
                        units
                            .unit(parent_hash)
                            .expect("Units are added to the store in order.")
                            .clone()
                            .unpack()
                            .into_unchecked()
                    })
                    .collect();
                DisseminationResponse::Parents(hash, parents)
            })
            .ok_or(Error::UnknownUnit(hash))
    }

    fn on_request_newest(
        &self,
        requester: NodeIndex,
        salt: Salt,
        units: &UnitStore<DagUnit<H, D, MK>>,
    ) -> DisseminationResponse<H, D, MK::Signature> {
        let unit = units
            .canonical_units(requester)
            .last()
            .map(|unit| unit.clone().unpack().into_unchecked());
        let response = NewestUnitResponse::new(requester, self.index(), unit, salt);

        let signed_response = Signed::sign(response, &self.keychain).into_unchecked();
        DisseminationResponse::NewestUnit(signed_response)
    }

    /// Handle an incoming request returning either the appropriate response or an error if we
    /// aren't able to help.
    pub fn handle_request(
        &self,
        request: Request<H>,
        units: &UnitStore<DagUnit<H, D, MK>>,
    ) -> Result<DisseminationResponse<H, D, MK::Signature>, Error<H>> {
        use Request::*;
        match request {
            Coord(coord) => self.on_request_coord(coord, units),
            ParentsOf(hash) => self.on_request_parents(hash, units),
        }
    }

    /// Handle an incoming request for the newest unit of a given node we know of.
    pub fn handle_newest_unit_request(
        &self,
        requester: NodeIndex,
        salt: Salt,
        units: &UnitStore<DagUnit<H, D, MK>>,
    ) -> DisseminationResponse<H, D, MK::Signature> {
        self.on_request_newest(requester, salt, units)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        dag::Request,
        dissemination::{
            responder::{Error, Responder},
            DisseminationResponse,
        },
        units::{
            random_full_parent_reconstrusted_units_up_to, TestingDagUnit, Unit, UnitCoord,
            UnitStore, UnitWithParents, WrappedUnit,
        },
        NodeCount, NodeIndex,
    };
    use aleph_bft_mock::{Data, Hasher64, Keychain};
    use std::iter::zip;

    const NODE_ID: NodeIndex = NodeIndex(0);
    const NODE_COUNT: NodeCount = NodeCount(7);

    fn setup() -> (
        Responder<Hasher64, Data, Keychain>,
        UnitStore<TestingDagUnit>,
        Vec<Keychain>,
    ) {
        let keychains = Keychain::new_vec(NODE_COUNT);
        (
            Responder::new(keychains[NODE_ID.0]),
            UnitStore::new(NODE_COUNT),
            keychains,
        )
    }

    #[test]
    fn empty_fails_to_respond_to_coords() {
        let (responder, store, _) = setup();
        let coord = UnitCoord::new(0, NodeIndex(1));
        let request = Request::Coord(coord);
        match responder.handle_request(request, &store) {
            Ok(response) => panic!("Unexpected response: {:?}.", response),
            Err(err) => assert_eq!(err, Error::NoCanonicalAt(coord)),
        }
    }

    #[test]
    fn empty_fails_to_respond_to_parents() {
        let (responder, store, keychains) = setup();
        let session_id = 2137;
        let hash =
            random_full_parent_reconstrusted_units_up_to(1, NODE_COUNT, session_id, &keychains)
                .last()
                .expect("just created this round")
                .last()
                .expect("the round has at least one unit")
                .hash();
        let request = Request::ParentsOf(hash);
        match responder.handle_request(request, &store) {
            Ok(response) => panic!("Unexpected response: {:?}.", response),
            Err(err) => assert_eq!(err, Error::UnknownUnit(hash)),
        }
    }

    #[test]
    fn empty_newest_responds_with_no_units() {
        let (responder, store, keychains) = setup();
        let requester = NodeIndex(1);
        let response = responder.handle_newest_unit_request(requester, rand::random(), &store);
        match response {
            DisseminationResponse::NewestUnit(newest_unit_response) => {
                let checked_newest_unit_response = newest_unit_response
                    .check(&keychains[NODE_ID.0])
                    .expect("should sign correctly");
                assert!(checked_newest_unit_response
                    .as_signable()
                    .included_data()
                    .is_empty());
            }
            other => panic!("Unexpected response: {:?}.", other),
        }
    }

    #[test]
    fn responds_to_coords_when_possible() {
        let (responder, mut store, keychains) = setup();
        let session_id = 2137;
        let coord = UnitCoord::new(3, NodeIndex(1));
        let units = random_full_parent_reconstrusted_units_up_to(
            coord.round() + 1,
            NODE_COUNT,
            session_id,
            &keychains,
        );
        for round_units in &units {
            for unit in round_units {
                store.insert(unit.clone());
            }
        }
        let request = Request::Coord(coord);
        let response = responder
            .handle_request(request, &store)
            .expect("should successfully respond");
        match response {
            DisseminationResponse::Coord(unit) => assert_eq!(
                unit,
                units[coord.round() as usize][coord.creator().0]
                    .clone()
                    .unpack()
                    .into_unchecked()
            ),
            other => panic!("Unexpected response: {:?}.", other),
        }
    }

    #[test]
    fn fails_to_responds_to_too_new_coords() {
        let (responder, mut store, keychains) = setup();
        let session_id = 2137;
        let coord = UnitCoord::new(3, NodeIndex(1));
        let units = random_full_parent_reconstrusted_units_up_to(
            coord.round() - 1,
            NODE_COUNT,
            session_id,
            &keychains,
        );
        for round_units in &units {
            for unit in round_units {
                store.insert(unit.clone());
            }
        }
        let request = Request::Coord(coord);
        match responder.handle_request(request, &store) {
            Ok(response) => panic!("Unexpected response: {:?}.", response),
            Err(err) => assert_eq!(err, Error::NoCanonicalAt(coord)),
        }
    }

    #[test]
    fn responds_to_parents_when_possible() {
        let (responder, mut store, keychains) = setup();
        let session_id = 2137;
        let units =
            random_full_parent_reconstrusted_units_up_to(5, NODE_COUNT, session_id, &keychains);
        for round_units in &units {
            for unit in round_units {
                store.insert(unit.clone());
            }
        }
        let requested_unit = units
            .last()
            .expect("just created this round")
            .last()
            .expect("the round has at least one unit")
            .clone();
        let request = Request::ParentsOf(requested_unit.hash());
        let response = responder
            .handle_request(request, &store)
            .expect("should successfully respond");
        match response {
            DisseminationResponse::Parents(response_hash, parents) => {
                assert_eq!(response_hash, requested_unit.hash());
                assert_eq!(parents.len(), requested_unit.parents().count());
                for (parent, parent_hash) in zip(parents, requested_unit.parents()) {
                    assert_eq!(&parent.as_signable().hash(), parent_hash);
                }
            }
            other => panic!("Unexpected response: {:?}.", other),
        }
    }

    #[test]
    fn fails_to_respond_to_unknown_parents() {
        let (responder, mut store, keychains) = setup();
        let session_id = 2137;
        let units =
            random_full_parent_reconstrusted_units_up_to(5, NODE_COUNT, session_id, &keychains);
        for round_units in &units {
            for unit in round_units {
                store.insert(unit.clone());
            }
        }
        let hash =
            random_full_parent_reconstrusted_units_up_to(1, NODE_COUNT, session_id, &keychains)
                .last()
                .expect("just created this round")
                .last()
                .expect("the round has at least one unit")
                .hash();
        let request = Request::ParentsOf(hash);
        match responder.handle_request(request, &store) {
            Ok(response) => panic!("Unexpected response: {:?}.", response),
            Err(err) => assert_eq!(err, Error::UnknownUnit(hash)),
        }
    }

    #[test]
    fn responds_to_existing_newest() {
        let (responder, mut store, keychains) = setup();
        let session_id = 2137;
        let units =
            random_full_parent_reconstrusted_units_up_to(5, NODE_COUNT, session_id, &keychains);
        for round_units in &units {
            for unit in round_units {
                store.insert(unit.clone());
            }
        }
        let requester = NodeIndex(1);
        let response = responder.handle_newest_unit_request(requester, rand::random(), &store);
        match response {
            DisseminationResponse::NewestUnit(newest_unit_response) => {
                newest_unit_response
                    .check(&keychains[NODE_ID.0])
                    .expect("should sign correctly");
            }
            other => panic!("Unexpected response: {:?}.", other),
        }
    }
}
