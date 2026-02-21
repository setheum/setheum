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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::collapsible_if)]

use frame_support::{pallet_prelude::*, transactional, PalletId};
use module_support::{EcdpUssdTreasury, EcdpUssdRiskManager};
use orml_traits::{Happened, MultiCurrency, MultiCurrencyExtended};
use primitives::{Amount, Balance, CurrencyId, EcdpPosition};
use sp_runtime::{
	traits::{AccountIdConversion, Zero},
	ArithmeticError, DispatchResult,
};

mod mock;
mod tests;

pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

/// Currency type for deposit/withdraw collateral assets to/from USSD Loans module
		type Currency: MultiCurrencyExtended<
			Self::AccountId,
			CurrencyId = CurrencyId,
			Balance = Balance,
			Amount = Amount,
		>;

/// Risk manager is used to limit the debit size of CDP.
		type EcdpUssdRiskManager: EcdpUssdRiskManager<Self::AccountId, CurrencyId, Balance, Balance>;

/// CDP treasury for issuing/burning USSD and debit value adjustment.
		type EcdpUssdTreasury: EcdpUssdTreasury<Self::AccountId, Balance = Balance, CurrencyId = CurrencyId>;

/// The loan's module id, keep all collaterals of CDPs.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

// Remove it based on `TODO:[src/lib.rs:0]`.
/// Event handler which calls when update loan.
// type OnUpdateLoan: Happened<(Self::AccountId, CurrencyId, Amount, Balance)>;
	}

	#[pallet::error]
	pub enum Error<T> {
		AmountConvertFailed,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
/// EcdpPosition updated.
		EcdpPositionUpdated {
			owner: T::AccountId,
			collateral_type: CurrencyId,
			collateral_adjustment: Amount,
			debit_adjustment: Amount,
		},
/// Confiscate CDP's collateral assets and eliminate its debit.
		ConfiscateCollateralAndDebit {
			owner: T::AccountId,
			collateral_type: CurrencyId,
			confiscated_collateral_amount: Balance,
			deduct_debit_amount: Balance,
		},
/// Transfer loan.
		TransferLoan {
			from: T::AccountId,
			to: T::AccountId,
			currency_id: CurrencyId,
		},
	}

/// The collateralized debit positions, map from
/// Owner -> CollateralType -> EcdpPosition
///
/// EcdpPositions: double_map CurrencyId, AccountId => EcdpPosition
	#[pallet::storage]
	#[pallet::getter(fn positions)]
	pub type EcdpPositions<T: Config> =
		StorageDoubleMap<_, Twox64Concat, CurrencyId, Twox64Concat, T::AccountId, EcdpPosition, ValueQuery>;

/// The total collateralized debit positions, map from
/// CollateralType -> EcdpPosition
///
/// TotalEcdpPositions: CurrencyId => EcdpPosition
	#[pallet::storage]
	#[pallet::getter(fn total_positions)]
	pub type TotalEcdpPositions<T: Config> = StorageMap<_, Twox64Concat, CurrencyId, EcdpPosition, ValueQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

/// confiscate collateral and debit to cdp treasury.
///
/// Ensured atomic.
	#[transactional]
	pub fn confiscate_collateral_and_debit(
		who: &T::AccountId,
		currency_id: CurrencyId,
		collateral_confiscate: Balance,
		debit_decrease: Balance,
	) -> DispatchResult {
// convert balance type to amount type
		let collateral_adjustment = Self::amount_try_from_balance(collateral_confiscate)?;
		let debit_adjustment = Self::amount_try_from_balance(debit_decrease)?;

// transfer collateral to cdp treasury
		T::EcdpUssdTreasury::deposit_collateral(&Self::account_id(), currency_id, collateral_confiscate)?;

// deposit debit to cdp treasury
		let bad_debt_value = T::EcdpUssdRiskManager::get_debit_value(currency_id, debit_decrease);
		T::EcdpUssdTreasury::on_system_debit(bad_debt_value)?;

// update loan
		Self::update_loan(
			who,
			currency_id,
			collateral_adjustment.saturating_neg(),
			debit_adjustment.saturating_neg(),
		)?;

		Self::deposit_event(Event::ConfiscateCollateralAndDebit {
			owner: who.clone(),
			collateral_type: currency_id,
			confiscated_collateral_amount: collateral_confiscate,
			deduct_debit_amount: debit_decrease,
		});
		Ok(())
	}

