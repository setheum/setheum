// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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
use frame_support::{assert_noop, assert_ok};
use mock::{
};
use module_support::{Swap, SwapError};
use module_traits::MultiReservableCurrency;
use sp_core::H160;
use sp_runtime::traits::BadOrigin;
use std::str::FromStr;

#[test]
fn list_provisioning_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			SwapLegacyModule::list_provisioning(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			BadOrigin
		);

		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::ListProvisioning {
		}));

		assert_noop!(
			SwapLegacyModule::list_provisioning(
				RuntimeOrigin::signed(ListingOrigin::get()),
				SEUSD,
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::InvalidCurrencyId
		);

		assert_noop!(
			SwapLegacyModule::list_provisioning(
				RuntimeOrigin::signed(ListingOrigin::get()),
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::MustBeDisabled
		);

		assert_noop!(
			SwapLegacyModule::list_provisioning(
				RuntimeOrigin::signed(ListingOrigin::get()),
				CurrencyId::ForeignAsset(0),
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::AssetUnregistered
		);
		assert_noop!(
			SwapLegacyModule::list_provisioning(
				RuntimeOrigin::signed(ListingOrigin::get()),
				SEUSD,
				CurrencyId::ForeignAsset(0),
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::AssetUnregistered
		);
	});
}

#[test]
fn update_provisioning_parameters_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			SwapLegacyModule::update_provisioning_parameters(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			BadOrigin
		);

		assert_noop!(
			SwapLegacyModule::update_provisioning_parameters(
				RuntimeOrigin::signed(ListingOrigin::get()),
				SEUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(SwapLegacyModule::update_provisioning_parameters(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			2_000_000_000_000u128,
			0,
			3_000_000_000_000u128,
			2_000_000_000_000u128,
			50,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (2_000_000_000_000u128, 0),
				target_provision: (3_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 50,
			})
		);
	});
}

#[test]
fn enable_diabled_trading_pair_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			BadOrigin
		);

		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(SwapLegacyModule::enable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::EnableTradingPair {
		}));

		assert_noop!(
			Error::<Runtime>::AlreadyEnabled
		);
	});
}

#[test]
fn enable_provisioning_without_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));

		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_ok!(SwapLegacyModule::enable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::EnableTradingPair {
		}));

		assert_noop!(
			SwapLegacyModule::enable_trading_pair(RuntimeOrigin::signed(ListingOrigin::get()), SEUSD, WBTC),
			Error::<Runtime>::StillProvisioning
		);
	});
}

#[test]
fn end_provisioning_trading_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			2_000_000_000_000u128
		));

		assert_noop!(
			SwapLegacyModule::end_provisioning(RuntimeOrigin::signed(ListingOrigin::get()), SEUSD, WBTC),
			Error::<Runtime>::UnqualifiedProvision
		);
		System::set_block_number(10);

		assert_eq!(
			SwapLegacyModule::trading_pair_statuses(SEUSDWBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 10,
			})
		);
		assert_eq!(
			SwapLegacyModule::initial_share_exchange_rates(SEUSDWBTCPair::get()),
			Default::default()
		);
		assert_eq!(SwapLegacyModule::liquidity_pool(SEUSDWBTCPair::get()), (0, 0));
		assert_eq!(Tokens::total_issuance(SEUSDWBTCPair::get().dex_share_currency_id()), 0);
		assert_eq!(
			Tokens::free_balance(SEUSDWBTCPair::get().dex_share_currency_id(), &SwapLegacyModule::account_id()),
			0
		);

		assert_ok!(SwapLegacyModule::end_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC
		));
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::ProvisioningToEnabled {
			trading_pair: SEUSDWBTCPair::get(),
			pool_0: 1_000_000_000_000u128,
			pool_1: 2_000_000_000_000u128,
			share_amount: 2_000_000_000_000u128,
		}));
		assert_eq!(
			SwapLegacyModule::trading_pair_statuses(SEUSDWBTCPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			SwapLegacyModule::initial_share_exchange_rates(SEUSDWBTCPair::get()),
			(ExchangeRate::one(), ExchangeRate::checked_from_rational(1, 2).unwrap())
		);
		assert_eq!(
			SwapLegacyModule::liquidity_pool(SEUSDWBTCPair::get()),
			(1_000_000_000_000u128, 2_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::total_issuance(SEUSDWBTCPair::get().dex_share_currency_id()),
			2_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(SEUSDWBTCPair::get().dex_share_currency_id(), &SwapLegacyModule::account_id()),
			2_000_000_000_000u128
		);
	});
}

