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
use mock::{RuntimeEvent, *};
use module_rewards::PoolInfo;
use module_traits::MultiCurrency;
use sp_runtime::{traits::BadOrigin, FixedPointNumber};

#[test]
fn deposit_dex_share_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TokensModule::deposit(BTC_SEUSD_LP, &ALICE::get(), 10000));
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &ALICE::get()), 10000);
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &IncentivesModule::account_id()), 0);
		assert_eq!(RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)), PoolInfo::default(),);

		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::Dex(BTC_SEUSD_LP), ALICE::get()),
			Default::default(),
		);

		assert_ok!(IncentivesModule::deposit_dex_share(RuntimeOrigin::signed(ALICE::get()), BTC_SEUSD_LP, 10000));
		System::assert_last_event(RuntimeEvent::IncentivesModule(crate::Event::DepositDexShare {
			who: ALICE::get(),
			dex_share_type: BTC_SEUSD_LP,
			deposit: 10000,
		}));
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &ALICE::get()), 0);
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &IncentivesModule::account_id()), 10000);
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 10000, ..Default::default() }
		);
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::Dex(BTC_SEUSD_LP), ALICE::get()),
			(10000, Default::default())
		);
	});
}

#[test]
fn withdraw_dex_share_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TokensModule::deposit(BTC_SEUSD_LP, &ALICE::get(), 10000));

		assert_noop!(
			IncentivesModule::withdraw_dex_share(RuntimeOrigin::signed(BOB::get()), BTC_SEUSD_LP, 10000),
			Error::<Runtime>::NotEnough,
		);

		assert_ok!(IncentivesModule::deposit_dex_share(RuntimeOrigin::signed(ALICE::get()), BTC_SEUSD_LP, 10000));
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &ALICE::get()), 0);
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &IncentivesModule::account_id()), 10000);
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 10000, ..Default::default() }
		);
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::Dex(BTC_SEUSD_LP), ALICE::get()),
			(10000, Default::default())
		);

		assert_ok!(IncentivesModule::withdraw_dex_share(RuntimeOrigin::signed(ALICE::get()), BTC_SEUSD_LP, 8000));
		System::assert_last_event(RuntimeEvent::IncentivesModule(crate::Event::WithdrawDexShare {
			who: ALICE::get(),
			dex_share_type: BTC_SEUSD_LP,
			withdraw: 8000,
		}));
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &ALICE::get()), 8000);
		assert_eq!(TokensModule::free_balance(BTC_SEUSD_LP, &IncentivesModule::account_id()), 2000);
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 2000, ..Default::default() }
		);
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::Dex(BTC_SEUSD_LP), ALICE::get()),
			(2000, Default::default())
		);
	});
}

#[test]
fn update_incentive_rewards_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_incentive_rewards(RuntimeOrigin::signed(ALICE::get()), vec![]),
			BadOrigin
		);
		assert_noop!(
			IncentivesModule::update_incentive_rewards(RuntimeOrigin::signed(ROOT::get()),),
			Error::<Runtime>::InvalidPoolId
		);

		assert_eq!(0);
		assert_eq!(0);

		assert_ok!(IncentivesModule::update_incentive_rewards(RuntimeOrigin::signed(ROOT::get()), vec![],));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::IncentiveRewardAmountUpdated {
			reward_currency_id: SEU,
			reward_amount_per_period: 1000,
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::IncentiveRewardAmountUpdated {
			reward_amount_per_period: 100,
		}));
		assert_eq!(1000);
		assert_eq!(true);
		assert_eq!(100);

		assert_ok!(IncentivesModule::update_incentive_rewards(RuntimeOrigin::signed(ROOT::get()),));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::IncentiveRewardAmountUpdated {
			reward_currency_id: SEU,
			reward_amount_per_period: 200,
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::IncentiveRewardAmountUpdated {
			reward_amount_per_period: 0,
		}));
		assert_eq!(200);
		assert_eq!(false);
	});
}

