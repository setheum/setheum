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
use frame_system::{Pallet as System, RawOrigin};

/// Helper trait for benchmarking.
pub trait BenchmarkHelper<OracleKey, OracleValue, L: Get<u32>> {
	/// Returns a list of `(oracle_key, oracle_value)` pairs to be used for
	/// benchmarking.
	///
	/// NOTE: User should ensure to at least submit two values, otherwise the
	/// benchmark linear analysis might fail.
	fn get_currency_id_value_pairs() -> BoundedVec<(OracleKey, OracleValue), L>;
}

impl<OracleKey, OracleValue, L: Get<u32>> BenchmarkHelper<OracleKey, OracleValue, L> for () {
	fn get_currency_id_value_pairs() -> BoundedVec<(OracleKey, OracleValue), L> {
		BoundedVec::default()
	}
}

#[instance_benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn feed_values(x: Linear<0, { T::BenchmarkHelper::get_currency_id_value_pairs().len() as u32 }>) {
		// Register the caller
		let caller: T::AccountId = whitelisted_caller();
		T::Members::add(&caller);

		let values = T::BenchmarkHelper::get_currency_id_value_pairs()[..x as usize]
			.to_vec()
			.try_into()
			.expect("Must succeed since at worst the length remained the same.");

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), values);

		assert!(HasDispatched::<T, I>::get().contains(&caller));
	}

	#[benchmark]
	fn on_finalize() {
		// Register the caller
		let caller: T::AccountId = whitelisted_caller();
		T::Members::add(&caller);

		// Feed some values before running `on_finalize` hook
		System::<T>::set_block_number(1u32.into());
		let values = T::BenchmarkHelper::get_currency_id_value_pairs();
		assert_ok!(Pallet::<T, I>::feed_values(RawOrigin::Signed(caller).into(), values));

		#[block]
		{
			Pallet::<T, I>::on_finalize(System::<T>::block_number());
		}

		assert!(!HasDispatched::<T, I>::exists());
	}

	impl_benchmark_test_suite! {
		Pallet,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