#[test]
fn abort_provisioning_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));
		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));

		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(BOB),
			SEUSD,
			WBTC,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
		));

// not expired, nothing happened.
		System::set_block_number(2000);
		assert_ok!(SwapLegacyModule::abort_provisioning(RuntimeOrigin::signed(ALICE), SEUSD, WBTC));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 1_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(
			Default::default()
		);
		assert_eq!(
			SwapLegacyModule::trading_pair_statuses(SEUSDWBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(
			SwapLegacyModule::initial_share_exchange_rates(SEUSDWBTCPair::get()),
			Default::default()
		);

// couldn't be aborted because it's already met the target.
		System::set_block_number(3001);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::ProvisioningAborted {
			accumulated_provision_0: 1_000_000_000_000u128,
			accumulated_provision_1: 1_000_000_000_000u128,
		}));

		assert_ok!(SwapLegacyModule::abort_provisioning(RuntimeOrigin::signed(ALICE), SEUSD, WBTC));
		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(
			Default::default()
		);
		assert_eq!(
			SwapLegacyModule::trading_pair_statuses(SEUSDWBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(
			SwapLegacyModule::initial_share_exchange_rates(SEUSDWBTCPair::get()),
			Default::default()
		);
	});
}

#[test]
fn refund_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			5_000_000_000_000_000_000u128,
			4_000_000_000_000_000_000u128,
			1000,
		));
		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			1000,
		));

		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			1_000_000_000_000_000_000u128,
			1_000_000_000_000_000_000u128
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(BOB),
			SEUSD,
			0,
			600_000_000_000_000_000u128,
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(BOB),
			SEUSD,
			WBTC,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
		));

		assert_noop!(
			Error::<Runtime>::MustBeDisabled
		);

		System::set_block_number(3001);
		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(
			Default::default()
		);

		assert_eq!(
			(1_000_000_000_000_000_000u128, 1_000_000_000_000_000_000u128)
		);
		assert_eq!(
			(0, 600_000_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
			1_100_000_000_000_000_000u128
		);
		assert_eq!(
			1_600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 0);
		assert_eq!(Tokens::free_balance(SEUSD, &BOB), 900_000_000_000_000_000u128);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(SwapLegacyModule::refund_provision(
			RuntimeOrigin::signed(ALICE),
			ALICE,
			SEUSD,
		));
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::RefundProvision {
			who: ALICE,
			currency_0: SEUSD,
			contribution_0: 1_000_000_000_000_000_000u128,
			contribution_1: 1_000_000_000_000_000_000u128,
		}));

		assert_eq!(
			Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(
			600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);

		assert_ok!(SwapLegacyModule::refund_provision(
			RuntimeOrigin::signed(ALICE),
			BOB,
			SEUSD,
		));
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::RefundProvision {
			who: BOB,
			currency_0: SEUSD,
			contribution_0: 0,
			contribution_1: 600_000_000_000_000_000u128,
		}));

		assert_eq!(
			Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(SEUSD, &BOB), 900_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);

// not allow refund if the provisioning has been ended before.
		assert_ok!(SwapLegacyModule::end_provisioning(RuntimeOrigin::signed(ALICE), SEUSD, WBTC));
		assert_ok!(SwapLegacyModule::disable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC
		));
		assert_eq!(
			SwapLegacyModule::trading_pair_statuses(SEUSDWBTCPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(
			SwapLegacyModule::provisioning_pool(SEUSDWBTCPair::get(), BOB),
			(100_000_000_000_000_000u128, 100_000_000_000_000_000u128)
		);
		assert_noop!(
			SwapLegacyModule::refund_provision(RuntimeOrigin::signed(BOB), BOB, SEUSD, WBTC),
			Error::<Runtime>::NotAllowedRefund
		);
	});
}

