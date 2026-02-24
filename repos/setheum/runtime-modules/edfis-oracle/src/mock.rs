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

use super::*;
use frame_support::{construct_runtime, derive_impl, ord_parameter_types, parameter_types, traits::ConstU64};
use frame_system::EnsureSignedBy;
use module_support::SwapLimit;
use primitives::{DexShare, Moment, TokenSymbol};
use sp_core::H160;
use sp_runtime::{
	traits::{IdentityLookup, Zero},
	BuildStorage, DispatchError,
};
use sp_std::cell::RefCell;

pub type AccountId = u128;

pub const SEE: CurrencyId = CurrencyId::Token(TokenSymbol::SEE);
pub const USSD: CurrencyId = CurrencyId::Token(TokenSymbol::USSD);
pub const EDF: CurrencyId = CurrencyId::Token(TokenSymbol::EDF);
pub const LP_USSD_EDF: CurrencyId =
	CurrencyId::DexShare(DexShare::Token(TokenSymbol::USSD), DexShare::Token(TokenSymbol::EDF));

mod dex_oracle {
	pub use super::super::*;
}

parameter_types! {
	pub static USSDEDFPair: TradingPair = TradingPair::from_currency_ids(USSD, EDF).unwrap();
	pub static SEEEDFPair: TradingPair = TradingPair::from_currency_ids(SEE, EDF).unwrap();
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = ();
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1000>;
	type WeightInfo = ();
}

thread_local! {
	static USSD_EDF_POOL: RefCell<(Balance, Balance)> = RefCell::new((Zero::zero(), Zero::zero()));
	static SEE_EDF_POOL: RefCell<(Balance, Balance)> = RefCell::new((Zero::zero(), Zero::zero()));
}

pub fn set_pool(trading_pair: &TradingPair, pool_0: Balance, pool_1: Balance) {
	if *trading_pair == USSDEDFPair::get() {
		USSD_EDF_POOL.with(|v| *v.borrow_mut() = (pool_0, pool_1));
	} else if *trading_pair == SEEEDFPair::get() {
		SEE_EDF_POOL.with(|v| *v.borrow_mut() = (pool_0, pool_1));
	}
}

pub struct MockDEX;
impl SwapManager<AccountId, Balance, CurrencyId> for MockDEX {
	fn get_liquidity_pool(currency_id_0: CurrencyId, currency_id_1: CurrencyId) -> (Balance, Balance) {
		TradingPair::from_currency_ids(currency_id_0, currency_id_1)
			.map(|trading_pair| {
				if trading_pair == USSDEDFPair::get() {
					USSD_EDF_POOL.with(|v| *v.borrow())
				} else if trading_pair == SEEEDFPair::get() {
					SEE_EDF_POOL.with(|v| *v.borrow())
				} else {
					(0, 0)
				}
			})
			.unwrap_or_else(|| (0, 0))
	}

	fn get_liquidity_token_address(_currency_id_a: CurrencyId, _currency_id_b: CurrencyId) -> Option<H160> {
		unimplemented!()
	}

	fn get_swap_amount(_: &[CurrencyId], _: SwapLimit<Balance>) -> Option<(Balance, Balance)> {
		unimplemented!()
	}

	fn get_best_price_swap_path(
		_: CurrencyId,
		_: CurrencyId,
		_: SwapLimit<Balance>,
		_: Vec<Vec<CurrencyId>>,
	) -> Option<(Vec<CurrencyId>, Balance, Balance)> {
		unimplemented!()
	}

	fn swap_with_specific_path(
		_: &AccountId,
		_: &[CurrencyId],
		_: SwapLimit<Balance>,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn add_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_max_amount_a: Balance,
		_max_amount_b: Balance,
		_min_share_increment: Balance,
		_stake_increment_share: bool,
	) -> sp_std::result::Result<(Balance, Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn remove_liquidity(
		_who: &AccountId,
		_currency_id_a: CurrencyId,
		_currency_id_b: CurrencyId,
		_remove_share: Balance,
		_min_withdrawn_a: Balance,
		_min_withdrawn_b: Balance,
		_by_unstake: bool,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}
}

ord_parameter_types! {
	pub const One: AccountId = 1;
}

impl Config for Runtime {
	type DEX = MockDEX;
	type Time = Timestamp;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type WeightInfo = ();
}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Timestamp: pallet_timestamp,
		EdfisOracle: edfis_oracle,
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
