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

/// Weight functions for module_nft.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_nft::WeightInfo for WeightInfo<T> {
	fn create_class() -> Weight {
		Weight::from_parts(168_026_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn mint(i: u32, ) -> Weight {
		Weight::from_parts(123_482_000, 0)
// Standard Error: 19_000
			.saturating_add(Weight::from_parts(16_056_000, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes(2 * i as u64))
	}
	fn transfer() -> Weight {
		Weight::from_parts(189_939_000, 0)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	fn burn() -> Weight {
		Weight::from_parts(87_989_000, 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn burn_with_remark(b: u32, ) -> Weight {
		Weight::from_parts(105_235_000, 0)
// Standard Error: 0
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn destroy_class() -> Weight {
		Weight::from_parts(175_231_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	fn update_class_properties() -> Weight {
		Weight::from_parts(21_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
