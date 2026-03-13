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
use frame_support::{construct_runtime, ord_parameter_types, parameter_types};
use frame_system::EnsureSignedBy;
use module_traits::parameter_type_with_key;
use primitives::{AccountId as AccId, Amount, TokenSymbol};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};

pub type AccountId = AccId;
pub type BlockNumber = u64;

pub const TREASURY: AccountId = AccountId32::new([0u8; 32]);
pub const ALICE: AccountId = AccountId32::new([2u8; 32]);
pub const BOB: AccountId = AccountId32::new([3u8; 32]);
pub const SEUSD: CurrencyId = CurrencyId::Token(TokenSymbol::SEUSD);
pub const SEU: CurrencyId = CurrencyId::Token(TokenSymbol::SEU);

mod airdrop {
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

impl module_tokens::Config for Runtime {
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

parameter_types! {
	pub StableCurrencyIds: Vec<CurrencyId> = vec![
		SEUSD,
	];
	pub const GetSetUSDId: CurrencyId = SEUSD;  // SetDollar currency ticker is SEUSD/
	pub const GetNativeCurrencyId: CurrencyId = SEU;  // Setheum native currency ticker is SEU/
	pub const AirdropPalletId: PalletId = PalletId(*b"set/drop");
	pub const MaxAirdropListSize: usize = 4;
}

ord_parameter_types! {
	pub const TreasuryAccount: AccountId = TREASURY;
	pub const One: AccountId = AccountId32::new([1u8; 32]);
}
impl Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type MaxAirdropListSize = MaxAirdropListSize;
	type FundingOrigin = TreasuryAccount;
	type DropOrigin = EnsureSignedBy<One, AccountId>;
	type PalletId = AirdropPalletId;
}

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, Call, u32, ()>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Storage, Call, Config, Event<T>},
		AirDrop: airdrop::{Pallet, Storage, Call, Event<T>},
		Tokens: module_tokens::{Pallet, Storage, Call, Event<T>},
	}
);

pub struct ExtBuilder {
	_balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			_balances: vec![
				(ALICE, SEUSD, 1000),
				(BOB, SEUSD, 1000),
				(TREASURY, SEUSD, 1000),
				(ALICE, SEU, 1000),
				(BOB, SEU, 1000),
				(TREASURY, SEU, 1000),
			],
		}
	}
}
