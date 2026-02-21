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

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use module_support::{AddressMapping, Erc20InfoMapping, EVMBridge, EvmAddress};
use sp_core::H160;
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;
use primitives::CurrencyId;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: frame_support::traits::Currency<Self::AccountId>;
        type EVMBridge: EVMBridge<Self::AccountId, BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type BalanceOf<T> =
        <<T as Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance;
}

pub struct EvmAddressMapping<T>(PhantomData<T>);

impl<T: Config> AddressMapping<T::AccountId> for EvmAddressMapping<T>
where
    T::AccountId: Default,
{
    fn get_account_id(_evm: &EvmAddress) -> T::AccountId {
        T::AccountId::default()
    }

    fn get_evm_address(_account_id: &T::AccountId) -> Option<EvmAddress> {
        None
    }

    fn get_or_create_evm_address(_account_id: &T::AccountId) -> EvmAddress {
        EvmAddress::default()
    }

    fn get_default_evm_address(_account_id: &T::AccountId) -> EvmAddress {
        EvmAddress::default()
    }

    fn is_linked(_account_id: &T::AccountId, _evm: &EvmAddress) -> bool {
        false
    }
}

pub struct EvmCurrencyIdMapping<T>(PhantomData<T>);

impl<T: Config> Erc20InfoMapping for EvmCurrencyIdMapping<T> {
    fn name(_currency_id: CurrencyId) -> Option<Vec<u8>> {
        None
    }
    fn symbol(_currency_id: CurrencyId) -> Option<Vec<u8>> {
        None
    }
    fn decimals(_currency_id: CurrencyId) -> Option<u8> {
        None
    }
    fn encode_evm_address(_v: CurrencyId) -> Option<EvmAddress> {
        None
    }
    fn decode_evm_address(_v: EvmAddress) -> Option<CurrencyId> {
        None
    }
}
