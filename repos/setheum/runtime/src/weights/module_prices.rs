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

use frame_support::{traits::Get, weights::Weight as _};
use sp_std::marker::PhantomData;
use super::Weight;

/// Weight functions for module_prices.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_prices::WeightInfo for WeightInfo<T> {
	fn lock_price() -> Weight {
		Weight::from_parts(87_924_000, 0)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn unlock_price() -> Weight {
		Weight::from_parts(24_114_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
