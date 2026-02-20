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

use frame_support::{
    pallet_prelude::{PalletInfoAccess, StorageVersion, Weight},
    traits::OnRuntimeUpgrade,
};

/// In order to run both pre- and post- checks around every migration, we entangle methods of
/// `OnRuntimeUpgrade` into the desired flow and expose it with `migrate` method.
///
/// This way, `try-runtime` no longer triggers checks. We do it by hand.
pub trait StorageMigration: OnRuntimeUpgrade {
    #[allow(clippy::let_and_return)]
    fn migrate() -> Weight {
        #[cfg(feature = "try-runtime")]
        let state = Self::pre_upgrade().expect("Pre upgrade should succeed");

        let weight = Self::on_runtime_upgrade();

        #[cfg(feature = "try-runtime")]
        Self::post_upgrade(state).expect("Post upgrade should succeed");

        weight
    }
}

impl<T: OnRuntimeUpgrade> StorageMigration for T {}

/// Ensure that the current pallet storage version matches `version`.
pub fn ensure_storage_version<P: PalletInfoAccess>(version: u16) -> Result<(), &'static str> {
    if StorageVersion::get::<P>() == StorageVersion::new(version) {
        Ok(())
    } else {
        Err("Bad storage version")
    }
}
