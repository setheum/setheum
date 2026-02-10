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

use crate::{units::Unit, Hasher, NodeCount, NodeIndex, NodeMap, Round};
use anyhow::Result;
use thiserror::Error;

#[derive(Eq, Error, Debug, PartialEq)]
pub enum ConstraintError {
    #[error("Not enough parents.")]
    NotEnoughParents,
    #[error("Missing own parent.")]
    MissingOwnParent,
}

#[derive(Clone)]
pub struct UnitsCollector<H: Hasher> {
    candidates: NodeMap<(H::Hash, Round)>,
    for_round: Round,
    direct_parents: NodeCount,
}

impl<H: Hasher> UnitsCollector<H> {
    pub fn new_initial(n_members: NodeCount) -> Self {
        UnitsCollector {
            candidates: NodeMap::with_size(n_members),
            for_round: 1,
            direct_parents: NodeCount(0),
        }
    }

    pub fn from_previous(previous: &UnitsCollector<H>) -> Self {
        UnitsCollector {
            candidates: previous.candidates.clone(),
            for_round: previous.for_round + 1,
            direct_parents: NodeCount(0),
        }
    }

    pub fn add_unit<U: Unit<Hasher = H>>(&mut self, unit: &U) {
        let node_id = unit.creator();
        let hash = unit.hash();
        let round = unit.round();

        if round >= self.for_round {
            return;
        }

        let to_insert = match self.candidates.get(node_id) {
            None => Some((hash, round)),
            Some((_, r)) if *r < round => Some((hash, round)),
            _ => None,
        };

        if let Some(data) = to_insert {
            self.candidates.insert(node_id, data);
            if round == self.for_round - 1 {
                self.direct_parents += NodeCount(1);
            }
        }
    }

    pub fn prospective_parents(
        &self,
        node_id: NodeIndex,
    ) -> Result<&NodeMap<(H::Hash, Round)>, ConstraintError> {
        if self.direct_parents < self.candidates.size().consensus_threshold() {
            return Err(ConstraintError::NotEnoughParents);
        }
        match self.candidates.get(node_id) {
            Some((_, r)) if *r == self.for_round - 1 => Ok(&self.candidates),
            _ => Err(ConstraintError::MissingOwnParent),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        creation::collector::{ConstraintError, UnitsCollector},
        units::{random_full_parent_units_up_to, Unit},
        NodeCount, NodeIndex,
    };
    use aleph_bft_mock::Hasher64;

    #[test]
    fn initial_fails_without_parents() {
        let n_members = NodeCount(4);
        let units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::NotEnoughParents);
    }

    #[test]
    fn initial_fails_with_too_few_parents() {
        let n_members = NodeCount(4);
        let mut units_collector = UnitsCollector::new_initial(n_members);
        let units = random_full_parent_units_up_to(0, n_members, 43);
        units_collector.add_unit(&units[0][0]);

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::NotEnoughParents);
    }

    #[test]
    fn initial_fails_without_own_parent() {
        let n_members = NodeCount(4);
        let mut units_collector = UnitsCollector::new_initial(n_members);
        let units = random_full_parent_units_up_to(0, n_members, 43);
        for unit in units[0].iter().skip(1) {
            units_collector.add_unit(unit);
        }

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::MissingOwnParent);
    }

    #[test]
    fn initial_successfully_computes_minimal_parents() {
        let n_members = NodeCount(4);
        let mut units_collector = UnitsCollector::new_initial(n_members);
        let units = random_full_parent_units_up_to(0, n_members, 43);
        for unit in units[0].iter().take(3) {
            units_collector.add_unit(unit);
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 3);

        let new_units: Vec<_> = units[0]
            .iter()
            .take(3)
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn initial_successfully_computes_full_parents() {
        let n_members = NodeCount(4);
        let mut units_collector = UnitsCollector::new_initial(n_members);
        let units = random_full_parent_units_up_to(0, n_members, 43);
        for unit in &units[0] {
            units_collector.add_unit(unit);
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 4);

        let new_units: Vec<_> = units[0]
            .iter()
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn initial_ignores_future_rounds() {
        let n_members = NodeCount(4);
        let mut units_collector = UnitsCollector::new_initial(n_members);
        let units = random_full_parent_units_up_to(2, n_members, 43);
        for round_units in &units {
            for unit in round_units {
                units_collector.add_unit(unit);
            }
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 4);

        let new_units: Vec<_> = units[0]
            .iter()
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn following_fails_without_parents() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let units_collector = UnitsCollector::from_previous(&initial_units_collector);

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::NotEnoughParents);
    }

    #[test]
    fn following_fails_with_too_few_parents() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        units_collector.add_unit(&units[1][0]);

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::NotEnoughParents);
    }

    #[test]
    fn following_fails_with_too_old_parents() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(0, n_members, 43);
        for unit in &units[0] {
            units_collector.add_unit(unit);
        }

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::NotEnoughParents);
    }

    #[test]
    fn following_fails_without_own_parent() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in units[1].iter().skip(1) {
            units_collector.add_unit(unit);
        }

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::MissingOwnParent);
    }

    #[test]
    fn following_fails_with_too_old_own_parent() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in units[1].iter().skip(1) {
            units_collector.add_unit(unit);
        }
        units_collector.add_unit(&units[0][0]);

        let err = units_collector
            .prospective_parents(NodeIndex(0))
            .expect_err("should fail without parents");
        assert_eq!(err, ConstraintError::MissingOwnParent);
    }

    #[test]
    fn following_successfully_computes_minimal_parents() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in units[1].iter().take(3) {
            units_collector.add_unit(unit);
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 3);

        let new_units: Vec<_> = units[1]
            .iter()
            .take(3)
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn following_successfully_computes_minimal_parents_with_ancient() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in units[1].iter().take(3) {
            units_collector.add_unit(unit);
        }
        units_collector.add_unit(&units[0][3]);

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 4);

        let mut new_units: Vec<_> = units[1]
            .iter()
            .take(3)
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        new_units.push((units[0][3].hash(), units[0][3].round()));
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn following_successfully_computes_full_parents() {
        let n_members = NodeCount(4);
        let initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in &units[1] {
            units_collector.add_unit(unit);
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 4);

        let new_units: Vec<_> = units[1]
            .iter()
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }

    #[test]
    fn following_inherits_units() {
        let n_members = NodeCount(4);
        let mut initial_units_collector = UnitsCollector::<Hasher64>::new_initial(n_members);
        let units = random_full_parent_units_up_to(1, n_members, 43);
        for unit in &units[0] {
            initial_units_collector.add_unit(unit);
        }
        let mut units_collector = UnitsCollector::from_previous(&initial_units_collector);
        for unit in units[1].iter().take(3) {
            units_collector.add_unit(unit);
        }

        let parents = units_collector
            .prospective_parents(NodeIndex(0))
            .expect("we should be able to retrieve parents");
        assert_eq!(parents.item_count(), 4);

        let mut new_units: Vec<_> = units[1]
            .iter()
            .take(3)
            .map(|unit| (unit.hash(), unit.round()))
            .collect();
        new_units.push((units[0][3].hash(), units[0][3].round()));
        let selected_parents: Vec<_> = parents.values().cloned().collect();
        assert_eq!(new_units, selected_parents);
    }
}