#[test]
fn update_claim_reward_deduction_rates_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_claim_reward_deduction_rates(RuntimeOrigin::signed(ALICE::get()), vec![]),
			BadOrigin
		);
		assert_noop!(
			IncentivesModule::update_claim_reward_deduction_rates(RuntimeOrigin::signed(ROOT::get()),),
			Error::<Runtime>::InvalidPoolId
		);
		assert_noop!(
			IncentivesModule::update_claim_reward_deduction_rates(RuntimeOrigin::signed(ROOT::get()),),
			Error::<Runtime>::InvalidRate,
		);

		assert_eq!(Rate::zero());
		assert_eq!(IncentivesModule::claim_reward_deduction_rates(&PoolId::Dex(BTC_SEUSD_LP)), Rate::zero());

		assert_ok!(IncentivesModule::update_claim_reward_deduction_rates(
			RuntimeOrigin::signed(ROOT::get()),
			vec![(PoolId::Dex(BTC_SEUSD_LP), Rate::saturating_from_rational(2, 100))]
		));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewardDeductionRateUpdated {
			deduction_rate: Rate::saturating_from_rational(1, 100),
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewardDeductionRateUpdated {
			pool: PoolId::Dex(BTC_SEUSD_LP),
			deduction_rate: Rate::saturating_from_rational(2, 100),
		}));
		assert_eq!(Rate::saturating_from_rational(1, 100));
		assert_eq!(ClaimRewardDeductionRates::<Runtime>::contains_key(PoolId::Dex(BTC_SEUSD_LP)), true);
		assert_eq!(
			IncentivesModule::claim_reward_deduction_rates(&PoolId::Dex(BTC_SEUSD_LP)),
			Rate::saturating_from_rational(2, 100)
		);

		assert_ok!(IncentivesModule::update_claim_reward_deduction_rates(
			RuntimeOrigin::signed(ROOT::get()),
			vec![(PoolId::Dex(BTC_SEUSD_LP), Rate::zero())]
		));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewardDeductionRateUpdated {
			deduction_rate: Rate::saturating_from_rational(5, 100),
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewardDeductionRateUpdated {
			pool: PoolId::Dex(BTC_SEUSD_LP),
			deduction_rate: Rate::zero(),
		}));
		assert_eq!(Rate::saturating_from_rational(5, 100));
		assert_eq!(ClaimRewardDeductionRates::<Runtime>::contains_key(PoolId::Dex(BTC_SEUSD_LP)), false);
		assert_eq!(IncentivesModule::claim_reward_deduction_rates(&PoolId::Dex(BTC_SEUSD_LP)), Rate::zero());
	});
}

#[test]
fn payout_works() {
	ExtBuilder::default().build().execute_with(|| {});
}

#[test]
fn transfer_failed_when_claim_rewards() {
	ExtBuilder::default().build().execute_with(|| {});
}

#[test]
fn claim_rewards_works() {
	ExtBuilder::default().build().execute_with(|| {});
}

#[test]
fn on_initialize_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TokensModule::deposit(SEU, &RewardsSource::get(), 10000));
		assert_ok!(TokensModule::deposit(SEUSD, &RewardsSource::get(), 10000));

		assert_ok!(IncentivesModule::update_incentive_rewards(
			RuntimeOrigin::signed(ROOT::get()),
			vec![(PoolId::Dex(BTC_SEUSD_LP), vec![(SEU, 100)]), (PoolId::MoyaEarnRewards(SEU), vec![(SEU, 100)]),],
		));

		RewardsModule::add_share(&ALICE::get(), &PoolId::Dex(BTC_SEUSD_LP), 1);
		RewardsModule::add_share(&ALICE::get(), &PoolId::MoyaEarnRewards(SEU), 1);

		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 10000);
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 10000);

		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 1, ..Default::default() }
		);
		assert_eq!(PoolInfo { total_shares: 1, ..Default::default() });
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 1, ..Default::default() }
		);

		// per 10 blocks will accumulate rewards, nothing happened when on_initialize(9)
		IncentivesModule::on_initialize(9);

		IncentivesModule::on_initialize(10);
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 10000 - (1000 + 200 + 100 + 100));
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 10000 - 500);

		// 100 SEU is incentive reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (100, 0))].into_iter().collect() }
		);
		// 200 SEU is incentive reward
		assert_eq!(PoolInfo { total_shares: 1, rewards: vec![(SEU, (200, 0))].into_iter().collect() });
		// 100 SEU is incentive reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (100, 0))].into_iter().collect() }
		);

		IncentivesModule::on_initialize(20);
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 8600 - (1000 + 2000 + 100 + 200 + 100));
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 9500 - 500);

		// 100 SEU is incentive reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (200, 0))].into_iter().collect() }
		);
		// 200 SEU is incentive reward
		assert_eq!(PoolInfo { total_shares: 1, rewards: vec![(SEU, (400, 0))].into_iter().collect() });
		// 100 SEU is incentive reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (200, 0))].into_iter().collect() }
		);

		mock_shutdown();
		IncentivesModule::on_initialize(30);
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 5200 - (100 + 200 + 100));
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 9000);

		// after shutdown, PoolId::Dex will accumulate incentive rewards
		// reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::Dex(BTC_SEUSD_LP)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (300, 0))].into_iter().collect() }
		);
		// after shutdown, PoolId::Dex will accumulate incentive rewards
		// reward
		assert_eq!(PoolInfo { total_shares: 1, rewards: vec![(SEU, (600, 0))].into_iter().collect() });
		// after shutdown, PoolId::MoyaEarnRewards will accumulate incentive rewards
		// reward
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 1, rewards: vec![(SEU, (300, 0))].into_iter().collect() }
		);
	});
}

