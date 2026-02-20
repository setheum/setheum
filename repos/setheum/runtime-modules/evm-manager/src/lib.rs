// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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
