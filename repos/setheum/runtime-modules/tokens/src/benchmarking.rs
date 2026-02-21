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
use sp_runtime::traits::SaturatedConversion;

/// Helper trait for benchmarking.
pub trait BenchmarkHelper<CurrencyId, Balance> {
	/// Returns a currency id and amount to be used in benchmarking.
	fn get_currency_id_and_amount() -> Option<(CurrencyId, Balance)>;
}

impl<CurrencyId, Balance> BenchmarkHelper<CurrencyId, Balance> for () {
	fn get_currency_id_and_amount() -> Option<(CurrencyId, Balance)> {
		None
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn transfer() {
		let from: T::AccountId = account("from", 0, 0);

		let (currency_id, amount) = T::BenchmarkHelper::get_currency_id_and_amount().unwrap();

		assert_ok!(<Pallet::<T> as MultiCurrencyExtended<_>>::update_balance(
			currency_id,
			&from,
			amount.saturated_into()
		));

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		#[extrinsic_call]
		_(RawOrigin::Signed(from), to_lookup, currency_id, amount);

		assert_eq!(Pallet::<T>::total_balance(currency_id, &to), amount);
	}

	#[benchmark]
	fn transfer_all() {
		let from: T::AccountId = account("from", 0, 0);

		let (currency_id, amount) = T::BenchmarkHelper::get_currency_id_and_amount().unwrap();

		assert_ok!(<Pallet::<T> as MultiCurrencyExtended<_>>::update_balance(
			currency_id,
			&from,
			amount.saturated_into()
		));

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		#[extrinsic_call]
		_(RawOrigin::Signed(from.clone()), to_lookup, currency_id, false);

		assert_eq!(
			<Pallet::<T> as MultiCurrency<_>>::total_balance(currency_id, &from),
			0u32.into()
		);
	}

	#[benchmark]
	fn transfer_keep_alive() {
		let from: T::AccountId = account("from", 0, 0);

		let (currency_id, amount) = T::BenchmarkHelper::get_currency_id_and_amount().unwrap();

		assert_ok!(<Pallet::<T> as MultiCurrencyExtended<_>>::update_balance(
			currency_id,
			&from,
			amount.saturating_mul(2u32.into()).saturated_into()
		));

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		#[extrinsic_call]
		_(RawOrigin::Signed(from), to_lookup, currency_id, amount);

		assert_eq!(
			<Pallet::<T> as MultiCurrency<_>>::total_balance(currency_id, &to),
			amount
		);
	}

	#[benchmark]
	fn force_transfer() {
		let from: T::AccountId = account("from", 0, 0);
		let from_lookup = T::Lookup::unlookup(from.clone());

		let (currency_id, amount) = T::BenchmarkHelper::get_currency_id_and_amount().unwrap();

		assert_ok!(<Pallet::<T> as MultiCurrencyExtended<_>>::update_balance(
			currency_id,
			&from,
			amount.saturated_into()
		));

		let to: T::AccountId = account("to", 0, 0);
		let to_lookup = T::Lookup::unlookup(to.clone());

		#[extrinsic_call]
		_(RawOrigin::Root, from_lookup, to_lookup, currency_id, amount);

		assert_eq!(
			<Pallet::<T> as MultiCurrency<_>>::total_balance(currency_id, &to),
			amount
		);
	}

	#[benchmark]
	fn set_balance() {
		let who: T::AccountId = account("who", 0, 0);
		let who_lookup = T::Lookup::unlookup(who.clone());

		let (currency_id, amount) = T::BenchmarkHelper::get_currency_id_and_amount().unwrap();

		#[extrinsic_call]
		_(RawOrigin::Root, who_lookup, currency_id, amount, amount);

		assert_eq!(
			<Pallet::<T> as MultiCurrency<_>>::total_balance(currency_id, &who),
			amount.saturating_mul(2u32.into())
		);
	}

	impl_benchmark_test_suite! {
		Pallet,
		crate::mock::ExtBuilder::default().build(),
		crate::mock::Runtime,
	}
}
