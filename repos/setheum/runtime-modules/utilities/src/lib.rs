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

use frame_support::storage::{with_transaction, TransactionOutcome};
use sp_runtime::DispatchError;
use sp_std::result::Result;

pub mod offchain_worker;
pub mod ordered_set;

pub use offchain_worker::OffchainErr;
pub use ordered_set::OrderedSet;

/// Execute the supplied function in a new storage transaction.
///
/// All changes to storage performed by the supplied function are discarded if
/// the returned outcome is `Result::Err`.
///
/// Transactions can be nested to any depth. Commits happen to the parent
/// transaction.
pub fn with_transaction_result<R>(f: impl FnOnce() -> Result<R, DispatchError>) -> Result<R, DispatchError> {
	with_transaction(|| {
		let res = f();
		if res.is_ok() {
			TransactionOutcome::Commit(res)
		} else {
			TransactionOutcome::Rollback(res)
		}
	})
}

/// Simulate execution of the supplied function in a new storage transaction.
/// Changes to storage performed by the supplied function are always discarded.
pub fn simulate_execution<R>(f: impl FnOnce() -> Result<R, DispatchError>) -> Result<R, DispatchError> {
	with_transaction(|| {
		let res = f();
		TransactionOutcome::Rollback(res)
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::{assert_noop, assert_ok, construct_runtime, derive_impl, pallet_prelude::*};
	use sp_io::TestExternalities;
	use sp_runtime::traits::IdentityLookup;
	use sp_runtime::{DispatchError, DispatchResult};
	use sp_std::result::Result;

	#[allow(dead_code)]
	#[frame_support::pallet]
	pub mod module {
		use super::*;

		#[pallet::config]
		pub trait Config: frame_system::Config {}

		#[pallet::pallet]
		pub struct Pallet<T>(_);

		#[pallet::storage]
		pub type Value<T: Config> = StorageValue<_, u32, ValueQuery>;

		#[pallet::storage]
		pub type Map<T: Config> = StorageMap<_, Twox64Concat, [u8; 4], u32, ValueQuery>;
	}

	use module::*;

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	impl frame_system::Config for Runtime {
		type AccountId = u128;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Block = Block;
	}

	impl module::Config for Runtime {}

	type Block = frame_system::mocking::MockBlock<Runtime>;

	construct_runtime!(
		pub enum Runtime {
			System: frame_system,
			TestModule: module,
		}
	);

	#[test]
	fn storage_transaction_basic_commit() {
		TestExternalities::default().execute_with(|| {
			assert_eq!(Value::<Runtime>::get(), 0);
			assert!(!Map::<Runtime>::contains_key(b"val0"));

			assert_ok!(with_transaction_result(|| -> DispatchResult {
				Value::<Runtime>::set(99);
				Map::<Runtime>::insert(b"val0", 99);
				assert_eq!(Value::<Runtime>::get(), 99);
				assert_eq!(Map::<Runtime>::get(b"val0"), 99);
				Ok(())
			}));

			assert_eq!(Value::<Runtime>::get(), 99);
			assert_eq!(Map::<Runtime>::get(b"val0"), 99);
		});
	}

	#[test]
	fn storage_transaction_basic_rollback() {
		TestExternalities::default().execute_with(|| {
			assert_eq!(Value::<Runtime>::get(), 0);
			assert_eq!(Map::<Runtime>::get(b"val0"), 0);

			assert_noop!(
				with_transaction_result(|| -> DispatchResult {
					Value::<Runtime>::set(99);
					Map::<Runtime>::insert(b"val0", 99);
					assert_eq!(Value::<Runtime>::get(), 99);
					assert_eq!(Map::<Runtime>::get(b"val0"), 99);
					Err("test".into())
				}),
				DispatchError::Other("test")
			);

			assert_eq!(Value::<Runtime>::get(), 0);
			assert_eq!(Map::<Runtime>::get(b"val0"), 0);
		});
	}

	#[test]
	fn simulate_execution_works() {
		TestExternalities::default().execute_with(|| {
			assert_eq!(Value::<Runtime>::get(), 0);
			assert_eq!(Map::<Runtime>::get(b"val0"), 0);

			// Roll back on `Err`.
			assert_noop!(
				simulate_execution(|| -> DispatchResult {
					Value::<Runtime>::set(99);
					Map::<Runtime>::insert(b"val0", 99);
					Err(DispatchError::Other("test"))
				}),
				DispatchError::Other("test")
			);
			assert_eq!(Value::<Runtime>::get(), 0);
			assert_eq!(Map::<Runtime>::get(b"val0"), 0);

			// Roll back on `Ok`, but returns `Ok` result.
			assert_ok!(
				simulate_execution(|| -> Result<u32, DispatchError> {
					Value::<Runtime>::set(99);
					Map::<Runtime>::insert(b"val0", 99);
					Ok(99)
				}),
				99
			);
			assert_eq!(Value::<Runtime>::get(), 0);
			assert_eq!(Map::<Runtime>::get(b"val0"), 0);
		});
	}
}
