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
#![allow(clippy::unused_unit)]

use frame_support::{
	pallet_prelude::*,
	traits::{CallMetadata, Contains, GetCallMetadata, PalletInfoAccess},
};
use frame_system::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::DispatchResult;
use sp_std::{prelude::*, vec::Vec};

mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

/// The origin which may set filter.
		type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
/// can not pause
		CannotPause,
/// invalid character encoding
		InvalidCharacter,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
/// Paused transaction
		TransactionPaused {
			pallet_name_bytes: Vec<u8>,
			function_name_bytes: Vec<u8>,
		},
/// Unpaused transaction
		TransactionUnpaused {
			pallet_name_bytes: Vec<u8>,
			function_name_bytes: Vec<u8>,
		},
/// Paused EVM precompile
		EvmPrecompilePaused { address: H160 },
/// Unpaused EVM precompile
		EvmPrecompileUnpaused { address: H160 },
	}

/// The paused transaction map
///
/// map (PalletNameBytes, FunctionNameBytes) => Option<()>
	#[pallet::storage]
	#[pallet::getter(fn paused_transactions)]
	pub type PausedTransactions<T: Config> = StorageMap<_, Twox64Concat, (Vec<u8>, Vec<u8>), (), OptionQuery>;

/// The paused EVM precompile map
///
/// map (PrecompileAddress) => Option<()>
	#[pallet::storage]
	#[pallet::getter(fn paused_evm_precompiles)]
	pub type PausedEvmPrecompiles<T: Config> = StorageMap<_, Blake2_128Concat, H160, (), OptionQuery>;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::pause_transaction())]
		pub fn pause_transaction(origin: OriginFor<T>, pallet_name: Vec<u8>, function_name: Vec<u8>) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;

// not allowed to pause calls of this pallet to ensure safe
			let pallet_name_string = sp_std::str::from_utf8(&pallet_name).map_err(|_| Error::<T>::InvalidCharacter)?;
			ensure!(
				pallet_name_string != <Self as PalletInfoAccess>::name(),
				Error::<T>::CannotPause
			);

			PausedTransactions::<T>::mutate_exists((pallet_name.clone(), function_name.clone()), |maybe_paused| {
				if maybe_paused.is_none() {
					*maybe_paused = Some(());
					Self::deposit_event(Event::TransactionPaused {
						pallet_name_bytes: pallet_name,
						function_name_bytes: function_name,
					});
				}
			});
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::unpause_transaction())]
		pub fn unpause_transaction(
			origin: OriginFor<T>,
			pallet_name: Vec<u8>,
			function_name: Vec<u8>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			if PausedTransactions::<T>::take((&pallet_name, &function_name)).is_some() {
				Self::deposit_event(Event::TransactionUnpaused {
					pallet_name_bytes: pallet_name,
					function_name_bytes: function_name,
				});
			};
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::pause_evm_precompile())]
		pub fn pause_evm_precompile(origin: OriginFor<T>, address: H160) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			PausedEvmPrecompiles::<T>::mutate_exists(address, |maybe_paused| {
				if maybe_paused.is_none() {
					*maybe_paused = Some(());
					Self::deposit_event(Event::EvmPrecompilePaused { address });
				}
			});
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::unpause_evm_precompile())]
		pub fn unpause_evm_precompile(origin: OriginFor<T>, address: H160) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			if PausedEvmPrecompiles::<T>::take(address).is_some() {
				Self::deposit_event(Event::EvmPrecompileUnpaused { address });
			};
			Ok(())
		}
	}
}

pub struct PausedTransactionFilter<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> Contains<T::RuntimeCall> for PausedTransactionFilter<T>
where
	<T as frame_system::Config>::RuntimeCall: GetCallMetadata,
{
	fn contains(call: &T::RuntimeCall) -> bool {
		let CallMetadata {
			function_name,
			pallet_name,
		} = call.get_call_metadata();
		PausedTransactions::<T>::contains_key((pallet_name.as_bytes(), function_name.as_bytes()))
	}
}

pub struct PausedPrecompileFilter<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> module_support::PrecompilePauseFilter for PausedPrecompileFilter<T> {
	fn is_paused(address: H160) -> bool {
		PausedEvmPrecompiles::<T>::contains_key(address)
	}
}
