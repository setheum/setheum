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
#![allow(clippy::unused_unit)]

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{EnsureOrigin, ExistenceRequirement, Get, LockIdentifier},
	BoundedVec,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use module_traits::{MultiCurrency, MultiLockableCurrency};
pub use primitives::{CurrencyId, VestingSchedule};
use sp_runtime::{
	traits::{Saturating, StaticLookup, Zero},
	ArithmeticError, DispatchResult,
};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod weights;

pub use module::*;
pub use weights::WeightInfo;

pub const VESTING_LOCK_ID: LockIdentifier = *b"set/vest";

#[frame_support::pallet]
pub mod module {
	use super::*;

	pub(crate) type BalanceOf<T> =
		<<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
	pub(crate) type CurrencyIdOf<T> =
		<<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::CurrencyId;
	pub(crate) type VestingScheduleOf<T> = VestingSchedule<BlockNumberFor<T>, BalanceOf<T>>;
	pub type ScheduledItem<T> = (
		<T as frame_system::Config>::AccountId,
		CurrencyIdOf<T>,
		BlockNumberFor<T>,
		BlockNumberFor<T>,
		u32,
		BalanceOf<T>,
	);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type MultiCurrency: MultiLockableCurrency<Self::AccountId, CurrencyId = CurrencyId, Moment = BlockNumberFor<Self>>;

		/// Native Setheum (SEU) currency id.
		#[pallet::constant]
		type GetNativeCurrencyId: Get<CurrencyId>;

		/// The SetUSD currency id.
		#[pallet::constant]
		type GetSetUSDId: Get<CurrencyId>;

		/// The minimum amount transferred to call `vested_transfer`.
		#[pallet::constant]
		type MinVestedTransfer: Get<BalanceOf<Self>>;

		/// The account that funds vested transfers.
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		/// Required origin for `vested_transfer` and `update_vesting_schedules`.
		type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Weight information for extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The maximum number of vesting schedules for SEU.
		#[pallet::constant]
		type MaxNativeVestingSchedules: Get<u32>;

