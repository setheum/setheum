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
#![allow(clippy::type_complexity)]

use frame_support::{pallet_prelude::*, transactional, PalletId, traits::Get};
use frame_system::pallet_prelude::*;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use primitives::{AccountId, Balance, CurrencyId};
use module_support::AirdropList;
use sp_std::vec::Vec;
use sp_std::collections::btree_set::BTreeSet;
use sp_runtime::traits::AccountIdConversion;
use frame_support::storage::TransactionOutcome;

mod mock;
mod tests;

pub use module::*;

type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

/// The Currency for managing assets.
		type MultiCurrency: MultiCurrencyExtended<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

/// The maximum size of an airdrop list
		type MaxAirdropListSize: Get<usize>;

		#[pallet::constant]
/// The Airdrop module pallet id, keeps airdrop funds.
		type PalletId: Get<PalletId>;
	}

	#[pallet::error]
	pub enum Error<T> {
// Duplicate Airdrop Account
		DuplicateAccounts,
// The airdrop list is over the max size limit `MaxAirdropListSize`
		OverSizedAirdropList,
// Error parsing the JSON data
		InvalidJson,
// Invalid Account ID
		InvalidAccountId,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance", CurrencyId = "CurrencyId")]
	pub enum Event<T: Config> {
/// Drop Airdrop
		Airdrop {
			currency_id: CurrencyId,
			airdrop_list: Vec<(T::AccountId, Balance)>
		},
/// Drop Airdrop with JSON Data
		AirdropWithJson {
			currency_id: CurrencyId,
			airdrop_list: AirdropList
		},
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
/// Make Airdrop to beneficiaries.
///
/// Any account can call this function.
///
/// - `currency_id`: `CurrencyId` airdrop currency type.
/// - `airdrop_list`: airdrop accounts and respective amounts in Vec<(T::AccountId, Balance)> format.
		#[pallet::weight((100_000_000 as Weight, DispatchClass::Operational))]
		#[transactional]
		pub fn make_airdrop(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			airdrop_list: Vec<(T::AccountId, Balance)>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				airdrop_list.len() <= T::MaxAirdropListSize::get(),
				Error::<T>::OverSizedAirdropList,
			);

			Self::do_make_airdrop(who, currency_id, airdrop_list)?;
			Ok(())
		}

/// Make Airdrop with JSON data.
///
/// Any account can call this function.
///
/// - `currency_id`: `CurrencyId` airdrop currency type.
/// - `airdrop_list_json`: airdrop accounts and respective amounts in json format as a byte vector.
        #[pallet::weight((100_000_000 as Weight, DispatchClass::Operational))]
        #[transactional]
        pub fn make_airdrop_with_json(
            origin: OriginFor<T>,
            currency_id: CurrencyId,
            airdrop_list_json: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let airdrop_entries = Self::parse_airdrop_json(airdrop_list_json)?;

            ensure!(
                airdrop_entries.len() <= T::MaxAirdropListSize::get(),
                Error::<T>::OverSizedAirdropList,
            );

            Self::do_make_airdrop(who, currency_id, airdrop_entries)?;
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
/// Get account of Airdrop module.
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account()
	}

	fn do_make_airdrop(who: T::AccountId, currency_id: CurrencyId, airdrop_list: Vec<(T::AccountId, Balance)>) -> DispatchResult {
        frame_support::storage::with_transaction(|| {
            let mut processed_accounts = sp_std::collections::btree_set::BTreeSet::new();
            for (beneficiary, amount) in airdrop_list.iter() {
                if !processed_accounts.insert(beneficiary) {
                    return TransactionOutcome::Rollback(Err(Error::<T>::DuplicateAccounts.into()));
                }
                let transfer_result = T::MultiCurrency::transfer(currency_id, &who, beneficiary, *amount);
                if transfer_result.is_err() {
                    return TransactionOutcome::Rollback(Err(transfer_result.err().unwrap()));
                }
            }
            TransactionOutcome::Commit(Ok(()))
        })
    }
	
	fn parse_airdrop_json(airdrop_list_json: Vec<u8>) -> Result<Vec<(T::AccountId, Balance)>, Error<T>> {
		let airdrop_list: AirdropList = serde_json::from_slice(&airdrop_list_json)
			.map_err(|_| Error::<T>::InvalidJson)?;
	
		airdrop_list.0.into_iter().map(|entry| {
			let account_id = T::AccountId::decode(&mut &entry.account.encode()[..])
				.map_err(|_| Error::<T>::InvalidAccountId)?;
			Ok((account_id, entry.amount))
		}).collect()
	}
}
