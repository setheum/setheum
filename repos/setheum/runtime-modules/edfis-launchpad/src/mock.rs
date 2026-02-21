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
use frame_support::{construct_runtime, ord_parameter_types, parameter_types, PalletId};
use frame_system::EnsureSignedBy;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::IdentityLookup,
};
use primitives::{Amount, Balance, TokenSymbol};

pub type AccountId = u128;
pub type BlockNumber = u64;
// The network Treasury account.
pub const TREASURY: AccountId = 0;
// Mock accounts.
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

pub const SEE: CurrencyId = CurrencyId::Token(TokenSymbol::SEE);
pub const USSD: CurrencyId = CurrencyId::Token(TokenSymbol::USSD);
pub const TEST: CurrencyId = CurrencyId::Token(TokenSymbol::SETR);
pub const EDF: CurrencyId = CurrencyId::Token(TokenSymbol::EDF);

mod launchpad_crowdsales {
	pub use super::super::*;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type DustRemovalWhitelist = ();
}

parameter_type_with_key! {
	pub MinRaise: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&USSD => 100,
			&SEE => 100,
			&EDF => 100,
			_ => 0,
		}
	};
}

parameter_type_with_key! {
	pub MinContribution: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&USSD => 100,
			&SEE => 100,
			&EDF => 100,
			_ => 0,
		}
	};
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SEE;  // Setheum native currency ticker is SEE/
	pub const GetCommission: (u32, u32) = (10, 100); // 10%
	pub const SubmissionDeposit: Balance = 101;
	pub const MaxProposalsCount: u32 = 3;
	pub const MaxCampaignsCount: u32 = 3;
	pub const MaxActivePeriod: BlockNumber = 20;
	pub const CampaignStartDelay: BlockNumber = 20;
	pub const RetirementPeriod: BlockNumber = 20;
	pub const CrowdsalesPalletId: PalletId = PalletId(*b"set/help");
}

ord_parameter_types! {
	pub const TreasuryAccount: AccountId = TREASURY;
	pub const Eleven: AccountId = 11;
}
impl Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type GetCommission = GetCommission;
	type SubmissionDeposit = SubmissionDeposit;
	type MinRaise = MinRaise;
	type MinContribution = MinContribution;
	type MaxProposalsCount = MaxProposalsCount;
	type MaxCampaignsCount = MaxCampaignsCount;
	type MaxActivePeriod = MaxActivePeriod;
	type CampaignStartDelay = CampaignStartDelay;
	type CampaignRetirementPeriod = RetirementPeriod;
	type ProposalRetirementPeriod = RetirementPeriod;
	type UpdateOrigin = EnsureSignedBy<Eleven, AccountId>;
	type PalletId = CrowdsalesPalletId;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		LaunchPad: launchpad_crowdsales::{Pallet, Storage, Call, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Call, Event<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![] }
	}
}

impl ExtBuilder {
	pub fn balances(mut self, balances: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	pub fn one_hundred_thousand_for_all(self) -> Self {
		self.balances(vec![
			(ALICE, SEE, 100_000),
			(ALICE, USSD, 100_000),
			(ALICE, EDF, 100_000),
			(ALICE, TEST, 100_000),
			(BOB, SEE, 100_000),
			(BOB, USSD, 100_000),
			(BOB, EDF, 100_000),
			(BOB, TEST, 100_000),
			(CHARLIE, SEE, 100_000),
			(CHARLIE, USSD, 100_000),
			(CHARLIE, EDF, 100_000),
			(CHARLIE, TEST, 100_000),
		])
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		orml_tokens::GenesisConfig::<Runtime> {
			balances: self
				.balances
				.into_iter()
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
