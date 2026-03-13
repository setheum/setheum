// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;
use xcm::v5::prelude::*;

use module_xcm_support::UnknownAsset;

pub use module::*;

mod mock;
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