#[test]
fn disable_trading_pair_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(SwapLegacyModule::enable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Enabled
		);

		assert_noop!(
			BadOrigin
		);

		assert_ok!(SwapLegacyModule::disable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::DisableTradingPair {
		}));

		assert_noop!(
			Error::<Runtime>::MustBeEnabled
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			WBTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_noop!(
			SwapLegacyModule::disable_trading_pair(RuntimeOrigin::signed(ListingOrigin::get()), SEUSD, WBTC),
			Error::<Runtime>::MustBeEnabled
		);
	});
}

#[test]
fn on_liquidity_pool_updated_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				WBTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			assert_eq!(
				(5000000000000, 1000000000000)
			);
		});
}

#[test]
fn add_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			SwapLegacyModule::add_provision(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				5_000_000_000_000u128,
				1_000_000_000_000u128,
			),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			5_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			10,
		));

		assert_noop!(
			SwapLegacyModule::add_provision(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				4_999_999_999_999u128,
				999_999_999_999u128,
			),
			Error::<Runtime>::InvalidContributionIncrement
		);

		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 0);
		let alice_ref_count_0 = System::consumers(&ALICE);

		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			5_000_000_000_000u128,
			0,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			(5_000_000_000_000u128, 0)
		);
		assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 999_995_000_000_000_000u128);
		assert_eq!(
			Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
			5_000_000_000_000u128
		);
		let alice_ref_count_1 = System::consumers(&ALICE);
		assert_eq!(alice_ref_count_1, alice_ref_count_0 + 1);
		System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::AddProvision {
			who: ALICE,
			currency_0: SEUSD,
			contribution_0: 5_000_000_000_000u128,
			contribution_1: 0,
		}));
	});
}

#[test]
fn claim_dex_share_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			5_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			0,
		));

		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(ALICE),
			SEUSD,
			1_000_000_000_000_000u128,
			200_000_000_000_000u128,
		));
		assert_ok!(SwapLegacyModule::add_provision(
			RuntimeOrigin::signed(BOB),
			SEUSD,
			4_000_000_000_000_000u128,
			800_000_000_000_000u128,
		));

		assert_noop!(
			Error::<Runtime>::StillProvisioning
		);

		assert_ok!(SwapLegacyModule::end_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));


		assert_eq!(
			(ExchangeRate::one(), ExchangeRate::saturating_from_rational(5, 1))
		);
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &SwapLegacyModule::account_id()),
			10_000_000_000_000_000u128
		);
		assert_eq!(
			(1_000_000_000_000_000u128, 200_000_000_000_000u128)
		);
		assert_eq!(
			(4_000_000_000_000_000u128, 800_000_000_000_000u128)
		);
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 0);
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 0);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(SwapLegacyModule::claim_dex_share(
			RuntimeOrigin::signed(ALICE),
			ALICE,
			SEUSD,
		));
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &SwapLegacyModule::account_id()),
			8_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 2_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);

		assert_ok!(SwapLegacyModule::disable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(Tokens::free_balance(lp_currency_id, &SwapLegacyModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 8_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);
	});
}

#[test]
fn get_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
	});
}

#[test]
fn get_target_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 0, 1000), 0);
		assert_eq!(SwapLegacyModule::get_target_amount(0, 20000, 1000), 0);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 20000, 0), 0);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 1, 1000000), 0);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 20000, 10000), 9949);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_supply_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(SwapLegacyModule::get_supply_amount(10000, 0, 1000), 0);
		assert_eq!(SwapLegacyModule::get_supply_amount(0, 20000, 1000), 0);
		assert_eq!(SwapLegacyModule::get_supply_amount(10000, 20000, 0), 0);
		assert_eq!(SwapLegacyModule::get_supply_amount(10000, 1, 1), 0);
		assert_eq!(SwapLegacyModule::get_supply_amount(10000, 20000, 9949), 9999);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 20000, 9999), 9949);
		assert_eq!(SwapLegacyModule::get_supply_amount(10000, 20000, 1801), 1000);
		assert_eq!(SwapLegacyModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_target_amounts_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SEUSDWBTCPair::get(), (100000, 10));
			assert_noop!(
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(
				Ok(vec![10000, 24874])
			);
			assert_eq!(
				Ok(vec![10000, 24874, 1])
			);
			assert_noop!(
				Error::<Runtime>::ZeroTargetAmount,
			);
			assert_noop!(
				Error::<Runtime>::InsufficientLiquidity,
			);
		});
}

