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

/// Weight functions needed for module_evm.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create2() -> Weight;
	fn create_nft_contract() -> Weight;
	fn create_predeploy_contract() -> Weight;
	fn call() -> Weight;
	fn transfer_maintainer() -> Weight;
	fn publish_contract() -> Weight;
	fn publish_free() -> Weight;
	fn enable_contract_development() -> Weight;
	fn disable_contract_development() -> Weight;
	fn set_code(c: u32, ) -> Weight;
	fn selfdestruct() -> Weight;
}

/// Weights for module_evm using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create() -> Weight {
// Minimum execution time: 202_922 nanoseconds.
		Weight::from_parts(204_527_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create2() -> Weight {
// Minimum execution time: 194_188 nanoseconds.
		Weight::from_parts(199_650_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM NetworkContractIndex (r:1 w:1)
// Proof Skipped: EVM NetworkContractIndex (max_values: Some(1), max_size: None, mode: Measured)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create_nft_contract() -> Weight {
// Minimum execution time: 223_480 nanoseconds.
		Weight::from_parts(227_640_000, 0)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(10))
	}
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create_predeploy_contract() -> Weight {
// Minimum execution time: 229_528 nanoseconds.
		Weight::from_parts(233_183_000, 0)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Codes (r:1 w:0)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
	fn call() -> Weight {
// Minimum execution time: 185_756 nanoseconds.
		Weight::from_parts(189_885_000, 0)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_maintainer() -> Weight {
// Minimum execution time: 120_422 nanoseconds.
		Weight::from_parts(122_117_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
	fn publish_contract() -> Weight {
// Minimum execution time: 149_010 nanoseconds.
		Weight::from_parts(150_918_000, 0)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
	fn publish_free() -> Weight {
// Minimum execution time: 39_214 nanoseconds.
		Weight::from_parts(40_271_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn enable_contract_development() -> Weight {
// Minimum execution time: 126_304 nanoseconds.
		Weight::from_parts(127_492_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn disable_contract_development() -> Weight {
// Minimum execution time: 128_756 nanoseconds.
		Weight::from_parts(129_795_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:2 w:2)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:2)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
// Minimum execution time: 221_718 nanoseconds.
		Weight::from_parts(218_913_195, 0)
// Standard Error: 17
			.saturating_add(Weight::from_parts(5_766, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:2 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:1 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:0)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn selfdestruct() -> Weight {
// Minimum execution time: 239_686 nanoseconds.
		Weight::from_parts(246_450_000, 0)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create() -> Weight {
// Minimum execution time: 202_922 nanoseconds.
		Weight::from_parts(204_527_000, 0)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create2() -> Weight {
// Minimum execution time: 194_188 nanoseconds.
		Weight::from_parts(199_650_000, 0)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM NetworkContractIndex (r:1 w:1)
// Proof Skipped: EVM NetworkContractIndex (max_values: Some(1), max_size: None, mode: Measured)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create_nft_contract() -> Weight {
// Minimum execution time: 223_480 nanoseconds.
		Weight::from_parts(227_640_000, 0)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(10))
	}
// Storage: EVM Accounts (r:2 w:2)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:2 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn create_predeploy_contract() -> Weight {
// Minimum execution time: 229_528 nanoseconds.
		Weight::from_parts(233_183_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:2 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: System Account (r:2 w:2)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: EVM Codes (r:1 w:0)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
	fn call() -> Weight {
// Minimum execution time: 185_756 nanoseconds.
		Weight::from_parts(189_885_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
	fn transfer_maintainer() -> Weight {
// Minimum execution time: 120_422 nanoseconds.
		Weight::from_parts(122_117_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
	fn publish_contract() -> Weight {
// Minimum execution time: 149_010 nanoseconds.
		Weight::from_parts(150_918_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
	fn publish_free() -> Weight {
// Minimum execution time: 39_214 nanoseconds.
		Weight::from_parts(40_271_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn enable_contract_development() -> Weight {
// Minimum execution time: 126_304 nanoseconds.
		Weight::from_parts(127_492_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
	fn disable_contract_development() -> Weight {
// Minimum execution time: 128_756 nanoseconds.
		Weight::from_parts(129_795_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts EvmAddresses (r:1 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:2 w:2)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: Balances Reserves (r:2 w:2)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: EVM Codes (r:0 w:2)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
// Minimum execution time: 221_718 nanoseconds.
		Weight::from_parts(218_913_195, 0)
// Standard Error: 17
			.saturating_add(Weight::from_parts(5_766, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(10))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
// Storage: EvmAccounts EvmAddresses (r:2 w:0)
// Proof: EvmAccounts EvmAddresses (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM Accounts (r:1 w:1)
// Proof Skipped: EVM Accounts (max_values: None, max_size: None, mode: Measured)
// Storage: EvmAccounts Accounts (r:2 w:0)
// Proof: EvmAccounts Accounts (max_values: None, max_size: Some(60), added: 2535, mode: MaxEncodedLen)
// Storage: EVM CodeInfos (r:1 w:1)
// Proof Skipped: EVM CodeInfos (max_values: None, max_size: None, mode: Measured)
// Storage: EVM ContractStorageSizes (r:1 w:1)
// Proof Skipped: EVM ContractStorageSizes (max_values: None, max_size: None, mode: Measured)
// Storage: Balances Reserves (r:1 w:1)
// Proof: Balances Reserves (max_values: None, max_size: Some(168), added: 2643, mode: MaxEncodedLen)
// Storage: System Account (r:1 w:1)
// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
// Storage: Tokens Accounts (r:1 w:0)
// Proof: Tokens Accounts (max_values: None, max_size: Some(147), added: 2622, mode: MaxEncodedLen)
// Storage: EVM Codes (r:0 w:1)
// Proof Skipped: EVM Codes (max_values: None, max_size: None, mode: Measured)
	fn selfdestruct() -> Weight {
// Minimum execution time: 239_686 nanoseconds.
		Weight::from_parts(246_450_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
}
