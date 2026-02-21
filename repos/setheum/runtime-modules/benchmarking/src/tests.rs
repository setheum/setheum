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

//! Tests for the module.

#![cfg(test)]

use super::*;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok, construct_runtime, derive_impl, ensure};
use frame_system::RawOrigin;
use sp_runtime::{testing::Header, traits::IdentityLookup, BuildStorage};
use sp_std::prelude::*;
pub use test::*;

#[frame_support::pallet(dev_mode)]
pub mod test {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::storage]
	#[pallet::getter(fn value)]
	pub(crate) type Value<T: Config> = StorageValue<_, u32, OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn set_value(origin: OriginFor<T>, n: u32) -> DispatchResult {
			let _sender = frame_system::ensure_signed(origin)?;
			Value::<T>::put(n);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn dummy(origin: OriginFor<T>, _n: u32) -> DispatchResult {
			let _sender = frame_system::ensure_none(origin)?;
			Ok(())
		}
	}
}

type AccountId = u128;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Nonce = u64;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
}

impl Config for Test {}

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, RuntimeCall, u32, ()>;

construct_runtime!(
	pub enum Test {
		System: frame_system,
		Pallet: test,
	}
);

// This function basically just builds a genesis storage key/value store
// according to our desired mockup.
fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into()
}

runtime_benchmarks! {
	{ Test, test }

	set_value {
		let b in 1 .. 1000;
		let caller = account::<AccountId>("caller", 0, 0);
	}: _ (RawOrigin::Signed(caller), b)
	verify {
		assert_eq!(Pallet::value(), Some(b));
	}

	other_name {
		let b in 1 .. 1000;
	}: dummy (RawOrigin::None, b)

	sort_vector {
		let x in 1 .. 10000;
		let mut m = Vec::<u32>::new();
		for i in (0..x).rev() {
			m.push(i);
		}
	}: {
		m.sort_unstable();
	} verify {
		ensure!(m[0] == 0, "You forgot to sort!")
	}

	bad_origin {
		let b in 1 .. 1000;
		let caller = account::<AccountId>("caller", 0, 0);
	}: dummy (RawOrigin::Signed(caller), b)

	bad_verify {
		let x in 1 .. 10000;
		let mut m = Vec::<u32>::new();
		for i in (0..x).rev() {
			m.push(i);
		}
	}: { }
	verify {
		ensure!(m[0] == 0, "You forgot to sort!")
	}
}

#[test]
fn benchmarks_macro_works() {
	// Check benchmark creation for `set_value`.
	let selected_benchmark = SelectedBenchmark::set_value;

	let components = <SelectedBenchmark as BenchmarkingSetup<Test>>::components(&selected_benchmark);
	assert_eq!(components, vec![(BenchmarkParameter::b, 1, 1000)]);

	new_test_ext().execute_with(|| {
		assert_ok!(<SelectedBenchmark as BenchmarkingSetup<Test>>::unit_test_instance(
			&selected_benchmark,
			&[(BenchmarkParameter::b, 1)],
		));
	});
}

#[test]
fn benchmarks_macro_rename_works() {
	// Check benchmark creation for `other_dummy`.
	let selected = SelectedBenchmark::other_name;
	let components = <SelectedBenchmark as BenchmarkingSetup<Test>>::components(&selected);
	assert_eq!(components, vec![(BenchmarkParameter::b, 1, 1000)]);

	new_test_ext().execute_with(|| {
		assert_ok!(<SelectedBenchmark as BenchmarkingSetup<Test>>::unit_test_instance(
			&selected,
			&[(BenchmarkParameter::b, 1)],
		));
	});
}

#[test]
fn benchmarks_macro_works_for_non_dispatchable() {
	let selected = SelectedBenchmark::sort_vector;

	let components = <SelectedBenchmark as BenchmarkingSetup<Test>>::components(&selected);
	assert_eq!(components, vec![(BenchmarkParameter::x, 1, 10000)]);

	new_test_ext().execute_with(|| {
		assert_ok!(<SelectedBenchmark as BenchmarkingSetup<Test>>::unit_test_instance(
			&selected,
			&[(BenchmarkParameter::x, 1)],
		));
	});
}

#[test]
fn benchmarks_macro_verify_works() {
	// Check postcondition for benchmark `set_value` is valid.
	let selected = SelectedBenchmark::set_value;

	new_test_ext().execute_with(|| {
		assert_ok!(<SelectedBenchmark as BenchmarkingSetup<Test>>::unit_test_instance(
			&selected,
			&[(BenchmarkParameter::b, 1)],
		));
	});

	// Check postcondition for benchmark `bad_verify` is invalid.
	let selected = SelectedBenchmark::bad_verify;

	new_test_ext().execute_with(|| {
		assert_err!(
			<SelectedBenchmark as BenchmarkingSetup<Test>>::unit_test_instance(
				&selected,
				&[(BenchmarkParameter::x, 10000)],
			),
			"You forgot to sort!"
		);
	});
}

#[test]
fn benchmarks_generate_unit_tests() {
	new_test_ext().execute_with(|| {
		assert_ok!(Benchmark::test_benchmark_set_value());
		assert_ok!(Benchmark::test_benchmark_other_name());
		assert_ok!(Benchmark::test_benchmark_sort_vector());
		assert_err!(Benchmark::test_benchmark_bad_origin(), "Bad origin");
		assert_err!(Benchmark::test_benchmark_bad_verify(), "You forgot to sort!");
	});
}
