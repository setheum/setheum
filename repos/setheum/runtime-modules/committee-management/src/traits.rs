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

use frame_support::pallet_prelude::Get;
use sp_staking::{EraIndex, SessionIndex};
use sp_std::vec::Vec;

pub trait EraInfoProvider {
    type AccountId;

/// Returns `Some(idx)` where idx is the active era index otherwise
/// if no era is active returns `None`.
    fn active_era() -> Option<EraIndex>;
/// Returns `Some(idx)` where idx is the current era index which is latest
/// planed era otherwise if no era has started returns `None`.
    fn current_era() -> Option<EraIndex>;
/// Returns the index of the starting session of the `era` if possible. Otherwise returns `None`.
    fn era_start_session_index(era: EraIndex) -> Option<SessionIndex>;
/// Returns how many sessions are in single era.
    fn sessions_per_era() -> SessionIndex;
/// Returns the elected authorities for provided era.
    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId>;
}

impl<T> EraInfoProvider for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
{
    type AccountId = T::AccountId;

    fn active_era() -> Option<EraIndex> {
        pallet_staking::ActiveEra::<T>::get().map(|ae| ae.index)
    }

    fn current_era() -> Option<EraIndex> {
        pallet_staking::CurrentEra::<T>::get()
    }

    fn era_start_session_index(era: EraIndex) -> Option<SessionIndex> {
        pallet_staking::ErasStartSessionIndex::<T>::get(era)
    }

    fn sessions_per_era() -> SessionIndex {
        T::SessionsPerEra::get()
    }

    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId> {
        pallet_staking::ErasStakers::<T>::iter_key_prefix(era).collect()
    }
}

pub trait ValidatorRewardsHandler {
    type AccountId;
/// Returns total exposure of validators for the `era`
    fn validator_totals(era: EraIndex) -> Vec<(Self::AccountId, u128)>;
/// Add reward for validators
    fn add_rewards(rewards: impl IntoIterator<Item = (Self::AccountId, u32)>);
}

impl<T> ValidatorRewardsHandler for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
    <T as pallet_staking::Config>::CurrencyBalance: Into<u128>,
{
    type AccountId = T::AccountId;
    fn validator_totals(era: EraIndex) -> Vec<(Self::AccountId, u128)> {
        pallet_staking::ErasStakers::<T>::iter_prefix(era)
            .map(|(validator, exposure)| (validator, exposure.total.into()))
            .collect()
    }

    fn add_rewards(rewards: impl IntoIterator<Item = (Self::AccountId, u32)>) {
        pallet_staking::Pallet::<T>::reward_by_ids(rewards);
    }
}

pub trait ValidatorExtractor {
    type AccountId;

/// Removes given validator from pallet's staking validators list
    fn remove_validator(who: &Self::AccountId);
}

impl<T> ValidatorExtractor for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
{
    type AccountId = T::AccountId;

    fn remove_validator(who: &Self::AccountId) {
        pallet_staking::Pallet::<T>::do_remove_validator(who);
    }
}
