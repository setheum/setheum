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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(warnings)]
#![allow(deprecated)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::{
	dispatch::DispatchClass,
	parameter_types,
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight},
		Weight,
	},
};
use frame_system::limits;
pub use frame_support::traits::EitherOfDiverse;
pub use frame_system::EnsureRoot;
pub use module_support::{ExchangeRate, Price, Rate, Ratio};
use primitives::{
	currency::{TokenInfo, SEU},
	evm::SYSTEM_CONTRACT_ADDRESS_PREFIX,
	Balance, CurrencyId,
};
use sp_core::H160;
use sp_runtime::{traits::Convert, transaction_validity::TransactionPriority, Perbill, RuntimeDebug};
use static_assertions::const_assert;

pub mod u32_trait {
	pub use sp_runtime::traits::Get;
	pub struct _1;
	impl Get<u32> for _1 { fn get() -> u32 { 1 } }
	pub struct _2;
	impl Get<u32> for _2 { fn get() -> u32 { 2 } }
	pub struct _3;
	impl Get<u32> for _3 { fn get() -> u32 { 3 } }
	pub struct _4;
	impl Get<u32> for _4 { fn get() -> u32 { 4 } }
}

pub const WEIGHT_PER_MILLIS: u64 = 1_000_000_000;

/// Start of the Setheum precompile address range (0x...0400).
pub const PRECOMPILE_ADDRESS_START: u64 = 0x400;
/// Start of the predeployed system-contract address range (0x...0800).
pub const PREDEPLOY_ADDRESS_START: u64 = 0x800;

pub use primitives::AccountId;

mod gas_to_weight_ratio;

pub type TimeStampedPrice = module_oracle::TimestampedValue<Price, primitives::Moment>;

// Priority of unsigned transactions
parameter_types! {
// Operational is 3/4 of TransactionPriority::max_value().
// Ensure Inherent -> Operational tx -> Unsigned tx -> Signed normal tx
	pub const CdpEngineUnsignedPriority: TransactionPriority = TransactionPriority::MAX / 2;      // 50%
	pub const AuctionManagerUnsignedPriority: TransactionPriority = TransactionPriority::MAX / 5; // 20%
}

/// Check if the given `address` is a system contract.
///
/// It's system contract if the address starts with SYSTEM_CONTRACT_ADDRESS_PREFIX.
pub fn is_system_contract(address: H160) -> bool {
	address.as_bytes().starts_with(&SYSTEM_CONTRACT_ADDRESS_PREFIX)
}

pub fn is_setheum_precompile(address: H160) -> bool {
	address >= H160::from_low_u64_be(PRECOMPILE_ADDRESS_START)
		&& address < H160::from_low_u64_be(PREDEPLOY_ADDRESS_START)
}

/// Convert gas to weight
pub struct GasToWeight;
impl Convert<u64, Weight> for GasToWeight {
	fn convert(gas: u64) -> Weight {
		Weight::from_parts(gas.saturating_mul(gas_to_weight_ratio::RATIO), 0)
	}
}

/// We assume that this part of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_perthousand(25);
/// The ratio that `Normal` extrinsics should occupy. Start from a conservative value.
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be
/// used by  Operational  extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 1 seconds of compute with a 3 seconds average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(1000 * WEIGHT_PER_MILLIS, 0);

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

parameter_types! {
/// Maximum length of block. Up to 5MB.
	pub BlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
/// Block weights base values and limits.
	pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
// Operational transactions have an extra reserved space, so that they
// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
}

parameter_types! {
/// A limit for off-chain phragmen unsigned solution submission.
///
/// We want to keep it as high as possible, but can't risk having it reject,
/// so we always subtract the base block execution weight.
	pub OffchainSolutionWeightLimit: Weight = BlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic
		.expect("Normal extrinsics have weight limit configured by default; qed")
		.saturating_sub(BlockExecutionWeight::get());
}

// TODO: make those const fn
pub fn dollar(currency_id: CurrencyId) -> Balance {
	10u128.saturating_pow(currency_id.decimals().expect("Does not support Non-Token decimals").into())
}

pub fn cent(currency_id: CurrencyId) -> Balance {
	dollar(currency_id) / 100
}

pub fn millicent(currency_id: CurrencyId) -> Balance {
	cent(currency_id) / 1000
}

pub fn microcent(currency_id: CurrencyId) -> Balance {
	millicent(currency_id) / 1000
}

// The nanoscent is only for currencies that have at least up to 18 decimals like the SEU
// 18 decimals = 1 Quintillion nanocents
// 1 Quintillion NANOCENTS = 1 DOLLAR
pub fn nanocent(currency_id: CurrencyId) -> Balance {
	microcent(currency_id) / 1000000
}

pub fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 1_000 * cent(SEU) + (bytes as Balance) * 100 * millicent(SEU)
}

pub type ShuraCouncilInstance = pallet_collective::Instance1;
pub type FinancialCouncilInstance = pallet_collective::Instance2;
pub type TechnicalCommitteeInstance = pallet_collective::Instance3;

pub type ShuraCouncilMembershipInstance = pallet_membership::Instance1;
pub type FinancialCouncilMembershipInstance = pallet_membership::Instance2;
pub type TechnicalCommitteeMembershipInstance = pallet_membership::Instance3;
pub type OperatorMembershipInstanceSetheum = pallet_membership::Instance4;

// Shura Council
pub type EnsureRootOrOneShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureMember<AccountId, ShuraCouncilInstance>,
>;

pub type EnsureRootOrAllShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, ShuraCouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, ShuraCouncilInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, ShuraCouncilInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, ShuraCouncilInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsShuraCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, ShuraCouncilInstance, 3, 4>,
>;

// Financial Council
pub type EnsureRootOrAllFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 3, 4>,
>;

// Technical Committee Council
pub type EnsureRootOrAllTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 1>,
>;

pub type EnsureRootOrHalfTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 3, 4>,
>;

/// The type used to represent the kinds of proxying allowed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, DecodeWithMemTracking, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum ProxyType {
	Any,
	CancelProxy,
	Governance,
	Auction,
	Swap,
	Loan,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_system_contract_works() {
		assert!(is_system_contract(H160::from_low_u64_be(0)));
		assert!(is_system_contract(H160::from_low_u64_be(u64::max_value())));

		let mut bytes = [0u8; 20];
		bytes[SYSTEM_CONTRACT_ADDRESS_PREFIX.len() - 1] = 1u8;

		assert!(!is_system_contract(bytes.into()));

		bytes = [0u8; 20];
		bytes[0] = 1u8;

		assert!(!is_system_contract(bytes.into()));
	}

	#[test]
	fn is_setheum_precompile_works() {
		assert!(!is_setheum_precompile(H160::from_low_u64_be(0)));
		assert!(!is_setheum_precompile(H160::from_low_u64_be(PRECOMPILE_ADDRESS_START - 1)));
		assert!(is_setheum_precompile(H160::from_low_u64_be(PRECOMPILE_ADDRESS_START)));
		assert!(is_setheum_precompile(H160::from_low_u64_be(PREDEPLOY_ADDRESS_START - 1)));
		assert!(!is_setheum_precompile(H160::from_low_u64_be(PREDEPLOY_ADDRESS_START)));
		assert!(!is_setheum_precompile([1u8; 20].into()));
	}
}
