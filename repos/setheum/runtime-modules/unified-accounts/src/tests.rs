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
use mock::{alice, bob, EvmAccountsModule, ExtBuilder, Runtime, RuntimeEvent, RuntimeOrigin, System, ALICE, BOB};
use std::str::FromStr;

#[test]
fn claim_account_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::eth_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));
		System::assert_last_event(RuntimeEvent::EvmAccountsModule(crate::Event::ClaimAccount {
			account_id: ALICE,
			evm_address: EvmAccountsModule::eth_address(&alice()),
		}));
		assert!(
			Accounts::<Runtime>::contains_key(EvmAccountsModule::eth_address(&alice()))
				&& EvmAddresses::<Runtime>::contains_key(ALICE)
		);
	});
}

#[test]
fn claim_account_should_not_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::eth_address(&bob()),
				EvmAccountsModule::eth_sign(&bob(), &BOB)
			),
			Error::<Runtime>::InvalidSignature
		);
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::eth_address(&bob()),
				EvmAccountsModule::eth_sign(&alice(), &ALICE)
			),
			Error::<Runtime>::InvalidSignature
		);
		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::eth_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				EvmAccountsModule::eth_address(&alice()),
				EvmAccountsModule::eth_sign(&alice(), &ALICE)
			),
			Error::<Runtime>::AccountIdHasMapped
		);
		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(BOB),
				EvmAccountsModule::eth_address(&alice()),
				EvmAccountsModule::eth_sign(&alice(), &BOB)
			),
			Error::<Runtime>::EthAddressHasMapped
		);
	});
}

#[test]
fn evm_get_account_id() {
	ExtBuilder::default().build().execute_with(|| {
		let evm_account = EvmAccountsModule::eth_address(&alice());
		let evm_account_to_default = {
			let mut bytes = *b"evm:aaaaaaaaaaaaaaaaaaaa\0\0\0\0\0\0\0\0";
			bytes[4..24].copy_from_slice(&evm_account[..]);
			AccountId32::from(bytes)
		};
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_account_id(&evm_account),
			evm_account_to_default
		);

		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			EvmAccountsModule::eth_address(&alice()),
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));

		assert_eq!(EvmAddressMapping::<Runtime>::get_account_id(&evm_account), ALICE);
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_evm_address(&ALICE).unwrap(),
			evm_account
		);

		assert!(EvmAddressMapping::<Runtime>::is_linked(
			&evm_account_to_default,
			&evm_account
		));
		assert!(EvmAddressMapping::<Runtime>::is_linked(&ALICE, &evm_account));
	});
}

#[test]
fn validate_evm_account_id() {
	ExtBuilder::default().build().execute_with(|| {
		assert!(EvmAddressMapping::<Runtime>::get_evm_address(&ALICE).is_none());

		let no_zero_padding = AccountId32::new(*b"evm:aaaaaaaaaaaaaaaaaaaaaaaaaaaa");
		assert!(EvmAddressMapping::<Runtime>::get_evm_address(&no_zero_padding).is_none());

		let valid_account_id = AccountId32::new(*b"evm:aaaaaaaaaaaaaaaaaaaa\0\0\0\0\0\0\0\0");
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_evm_address(&valid_account_id).unwrap(),
			EvmAddress::from(b"aaaaaaaaaaaaaaaaaaaa")
		);
	});
}

#[test]
fn account_to_evm() {
	ExtBuilder::default().build().execute_with(|| {
		let default_evm_account = EvmAddress::from_str("f0bd9ffde7f9f4394d8cc1d86bf24d87e5d5a9a9").unwrap();
		assert_eq!(EvmAddressMapping::<Runtime>::get_evm_address(&ALICE), None);

		let alice_evm_account = EvmAccountsModule::eth_address(&alice());

		assert_ok!(EvmAccountsModule::claim_account(
			RuntimeOrigin::signed(ALICE),
			alice_evm_account,
			EvmAccountsModule::eth_sign(&alice(), &ALICE)
		));

		assert_eq!(EvmAddressMapping::<Runtime>::get_account_id(&alice_evm_account), ALICE);
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_evm_address(&ALICE).unwrap(),
			alice_evm_account
		);

		assert_eq!(
			EvmAddressMapping::<Runtime>::get_or_create_evm_address(&ALICE),
			alice_evm_account
		);

		assert!(EvmAddressMapping::<Runtime>::is_linked(&ALICE, &alice_evm_account));
		assert!(EvmAddressMapping::<Runtime>::is_linked(&ALICE, &default_evm_account));
	});
}

#[test]
fn account_to_evm_with_create_default() {
	ExtBuilder::default().build().execute_with(|| {
		let default_evm_account = EvmAddress::from_str("f0bd9ffde7f9f4394d8cc1d86bf24d87e5d5a9a9").unwrap();
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_or_create_evm_address(&ALICE),
			default_evm_account
		);
		System::assert_last_event(RuntimeEvent::EvmAccountsModule(crate::Event::ClaimAccount {
			account_id: ALICE,
			evm_address: default_evm_account,
		}));
		assert_eq!(
			EvmAddressMapping::<Runtime>::get_evm_address(&ALICE),
			Some(default_evm_account)
		);

		assert_eq!(
			EvmAddressMapping::<Runtime>::get_account_id(&default_evm_account),
			ALICE
		);

		assert!(EvmAddressMapping::<Runtime>::is_linked(&ALICE, &default_evm_account));

		let alice_evm_account = EvmAccountsModule::eth_address(&alice());

		assert_noop!(
			EvmAccountsModule::claim_account(
				RuntimeOrigin::signed(ALICE),
				alice_evm_account,
				EvmAccountsModule::eth_sign(&alice(), &ALICE)
			),
			Error::<Runtime>::AccountIdHasMapped
		);
	});
}
