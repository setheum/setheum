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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_edfis_launchpad.
pub trait WeightInfo {
	fn on_initialize(n: u32, ) -> Weight;
	fn make_proposal() -> Weight;
	fn contribute() -> Weight;
	fn claim_contribution_allocation() -> Weight;
	fn claim_campaign_fundraise() -> Weight;
	fn approve_proposal() -> Weight;
	fn reject_proposal() -> Weight;
	fn activate_waiting_campaign() -> Weight;
}

/// Weights for module_edfis_launchpad using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
	fn on_initialize(n: u32, ) -> Weight {
		Weight::from_parts(16_173_000, 0)
// Standard Error: 0
			.saturating_add((2_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	fn make_proposal() -> Weight {
		Weight::from_parts(180_406_000, 0)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	fn contribute() -> Weight {
		Weight::from_parts(127_732_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn claim_contribution_allocation() -> Weight {
		Weight::from_parts(121_893_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn claim_campaign_fundraise() -> Weight {
		Weight::from_parts(38_662_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn approve_proposal() -> Weight {
		Weight::from_parts(47_547_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn reject_proposal() -> Weight {
		Weight::from_parts(38_272_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn activate_waiting_campaign() -> Weight {
		Weight::from_parts(35_868_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize(n: u32, ) -> Weight {
		Weight::from_parts(16_173_000, 0)
// Standard Error: 0
			.saturating_add((2_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn make_proposal() -> Weight {
		Weight::from_parts(180_406_000, 0)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn contribute() -> Weight {
		Weight::from_parts(127_732_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn claim_contribution_allocation() -> Weight {
		Weight::from_parts(121_893_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn claim_campaign_fundraise() -> Weight {
		Weight::from_parts(38_662_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	fn approve_proposal() -> Weight {
		Weight::from_parts(47_547_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn reject_proposal() -> Weight {
		Weight::from_parts(38_272_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn activate_waiting_campaign() -> Weight {
		Weight::from_parts(35_868_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
