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

/// Weight functions needed for module_payment.
pub trait WeightInfo {
	fn pay(x: u32, ) -> Weight;
	fn release() -> Weight;
	fn cancel() -> Weight;
	fn resolve_payment() -> Weight;
	fn request_refund() -> Weight;
	fn dispute_refund() -> Weight;
	fn request_payment() -> Weight;
	fn accept_and_pay() -> Weight;
	fn remove_task() -> Weight;
}

/// Weights for module_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
// Storage: Payment Payment (r:1 w:1)
// Storage: Sudo Key (r:1 w:0)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		Weight::from_parts(55_900_000, 0)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		Weight::from_parts(48_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		Weight::from_parts(21_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		Weight::from_parts(17_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		Weight::from_parts(58_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		Weight::from_parts(4_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
// Storage: Payment Payment (r:1 w:1)
// Storage: Sudo Key (r:1 w:0)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:1)
	fn pay(_x: u32, ) -> Weight {
		Weight::from_parts(55_900_000, 0)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
	fn release() -> Weight {
		Weight::from_parts(36_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:0)
	fn cancel() -> Weight {
		Weight::from_parts(48_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
	fn resolve_payment() -> Weight {
		Weight::from_parts(35_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn request_refund() -> Weight {
		Weight::from_parts(20_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn dispute_refund() -> Weight {
		Weight::from_parts(21_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Sudo Key (r:1 w:0)
	fn request_payment() -> Weight {
		Weight::from_parts(17_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
// Storage: Payment Payment (r:1 w:1)
// Storage: Assets Accounts (r:2 w:2)
// Storage: System Account (r:1 w:1)
	fn accept_and_pay() -> Weight {
		Weight::from_parts(58_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
// Storage: Payment ScheduledTasks (r:1 w:1)
	fn remove_task() -> Weight {
		Weight::from_parts(4_000_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