#[test]
fn calculate_amount_for_big_number_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::<Runtime>::insert(
			(171_000_000_000_000_000_000_000, 56_000_000_000_000_000_000_000),
		);
		assert_eq!(
			SwapLegacyModule::get_supply_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				1_000_000_000_000_000_000_000
			),
			3_140_495_867_768_595_041_323
		);
		assert_eq!(
			SwapLegacyModule::get_target_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				3_140_495_867_768_595_041_323
			),
			1_000_000_000_000_000_000_000
		);
	});
}

#[test]
fn get_supply_amounts_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SEUSDWBTCPair::get(), (100000, 10));
			assert_noop!(
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(
				Ok(vec![10000, 24874])
			);
			assert_eq!(
				Ok(vec![10102, 25000])
			);
			assert_noop!(
				Error::<Runtime>::ZeroSupplyAmount,
			);
			assert_noop!(
				Error::<Runtime>::InsufficientLiquidity,
			);
		});
}

#[test]
fn _swap_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {

			assert_noop!(
				Error::<Runtime>::InvariantCheckFailed
			);
		});
}

#[test]
fn _swap_by_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SEUSDWBTCPair::get(), (100000, 10));

			assert_eq!(SwapLegacyModule::get_liquidity(SEUSD, WBTC), (100000, 10));
			assert_eq!(SwapLegacyModule::get_liquidity(SEUSD, WBTC), (120000, 9));
		});
}

#[test]
fn add_liquidity_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_noop!(
				SwapLegacyModule::add_liquidity(
					RuntimeOrigin::signed(ALICE),
					SEU,
					SEUSD,
					100_000_000,
					100_000_000,
					0,
					false
				),
				Error::<Runtime>::MustBeEnabled
			);
			assert_noop!(
				Error::<Runtime>::InvalidLiquidityIncrement
			);

			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 0);
			assert_eq!(
				0
			);
			assert_eq!(
				0
			);
			assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 1_000_000_000_000_000_000);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::AddLiquidity {
				who: ALICE,
				currency_0: SEUSD,
				pool_0: 5_000_000_000_000,
				pool_1: 1_000_000_000_000,
				share_increment: 10_000_000_000_000,
			}));
			assert_eq!(
				(5_000_000_000_000, 1_000_000_000_000)
			);
			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 5_000_000_000_000);
			assert_eq!(
				10_000_000_000_000
			);
			assert_eq!(
				0
			);
			assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 999_995_000_000_000_000);
			assert_eq!(
				0
			);
			assert_eq!(
				0
			);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				Error::<Runtime>::InvalidLiquidityIncrement,
			);

			assert_noop!(
				SwapLegacyModule::add_liquidity(
					RuntimeOrigin::signed(BOB),
					SEUSD,
					50_000_000_000_000,
					8_000_000_000_000,
					80_000_000_000_001,
					true,
				),
				Error::<Runtime>::UnacceptableShareIncrement
			);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(BOB),
				SEUSD,
				50_000_000_000_000,
				8_000_000_000_000,
				80_000_000_000_000,
				true,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::AddLiquidity {
				who: BOB,
				currency_0: SEUSD,
				pool_0: 40_000_000_000_000,
				pool_1: 8_000_000_000_000,
				share_increment: 80_000_000_000_000,
			}));
			assert_eq!(
				(45_000_000_000_000, 9_000_000_000_000)
			);
			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 45_000_000_000_000);
			assert_eq!(
				0
			);
			assert_eq!(
				80_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 999_960_000_000_000_000);
		});
}