/// adjust the position.
///
/// Ensured atomic.
	#[transactional]
	pub fn adjust_position(
		who: &T::AccountId,
		currency_id: CurrencyId,
		collateral_adjustment: Amount,
		debit_adjustment: Amount,
	) -> DispatchResult {
// mutate collateral and debit
// Note: if a new position, will inc consumer
		Self::update_loan(who, currency_id, collateral_adjustment, debit_adjustment)?;

		let collateral_balance_adjustment = Self::balance_try_from_amount_abs(collateral_adjustment)?;
		let debit_balance_adjustment = Self::balance_try_from_amount_abs(debit_adjustment)?;
		let module_account = Self::account_id();

		if collateral_adjustment.is_positive() {
			T::Currency::transfer(currency_id, who, &module_account, collateral_balance_adjustment)?;
		} else if collateral_adjustment.is_negative() {
			T::Currency::transfer(currency_id, &module_account, who, collateral_balance_adjustment)?;
		}

		if debit_adjustment.is_positive() {
// check debit cap when increase debit
			T::EcdpUssdRiskManager::check_debit_cap(currency_id, Self::total_positions(currency_id).debit)?;

// issue debit with collateral backed by cdp treasury
			T::EcdpUssdTreasury::issue_debit(
				who,
				T::EcdpUssdRiskManager::get_debit_value(currency_id, debit_balance_adjustment),
				true,
			)?;
		} else if debit_adjustment.is_negative() {
// repay debit
// burn debit by cdp treasury
			T::EcdpUssdTreasury::burn_debit(
				who,
				T::EcdpUssdRiskManager::get_debit_value(currency_id, debit_balance_adjustment),
			)?;
		}

// ensure pass risk check
		let EcdpPosition { collateral, debit } = Self::positions(currency_id, who);
		T::EcdpUssdRiskManager::check_position_valid(
			currency_id,
			collateral,
			debit,
			collateral_adjustment.is_negative() || debit_adjustment.is_positive(),
		)?;

		Ok(())
	}

/// transfer whole loan of `from` to `to`
	pub fn transfer_loan(from: &T::AccountId, to: &T::AccountId, currency_id: CurrencyId) -> DispatchResult {
// get `from` position data
		let EcdpPosition { collateral, debit } = Self::positions(currency_id, from);

		let EcdpPosition {
			collateral: to_collateral,
			debit: to_debit,
		} = Self::positions(currency_id, to);
		let new_to_collateral_balance = to_collateral
			.checked_add(collateral)
			.expect("existing collateral balance cannot overflow; qed");
		let new_to_debit_balance = to_debit
			.checked_add(debit)
			.expect("existing debit balance cannot overflow; qed");

// check new position
		T::EcdpUssdRiskManager::check_position_valid(currency_id, new_to_collateral_balance, new_to_debit_balance, true)?;

// balance -> amount
		let collateral_adjustment = Self::amount_try_from_balance(collateral)?;
		let debit_adjustment = Self::amount_try_from_balance(debit)?;

		Self::update_loan(
			from,
			currency_id,
			collateral_adjustment.saturating_neg(),
			debit_adjustment.saturating_neg(),
		)?;
		Self::update_loan(to, currency_id, collateral_adjustment, debit_adjustment)?;

		Self::deposit_event(Event::TransferLoan {
			from: from.clone(),
			to: to.clone(),
			currency_id,
		});
		Ok(())
	}