#[test]
fn earning_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		OnEarnBonded::<Runtime>::happened(&(ALICE::get(), 80));
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 80, ..Default::default() }
		);
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::MoyaEarnRewards(SEU), ALICE::get()),
			(80, Default::default())
		);

		OnEarnUnbonded::<Runtime>::happened(&(ALICE::get(), 20));
		assert_eq!(
			RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)),
			PoolInfo { total_shares: 60, ..Default::default() }
		);
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::MoyaEarnRewards(SEU), ALICE::get()),
			(60, Default::default())
		);

		OnEarnUnbonded::<Runtime>::happened(&(ALICE::get(), 60));
		assert_eq!(RewardsModule::pool_infos(PoolId::MoyaEarnRewards(SEU)), PoolInfo { ..Default::default() });
		assert_eq!(
			RewardsModule::shares_and_withdrawn_rewards(PoolId::MoyaEarnRewards(SEU), ALICE::get()),
			(0, Default::default())
		);
	});
}

#[test]
fn transfer_reward_and_update_rewards_storage_atomically_when_accumulate_incentives_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(TokensModule::deposit(SEUSD, &RewardsSource::get(), 100));
		assert_ok!(TokensModule::deposit(SEU, &RewardsSource::get(), 100));
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 100);
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 100);

		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 100);
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 100);

		// accumulate SEU and SEUSD rewards succeeded
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 70);
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 10);

		// accumulate SEU reward succeeded， accumulate SEUSD reward failed
		assert_eq!(TokensModule::free_balance(SEU, &RewardsSource::get()), 40);
		assert_eq!(TokensModule::free_balance(SEUSD, &RewardsSource::get()), 10);
	});
}

#[test]
fn update_claim_reward_deduction_currency() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			IncentivesModule::update_claim_reward_deduction_currency(RuntimeOrigin::signed(ALICE::get()), Some(SEU)),
			BadOrigin
		);

		assert_ok!(IncentivesModule::update_claim_reward_deduction_rates(RuntimeOrigin::signed(ROOT::get()),));
		assert_ok!(IncentivesModule::update_claim_reward_deduction_currency(
			RuntimeOrigin::signed(ROOT::get()),
			Some(SEU)
		),);
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewardDeductionCurrencyUpdated {
			currency: Some(SEU),
		}));

		assert_eq!(Some(SEU));
	});
}

#[test]
fn claim_reward_deduction_currency_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(IncentivesModule::update_claim_reward_deduction_rates(
			RuntimeOrigin::signed(ROOT::get()),
			vec![(pool_id, Rate::saturating_from_rational(10, 100)),]
		));
		assert_ok!(IncentivesModule::update_claim_reward_deduction_currency(
			RuntimeOrigin::signed(ROOT::get()),
			pool_id,
			Some(SEU)
		));

		// alice add shares before accumulate rewards
		RewardsModule::add_share(&ALICE::get(), &pool_id, 100);

		// bob add shares before accumulate rewards
		RewardsModule::add_share(&BOB::get(), &pool_id, 100);

		// accumulate rewards
		assert_ok!(RewardsModule::accumulate_reward(&pool_id, SEU, 1000));
		assert_ok!(RewardsModule::accumulate_reward(&pool_id, SEUSD, 2000));

		// alice claim rewards
		assert_ok!(IncentivesModule::claim_rewards(RuntimeOrigin::signed(ALICE::get()), pool_id));

		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewards {
			who: ALICE::get(),
			pool: pool_id,
			reward_currency_id: SEU,
			actual_amount: 450,
			deduction_amount: 50,
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewards {
			who: ALICE::get(),
			pool: pool_id,
			reward_currency_id: SEUSD,
			actual_amount: 1000,
			deduction_amount: 0,
		}));

		System::reset_events();

		assert_eq!(TokensModule::free_balance(SEU, &ALICE::get()), 450);
		assert_eq!(TokensModule::free_balance(SEUSD, &ALICE::get()), 1000);

		// apply deduction currency to all rewards
		assert_ok!(IncentivesModule::update_claim_reward_deduction_currency(
			RuntimeOrigin::signed(ROOT::get()),
			pool_id,
			None
		));

		// accumulate rewards
		assert_ok!(RewardsModule::accumulate_reward(&pool_id, SEU, 1000));
		assert_ok!(RewardsModule::accumulate_reward(&pool_id, SEUSD, 2000));

		// alice claim rewards
		assert_ok!(IncentivesModule::claim_rewards(RuntimeOrigin::signed(ALICE::get()), pool_id));

		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewards {
			who: ALICE::get(),
			pool: pool_id,
			reward_currency_id: SEU,
			actual_amount: 473,
			deduction_amount: 52,
		}));
		System::assert_has_event(RuntimeEvent::IncentivesModule(crate::Event::ClaimRewards {
			who: ALICE::get(),
			pool: pool_id,
			reward_currency_id: SEUSD,
			actual_amount: 900,
			deduction_amount: 100,
		}));

		assert_eq!(TokensModule::free_balance(SEU, &ALICE::get()), 923);
		assert_eq!(TokensModule::free_balance(SEUSD, &ALICE::get()), 1900);
	});
}
