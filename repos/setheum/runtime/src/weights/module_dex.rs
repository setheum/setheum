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
use module_swap_legacy as swap_legacy_module;
use super::Weight;

/// Weight functions for swap_legacy_module.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> swap_legacy_module::WeightInfo for WeightInfo<T> {
	fn enable_trading_pair() -> Weight {
		Weight::from_parts(25_878_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn disable_trading_pair() -> Weight {
		Weight::from_parts(25_740_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn list_provisioning() -> Weight {
		Weight::from_parts(39_243_000, 0)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn update_provisioning_parameters() -> Weight {
		Weight::from_parts(12_764_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn end_provisioning() -> Weight {
		Weight::from_parts(80_534_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	fn add_provision() -> Weight {
		Weight::from_parts(132_773_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn claim_dex_share() -> Weight {
		Weight::from_parts(109_807_000, 0)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn add_liquidity() -> Weight {
		Weight::from_parts(191_996_000, 0)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	fn add_liquidity_and_stake() -> Weight {
		Weight::from_parts(268_295_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	fn remove_liquidity() -> Weight {
		Weight::from_parts(164_184_000, 0)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	fn remove_liquidity_by_unstake() -> Weight {
		Weight::from_parts(288_275_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		Weight::from_parts(97_602_000, 0)
// Standard Error: 130_000
			.saturating_add(Weight::from_parts(16_421_000, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads(2 * u as u64))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes(1 * u as u64))
	}
	fn swap_with_exact_target(u: u32, ) -> Weight {
		Weight::from_parts(99_807_000, 0)
// Standard Error: 837_000
			.saturating_add(Weight::from_parts(16_033_000, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads(2 * u as u64))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes(1 * u as u64))
	}
	fn refund_provision() -> Weight {
		Weight::from_parts(80_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	fn abort_provisioning() -> Weight {
		Weight::from_parts(80_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
