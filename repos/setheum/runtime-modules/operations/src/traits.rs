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

use frame_support::traits::StoredMap;
use sp_runtime::traits::Zero;
use sp_staking::StakingAccount;

pub trait AccountInfoProvider {
    /// Account id type used by runtime
    type AccountId;
    /// Reference counter type used by runtime
    type RefCount;

    /// Retrieves account's consumer counter
    fn get_consumers(who: &Self::AccountId) -> Self::RefCount;
}

impl<T> AccountInfoProvider for frame_system::Pallet<T>
where
    T: frame_system::Config,
{
    type AccountId = T::AccountId;
    type RefCount = frame_system::RefCount;

    fn get_consumers(who: &Self::AccountId) -> Self::RefCount {
        frame_system::Pallet::<T>::consumers(who)
    }
}

pub trait BalancesProvider {
    /// Account id type used by runtime
    type AccountId;
    /// Balance type used by runtime
    type Balance;

    /// Returns reserved funds of an account
    fn is_reserved_zero(who: &Self::AccountId) -> bool;

    /// Returns frozen funds of an account
    fn is_frozen_zero(who: &Self::AccountId) -> bool;
}

impl<T: pallet_balances::Config<I>, I: 'static> BalancesProvider for pallet_balances::Pallet<T, I> {
    type AccountId = T::AccountId;
    type Balance = T::Balance;

    fn is_reserved_zero(who: &Self::AccountId) -> bool {
        T::AccountStore::get(who).reserved.is_zero()
    }

    fn is_frozen_zero(who: &Self::AccountId) -> bool {
        T::AccountStore::get(who).frozen.is_zero()
    }
}

pub trait NextKeysSessionProvider {
    /// Account id type used by runtime
    type AccountId;

    /// Retrieves information whether given account is in the next session keys
    fn has_next_session_keys(who: &Self::AccountId) -> bool;
}

impl<T> NextKeysSessionProvider for pallet_session::Pallet<T>
where
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
{
    type AccountId = T::AccountId;

    fn has_next_session_keys(who: &Self::AccountId) -> bool {
        pallet_session::NextKeys::<T>::get(who).is_some()
    }
}

pub trait BondedStashProvider {
    /// Account id type used by runtime
    type AccountId;

    /// Retrieves information about controller of given stash account, or None if account
    /// have not bonded yet
    fn get_controller(stash: &Self::AccountId) -> Option<Self::AccountId>;

    /// Retrieves information about stash of given controller account, or None if account
    /// have not bonded yet
    fn get_stash(stash: &Self::AccountId) -> Option<Self::AccountId>;
}

pub trait ContractInfoProvider {
    /// Account id type used by runtime
    type AccountId;

    /// Returns true if `who` is a contract account
    fn is_contract_account(who: &Self::AccountId) -> bool;
}

impl<T> BondedStashProvider for pallet_staking::Pallet<T>
where
    T: frame_system::Config + pallet_staking::Config,
{
    type AccountId = T::AccountId;

    fn get_controller(stash: &Self::AccountId) -> Option<Self::AccountId> {
        pallet_staking::Pallet::<T>::bonded(stash)
    }

    fn get_stash(controller: &Self::AccountId) -> Option<Self::AccountId> {
        pallet_staking::Pallet::<T>::ledger(StakingAccount::Controller(controller.clone()))
            .ok()
            .map(|ledger| ledger.stash)
    }
}

impl<T: pallet_contracts::Config> ContractInfoProvider for pallet_contracts::Pallet<T> {
    type AccountId = T::AccountId;

    fn is_contract_account(who: &Self::AccountId) -> bool {
        pallet_contracts::Pallet::<T>::code_hash(who).is_some()
    }
}
