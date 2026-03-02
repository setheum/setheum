// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::{DispatchClass, Pays, Weight};
use primitives::{task::TaskResult, AccountId, Balance, CurrencyId, Fees, Multiplier, Nonce, ReserveIdentifier};
use sp_runtime::{
	traits::CheckedDiv, transaction_validity::TransactionValidityError, DispatchError, DispatchResult, FixedU128,
};
use sp_std::{prelude::*, result::Result};
use xcm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AirdropEntry {
    pub account: AccountId,
    pub amount: Balance,
}

#[derive(Deserialize, Debug)]
pub struct AirdropList(pub Vec<AirdropEntry>);

pub mod bounded;
// pub mod ecdp;
pub mod edfis_launchpad;
pub mod edfis_swap;
pub mod edfis_swap_legacy;
pub mod evm;
pub mod migration;
pub mod mocks;

pub use crate::bounded::*;
// pub use crate::ecdp::*;
pub use crate::edfis_launchpad::*;
pub use crate::edfis_swap::*;
pub use crate::edfis_swap_legacy::*;
pub use crate::evm::*;
pub use crate::migration::*;

pub type Price = FixedU128;
pub type ExchangeRate = FixedU128;
pub type Ratio = FixedU128;
pub type Rate = FixedU128;

/// Implement this StoredMap to replace https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/system/src/lib.rs#L1679
/// NOTE: If use module-evm, need regards existed `frame_system::Account` also exists
/// `pallet_balances::Account`, even if it's AccountData is default. (This kind of account is
/// usually created by inc_provider), so that `repatriate_reserved` can transfer reserved balance to
/// contract account, which is created by `inc_provider`.
pub struct SystemAccountStore<T>(sp_std::marker::PhantomData<T>);
impl<T: frame_system::Config> frame_support::traits::StoredMap<T::AccountId, T::AccountData> for SystemAccountStore<T> {
	fn get(k: &T::AccountId) -> T::AccountData {
		frame_system::Account::<T>::get(k).data
	}

	fn try_mutate_exists<R, E: From<DispatchError>>(
		k: &T::AccountId,
		f: impl FnOnce(&mut Option<T::AccountData>) -> Result<R, E>,
	) -> Result<R, E> {
		let account = frame_system::Account::<T>::get(k);
		let is_default = account.data == T::AccountData::default();

// if System Account exists, act its Balances Account also exists.
		let mut some_data = if is_default && !frame_system::Pallet::<T>::account_exists(k) {
			None
		} else {
			Some(account.data)
		};

		let result = f(&mut some_data)?;
		if frame_system::Pallet::<T>::providers(k) > 0 || frame_system::Pallet::<T>::sufficients(k) > 0 {
			frame_system::Account::<T>::mutate(k, |a| a.data = some_data.unwrap_or_default());
		} else {
			frame_system::Account::<T>::remove(k)
		}
		Ok(result)
	}
}

pub trait PriceProvider<CurrencyId> {
	fn get_price(currency_id: CurrencyId) -> Option<Price>;
	fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<Price> {
		if let (Some(base_price), Some(quote_price)) = (Self::get_price(base), Self::get_price(quote)) {
			base_price.checked_div(&quote_price)
		} else {
			None
		}
	}
}

pub trait SwapPriceProvider<CurrencyId> {
	fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<ExchangeRate>;
}

pub trait LockablePrice<CurrencyId> {
	fn lock_price(currency_id: CurrencyId) -> DispatchResult;
	fn unlock_price(currency_id: CurrencyId) -> DispatchResult;
}

pub trait ExchangeRateProvider {
	fn get_exchange_rate() -> ExchangeRate;
}

pub trait TransactionPayment<AccountId, Balance, NegativeImbalance> {
	fn reserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Result<Balance, DispatchError>;
	fn unreserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Balance;
	fn unreserve_and_charge_fee(
		who: &AccountId,
		weight: Weight,
	) -> Result<(Balance, NegativeImbalance), TransactionValidityError>;
	fn refund_fee(who: &AccountId, weight: Weight, payed: NegativeImbalance) -> Result<(), TransactionValidityError>;
	fn charge_fee(
		who: &AccountId,
		len: u32,
		weight: Weight,
		tip: Balance,
		pays_fee: Pays,
		class: DispatchClass,
	) -> Result<(), TransactionValidityError>;
	fn weight_to_fee(weight: Weight) -> Balance;
	fn apply_multiplier_to_fee(fee: Balance, multiplier: Option<Multiplier>) -> Balance;
}

/// Dispatchable tasks
pub trait DispatchableTask {
	fn dispatch(self, weight: Weight) -> TaskResult;
}

#[cfg(feature = "std")]
impl DispatchableTask for () {
	fn dispatch(self, _weight: Weight) -> TaskResult {
		unimplemented!()
	}
}

/// Idle scheduler for dispatching tasks during on_idle
pub trait IdleScheduler<Task> {
	fn schedule(task: Task) -> DispatchResult;
}

pub trait OnNewEra<EraIndex> {
	fn on_new_era(era: EraIndex);
}

impl<EraIndex> OnNewEra<EraIndex> for () {
	fn on_new_era(_era: EraIndex) {}
}

pub trait NomineesProvider<AccountId> {
	fn nominees() -> Vec<AccountId>;
}

pub trait LiquidateCollateral<AccountId> {
	fn liquidate(
		who: &AccountId,
		currency_id: CurrencyId,
		amount: Balance,
		target_seusd_amount: Balance,
	) -> DispatchResult;
}

impl<AccountId> LiquidateCollateral<AccountId> for () {
	fn liquidate(
		_who: &AccountId,
		_currency_id: CurrencyId,
		_amount: Balance,
		_target_seusd_amount: Balance,
	) -> DispatchResult {
		Err(DispatchError::Other("No liquidation impl."))
	}
}

pub trait BuyWeightRate {
	fn calculate_rate(location: Location) -> Option<Ratio>;
}
