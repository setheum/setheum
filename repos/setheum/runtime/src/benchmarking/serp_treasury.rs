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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
use crate::{
	AccountId, Balance, CurrencyId, Currencies, dollar, Dex,
	MaxSwapSlippageComparedToOracle, Prices, Ratio, Runtime,
	SerpTreasury, StableCurrencyIds, StableCurrencyInflationPeriod,
	System, GetDinarCurrencyId, GetSerpCurrencyId, GetNativeCurrencyId,
	GetHelpCurrencyId, GetSetUSDId, SetterCurrencyId, 
};

use super::utils::set_balance;
use frame_benchmarking::whitelisted_caller;
use frame_system::RawOrigin;
use module_benchmarking::runtime_benchmarks;
use frame_support::traits::OnInitialize;
use module_traits::MultiCurrency;
use sp_runtime::traits::Zero;
use sp_std::prelude::*;
use module_support::{SwapManager, SerpTreasury as SerpTreasurySupport, SerpTreasuryExtended, SwapLimit};

const SEU: CurrencyId = GetNativeCurrencyId::get();
const SEUSD: CurrencyId = GetSetUSDId::get();

runtime_benchmarks! {
	{ Runtime, serp_treasury }
	on_initialize {
		let c in 0 .. StableCurrencyIds::get().len().saturating_sub(1) as u32;
		let currency_ids = StableCurrencyIds::get();

		let block_number = StableCurrencyInflationPeriod::get();
		
		let caller: AccountId = whitelisted_caller();
		set_balance(SEU, &caller, 1000000000 * dollar(SEU));
		set_balance(SEUSD, &caller, 1000000000 * dollar(SEUSD));
		let _ = Dex::enable_trading_pair(RawOrigin::Root.into(), SEUSD, SEU);
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEU,
			100 * dollar(SEU),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEUSD,
			1000 * dollar(SEUSD),
			0,
		)?;Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEUSD,
			1000 * dollar(SEUSD),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEUSD,
			1000 * dollar(SEUSD),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEUSD,
			SEU,
			1000 * dollar(SEUSD),
			100 * dollar(SEU),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			SEUSD,
			1000 * dollar(SEUSD),
			0,
		)?;

		for i in 0 .. c {

			let currency_id = currency_ids[i as usize];
			
			let one: Balance = 1;
			let inflation_amount = SerpTreasury::stable_currency_inflation_rate(currency_id);
			let inflamounts: Balance = one.saturating_mul(inflation_amount // 5);

			if inflation_amount != 0 {
				
// inflation distros
// 1
				<SerpTreasury as SerpTreasurySupport<AccountId>>::add_cashdrop_to_pool(currency_id, inflamounts)?;
// 2
				<SerpTreasury as SerpTreasuryExtended<AccountId>>::buyback_swap_with_exact_supply(
					currency_id,
					SwapLimit::ExactSupply(inflamounts, 0),
				)?;
// 3
				<SerpTreasury as SerpTreasuryExtended<AccountId>>::buyback_swap_with_exact_supply(
					currency_id,
					SwapLimit::ExactSupply(inflamounts, 0),
				)?;
// 4
				<SerpTreasury as SerpTreasuryExtended<AccountId>>::buyback_swap_with_exact_supply(
					currency_id,
					SEU,
					SwapLimit::ExactSupply(inflamounts, 0),
				)?;
// 5
				<SerpTreasury as SerpTreasuryExtended<AccountId>>::buyback_swap_with_exact_supply(
					currency_id,
					SwapLimit::ExactSupply(inflamounts, 0),
				)?;
			};
		}


		let setter_peg: Balance = 4;

		let base_unit = setter_pool.saturating_mul(setter_peg);

		match setdollar_pool {
			0 => {} 
			setdollar_pool if setdollar_pool > base_unit => {
// safe from underflow because `setdollar_pool` is checked to be greater than `base_unit`
				let expand_by = <SerpTreasury as SerpTreasurySupport<AccountId>>::calculate_supply_change(setdollar_pool, base_unit, supply);
			}
			setdollar_pool if setdollar_pool < base_unit => {
// safe from underflow because `setdollar_pool` is checked to be less than `base_unit`
				let contract_by = <SerpTreasury as SerpTreasurySupport<AccountId>>::calculate_supply_change(base_unit, setdollar_pool, supply);
			}
			_ => {}
		}

		SerpTreasury::on_initialize(1);
		System::set_block_number(block_number);
	}: {
		SerpTreasury::on_initialize(System::block_number());
	}

	set_stable_currency_inflation_rate {
	
	force_serpdown {
		let caller: AccountId = whitelisted_caller();
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			0,
		)?;
		Dex::add_liquidity(
			RawOrigin::Signed(caller.clone()).into(),
			0,
		)?;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::benchmarking::utils::tests::new_test_ext;
	use module_benchmarking::impl_benchmark_test_suite;

	impl_benchmark_test_suite!(new_test_ext(),);
}
