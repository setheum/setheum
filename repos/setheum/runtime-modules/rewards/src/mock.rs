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

//! Mocks for the rewards module.

#![cfg(test)]

use super::*;
use frame_support::{construct_runtime, derive_impl};
use orml_traits::parameter_type_with_key;
use sp_runtime::{traits::IdentityLookup, BuildStorage};
use sp_std::cell::RefCell;
use std::collections::HashMap;

use crate as rewards;

pub type AccountId = u128;
pub type Balance = u128;
pub type Share = u128;
pub type PoolId = u32;
pub type CurrencyId = u32;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;
pub const DOT_POOL: PoolId = 1;
pub const NATIVE_COIN: CurrencyId = 0;
pub const STABLE_COIN: CurrencyId = 1;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
}

thread_local! {
	pub static RECEIVED_PAYOUT: RefCell<HashMap<(PoolId, AccountId, CurrencyId), Balance>> = RefCell::new(HashMap::new());
}

pub struct Handler;
impl RewardHandler<AccountId, CurrencyId> for Handler {
	type Balance = Balance;
	type PoolId = PoolId;

	fn payout(who: &AccountId, pool: &Self::PoolId, currency_id: CurrencyId, amount: Self::Balance) {
		RECEIVED_PAYOUT.with(|v| {
			let mut old_map = v.borrow().clone();
			if let Some(before) = old_map.get_mut(&(*pool, *who, currency_id)) {
				*before += amount;
			} else {
				old_map.insert((*pool, *who, currency_id), amount);
			};

			*v.borrow_mut() = old_map;
		});
	}
}

parameter_type_with_key! {
	pub MinimalShares: |pool_id: PoolId| -> Share {
		match pool_id {
			&DOT_POOL => 10,
			_ => 0,
		}
	};
}

impl Config for Runtime {
	type Share = Share;
	type Balance = Balance;
	type PoolId = PoolId;
	type CurrencyId = CurrencyId;
	type Handler = Handler;
	type MinimalShares = MinimalShares;
}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		RewardsModule: rewards,
	}
);

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.unwrap();

		t.into()
	}
}
