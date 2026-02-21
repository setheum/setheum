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

#![allow(clippy::nonminimal_bool)]

use frame_support::{
    dispatch::DispatchResultWithPostInfo, pallet_prelude::Get, traits::LockIdentifier,
    WeakBoundedVec,
};
use pallet_balances::BalanceLock;
use parity_scale_codec::Encode;
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::DispatchError;

use crate::{
    pallet::{Config, Event, Pallet},
    traits::{AccountInfoProvider, BalancesProvider, NextKeysSessionProvider},
    LOG_TARGET, STAKING_ID, VESTING_ID,
};

impl<T: Config> Pallet<T> {
/// Checks if account has an underflow of `consumers` counter. In such case, it increments
/// it by one.
    pub fn fix_underflow_consumer_counter(who: T::AccountId) -> DispatchResultWithPostInfo {
        let mut weight = T::DbWeight::get().reads(1);
        let consumers = T::AccountInfoProvider::get_consumers(&who);

        weight += T::DbWeight::get().reads(1);
        if Self::no_consumers_some_reserved(&who, consumers) {
            Self::increment_consumers(who)?;
            weight += T::DbWeight::get().writes(1);
            return Ok(Some(weight).into());
        }

        weight += T::DbWeight::get().reads(2);
        if Self::staker_has_consumers_underflow(&who, consumers) {
            Self::increment_consumers(who)?;
            weight += T::DbWeight::get().writes(1);
            return Ok(Some(weight).into());
        }

        log::debug!(
            target: LOG_TARGET,
            "Account {:?} has correct consumer counter, not incrementing",
            HexDisplay::from(&who.encode())
        );
        Ok(Some(weight).into())
    }

    fn staker_has_consumers_underflow(who: &T::AccountId, consumers: u32) -> bool {
        let locks = T::BalancesProvider::locks(who);
        let has_vesting_lock = Self::has_lock(&locks, VESTING_ID);
        let vester_has_consumers_underflow = consumers == 1 && has_vesting_lock;
        let has_staking_lock = Self::has_lock(&locks, STAKING_ID);
        let nominator_has_consumers_underflow = consumers == 2 && has_staking_lock;
        let has_next_session_keys = T::NextKeysSessionProvider::has_next_session_keys(who);
        let validator_has_consumers_underflow =
            consumers == 3 && has_staking_lock && has_next_session_keys;
        vester_has_consumers_underflow
            || nominator_has_consumers_underflow
            || validator_has_consumers_underflow
    }

    fn no_consumers_some_reserved(who: &T::AccountId, consumers: u32) -> bool {
        let is_reserved_not_zero = T::BalancesProvider::is_reserved_not_zero(who);

        consumers == 0 && is_reserved_not_zero
    }

    fn has_lock<U, V>(locks: &WeakBoundedVec<BalanceLock<U>, V>, id: LockIdentifier) -> bool {
        locks.iter().any(|x| x.id == id)
    }

    fn increment_consumers(who: T::AccountId) -> Result<(), DispatchError> {
        frame_system::Pallet::<T>::inc_consumers_without_limit(&who)?;
        Self::deposit_event(Event::ConsumersUnderflowFixed { who });
        Ok(())
    }
}
