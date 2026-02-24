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

//! Mocks for the authority module.

#![cfg(test)]

use super::*;
use frame_support::{derive_impl, parameter_types, traits::EqualPrivilegeOnly, weights::Weight};
use frame_system::{ensure_root, ensure_signed, EnsureRoot};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{
	traits::{BadOrigin, IdentityLookup},
	BuildStorage, Perbill,
};

pub use crate as authority;

pub type AccountId = u128;
pub type BlockNumber = u64;

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::simple_max(Weight::from_parts(2_000_000_000_000, 0).set_proof_size(u64::MAX));
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Nonce = u64;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
}

impl pallet_preimage::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = ();
	type ManagerOrigin = EnsureRoot<u128>;
	type Consideration = ();
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}
impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<u128>;
	type MaxScheduledPerBlock = ConstU32<10>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = Preimage;
	type BlockNumberProvider = System;
}

impl pallet_root_testing::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Ord, PartialOrd, Debug, TypeInfo, DecodeWithMemTracking)]
pub enum MockAsOriginId {
	Root,
	Account1,
	Account2,
}

pub struct AuthorityConfigImpl;

impl AuthorityConfig<RuntimeOrigin, OriginCaller, BlockNumber> for AuthorityConfigImpl {
	fn check_schedule_dispatch(origin: RuntimeOrigin, _priority: Priority) -> DispatchResult {
		let origin: Result<frame_system::RawOrigin<u128>, _> = origin.into();
		match origin {
			Ok(frame_system::RawOrigin::Root)
			| Ok(frame_system::RawOrigin::Signed(1))
			| Ok(frame_system::RawOrigin::Signed(2)) => Ok(()),
			_ => Err(BadOrigin.into()),
		}
	}
	fn check_fast_track_schedule(
		origin: RuntimeOrigin,
		_initial_origin: &OriginCaller,
		_new_delay: BlockNumber,
	) -> DispatchResult {
		ensure_root(origin)?;
		Ok(())
	}
	fn check_delay_schedule(origin: RuntimeOrigin, initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if origin.caller() == initial_origin {
				Ok(())
			} else {
				Err(BadOrigin.into())
			}
		})
	}
	fn check_cancel_schedule(origin: RuntimeOrigin, initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if origin.caller() == initial_origin {
				Ok(())
			} else {
				Err(BadOrigin.into())
			}
		})
	}
}

impl AsOriginId<RuntimeOrigin, OriginCaller> for MockAsOriginId {
	fn into_origin(self) -> OriginCaller {
		match self {
			MockAsOriginId::Root => RuntimeOrigin::root().caller().clone(),
			MockAsOriginId::Account1 => RuntimeOrigin::signed(1).caller().clone(),
			MockAsOriginId::Account2 => RuntimeOrigin::signed(2).caller().clone(),
		}
	}
	fn check_dispatch_from(&self, origin: RuntimeOrigin) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if let OriginCaller::Authority(ref sign) = origin.caller() {
				if *sign.origin == RuntimeOrigin::root().caller().clone() {
					return Ok(());
				} else {
					return Err(BadOrigin.into());
				}
			}

			let ok = match self {
				MockAsOriginId::Root => false,
				MockAsOriginId::Account1 => ensure_signed(origin)? == 1,
				MockAsOriginId::Account2 => ensure_signed(origin)? == 2,
			};
			if ok {
				Ok(())
			} else {
				Err(BadOrigin.into())
			}
		})
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub struct MockBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl BenchmarkHelper<MockAsOriginId> for MockBenchmarkHelper {
	fn get_as_origin_id() -> Option<MockAsOriginId> {
		Some(MockAsOriginId::Root)
	}
}

impl Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type Scheduler = Scheduler;
	type RuntimeCall = RuntimeCall;
	type AsOriginId = MockAsOriginId;
	type AuthorityConfig = AuthorityConfigImpl;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = MockBenchmarkHelper;
}

type Block = frame_system::mocking::MockBlock<Runtime>;

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Authority: authority,
		Scheduler: pallet_scheduler,
		Preimage: pallet_preimage,
		RootTesting: pallet_root_testing,
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

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Scheduler::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		Scheduler::on_initialize(System::block_number());
	}
}
