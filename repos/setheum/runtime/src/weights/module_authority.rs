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

/// Weight functions for module_authority.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_authority::WeightInfo for WeightInfo<T> {
	fn dispatch_as() -> Weight {
		Weight::from_parts(20_989_000, 0)
	}
	fn schedule_dispatch_without_delay() -> Weight {
		Weight::from_parts(55_153_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn schedule_dispatch_with_delay() -> Weight {
		Weight::from_parts(57_754_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn fast_track_scheduled_dispatch() -> Weight {
		Weight::from_parts(74_949_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn delay_scheduled_dispatch() -> Weight {
		Weight::from_parts(74_280_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn cancel_scheduled_dispatch() -> Weight {
		Weight::from_parts(51_250_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	fn authorize_call() -> Weight {
		Weight::from_parts(14_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn remove_authorized_call() -> Weight {
		Weight::from_parts(16_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn trigger_call() -> Weight {
		Weight::from_parts(29_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