/// mutate records of collaterals and debits
	pub fn update_loan(
		who: &T::AccountId,
		currency_id: CurrencyId,
		collateral_adjustment: Amount,
		debit_adjustment: Amount,
	) -> DispatchResult {
		let collateral_balance = Self::balance_try_from_amount_abs(collateral_adjustment)?;
		let debit_balance = Self::balance_try_from_amount_abs(debit_adjustment)?;

		<EcdpPositions<T>>::try_mutate_exists(currency_id, who, |may_be_position| -> DispatchResult {
			let mut p = may_be_position.take().unwrap_or_default();
			let new_collateral = if collateral_adjustment.is_positive() {
				p.collateral
					.checked_add(collateral_balance)
					.ok_or(ArithmeticError::Overflow)
			} else {
				p.collateral
					.checked_sub(collateral_balance)
					.ok_or(ArithmeticError::Underflow)
			}?;
			let new_debit = if debit_adjustment.is_positive() {
				p.debit.checked_add(debit_balance).ok_or(ArithmeticError::Overflow)
			} else {
				p.debit.checked_sub(debit_balance).ok_or(ArithmeticError::Underflow)
			}?;

// increase account ref if new position
			if p.collateral.is_zero() && p.debit.is_zero() {
				if frame_system::Pallet::<T>::inc_consumers(who).is_err() {
// No providers for the locks. This is impossible under normal circumstances
// since the funds that are under the lock will themselves be stored in the
// account and therefore will need a reference.
					log::warn!(
						"Warning: Attempt to introduce lock consumer reference, yet no providers. \
						This is unexpected but should be safe."
					);
				}
			}

// TODO:[src/lib.rs:0] - Remove this from this module and add it to `EcdpLoans` module.
// Use the collateral amount (of a position that has fulfilled the `EcdpPositionCloudCreditRequirements` -
// this is only offered for SETR) as the shares for Cloud Credit.
//
// T::OnUpdateLoan::happened(&(who.clone(), currency_id, collateral_adjustment, p.collateral));
			
			p.collateral = new_collateral;
			p.debit = new_debit;

			if p.collateral.is_zero() && p.debit.is_zero() {
// decrease account ref if zero position
				frame_system::Pallet::<T>::dec_consumers(who);

// remove position storage if zero position
				*may_be_position = None;
			} else {
				*may_be_position = Some(p);
			}

			Ok(())
		})?;

		TotalEcdpPositions::<T>::try_mutate(currency_id, |total_positions| -> DispatchResult {
			total_positions.collateral = if collateral_adjustment.is_positive() {
				total_positions
					.collateral
					.checked_add(collateral_balance)
					.ok_or(ArithmeticError::Overflow)
			} else {
				total_positions
					.collateral
					.checked_sub(collateral_balance)
					.ok_or(ArithmeticError::Underflow)
			}?;

			total_positions.debit = if debit_adjustment.is_positive() {
				total_positions
					.debit
					.checked_add(debit_balance)
					.ok_or(ArithmeticError::Overflow)
			} else {
				total_positions
					.debit
					.checked_sub(debit_balance)
					.ok_or(ArithmeticError::Underflow)
			}?;

			Ok(())
		})?;

		Self::deposit_event(Event::EcdpPositionUpdated {
			owner: who.clone(),
			collateral_type: currency_id,
			collateral_adjustment,
			debit_adjustment,
		});
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
/// Convert `Balance` to `Amount`.
	pub fn amount_try_from_balance(b: Balance) -> Result<Amount, Error<T>> {
		TryInto::<Amount>::try_into(b).map_err(|_| Error::<T>::AmountConvertFailed)
	}

/// Convert the absolute value of `Amount` to `Balance`.
	pub fn balance_try_from_amount_abs(a: Amount) -> Result<Balance, Error<T>> {
		TryInto::<Balance>::try_into(a.saturating_abs()).map_err(|_| Error::<T>::AmountConvertFailed)
	}
}
