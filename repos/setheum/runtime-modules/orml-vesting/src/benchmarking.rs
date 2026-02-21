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

pub use crate::*;

use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::vec;

/// Helper trait for benchmarking.
pub trait BenchmarkHelper<AccountId, Balance> {
	fn get_vesting_account_and_amount() -> Option<(AccountId, Balance)>;
}

impl<AccountId, Balance> BenchmarkHelper<AccountId, Balance> for () {
	fn get_vesting_account_and_amount() -> Option<(AccountId, Balance)> {
		None
	}
}

fn set_balance<T: Config>(who: &T::AccountId, amount: BalanceOf<T>) {
	let _ = <<T as Config>::Currency as Currency<_>>::deposit_creating(&who, amount);
}

fn total_balance<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	<<T as Config>::Currency as Currency<_>>::total_balance(who)
}

fn free_balance<T: Config>(who: &T::AccountId) -> BalanceOf<T> {
	<<T as Config>::Currency as Currency<_>>::free_balance(who)
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn vested_transfer() {
		let schedule = VestingScheduleOf::<T> {
			start: 0u32.into(),
			period: 2u32.into(),
			period_count: 3u32.into(),
			per_period: T::MinVestedTransfer::get(),
		};

		// extra 1 dollar to pay fees
		let (from, amount) = T::BenchmarkHelper::get_vesting_account_and_amount().unwrap();
		set_balance::<T>(&from, schedule.total_amount().unwrap() + amount);

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		#[extrinsic_call]
		_(RawOrigin::Signed(from), to_lookup, schedule.clone());

		assert_eq!(total_balance::<T>(&to), schedule.total_amount().unwrap());
	}

	#[benchmark]
	fn claim(i: Linear<1, { T::MaxVestingSchedules::get() }>) {
		let mut schedule = VestingScheduleOf::<T> {
			start: 0u32.into(),
			period: 2u32.into(),
			period_count: 3u32.into(),
			per_period: T::MinVestedTransfer::get(),
		};

		// extra 1 dollar to pay fees
		let (from, amount) = T::BenchmarkHelper::get_vesting_account_and_amount().unwrap();
		set_balance::<T>(
			&from,
			schedule.total_amount().unwrap().saturating_mul(i.into()) + amount,
		);

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		for _ in 0..i {
			schedule.start = i.into();
			assert_ok!(Pallet::<T>::vested_transfer(
				RawOrigin::Signed(from.clone()).into(),
				to_lookup.clone(),
				schedule.clone()
			));
		}
		frame_system::Pallet::<T>::set_block_number(schedule.end().unwrap() + 1u32.into());

		#[extrinsic_call]
		_(RawOrigin::Signed(to.clone()));

		assert_eq!(
			free_balance::<T>(&to),
			schedule.total_amount().unwrap().saturating_mul(i.into()),
		);
	}

	#[benchmark]
	fn update_vesting_schedules(i: Linear<1, { T::MaxVestingSchedules::get() }>) {
		let mut schedule = VestingScheduleOf::<T> {
			start: 0u32.into(),
			period: 2u32.into(),
			period_count: 3u32.into(),
			per_period: T::MinVestedTransfer::get(),
		};

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		set_balance::<T>(&to, schedule.total_amount().unwrap().saturating_mul(i.into()));

		let mut schedules = vec![];
		for _ in 0..i {
			schedule.start = i.into();
			schedules.push(schedule.clone());
		}

		#[extrinsic_call]
		_(RawOrigin::Root, to_lookup, schedules);

		assert_eq!(
			free_balance::<T>(&to),
			schedule.total_amount().unwrap().saturating_mul(i.into()),
		);
	}

	impl_benchmark_test_suite! {
		Pallet,
		crate::mock::ExtBuilder::build(),
		crate::mock::Runtime,
	}
}
