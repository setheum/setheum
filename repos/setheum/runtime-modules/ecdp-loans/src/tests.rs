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
use mock::{RuntimeEvent, *};

#[test]
fn debits_key() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 0);
		assert_ok!(EcdpLoansModule::adjust_position(&ALICE, BTC, 200, 200));
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 200);
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 200);
		assert_ok!(EcdpLoansModule::adjust_position(&ALICE, BTC, -100, -100));
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 100);
	});
}

#[test]
fn check_update_loan_underflow_work() {
	ExtBuilder::default().build().execute_with(|| {
// collateral underflow
		assert_noop!(
			EcdpLoansModule::update_loan(&ALICE, BTC, -100, 0),
			ArithmeticError::Underflow,
		);

// debit underflow
		assert_noop!(
			EcdpLoansModule::update_loan(&ALICE, BTC, 0, -100),
			ArithmeticError::Underflow,
		);
	});
}

#[test]
fn adjust_position_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 1000);

// balance too low
		assert_noop!(
			EcdpLoansModule::adjust_position(&ALICE, BTC, 2000, 0),
			orml_tokens::Error::<Runtime>::BalanceTooLow
		);

// mock can't pass liquidation ratio check
		assert_noop!(
			EcdpLoansModule::adjust_position(&ALICE, EDF, 500, 0),
			sp_runtime::DispatchError::Other("mock below liquidation ratio error")
		);

// mock can't pass required ratio check
		assert_noop!(
			EcdpLoansModule::adjust_position(&ALICE, EDF, 500, 1),
			sp_runtime::DispatchError::Other("mock below required collateral ratio error")
		);

// mock exceed debit value cap
		assert_noop!(
			EcdpLoansModule::adjust_position(&ALICE, BTC, 1000, 1000),
			sp_runtime::DispatchError::Other("mock exceed debit value cap error")
		);

// failed because ED of collateral
		assert_noop!(
			EcdpLoansModule::adjust_position(&ALICE, BTC, 99, 0),
			orml_tokens::Error::<Runtime>::ExistentialDeposit,
		);

		assert_eq!(Currencies::free_balance(BTC, &ALICE), 1000);
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 0);
		assert_eq!(EcdpLoansModule::total_positions(BTC).debit, 0);
		assert_eq!(EcdpLoansModule::total_positions(BTC).collateral, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 0);
		assert_eq!(Currencies::free_balance(USSD, &ALICE), 0);

// success
		assert_ok!(EcdpLoansModule::adjust_position(&ALICE, BTC, 500, 300));
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 500);
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 500);
		assert_eq!(EcdpLoansModule::total_positions(BTC).debit, 300);
		assert_eq!(EcdpLoansModule::total_positions(BTC).collateral, 500);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 300);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 500);
		assert_eq!(Currencies::free_balance(USSD, &ALICE), 150);
		System::assert_has_event(RuntimeEvent::EcdpLoansModule(crate::Event::EcdpPositionUpdated {
			owner: ALICE,
			collateral_type: BTC,
			collateral_adjustment: 500,
			debit_adjustment: 300,
		}));

// collateral_adjustment is negatives
		assert_eq!(Currencies::total_balance(BTC, &EcdpLoansModule::account_id()), 500);
		assert_ok!(EcdpLoansModule::adjust_position(&ALICE, BTC, -500, 0));
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 0);
	});
}

#[test]
fn update_loan_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 0);
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 1000);
		assert_eq!(EcdpLoansModule::total_positions(BTC).debit, 0);
		assert_eq!(EcdpLoansModule::total_positions(BTC).collateral, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 0);
		assert!(!<EcdpPositions<Runtime>>::contains_key(BTC, &ALICE));

		let alice_ref_count_0 = System::consumers(&ALICE);

		assert_ok!(EcdpLoansModule::update_loan(&ALICE, BTC, 3000, 2000));

// just update records
		assert_eq!(EcdpLoansModule::total_positions(BTC).debit, 2000);
		assert_eq!(EcdpLoansModule::total_positions(BTC).collateral, 3000);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 2000);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 3000);

