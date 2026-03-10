// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

use sp_staking::EraIndex;
use sp_std::vec::Vec;

pub trait ValidatorProvider {
    type AccountId;
    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId>;
}

impl<T: pallet_staking::Config> ValidatorProvider for pallet_staking::Pallet<T> {
    type AccountId = T::AccountId;

    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId> {
        pallet_staking::ErasStakers::<T>::iter_key_prefix(era)
            .chain(pallet_staking::ErasStakersOverview::<T>::iter_key_prefix(
                era,
            ))
            .collect()
    }
}
