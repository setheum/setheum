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

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use sp_runtime::traits::BadOrigin;

#[test]
fn make_airdrop_works() {
    ExtBuilder::default().build().execute_with(|| {
        let airdrop_list = vec![
            (ALICE, 10),
            (BOB, 5),
        ];

        assert_ok!(Airdrop::make_airdrop(Origin::signed(ALICE), SETR, airdrop_list.clone()));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETR,
                airdrop_list: airdrop_list.clone()
            },
        ));
        assert_ok!(Airdrop::make_airdrop(Origin::signed(ALICE), SETUSD, airdrop_list));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETUSD,
                airdrop_list
            },
        ));
    });
}

#[test]
fn make_airdrop_with_json_works() {
    ExtBuilder::default().build().execute_with(|| {
        let valid_json = br#"
        [
            {"account": "ALICE", "amount": 10},
            {"account": "BOB", "amount": 5}
        ]
        "#.as_bytes().to_vec();

        assert_ok!(Airdrop::make_airdrop_with_json(Origin::signed(ALICE), SETR, valid_json.clone()));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETR,
                airdrop_list: vec![
                    (ALICE, 10),
                    (BOB, 5),
                ]
            },
        ));
        assert_ok!(Airdrop::make_airdrop_with_json(Origin::signed(ALICE), SETUSD, valid_json));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETUSD,
                airdrop_list: vec![
                    (ALICE, 10),
                    (BOB, 5),
                ]
            },
        ));
    });
}

#[test]
fn make_airdrop_does_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let airdrop_list = vec![
            (ALICE, 10),
            (BOB, 5),
        ];

        assert_ok!(Airdrop::make_airdrop(Origin::signed(ALICE), SETR, airdrop_list.clone()));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETR,
                airdrop_list: airdrop_list.clone()
            },
        ));
        assert_eq!(Tokens::free_balance(SETR, Airdrop::account_id()), 258);

        assert_noop!(
            Airdrop::make_airdrop(Origin::signed(ALICE), SETR, airdrop_list),
            Error::<Runtime>::OverSizedAirdropList,
        );
    });
}

#[test]
fn make_airdrop_with_json_does_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let oversized_json = br#"
        [
            {"account": "ALICE", "amount": 10},
            {"account": "BOB", "amount": 5}
        ]
        "#.as_bytes().to_vec();

        assert_ok!(Airdrop::make_airdrop_with_json(Origin::signed(ALICE), SETR, oversized_json.clone()));
        System::assert_last_event(Event::AirDrop(
            crate::Event::Airdrop {
                currency_id: SETR,
                airdrop_list: vec![
                    (ALICE, 10),
                    (BOB, 5),
                ]
            },
        ));
        assert_eq!(Tokens::free_balance(SETR, Airdrop::account_id()), 258);

        assert_noop!(
            Airdrop::make_airdrop_with_json(Origin::signed(ALICE), SETR, oversized_json),
            Error::<Runtime>::OverSizedAirdropList
        );
    });
}