#[test]
fn remove_liquidity_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false
			));
			assert_noop!(
				SwapLegacyModule::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					100_000_000,
					0,
					0,
					false,
				),
				Error::<Runtime>::InvalidCurrencyId
			);

			assert_eq!(
				(5_000_000_000_000, 1_000_000_000_000)
			);
			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 5_000_000_000_000);
			assert_eq!(
				10_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 999_995_000_000_000_000);

			assert_noop!(
				SwapLegacyModule::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					SEUSD,
					8_000_000_000_000,
					4_000_000_000_001,
					800_000_000_000,
					false,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_noop!(
				SwapLegacyModule::remove_liquidity(
					RuntimeOrigin::signed(ALICE),
					SEUSD,
					8_000_000_000_000,
					4_000_000_000_000,
					800_000_000_001,
					false,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_ok!(SwapLegacyModule::remove_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				8_000_000_000_000,
				4_000_000_000_000,
				800_000_000_000,
				false,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::RemoveLiquidity {
				who: ALICE,
				currency_0: SEUSD,
				pool_0: 4_000_000_000_000,
				pool_1: 800_000_000_000,
				share_decrement: 8_000_000_000_000,
			}));
			assert_eq!(
				(1_000_000_000_000, 200_000_000_000)
			);
			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 1_000_000_000_000);
			assert_eq!(
				2_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 999_999_000_000_000_000);

			assert_ok!(SwapLegacyModule::remove_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				2_000_000_000_000,
				0,
				0,
				false,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::RemoveLiquidity {
				who: ALICE,
				currency_0: SEUSD,
				pool_0: 1_000_000_000_000,
				pool_1: 200_000_000_000,
				share_decrement: 2_000_000_000_000,
			}));
			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 0);
			assert_eq!(
				0
			);
			assert_eq!(Tokens::free_balance(SEUSD, &ALICE), 1_000_000_000_000_000_000);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(BOB),
				SEUSD,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				true
			));
			assert_eq!(
				0
			);
			assert_eq!(
				10_000_000_000_000
			);
			assert_ok!(SwapLegacyModule::remove_liquidity(
				RuntimeOrigin::signed(BOB),
				SEUSD,
				2_000_000_000_000,
				0,
				0,
				true,
			));
			assert_eq!(
				0
			);
			assert_eq!(
				8_000_000_000_000
			);
		});
}

#[test]
fn do_swap_with_exact_supply_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				WBTC,
				100_000_000_000_000,
				10_000_000_000,
				0,
				false,
			));

			assert_eq!(
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(100_000_000_000_000, 10_000_000_000)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				600_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				Error::<Runtime>::InsufficientTargetAmount
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::MustBeEnabled,
			);

			assert_ok!(SwapLegacyModule::do_swap_with_exact_supply(
				&BOB,
				100_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));
			assert_eq!(
				(251_256_281_407_036, 200_000_000_000_000)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(100_000_000_000_000, 10_000_000_000)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				351_256_281_407_036
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_248_743_718_592_964);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(SwapLegacyModule::do_swap_with_exact_supply(
				&BOB,
				200_000_000_000_000,
				1,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![200_000_000_000_000, 124_996_843_514_053, 5_530_663_837],
			}));
			assert_eq!(
				(126_259_437_892_983, 400_000_000_000_000)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(224_996_843_514_053, 4_469_336_163)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				351_256_281_407_036
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 4_469_336_163);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_248_743_718_592_964);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_005_530_663_837);
		});
}

#[test]
fn do_swap_with_exact_target_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				WBTC,
				100_000_000_000_000,
				10_000_000_000,
				0,
				false,
			));

			assert_eq!(
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(100_000_000_000_000, 10_000_000_000)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				600_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				Error::<Runtime>::ExcessiveSupplyAmount
			);
			assert_noop!(
				SwapLegacyModule::do_swap_with_exact_target(
					&BOB,
					250_000_000_000_000,
					200_000_000_000_000,
				),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				Error::<Runtime>::MustBeEnabled,
			);

			assert_ok!(SwapLegacyModule::do_swap_with_exact_target(
				&BOB,
				250_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![101_010_101_010_102, 250_000_000_000_000],
			}));
			assert_eq!(
				(250_000_000_000_000, 201_010_101_010_102)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(100_000_000_000_000, 10_000_000_000)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				350_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_250_000_000_000_000);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(SwapLegacyModule::do_swap_with_exact_target(
				&BOB,
				5_000_000_000,
				2_000_000_000_000_000,
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![137_654_580_386_993, 101_010_101_010_102, 5_000_000_000],
			}));
			assert_eq!(
				(148_989_898_989_898, 338_664_681_397_095)
			);
			assert_eq!(
				SwapLegacyModule::get_liquidity(SEUSD, WBTC),
				(201_010_101_010_102, 5_000_000_000)
			);
			assert_eq!(
				Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()),
				350_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(WBTC, &SwapLegacyModule::account_id()), 5_000_000_000);
			assert_eq!(Tokens::free_balance(SEUSD, &BOB), 1_000_250_000_000_000_000);
			assert_eq!(Tokens::free_balance(WBTC, &BOB), 1_000_000_005_000_000_000);
		});
}

