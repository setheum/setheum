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

/// Weight functions needed for module_nft.
pub trait WeightInfo {
	fn create_class() -> Weight;
	fn mint(i: u32, ) -> Weight;
	fn transfer() -> Weight;
	fn burn() -> Weight;
	fn burn_with_remark(b: u32, ) -> Weight;
	fn destroy_class() -> Weight;
	fn update_class_properties() -> Weight;
}

/// Weights for module_nft using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
	fn create_class() -> Weight {
		Weight::from_parts(177_661_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn mint(i: u32, ) -> Weight {
		Weight::from_parts(44_387_000, 0)
// Standard Error: 46_000
			.saturating_add(Weight::from_parts(72_699_000, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(i as u64)))
	}
	fn transfer() -> Weight {
		Weight::from_parts(266_936_000, 0)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	fn burn() -> Weight {
		Weight::from_parts(189_094_000, 0)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn burn_with_remark(b: u32, ) -> Weight {
		Weight::from_parts(196_036_000, 0)
// Standard Error: 0
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn destroy_class() -> Weight {
		Weight::from_parts(217_091_000, 0)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn update_class_properties() -> Weight {
		Weight::from_parts(52_914_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_class() -> Weight {
		Weight::from_parts(177_661_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn mint(i: u32, ) -> Weight {
		Weight::from_parts(44_387_000, 0)
// Standard Error: 46_000
			.saturating_add(Weight::from_parts(72_699_000, 0).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(i as u64)))
	}
	fn transfer() -> Weight {
		Weight::from_parts(266_936_000, 0)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn burn() -> Weight {
		Weight::from_parts(189_094_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn burn_with_remark(b: u32, ) -> Weight {
		Weight::from_parts(196_036_000, 0)
// Standard Error: 0
			.saturating_add(Weight::from_parts(2_000, 0).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn destroy_class() -> Weight {
		Weight::from_parts(217_091_000, 0)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn update_class_properties() -> Weight {
		Weight::from_parts(52_914_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
