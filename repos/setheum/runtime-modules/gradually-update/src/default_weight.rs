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

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn gradually_update() -> Weight {
		Weight::from_parts(57_922_000, 0)
			.saturating_add(DbWeight::get().reads(2 as u64))
			.saturating_add(DbWeight::get().writes(1 as u64))
	}
	fn cancel_gradually_update() -> Weight {
		Weight::from_parts(66_687_000, 0)
			.saturating_add(DbWeight::get().reads(1 as u64))
			.saturating_add(DbWeight::get().writes(1 as u64))
	}
	fn on_finalize(u: u32) -> Weight {
		Weight::from_parts(37_067_000, 0)
			.saturating_add(Weight::from_parts(20_890_000, 0).saturating_mul(u as u64))
			.saturating_add(DbWeight::get().reads(3 as u64))
			.saturating_add(DbWeight::get().writes(3 as u64))
	}
}
