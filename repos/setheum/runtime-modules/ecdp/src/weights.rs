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

/// Weight functions needed for module_ecdp.
pub trait WeightInfo {
	fn authorize() -> Weight;
	fn unauthorize() -> Weight;
	fn unauthorize_all(c: u32, ) -> Weight;
	fn adjust_loan() -> Weight;
	fn transfer_loan_from() -> Weight;
	fn close_loan_has_debit_by_dex() -> Weight;
	fn expand_position_collateral() -> Weight;
	fn shrink_position_debit() -> Weight;
	fn transfer_debit() -> Weight;
	fn precompile_get_current_collateral_ratio() -> Weight;
}

/// Weights for module_ecdp using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: ECDP Authorization (r:1 w:1)
// Storage: Balances Reserves (r:1 w:1)
	fn authorize() -> Weight {
		Weight::from_parts(45_674_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: ECDP Authorization (r:1 w:1)
// Storage: Balances Reserves (r:1 w:1)
	fn unauthorize() -> Weight {
		Weight::from_parts(91_834_000, 0)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: Balances Reserves (r:1 w:0)
// Storage: ECDP Authorization (r:0 w:1)
	fn unauthorize_all(c: u32, ) -> Weight {
		Weight::from_parts(51_744_000, 0)
// Standard Error: 866_000
			.saturating_add(Weight::from_parts(652_000, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: EcdpEmergencyShutdown IsShutdown (r:1 w:0)
// Storage: CdpEngine CollateralParams (r:1 w:0)
// Storage: Loans EcdpPositions (r:1 w:1)
// Storage: Rewards PoolInfos (r:1 w:1)
// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
// Storage: Loans TotalEcdpPositions (r:1 w:1)
// Storage: System Account (r:1 w:1)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
// Storage: Tokens Accounts (r:1 w:1)
// Storage: Tokens TotalIssuance (r:1 w:1)
// Storage: Prices LockedPrice (r:2 w:0)
// Storage: SetheumOracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn adjust_loan() -> Weight {
		Weight::from_parts(142_855_000, 0)
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: EcdpEmergencyShutdown IsShutdown (r:1 w:0)
// Storage: ECDP Authorization (r:1 w:0)
// Storage: Loans EcdpPositions (r:2 w:2)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
// Storage: Prices LockedPrice (r:2 w:0)
// Storage: Oracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
// Storage: CdpEngine CollateralParams (r:1 w:0)
// Storage: Rewards SharesAndWithdrawnRewards (r:2 w:2)
// Storage: Rewards PoolInfos (r:1 w:1)
// Storage: System Account (r:1 w:1)
// Storage: Loans TotalEcdpPositions (r:1 w:1)
	fn transfer_loan_from() -> Weight {
		Weight::from_parts(120_478_000, 0)
			.saturating_add(T::DbWeight::get().reads(17 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: EcdpEmergencyShutdown IsShutdown (r:1 w:0)
// Storage: Loans EcdpPositions (r:1 w:1)
// Storage: Prices LockedPrice (r:2 w:0)
// Storage: SetheumOracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
// Storage: Homa StakingLedgers (r:1 w:0)
// Storage: Homa ToBondPool (r:1 w:0)
// Storage: Tokens TotalIssuance (r:1 w:0)
// Storage: Homa TotalVoidLiquid (r:1 w:0)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
// Storage: CdpEngine CollateralParams (r:1 w:0)
// Storage: Tokens Accounts (r:6 w:6)
// Storage: System Account (r:3 w:2)
// Storage: CdpTreasury DebitPool (r:1 w:1)
// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
// Storage: Rewards PoolInfos (r:1 w:1)
// Storage: Loans TotalEcdpPositions (r:1 w:1)
// Storage: EcdpAuctionsManager TotalCollateralInAuction (r:1 w:0)
// Storage: Dex TradingPairStatuses (r:3 w:0)
// Storage: Dex LiquidityPool (r:2 w:2)
// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	fn close_loan_has_debit_by_dex() -> Weight {
		Weight::from_parts(349_743_000, 0)
			.saturating_add(T::DbWeight::get().reads(35 as u64))
			.saturating_add(T::DbWeight::get().writes(16 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: CdpEngine CollateralParams (r:1 w:0)
// Storage: Tokens Accounts (r:4 w:4)
// Storage: Tokens TotalIssuance (r:1 w:1)
// Storage: System Account (r:2 w:1)
// Storage: Dex TradingPairStatuses (r:1 w:0)
// Storage: Dex LiquidityPool (r:1 w:1)
// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
// Storage: Loans EcdpPositions (r:1 w:1)
// Storage: Rewards PoolInfos (r:1 w:1)
// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
// Storage: Loans TotalEcdpPositions (r:1 w:1)
// Storage: Prices LockedPrice (r:2 w:0)
// Storage: SetheumOracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn expand_position_collateral() -> Weight {
		Weight::from_parts(227_393_000, 0)
			.saturating_add(T::DbWeight::get().reads(23 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: CdpEngine CollateralParams (r:1 w:0)
// Storage: Loans EcdpPositions (r:1 w:1)
// Storage: Dex TradingPairStatuses (r:1 w:0)
// Storage: Dex LiquidityPool (r:1 w:1)
// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
// Storage: Tokens Accounts (r:5 w:5)
// Storage: System Account (r:2 w:1)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
// Storage: Rewards PoolInfos (r:1 w:1)
// Storage: Loans TotalEcdpPositions (r:1 w:1)
// Storage: Tokens TotalIssuance (r:1 w:1)
	fn shrink_position_debit() -> Weight {
		Weight::from_parts(230_779_000, 0)
			.saturating_add(T::DbWeight::get().reads(19 as u64))
			.saturating_add(T::DbWeight::get().writes(13 as u64))
	}
// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
// Storage: Tokens Accounts (r:1 w:1)
// Storage: Tokens TotalIssuance (r:2 w:1)
// Storage: CdpEngine CollateralParams (r:2 w:0)
// Storage: Loans EcdpPositions (r:2 w:2)
// Storage: Loans TotalEcdpPositions (r:2 w:2)
// Storage: CdpEngine DebitExchangeRate (r:2 w:0)
// Storage: Prices LockedPrice (r:3 w:0)
// Storage: SetheumOracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
// Storage: Homa StakingLedgers (r:1 w:0)
// Storage: Homa ToBondPool (r:1 w:0)
// Storage: Homa TotalVoidLiquid (r:1 w:0)
	fn transfer_debit() -> Weight {
		Weight::from_parts(196_453_000, 0)
			.saturating_add(T::DbWeight::get().reads(21 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
// Storage: Loans EcdpPositions (r:1 w:0)
// Storage: Prices LockedPrice (r:2 w:0)
// Storage: SetheumOracle Values (r:1 w:0)
// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
// Storage: Homa StakingLedgers (r:1 w:0)
// Storage: Homa ToBondPool (r:1 w:0)
// Storage: Tokens TotalIssuance (r:1 w:0)
// Storage: Homa TotalVoidLiquid (r:1 w:0)
// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	fn precompile_get_current_collateral_ratio() -> Weight {
		Weight::from_parts(44_244_000, 0)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn authorize() -> Weight {
		Weight::from_parts(45_674_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn unauthorize() -> Weight {
		Weight::from_parts(91_834_000, 0)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn unauthorize_all(c: u32, ) -> Weight {
		Weight::from_parts(51_744_000, 0)
// Standard Error: 866_000
			.saturating_add(Weight::from_parts(652_000, 0).saturating_mul(c as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
	fn adjust_loan() -> Weight {
		Weight::from_parts(142_855_000, 0)
			.saturating_add(RocksDbWeight::get().reads(16 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	fn transfer_loan_from() -> Weight {
		Weight::from_parts(120_478_000, 0)
			.saturating_add(RocksDbWeight::get().reads(17 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	fn close_loan_has_debit_by_dex() -> Weight {
		Weight::from_parts(349_743_000, 0)
			.saturating_add(RocksDbWeight::get().reads(35 as u64))
			.saturating_add(RocksDbWeight::get().writes(16 as u64))
	}
	fn expand_position_collateral() -> Weight {
		Weight::from_parts(227_393_000, 0)
			.saturating_add(RocksDbWeight::get().reads(23 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	fn shrink_position_debit() -> Weight {
		Weight::from_parts(230_779_000, 0)
			.saturating_add(RocksDbWeight::get().reads(19 as u64))
			.saturating_add(RocksDbWeight::get().writes(13 as u64))
	}
	fn transfer_debit() -> Weight {
		Weight::from_parts(196_453_000, 0)
			.saturating_add(RocksDbWeight::get().reads(21 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	fn precompile_get_current_collateral_ratio() -> Weight {
		Weight::from_parts(44_244_000, 0)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
	}
}
