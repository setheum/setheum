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
#![allow(warnings)]
#![allow(deprecated)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::unused_unit)]

use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;
use xcm::v5::prelude::*;

use module_xcm_support::UnknownAsset;

pub use module::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event {
		/// Deposit success.
		Deposited { asset: Asset, who: Location },
		/// Withdraw success.
		Withdrawn { asset: Asset, who: Location },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The balance is too low.
		BalanceTooLow,
		/// The operation will cause balance to overflow.
		BalanceOverflow,
		/// Unhandled asset.
		UnhandledAsset,
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Concrete fungible balances under a given location and a concrete
	/// fungible id.
	///
	/// double_map: who, asset_id => u128
	#[pallet::storage]
	#[pallet::getter(fn concrete_fungible_balances)]
	pub(crate) type ConcreteFungibleBalances<T> =
		StorageDoubleMap<_, Blake2_128Concat, Location, Blake2_128Concat, Location, u128, ValueQuery>;

	/// Abstract fungible balances under a given location and a abstract
	/// fungible id.
	///
	/// double_map: who, asset_id => u128
	#[pallet::storage]
	#[pallet::getter(fn abstract_fungible_balances)]
	pub(crate) type AbstractFungibleBalances<T> =
		StorageDoubleMap<_, Blake2_128Concat, Location, Blake2_128Concat, Vec<u8>, u128, ValueQuery>;
}

impl<T: Config> UnknownAsset for Pallet<T> {
	fn deposit(asset: &Asset, to: &Location) -> DispatchResult {
		match asset {
			Asset { fun: Fungible(amount), id: AssetId(location) } => {
				ConcreteFungibleBalances::<T>::try_mutate(to, location, |b| -> DispatchResult {
					*b = b.checked_add(*amount).ok_or(Error::<T>::BalanceOverflow)?;
					Ok(())
				})
			},
			_ => Err(Error::<T>::UnhandledAsset.into()),
		}?;

		Self::deposit_event(Event::Deposited { asset: asset.clone(), who: to.clone() });

		Ok(())
	}

	fn withdraw(asset: &Asset, from: &Location) -> DispatchResult {
		match asset {
			Asset { fun: Fungible(amount), id: AssetId(location) } => {
				ConcreteFungibleBalances::<T>::try_mutate(from, location, |b| -> DispatchResult {
					*b = b.checked_sub(*amount).ok_or(Error::<T>::BalanceTooLow)?;
					Ok(())
				})
			},
			_ => Err(Error::<T>::UnhandledAsset.into()),
		}?;

		Self::deposit_event(Event::Withdrawn { asset: asset.clone(), who: from.clone() });

		Ok(())
	}
}
