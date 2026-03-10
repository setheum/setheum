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

use primitives::{CommitteeSeats, EraValidators};
use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg32;
use sp_staking::EraIndex;
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

use primitives::ValidatorProvider;

use crate::{
    CommitteeSize, Config, CurrentEraValidators, NextEraCommitteeSize,
    NextEraNonReservedValidators, NextEraReservedValidators, Pallet,
};

impl<T> Pallet<T>
where
    T: Config,
{
    fn populate_next_era_validators_on_next_era_start(era: EraIndex) {
        let parent_hash = frame_system::Pallet::<T>::parent_hash();
        let mut bytes = [0u8; 8];
        bytes.clone_from_slice(&parent_hash.as_ref()[..8]);
        let seed = u64::from_le_bytes(bytes);

        let mut rng = Pcg32::seed_from_u64(seed);
        let elected_committee = BTreeSet::from_iter(T::ValidatorProvider::elected_validators(era));

        let mut retain_shuffle_elected = |vals: Vec<T::AccountId>| -> Vec<T::AccountId> {
            let mut vals: Vec<_> = vals
                .into_iter()
                .filter(|v| elected_committee.contains(v))
                .collect();
            vals.shuffle(&mut rng);

            vals
        };

        let reserved_validators = NextEraReservedValidators::<T>::get();
        let non_reserved_validators = NextEraNonReservedValidators::<T>::get();
        let committee_size = NextEraCommitteeSize::<T>::get();

        CurrentEraValidators::<T>::put(EraValidators {
            reserved: retain_shuffle_elected(reserved_validators),
            non_reserved: retain_shuffle_elected(non_reserved_validators),
        });
        CommitteeSize::<T>::put(committee_size);
    }
}

impl<T: Config> primitives::EraManager for Pallet<T> {
    fn on_new_era(era: EraIndex) {
        Self::populate_next_era_validators_on_next_era_start(era);
    }
}

impl<T: Config> primitives::BanHandler for Pallet<T> {
    type AccountId = T::AccountId;
    fn can_ban(account_id: &Self::AccountId) -> bool {
        !NextEraReservedValidators::<T>::get().contains(account_id)
    }
}

impl<T: Config + pallet_staking::Config> primitives::ValidatorProvider for Pallet<T> {
    type AccountId = T::AccountId;
    fn current_era_validators() -> EraValidators<Self::AccountId> {
        CurrentEraValidators::<T>::get()
    }
    fn current_era_committee_size() -> CommitteeSeats {
        CommitteeSize::<T>::get()
    }
    fn elected_validators(era: sp_staking::EraIndex) -> Vec<Self::AccountId> {
        pallet_staking::ErasStakersOverview::<T>::iter_key_prefix(era).collect()
    }
}
