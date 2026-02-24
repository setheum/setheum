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

use crate::Rate;
use parity_scale_codec::{Decode, Encode};
use primitives::CurrencyId;
use scale_info::TypeInfo;
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::prelude::*;

/// PoolId for various rewards pools
// Pool types:
// 1. EcdpSetrLiquidityRewards: record the shares and rewards for Setter (SETR)) ECDP users who are staking LP tokens.
// 2. EcdpUssdLiquidityRewards: record the shares and rewards for Slick USD (USSD) ECDP users who are staking LP tokens.
// 3. EdfisLiquidityRewards: record the shares and rewards for Edfis makers who are staking LP token.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum PoolId {
/// Rewards and shares pool for Setter (SETR)) ECDP users who are staking LP token(LPCurrencyId)
	EcdpSetrLiquidityRewards(CurrencyId),

/// Rewards and shares pool for Slick USD (USSD) ECDP users who are staking LP token(LPCurrencyId)
	EcdpUssdLiquidityRewards(CurrencyId),

/// Rewards and shares pool for Edfis market makers who stake LP token(LPCurrencyId)
	EdfisLiquidityRewards(CurrencyId),

/// Rewards and shares pool for Moya Earn
	MoyaEarnRewards(CurrencyId),
}

pub trait IncentivesManager<AccountId, Balance, CurrencyId, PoolId> {
/// Gets reward amount for the given reward currency added per period
	fn get_incentive_reward_amount(pool_id: PoolId, currency_id: CurrencyId) -> Balance;
/// Stake LP token to add shares to pool
	fn deposit_edfis_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
/// Unstake LP token to remove shares from pool
	fn withdraw_edfis_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
/// Claim all available rewards for specific `PoolId`
	fn claim_rewards(who: AccountId, pool_id: PoolId) -> DispatchResult;
/// Gets deduction reate for claiming reward early
	fn get_claim_reward_deduction_rate(pool_id: PoolId) -> Rate;
/// Gets the pending rewards for a pool, for an account
	fn get_pending_rewards(pool_id: PoolId, who: AccountId, reward_currency: Vec<CurrencyId>) -> Vec<Balance>;
}

pub trait Incentives<AccountId, CurrencyId, Balance> {
	fn do_deposit_edfis_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
	fn do_withdraw_edfis_share(who: &AccountId, lp_currency_id: CurrencyId, amount: Balance) -> DispatchResult;
}

#[cfg(feature = "std")]
impl<AccountId, CurrencyId, Balance> Incentives<AccountId, CurrencyId, Balance> for () {
	fn do_deposit_edfis_share(_: &AccountId, _: CurrencyId, _: Balance) -> DispatchResult {
		Ok(())
	}

	fn do_withdraw_edfis_share(_: &AccountId, _: CurrencyId, _: Balance) -> DispatchResult {
		Ok(())
	}
}
