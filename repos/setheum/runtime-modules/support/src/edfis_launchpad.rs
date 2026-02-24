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

use codec::{Decode, Encode};
use sp_runtime::{
	DispatchError, DispatchResult,
};
use sp_std::{
	cmp::{Eq, PartialEq},
};

/// Abstraction over th Launchpad Proposal system.
pub trait Proposal<AccountId, BlockNumber> {
	type CurrencyId;

/// The Campaign Proposal info of `id`
	fn proposal_info(id: Self::CurrencyId) -> Option<CampaignInfo<AccountId, Balance, BlockNumber>>;
/// Get all proposals
	fn all_proposals() -> Vec<CampaignInfo<AccountId, Balance, BlockNumber>>;
/// Create new Campaign Proposal with specific `CampaignInfo`, return the `id` of the Campaign
	fn new_proposal(
		origin: AccountId,
		beneficiary: AccountId,
		raise_currency: CurrencyId,
		sale_token: CurrencyId,
		token_price: Balance,
		crowd_allocation: Balance,
		goal: Balance,
		period: BlockNumber,
	) -> DispatchResult;
/// Approve Proposal by `id` at `now`.
    fn on_approve_proposal(id: Self::CurrencyId) -> sp_std::result::Result<(), DispatchError>;
/// Reject Proposal by `id` and update storage
	fn on_reject_proposal(id: Self::CurrencyId) -> sp_std::result::Result<(), DispatchError>;
/// Remove Proposal by `id` from storage
	fn remove_proposal(id: Self::CurrencyId) -> sp_std::result::Result<(), DispatchError>;
}

/// Abstraction over the Launchpad Campaign system.
pub trait CampaignManager<AccountId, BlockNumber> {
	type CurrencyId;

/// The Campaign info of `id`
	fn campaign_info(id: Self::CurrencyId) -> Option<CampaignInfo<AccountId, Balance, BlockNumber>>;
/// Get all proposals
	fn all_campaigns() -> Vec<CampaignInfo<AccountId, Balance, BlockNumber>>;
/// Called when a contribution is received.
	fn on_contribution(
		who: AccountId,
		id: Self::CurrencyId,
		amount: Balance,
	) -> DispatchResult;
/// Called when a contribution allocation is claimed
	fn on_claim_allocation(
		who: AccountId,
		id: Self::CurrencyId,
	) -> DispatchResult;
/// Called when a campaign's raised fund is claimed
	fn on_claim_campaign(
		who: AccountId,
		id: Self::CurrencyId,
	) -> DispatchResult;
/// Called when a failed campaign is claimed by the proposer
	fn on_claim_failed_campaign(
		who: AccountId,
		id: Self::CurrencyId,
	) -> DispatchResult;
/// Activate a campaign by `id`
	fn activate_campaign(id: Self::CurrencyId) -> DispatchResult;
/// Ensure campaign is Valid and Successfully Ended
	fn ensure_successfully_ended_campaign(id: Self::CurrencyId) -> DispatchResult;
/// Record Successful Campaign by `id`
	fn on_successful_campaign(now: BlockNumber, id: Self::CurrencyId) -> DispatchResult ;
/// Record Failed Campaign by `id`
	fn on_failed_campaign(now: BlockNumber, id: Self::CurrencyId) -> DispatchResult ;
/// Called when pool is retired
	fn on_retire(id: Self::CurrencyId)-> DispatchResult;
/// Get amount of contributors in a campaign
	fn get_contributors_count(id: Self::CurrencyId) -> u32;
/// Get the total amounts raised in protocol
	fn get_total_amounts_raised() -> Vec<(CurrencyId, Balance)>;
}