		/// The maximum number of vesting schedules for SetUSD.
		#[pallet::constant]
		type MaxSetUSDVestingSchedules: Get<u32>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Vesting period is zero
		ZeroVestingPeriod,
		/// Number of vests is zero
		ZeroVestingPeriodCount,
		/// Insufficient amount of balance to lock
		InsufficientBalanceToLock,
		/// This account have too many vesting schedules
		TooManyVestingSchedules,
		/// The vested transfer amount is too low
		AmountLow,
		/// Failed because the maximum vesting schedules for SEU was exceeded
		MaxNativeVestingSchedulesExceeded,
		/// Failed because the maximum vesting schedules for SetUSD was exceeded
		MaxSetUSDVestingSchedulesExceeded,
		/// Vesting is only supported for SEU and SetUSD
		UnsupportedCurrency,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Added new vesting schedule.
		VestingScheduleAdded {
			currency_id: CurrencyIdOf<T>,
			from: T::AccountId,
			to: T::AccountId,
			vesting_schedule: VestingScheduleOf<T>,
		},
		/// Claimed vesting.
		Claimed { currency_id: CurrencyIdOf<T>, who: T::AccountId, amount: BalanceOf<T> },
		/// Updated vesting schedules.
		VestingSchedulesUpdated { currency_id: CurrencyIdOf<T>, who: T::AccountId },
	}

	/// Vesting schedules of an account under SetUSD.
	///
	/// VestingSchedules: map AccountId => Vec<VestingSchedule>
	#[pallet::storage]
	#[pallet::getter(fn vesting_schedules)]
	pub type VestingSchedules<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<VestingScheduleOf<T>, T::MaxSetUSDVestingSchedules>,
		ValueQuery,
	>;

	/// Vesting schedules of an account under Native Currency (SEU).
	///
	/// NativeVestingSchedules: map AccountId => Vec<VestingSchedule>
	#[pallet::storage]
	#[pallet::getter(fn native_vesting_schedules)]
	pub type NativeVestingSchedules<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<VestingScheduleOf<T>, T::MaxNativeVestingSchedules>,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vesting: Vec<ScheduledItem<T>>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { vesting: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			self.vesting.iter().for_each(|(who, currency_id, start, period, period_count, per_period)| {
				let schedule = VestingSchedule {
					start: *start,
					period: *period,
					period_count: *period_count,
					per_period: *per_period,
				};

				let _ = ensure_valid_vesting_schedule::<T>(*currency_id, &schedule)
					.expect("Invalid vesting schedule");
				let total = schedule.total_amount().unwrap();
				assert!(
					T::MultiCurrency::free_balance(*currency_id, who) >= total,
					"Account does not have enough balance"
				);

				if *currency_id == T::GetNativeCurrencyId::get() {
					<NativeVestingSchedules<T>>::try_append(who, schedule)
						.expect("Max native vesting schedules exceeded");
				} else if *currency_id == T::GetSetUSDId::get() {
					<VestingSchedules<T>>::try_append(who, schedule)
						.expect("Max SetUSD vesting schedules exceeded");
				} else {
					panic!("Unsupported vesting currency");
				}

				let locked = Pallet::<T>::locked_balance(*currency_id, who);
				let _ = T::MultiCurrency::set_lock(VESTING_LOCK_ID, *currency_id, who, locked);
			});
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claim a vested transfer for the caller.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::claim(<T as Config>::MaxNativeVestingSchedules::get() / 2))]
		pub fn claim(origin: OriginFor<T>, currency_id: CurrencyIdOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let locked_amount = Self::do_claim(currency_id, &who);

			Self::deposit_event(Event::Claimed { currency_id, who, amount: locked_amount });
			Ok(())
		}

		/// Create a vested transfer from the treasury account.
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::vested_transfer())]
		pub fn vested_transfer(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			schedule: VestingScheduleOf<T>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			let from = T::TreasuryAccount::get();
			let to = T::Lookup::lookup(dest)?;

			if to == from {
				ensure!(
					T::MultiCurrency::free_balance(currency_id, &from)
						>= schedule.total_amount().ok_or(ArithmeticError::Overflow)?,
					Error::<T>::InsufficientBalanceToLock,
				);
			}

			Self::do_vested_transfer(currency_id, &from, &to, schedule.clone())?;

			Self::deposit_event(Event::VestingScheduleAdded {
				currency_id,
				from,
				to,
				vesting_schedule: schedule,
			});
			Ok(())
		}

		/// Replace vesting schedules of an account.
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::update_vesting_schedules(vesting_schedules.len() as u32))]
		pub fn update_vesting_schedules(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			who: <T::Lookup as StaticLookup>::Source,
			vesting_schedules: Vec<VestingScheduleOf<T>>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;

			let account = T::Lookup::lookup(who)?;
			Self::do_update_vesting_schedules(currency_id, &account, vesting_schedules)?;

			Self::deposit_event(Event::VestingSchedulesUpdated { currency_id, who: account });
			Ok(())
		}

		/// Claim a vested transfer for another account.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::claim(<T as Config>::MaxNativeVestingSchedules::get() / 2))]
		pub fn claim_for(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let who = T::Lookup::lookup(dest)?;
			let locked_amount = Self::do_claim(currency_id, &who);

			Self::deposit_event(Event::Claimed { currency_id, who, amount: locked_amount });
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn do_claim(currency_id: CurrencyIdOf<T>, who: &T::AccountId) -> BalanceOf<T> {
		let locked = Self::locked_balance(currency_id, who);
		if locked.is_zero() {
			if currency_id == T::GetNativeCurrencyId::get() {
				<NativeVestingSchedules<T>>::remove(who);
				let _ = T::MultiCurrency::remove_lock(VESTING_LOCK_ID, currency_id, who);
			} else if currency_id == T::GetSetUSDId::get() {
				<VestingSchedules<T>>::remove(who);
				let _ = T::MultiCurrency::remove_lock(VESTING_LOCK_ID, currency_id, who);
			}
		} else {
			let _ = T::MultiCurrency::set_lock(VESTING_LOCK_ID, currency_id, who, locked);
		}
		locked
	}

	/// Returns locked balance based on the current block number.
	fn locked_balance(currency_id: CurrencyIdOf<T>, who: &T::AccountId) -> BalanceOf<T> {
		let now = frame_system::Pallet::<T>::block_number();

		let update = |maybe_schedules: &mut Option<BoundedVec<VestingScheduleOf<T>, T::MaxNativeVestingSchedules>>| {
			let total = if let Some(schedules) = maybe_schedules.as_mut() {
				let mut total: BalanceOf<T> = Zero::zero();
				schedules.retain(|s| {
					let amount = s.locked_amount(now);
					total = total.saturating_add(amount);
					!amount.is_zero()
				});
				total
			} else {
				Zero::zero()
			};
			if total.is_zero() {
				*maybe_schedules = None;
			}
			total
		};

		let update_setusd =
			|maybe_schedules: &mut Option<BoundedVec<VestingScheduleOf<T>, T::MaxSetUSDVestingSchedules>>| {
				let total = if let Some(schedules) = maybe_schedules.as_mut() {
					let mut total: BalanceOf<T> = Zero::zero();
					schedules.retain(|s| {
						let amount = s.locked_amount(now);
						total = total.saturating_add(amount);
						!amount.is_zero()
					});
					total
				} else {
					Zero::zero()
				};
				if total.is_zero() {
					*maybe_schedules = None;
				}
				total
			};

		if currency_id == T::GetNativeCurrencyId::get() {
			<NativeVestingSchedules<T>>::mutate_exists(who, update)
		} else if currency_id == T::GetSetUSDId::get() {
			<VestingSchedules<T>>::mutate_exists(who, update_setusd)
		} else {
			Zero::zero()
		}
	}

	fn do_vested_transfer(
		currency_id: CurrencyIdOf<T>,
		from: &T::AccountId,
		to: &T::AccountId,
		schedule: VestingScheduleOf<T>,
	) -> DispatchResult {
		if currency_id == T::GetNativeCurrencyId::get() {
			let schedule_amount = ensure_valid_vesting_schedule::<T>(currency_id, &schedule)?;

			T::MultiCurrency::transfer(currency_id, from, to, schedule_amount, ExistenceRequirement::AllowDeath)?;
			<NativeVestingSchedules<T>>::try_append(to, schedule)
				.map_err(|_| Error::<T>::MaxNativeVestingSchedulesExceeded)?;

			let total_amount = Self::locked_balance(currency_id, to);
			T::MultiCurrency::set_lock(VESTING_LOCK_ID, currency_id, to, total_amount)?;
		} else if currency_id == T::GetSetUSDId::get() {
			let schedule_amount = ensure_valid_vesting_schedule::<T>(currency_id, &schedule)?;

			T::MultiCurrency::transfer(currency_id, from, to, schedule_amount, ExistenceRequirement::AllowDeath)?;
			<VestingSchedules<T>>::try_append(to, schedule)
				.map_err(|_| Error::<T>::MaxSetUSDVestingSchedulesExceeded)?;

			let total_amount = Self::locked_balance(currency_id, to);
			T::MultiCurrency::set_lock(VESTING_LOCK_ID, currency_id, to, total_amount)?;
		} else {
			return Err(Error::<T>::UnsupportedCurrency.into());
		}
		Ok(())
	}

	fn do_update_vesting_schedules(
		currency_id: CurrencyIdOf<T>,
		who: &T::AccountId,
		schedules: Vec<VestingScheduleOf<T>>,
	) -> DispatchResult {
		if currency_id == T::GetNativeCurrencyId::get() {
			let bounded_schedules: BoundedVec<VestingScheduleOf<T>, T::MaxNativeVestingSchedules> = schedules
				.try_into()
				.map_err(|_| Error::<T>::MaxNativeVestingSchedulesExceeded)?;

			// empty vesting schedules cleanup the storage and unlock the fund
			if bounded_schedules.len().is_zero() {
				<NativeVestingSchedules<T>>::remove(who);
				let _ = T::MultiCurrency::remove_lock(VESTING_LOCK_ID, currency_id, who);
				return Ok(());
			}

			<NativeVestingSchedules<T>>::insert(who, bounded_schedules);
			let total_amount = Self::locked_balance(currency_id, who);

			ensure!(
				T::MultiCurrency::free_balance(currency_id, who) >= total_amount,
				Error::<T>::InsufficientBalanceToLock,
			);
			T::MultiCurrency::set_lock(VESTING_LOCK_ID, currency_id, who, total_amount)?;
		} else if currency_id == T::GetSetUSDId::get() {
			let bounded_schedules: BoundedVec<VestingScheduleOf<T>, T::MaxSetUSDVestingSchedules> = schedules
				.try_into()
				.map_err(|_| Error::<T>::MaxSetUSDVestingSchedulesExceeded)?;

			if bounded_schedules.len().is_zero() {
				<VestingSchedules<T>>::remove(who);
				let _ = T::MultiCurrency::remove_lock(VESTING_LOCK_ID, currency_id, who);
				return Ok(());
			}

			<VestingSchedules<T>>::insert(who, bounded_schedules);
			let total_amount = Self::locked_balance(currency_id, who);

			ensure!(
				T::MultiCurrency::free_balance(currency_id, who) >= total_amount,
				Error::<T>::InsufficientBalanceToLock,
			);
			T::MultiCurrency::set_lock(VESTING_LOCK_ID, currency_id, who, total_amount)?;
		} else {
			return Err(Error::<T>::UnsupportedCurrency.into());
		}
		Ok(())
	}
}

/// Returns `Ok(total_amount)` if the schedule is valid, or an error.
fn ensure_valid_vesting_schedule<T: Config>(
	currency_id: CurrencyIdOf<T>,
	schedule: &VestingScheduleOf<T>,
) -> Result<BalanceOf<T>, DispatchError> {
	ensure!(!schedule.period.is_zero(), Error::<T>::ZeroVestingPeriod);
	ensure!(!schedule.period_count.is_zero(), Error::<T>::ZeroVestingPeriodCount);
	ensure!(schedule.end().is_some(), ArithmeticError::Overflow);

	let total_amount = schedule.total_amount().ok_or(ArithmeticError::Overflow)?;

	if currency_id == T::GetNativeCurrencyId::get() || currency_id == T::GetSetUSDId::get() {
		ensure!(total_amount >= T::MinVestedTransfer::get(), Error::<T>::AmountLow);
	}

	Ok(total_amount)
}
