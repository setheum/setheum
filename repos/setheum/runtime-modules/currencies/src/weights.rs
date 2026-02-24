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

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_currencies.
pub trait WeightInfo {
	fn transfer_non_native_currency() -> Weight;
	fn transfer_native_currency() -> Weight;
	fn update_balance_non_native_currency() -> Weight;
	fn update_balance_native_currency_creating() -> Weight;
	fn update_balance_native_currency_killing() -> Weight;
	fn sweep_dust(c: u32, ) -> Weight;
	fn force_set_lock() -> Weight;
	fn force_remove_lock() -> Weight;
}

/// Weights for module_currencies using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
// Storage: Tokens Accounts (r:2 w:2)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_non_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2493`
//  Estimated: `13352`
// Minimum execution time: 86_216 nanoseconds.
		Weight::from_parts(88_106_000, 13352)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1977`
//  Estimated: `7118`
// Minimum execution time: 68_140 nanoseconds.
		Weight::from_parts(69_315_000, 7118)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: Tokens TotalIssuance (r:1 w:1)
// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_non_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2008`
//  Estimated: `10737`
// Minimum execution time: 54_990 nanoseconds.
		Weight::from_parts(55_756_000, 10737)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_native_currency_creating() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1707`
//  Estimated: `3593`
// Minimum execution time: 50_095 nanoseconds.
		Weight::from_parts(51_020_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn update_balance_native_currency_killing() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1846`
//  Estimated: `7118`
// Minimum execution time: 49_296 nanoseconds.
		Weight::from_parts(50_228_000, 7118)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: Tokens Accounts (r:4 w:4)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:3 w:3)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
/// The range of component `c` is `[1, 3]`.
	fn sweep_dust(c: u32, ) -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1880 + c * (339 ±0)`
//  Estimated: `4602 + c * (5225 ±0)`
// Minimum execution time: 63_930 nanoseconds.
		Weight::from_parts(28_195_038, 4602)
// Standard Error: 55_030
			.saturating_add(Weight::from_parts(37_716_994, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5225).saturating_mul(c.into()))
	}
// Storage: Tokens Locks (r:1 w:1)
// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_set_lock() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2209`
//  Estimated: `11970`
// Minimum execution time: 56_749 nanoseconds.
		Weight::from_parts(57_522_000, 11970)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
// Storage: Tokens Locks (r:1 w:1)
// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_remove_lock() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2314`
//  Estimated: `11970`
// Minimum execution time: 57_795 nanoseconds.
		Weight::from_parts(58_743_000, 11970)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
// Storage: Tokens Accounts (r:2 w:2)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_non_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2493`
//  Estimated: `13352`
// Minimum execution time: 86_216 nanoseconds.
		Weight::from_parts(88_106_000, 13352)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1977`
//  Estimated: `7118`
// Minimum execution time: 68_140 nanoseconds.
		Weight::from_parts(69_315_000, 7118)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: Tokens TotalIssuance (r:1 w:1)
// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(67), added: 2542, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_non_native_currency() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2008`
//  Estimated: `10737`
// Minimum execution time: 54_990 nanoseconds.
		Weight::from_parts(55_756_000, 10737)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn update_balance_native_currency_creating() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1707`
//  Estimated: `3593`
// Minimum execution time: 50_095 nanoseconds.
		Weight::from_parts(51_020_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn update_balance_native_currency_killing() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1846`
//  Estimated: `7118`
// Minimum execution time: 49_296 nanoseconds.
		Weight::from_parts(50_228_000, 7118)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: Tokens Accounts (r:4 w:4)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:3 w:3)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
/// The range of component `c` is `[1, 3]`.
	fn sweep_dust(c: u32, ) -> Weight {
// Proof Size summary in bytes:
//  Measured:  `1880 + c * (339 ±0)`
//  Estimated: `4602 + c * (5225 ±0)`
// Minimum execution time: 63_930 nanoseconds.
		Weight::from_parts(28_195_038, 4602)
// Standard Error: 55_030
			.saturating_add(Weight::from_parts(37_716_994, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5225).saturating_mul(c.into()))
	}
// Storage: Tokens Locks (r:1 w:1)
// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_set_lock() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2209`
//  Estimated: `11970`
// Minimum execution time: 56_749 nanoseconds.
		Weight::from_parts(57_522_000, 11970)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
// Storage: Tokens Locks (r:1 w:1)
// Proof: Tokens Locks (max_values: None, max_size: Some(1300), added: 3775, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:1)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_remove_lock() -> Weight {
// Proof Size summary in bytes:
//  Measured:  `2314`
//  Estimated: `11970`
// Minimum execution time: 57_795 nanoseconds.
		Weight::from_parts(58_743_000, 11970)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}