// increase ref count when open new position
		let alice_ref_count_1 = System::consumers(&ALICE);
		assert_eq!(alice_ref_count_1, alice_ref_count_0 + 1);

// does not manipulate balance
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 0);
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 1000);

// should remove position storage if zero
		assert!(<EcdpPositions<Runtime>>::contains_key(BTC, &ALICE));
		assert_ok!(EcdpLoansModule::update_loan(&ALICE, BTC, -3000, -2000));
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 0);
		assert!(!<EcdpPositions<Runtime>>::contains_key(BTC, &ALICE));

// decrease ref count after remove position
		let alice_ref_count_2 = System::consumers(&ALICE);
		assert_eq!(alice_ref_count_2, alice_ref_count_1 - 1);
	});
}

#[test]
fn transfer_loan_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(EcdpLoansModule::update_loan(&ALICE, BTC, 400, 500));
		assert_ok!(EcdpLoansModule::update_loan(&BOB, BTC, 100, 600));
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 500);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 400);
		assert_eq!(EcdpLoansModule::positions(BTC, &BOB).debit, 600);
		assert_eq!(EcdpLoansModule::positions(BTC, &BOB).collateral, 100);

		assert_ok!(EcdpLoansModule::transfer_loan(&ALICE, &BOB, BTC));
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &BOB).debit, 1100);
		assert_eq!(EcdpLoansModule::positions(BTC, &BOB).collateral, 500);
		System::assert_last_event(RuntimeEvent::EcdpLoansModule(crate::Event::TransferLoan {
			from: ALICE,
			to: BOB,
			currency_id: BTC,
		}));
	});
}

#[test]
fn confiscate_collateral_and_debit_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(EcdpLoansModule::update_loan(&BOB, BTC, 5000, 1000));
		assert_eq!(Currencies::free_balance(BTC, &EcdpLoansModule::account_id()), 0);

// have no sufficient balance
		assert_noop!(
			EcdpLoansModule::confiscate_collateral_and_debit(&BOB, BTC, 5000, 1000),
			orml_tokens::Error::<Runtime>::BalanceTooLow
		);

		assert_ok!(EcdpLoansModule::adjust_position(&ALICE, BTC, 500, 300));
		assert_eq!(EcdpUssdTreasuryModule::get_total_collaterals(BTC), 0);
		assert_eq!(EcdpUssdTreasuryModule::debit_pool(), 0);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 300);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 500);

		assert_ok!(EcdpLoansModule::confiscate_collateral_and_debit(&ALICE, BTC, 300, 200));
		assert_eq!(EcdpUssdTreasuryModule::get_total_collaterals(BTC), 300);
		assert_eq!(EcdpUssdTreasuryModule::debit_pool(), 100);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).debit, 100);
		assert_eq!(EcdpLoansModule::positions(BTC, &ALICE).collateral, 200);
		System::assert_last_event(RuntimeEvent::EcdpLoansModule(crate::Event::ConfiscateCollateralAndDebit {
			owner: ALICE,
			collateral_type: BTC,
			confiscated_collateral_amount: 300,
			deduct_debit_amount: 200,
		}));
	});
}

// #[test]
// fn loan_updated_updated_when_adjust_collateral() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(EDF_SHARES.with(|v| *v.borrow().get(&BOB).unwrap_or(&0)), 0);

// 		assert_ok!(EcdpLoansModule::update_loan(&BOB, EDF, 1000, 0));
// 		assert_eq!(EDF_SHARES.with(|v| *v.borrow().get(&BOB).unwrap_or(&0)), 1000);

// 		assert_ok!(EcdpLoansModule::update_loan(&BOB, EDF, 0, 200));
// 		assert_eq!(EDF_SHARES.with(|v| *v.borrow().get(&BOB).unwrap_or(&0)), 1000);

// 		assert_ok!(EcdpLoansModule::update_loan(&BOB, EDF, -800, 500));
// 		assert_eq!(EDF_SHARES.with(|v| *v.borrow().get(&BOB).unwrap_or(&0)), 200);
// 	});
// }
