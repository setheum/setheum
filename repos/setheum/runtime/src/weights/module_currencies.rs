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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_currencies.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_currencies::WeightInfo for WeightInfo<T> {
	fn transfer_non_native_currency() -> Weight {
		Weight::from_parts(118_970_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn transfer_native_currency() -> Weight {
		Weight::from_parts(118_406_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn update_balance_non_native_currency() -> Weight {
		Weight::from_parts(67_290_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn update_balance_native_currency_creating() -> Weight {
		Weight::from_parts(62_901_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn update_balance_native_currency_killing() -> Weight {
		Weight::from_parts(73_159_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn sweep_dust(c: u32, ) -> Weight {
		Weight::from_parts(7_400_000, 0)
// Standard Error: 111_000
			.saturating_add(Weight::from_parts(25_100_000, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads(2_u64.saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes(2_u64.saturating_mul(c.into())))
	}
	fn force_set_lock() -> Weight {
		Weight::from_parts(67_290_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn force_remove_lock() -> Weight {
		Weight::from_parts(67_290_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
