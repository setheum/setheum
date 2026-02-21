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

//! # Oracle
//! A module to allow oracle operators to feed external data.
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This module exposes capabilities for oracle operators to feed external
//! offchain data. The raw values can be combined to provide an aggregated
//! value.
//!
//! The data is valid only if fed by an authorized operator.
//! `pallet_membership` in FRAME can be used to as source of `T::Members`.

#![cfg_attr(not(feature = "std"), no_std)]
// Disable the following two lints since they originate from an external macro (namely decl_storage)
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::useless_conversion)]

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::{
	dispatch::Pays,
	ensure,
	pallet_prelude::*,
	traits::{ChangeMembers, Get, SortedMembers, Time},
	weights::Weight,
	Parameter,
};
use frame_system::{ensure_root, ensure_signed, pallet_prelude::*};
pub use orml_traits::{CombineData, DataFeeder, DataProvider, DataProviderExtended, OnNewData};
use orml_utilities::OrderedSet;
use scale_info::TypeInfo;
use sp_runtime::{traits::Member, DispatchResult, RuntimeDebug};
use sp_std::{prelude::*, vec};

pub use crate::default_combine_data::DefaultCombineData;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod default_combine_data;
mod mock;
mod tests;
mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub use benchmarking::BenchmarkHelper;
pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
	use super::*;

	pub(crate) type MomentOf<T, I = ()> = <<T as Config<I>>::Time as Time>::Moment;
	pub(crate) type TimestampedValueOf<T, I = ()> = TimestampedValue<<T as Config<I>>::OracleValue, MomentOf<T, I>>;

	#[derive(
		Encode,
		Decode,
		DecodeWithMemTracking,
		RuntimeDebug,
		Eq,
		PartialEq,
		Clone,
		Copy,
		Ord,
		PartialOrd,
		TypeInfo,
		MaxEncodedLen,
	)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct TimestampedValue<Value, Moment> {
		pub value: Value,
		pub timestamp: Moment,
	}

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// Hook on new data received
		type OnNewData: OnNewData<Self::AccountId, Self::OracleKey, Self::OracleValue>;

		/// Provide the implementation to combine raw values to produce
		/// aggregated value
		type CombineData: CombineData<Self::OracleKey, TimestampedValueOf<Self, I>>;

		/// Time provider
		type Time: Time;

		/// The data key type
		type OracleKey: Parameter + Member + MaxEncodedLen;

		/// The data value type
		type OracleValue: Parameter + Member + Ord + MaxEncodedLen;

		/// The root operator account id, record all sudo feeds on this account.
		#[pallet::constant]
		type RootOperatorAccountId: Get<Self::AccountId>;

		/// Oracle operators.
		type Members: SortedMembers<Self::AccountId>;

		/// Weight information for extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// Maximum size of HasDispatched
		#[pallet::constant]
		type MaxHasDispatchedSize: Get<u32>;

		/// Maximum size the vector used for feed values
		#[pallet::constant]
		type MaxFeedValues: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper: BenchmarkHelper<Self::OracleKey, Self::OracleValue, Self::MaxFeedValues>;
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Sender does not have permission
		NoPermission,
		/// Feeder has already fed at this block
		AlreadyFeeded,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// New feed data is submitted.
		NewFeedData {
			sender: T::AccountId,
			values: Vec<(T::OracleKey, T::OracleValue)>,
		},
	}

	/// Raw values for each oracle operators
	#[pallet::storage]
	#[pallet::getter(fn raw_values)]
	pub type RawValues<T: Config<I>, I: 'static = ()> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::OracleKey, TimestampedValueOf<T, I>>;

	/// Up to date combined value from Raw Values
	#[pallet::storage]
	#[pallet::getter(fn values)]
	pub type Values<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Twox64Concat, <T as Config<I>>::OracleKey, TimestampedValueOf<T, I>>;

	/// If an oracle operator has fed a value in this block
	#[pallet::storage]
	pub(crate) type HasDispatched<T: Config<I>, I: 'static = ()> =
		StorageValue<_, OrderedSet<T::AccountId, T::MaxHasDispatchedSize>, ValueQuery>;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		/// `on_initialize` to return the weight used in `on_finalize`.
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			T::WeightInfo::on_finalize()
		}

		fn on_finalize(_n: BlockNumberFor<T>) {
			// cleanup for next block
			<HasDispatched<T, I>>::kill();
		}
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Feed the external value.
		///
		/// Require authorized operator.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::feed_values(values.len() as u32))]
		pub fn feed_values(
			origin: OriginFor<T>,
			values: BoundedVec<(T::OracleKey, T::OracleValue), T::MaxFeedValues>,
		) -> DispatchResultWithPostInfo {
			let feeder = ensure_signed(origin.clone())
				.map(Some)
				.or_else(|_| ensure_root(origin).map(|_| None))?;

			let who = Self::ensure_account(feeder)?;

			// ensure account hasn't dispatched an updated yet
			ensure!(
				HasDispatched::<T, I>::mutate(|set| set.insert(who.clone())),
				Error::<T, I>::AlreadyFeeded
			);

			Self::do_feed_values(who, values.into())?;
			Ok(Pays::No.into())
		}
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn read_raw_values(key: &T::OracleKey) -> Vec<TimestampedValueOf<T, I>> {
		T::Members::sorted_members()
			.iter()
			.chain([T::RootOperatorAccountId::get()].iter())
			.filter_map(|x| Self::raw_values(x, key))
			.collect()
	}

	/// Fetch current combined value.
	pub fn get(key: &T::OracleKey) -> Option<TimestampedValueOf<T, I>> {
		Self::values(key)
	}

	#[allow(clippy::complexity)]
	pub fn get_all_values() -> Vec<(T::OracleKey, Option<TimestampedValueOf<T, I>>)> {
		<Values<T, I>>::iter().map(|(k, v)| (k, Some(v))).collect()
	}

	fn combined(key: &T::OracleKey) -> Option<TimestampedValueOf<T, I>> {
		let values = Self::read_raw_values(key);
		T::CombineData::combine_data(key, values, Self::values(key))
	}

	fn ensure_account(who: Option<T::AccountId>) -> Result<T::AccountId, DispatchError> {
		// ensure feeder is authorized
		if let Some(who) = who {
			ensure!(T::Members::contains(&who), Error::<T, I>::NoPermission);
			Ok(who)
		} else {
			Ok(T::RootOperatorAccountId::get())
		}
	}

	fn do_feed_values(who: T::AccountId, values: Vec<(T::OracleKey, T::OracleValue)>) -> DispatchResult {
		let now = T::Time::now();
		for (key, value) in &values {
			let timestamped = TimestampedValue {
				value: value.clone(),
				timestamp: now,
			};
			RawValues::<T, I>::insert(&who, key, timestamped);

			// Update `Values` storage if `combined` yielded result.
			if let Some(combined) = Self::combined(key) {
				<Values<T, I>>::insert(key, combined);
			}

			T::OnNewData::on_new_data(&who, key, value);
		}
		Self::deposit_event(Event::NewFeedData { sender: who, values });
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> ChangeMembers<T::AccountId> for Pallet<T, I> {
	fn change_members_sorted(_incoming: &[T::AccountId], outgoing: &[T::AccountId], _new: &[T::AccountId]) {
		// remove values
		for removed in outgoing {
			let _ = RawValues::<T, I>::clear_prefix(removed, u32::MAX, None);
		}
	}

	fn set_prime(_prime: Option<T::AccountId>) {
		// nothing
	}
}

impl<T: Config<I>, I: 'static> DataProvider<T::OracleKey, T::OracleValue> for Pallet<T, I> {
	fn get(key: &T::OracleKey) -> Option<T::OracleValue> {
		Self::get(key).map(|timestamped_value| timestamped_value.value)
	}
}
impl<T: Config<I>, I: 'static> DataProviderExtended<T::OracleKey, TimestampedValueOf<T, I>> for Pallet<T, I> {
	fn get_no_op(key: &T::OracleKey) -> Option<TimestampedValueOf<T, I>> {
		Self::get(key)
	}

	#[allow(clippy::complexity)]
	fn get_all_values() -> Vec<(T::OracleKey, Option<TimestampedValueOf<T, I>>)> {
		Self::get_all_values()
	}
}

impl<T: Config<I>, I: 'static> DataFeeder<T::OracleKey, T::OracleValue, T::AccountId> for Pallet<T, I> {
	fn feed_value(who: Option<T::AccountId>, key: T::OracleKey, value: T::OracleValue) -> DispatchResult {
		Self::do_feed_values(Self::ensure_account(who)?, vec![(key, value)])
	}
}
