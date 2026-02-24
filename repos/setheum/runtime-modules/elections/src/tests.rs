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

#![cfg(test)]

use frame_election_provider_support::{ElectionProvider, Support};
use primitives::CommitteeSeats;
use sp_core::bounded_vec;

use crate::{
    mock::{
        with_electable_targets, with_electing_voters, AccountId, Balance, Elections, Test,
        TestExtBuilder,
    },
    CommitteeSize, CurrentEraValidators, NextEraCommitteeSize, NextEraNonReservedValidators,
    NextEraReservedValidators,
};

fn no_support() -> Support<AccountId> {
    Default::default()
}

fn support(total: Balance, voters: Vec<(AccountId, Balance)>) -> Support<AccountId> {
    Support { total, voters }
}

#[test]
fn storage_is_initialized_already_in_genesis() {
    const RESERVED: [AccountId; 3] = [1, 2, 3];
    const NON_RESERVED: [AccountId; 2] = [4, 5];
    const COMMITTEE_SEATS: CommitteeSeats = CommitteeSeats {
        reserved_seats: 3,
        non_reserved_seats: 2,
        non_reserved_finality_seats: 2,
    };

    TestExtBuilder::new(RESERVED.to_vec(), NON_RESERVED.to_vec())
        .with_committee_seats(COMMITTEE_SEATS)
        .build()
        .execute_with(|| {
            assert_eq!(CommitteeSize::<Test>::get(), COMMITTEE_SEATS);
            assert_eq!(NextEraCommitteeSize::<Test>::get(), COMMITTEE_SEATS);
            assert_eq!(NextEraReservedValidators::<Test>::get(), RESERVED);
            assert_eq!(NextEraNonReservedValidators::<Test>::get(), NON_RESERVED);
            assert_eq!(CurrentEraValidators::<Test>::get().reserved, RESERVED);
            assert_eq!(
                CurrentEraValidators::<Test>::get().non_reserved,
                NON_RESERVED
            );
// We do not expect SessionValidatorBlockCount and ValidatorEraTotalReward to be
// populated from genesis, so does the ban related storages:
// UnderperformedValidatorSessionCount and Banned
        });
}

#[test]
fn validators_are_elected_only_when_staking() {
    TestExtBuilder::new(vec![1, 2, 3, 4], vec![5, 6, 7, 8])
        .build()
        .execute_with(|| {
// We check all 4 possibilities for both reserved and non reserved validators:
// { staking validator, not staking validator } x { any support, no support }.
//
// Only those considered as staking should be elected.

            with_electable_targets(vec![1, 2, 5, 6]);
            with_electing_voters(vec![
                (1, 10, bounded_vec![1]),
                (3, 10, bounded_vec![3]),
                (5, 10, bounded_vec![5]),
                (7, 10, bounded_vec![7]),
            ]);

            let elected =
                <Elections as ElectionProvider>::elect().expect("`elect()` should succeed");

            assert_eq!(
                elected.into_inner(),
                &[
                    (1, support(10, vec![(1, 10)])),
                    (2, no_support()),
                    (5, support(10, vec![(5, 10)])),
                    (6, no_support()),
                ]
            );
        });
}
