// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![cfg(test)]

use crate as module_idle_scheduler;
use setheum_primitives::{define_combined_task, task::TaskResult};
use frame_support::weights::Weight;
use frame_support::{construct_runtime, parameter_types, traits::Everything};
use module_support::DispatchableTask;

use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub const BASE_WEIGHT: Weight = 1_000_000;

parameter_types!(
	pub const BlockHashCount: u32 = 250;
);

pub type AccountId = u32;
impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = sp_runtime::testing::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::testing::Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types!(
	pub const MinimumWeightRemainInBlock: Weight = 100_000_000_000;
);

impl module_idle_scheduler::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
}

// Mock dispatachable tasks
#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub enum BalancesTask {
	#[codec(index = 0)]
	OnIdle,
}
impl DispatchableTask for BalancesTask {
	fn dispatch(self, weight: Weight) -> TaskResult {
		TaskResult {
			result: Ok(()),
			used_weight: BASE_WEIGHT,
			finished: weight >= BASE_WEIGHT,
		}
	}
}

define_combined_task! {
	#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
	pub enum ScheduledTasks {
		BalancesTask(BalancesTask),
	}
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Event<T>},
		IdleScheduler: module_idle_scheduler::{Pallet, Call, Event<T>, Storage},
	}
);

#[derive(Default)]
pub struct ExtBuilder;
impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