#[test]
fn initialize_added_liquidity_pools_genesis_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.initialize_added_liquidity_pools(ALICE)
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_eq!(Tokens::free_balance(SEUSD, &SwapLegacyModule::account_id()), 2000000);
			assert_eq!(
				2000000
			);
		});
}

#[test]
fn get_swap_amount_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			assert_eq!(
				Some((10000, 24874))
			);
			assert_eq!(
				None
			);
			assert_eq!(
				Some((10000, 24874))
			);
			assert_eq!(
				None
			);
		});
}

#[test]
fn get_best_price_swap_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SEUSDWBTCPair::get(), (50000, 10000));

			assert_eq!(
			);
			assert_eq!(
				None
			);
			assert_eq!(
				None
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);

			assert_eq!(
			);
			assert_eq!(
				None
			);
			assert_eq!(
				None
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);
			assert_eq!(
			);
		});
}

#[test]
fn swap_with_specific_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));

			assert_noop!(
				SwapLegacyModule::swap_with_specific_path(
					&BOB,
					SwapLimit::ExactSupply(100_000_000_000_000, 248_743_718_592_965)
				),
				Error::<Runtime>::InsufficientTargetAmount
			);

			assert_ok!(SwapLegacyModule::swap_with_specific_path(
				&BOB,
				SwapLimit::ExactSupply(100_000_000_000_000, 200_000_000_000_000)
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));

			assert_noop!(
				SwapLegacyModule::swap_with_specific_path(
					&BOB,
					SwapLimit::ExactTarget(253_794_223_643_470, 100_000_000_000_000)
				),
				Error::<Runtime>::ExcessiveSupplyAmount
			);

			assert_ok!(SwapLegacyModule::swap_with_specific_path(
				&BOB,
				SwapLimit::ExactTarget(300_000_000_000_000, 100_000_000_000_000)
			));
			System::assert_last_event(RuntimeEvent::SwapLegacyModule(crate::Event::Swap {
				trader: BOB,
				liquidity_changes: vec![253_794_223_643_471, 100_000_000_000_000],
			}));
		});
}

#[test]
fn get_liquidity_token_address_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(
			TradingPairStatus::<_, _>::Disabled
		);

		assert_ok!(SwapLegacyModule::list_provisioning(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			Some(H160::from_str("0x0000000000000000000200000000010000000002").unwrap())
		);

		assert_ok!(SwapLegacyModule::enable_trading_pair(
			RuntimeOrigin::signed(ListingOrigin::get()),
			SEUSD,
		));
		assert_eq!(
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			Some(H160::from_str("0x0000000000000000000200000000010000000002").unwrap())
		);
	});
}

#[test]
fn specific_joint_swap_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			assert_ok!(SwapLegacyModule::add_liquidity(
				RuntimeOrigin::signed(ALICE),
				SEUSD,
				WBTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));

			assert_eq!(
				Some((10000, 9800))
			);
			assert_eq!(
				None
			);

			assert_noop!(
				module_tokens::Error::<Runtime>::BalanceTooLow,
			);
			assert_noop!(
				SwapError::CannotSwap,
			);
			assert_noop!(
				SwapError::CannotSwap,
			);

			assert_eq!(
				Ok((10000, 9800)),
			);

			assert_eq!(
				Ok((10204, 10000)),
			);
		});
}
