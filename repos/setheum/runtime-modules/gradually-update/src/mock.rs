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

//! Mocks for the gradually-update module.

#![cfg(test)]

use super::*;
use frame_support::{
	construct_runtime, derive_impl,
	traits::{ConstU32, ConstU64},
};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

use crate as gradually_update;

pub type AccountId = u128;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Nonce = u64;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
}

impl Config for Runtime {
	type UpdateFrequency = ConstU64<10>;
	type DispatchOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type MaxGraduallyUpdate = ConstU32<3>;
	type MaxStorageKeyBytes = ConstU32<100_000>;
	type MaxStorageValueBytes = ConstU32<100_000>;
}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		GraduallyUpdateModule: gradually_update,
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
