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
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use core::convert::{TryFrom, TryInto};
use codec::{Compact, Decode, Encode};
use sp_std::prelude::*;
use sp_core::{
	crypto::KeyTypeId,
// u32_trait::{_2, _3, _4},
	H160, OpaqueMetadata,
};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, BadOrigin, BlakeTwo256, Block as BlockT, Convert, SaturatedConversion, StaticLookup,
	},
	transaction_validity::{TransactionSource, TransactionValidity, TransactionPriority},
	ApplyExtrinsicResult, DispatchResult, FixedPointNumber, curve::PiecewiseLinear,
};
use sp_runtime::traits::{
	NumberFor,
// Zero,
	OpaqueKeys,
};
pub use sp_runtime::{
	Perbill, Percent, Permill, Perquintill,
};
use sp_api::impl_runtime_apis;
use frame_election_provider_support::onchain;
pub use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
pub use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;

use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
use frame_support::pallet_prelude::InvalidTransaction;
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		Contains, ContainsLengthBound, Currency as PalletCurrency, EnsureOrigin, Everything, Get, Imbalance,
		InstanceFilter, IsSubType, IsType, KeyOwnerProofSystem, LockIdentifier, Nothing, OnUnbalanced, Randomness,
		SortedMembers, WithdrawReasons, EitherOfDiverse,
	},
	dispatch::DispatchClass,
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
		IdentityFee, Weight,
	},
	PalletId, StorageValue,
};
use sp_runtime::RuntimeDebug;
use sp_staking::currency_to_vote::U128CurrencyToVote;
pub use frame_system::{ensure_root, EnsureRoot, RawOrigin};
use module_traits::{
	create_median_value_data_provider, parameter_type_with_key, DataFeeder, DataProviderExtended,
// MultiCurrency,
};
use module_currencies::BasicCurrencyAdapter;
use module_swap_legacy as swap_legacy_module;
use module_dex_oracle as dex_oracle;
use module_transaction_payment::TargetedFeeAdjustment;
pub use primitives::Multiplier;

// re-exports

pub use pallet_staking::StakerStatus;

pub use authority::AuthorityConfigImpl;
pub use constants::{fee::*, time::*};
pub use primitives::{
	AccountId, AccountIndex, Amount, AuthoritysOriginId, Balance, BlockNumber, CurrencyId,
	DataProviderId, EraIndex, Hash, Moment, Nonce, ReserveIdentifier, Share, Signature, TokenSymbol, TradingPair,
};
pub use primitives::currency::{SEU, SEUSD};
// use module_support::Web3SettersClubAccounts;
pub use runtime_common::{
	BlockLength, BlockWeights, GasToWeight, OffchainSolutionWeightLimit,
	Price, Rate, Ratio, ExchangeRate, TimeStampedPrice,
	cent, dollar, microcent, millicent, nanocent, ProxyType,

	EnsureRootOrOneShuraCouncil, EnsureRootOrAllShuraCouncil, EnsureRootOrHalfShuraCouncil,
	EnsureRootOrOneThirdsShuraCouncil, EnsureRootOrTwoThirdsShuraCouncil,
	EnsureRootOrThreeFourthsShuraCouncil, ShuraCouncilInstance, ShuraCouncilMembershipInstance,

	EnsureRootOrAllFinancialCouncil, EnsureRootOrHalfFinancialCouncil,
	EnsureRootOrOneThirdsFinancialCouncil, EnsureRootOrTwoThirdsFinancialCouncil,
	EnsureRootOrThreeFourthsFinancialCouncil, FinancialCouncilInstance, FinancialCouncilMembershipInstance,

	EnsureRootOrAllTechnicalCommittee, EnsureRootOrHalfTechnicalCommittee,
	EnsureRootOrOneThirdsTechnicalCommittee, EnsureRootOrTwoThirdsTechnicalCommittee,
	EnsureRootOrThreeFourthsTechnicalCommittee, TechnicalCommitteeInstance, TechnicalCommitteeMembershipInstance,

	OperatorMembershipInstanceSetheum,
};

/// Milliseconds per block (Set-BFT).
pub const MILLISECS_PER_BLOCK: u64 = primitives::setbft::MILLISECS_PER_BLOCK;
/// Slot duration.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
/// Session (epoch) duration in slots.
pub const EPOCH_DURATION_IN_SLOTS: u32 = primitives::setbft::DEFAULT_SESSION_PERIOD;


mod weights;
mod authority;
pub mod constants;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// Pallet accounts of runtime
parameter_types! {
	pub const AirdropPalletId: PalletId = PalletId(*b"set/drop");		// 5EYCAe5jKgkuY1B3CkWQF41wzN62tTt8ptfmao31qYvMiVRD
	pub const CDPTreasuryPalletId: PalletId = PalletId(*b"set/cdpt");	// 5EYCAe5jKgkuXyJQ3G8CXrRfmmqqe54Tye5wJDqim8cvHQi7
	pub const DEXPalletId: PalletId = PalletId(*b"edf/swap");			// 5EYCAe5jKgkuYTiXRpXnghiur9sW2zJCp91xQRKKzhwjS2DC
	pub const LoansPalletId: PalletId = PalletId(*b"set/loan");			// 5EYCAe5jKgkuYFMt7CDpD9JGyD8eLr9DKZZ9mBNibUbs5xXo
	pub const NftPalletId: PalletId = PalletId(*b"set/sNFT");			// 5EYCAe5jKgkuYTZd9to8S5wCPjCUQnDg57tU9BDgakrywBM2
	pub const SerpTreasuryPalletId: PalletId = PalletId(*b"set/serp");	// 5EYCAe5jKgkuYTiwwziYLaTt4ZTSEikGfWNVyZ1PUdkBg78Z
	pub const TreasuryPalletId: PalletId = PalletId(*b"set/trsry");		// 5EYCAe5jKgkuYVbBxj3Gqkgew54j9TmR4Q8QLuBWHCApVqWn
	pub const TransactionPaymentPalletId: PalletId = PalletId(*b"set/tx\0\0");
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![
		AirdropPalletId::get().into_account_truncating(),
		CDPTreasuryPalletId::get().into_account_truncating(),
		DEXPalletId::get().into_account_truncating(),
		LoansPalletId::get().into_account_truncating(),
		SerpTreasuryPalletId::get().into_account_truncating(),
		TreasuryPalletId::get().into_account_truncating(),
		ZeroAccountId::get(),		 	// ACCOUNT 0
	]
}

parameter_types! {
	pub Web3SettersClubAccounts: Vec<AccountId> = vec![
// hex_literal::hex!("608fbd3f7ec6a45fb6d5b2967f54da4713c21d75efcc715544e091fa63c1fd0e").into(),	// VQho4edpR5upbDZUSt1JP6TR8oQkBrPSHz1XChMFqyHawRab1
// hex_literal::hex!("3c5dca516188b2ac077e33a886ac1ea2c03d2a157f56b70ca182c9f7fe5f9055").into(),	// VQgyc63yJgmrhrsDfH73ipq6TfEyiPMNQ3QYK3a82Sskb3mFx
// hex_literal::hex!("2e70349d7140ec49b7cf1ae03b6ae3405103dab86c5a463ceef77ffb4a769868").into(),	// VQgfLtTS8oZCreyX3FzHuaAbUovtbcuSFLnUFS3tkRvwWGkbD
// hex_literal::hex!("22b565e2303579c0d50884a3524c32ed12c8b91a8621dd72270b8fd17d20d009").into(),	// VQgPxsHbvGdXC7HhUvYvPifu1SyAuRnUhbMw4hAaTm9fwvkkz
// hex_literal::hex!("78d105e22be9735d200591ebe506fbc0d0be3f18afa5f5b2fbdb370ee4c2fd47").into(),	// VQiLsC6xs5xSG7jFUbcRCjKPZqnacJmrNANovRHzbtgThHzhy
		TreasuryPalletId::get().into_account_truncating(),
	];
}

pub struct EnsureWeb3SettersClub;
impl EnsureOrigin<RuntimeOrigin> for EnsureWeb3SettersClub {
	type Success = AccountId;

	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		Into::<Result<RawOrigin<AccountId>, RuntimeOrigin>>::into(o).and_then(|o| match o {
			RawOrigin::Signed(caller) => {
				if Web3SettersClubAccounts::get().contains(&caller) {
					Ok(caller)
				} else {
					Err(RuntimeOrigin::signed(caller))
				}
			}
			r => Err(RuntimeOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(RuntimeOrigin::signed(AccountId::new([0u8; 32])))
	}
}

// /// Ensures for SetheumFoundation collectives: 
// // Shura Council
// pub type EnsureRootOrOneShuraCouncil = EnsureOneOf<
// AccountId, EnsureWeb3SettersClub, pallet_collective::EnsureMember<AccountId, ShuraCouncilInstance>>;

// pub type EnsureRootOrTwoThirdsShuraCouncil = EnsureOneOf<
// 	AccountId,
// 	EnsureWeb3SettersClub,
// 	pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, ShuraCouncilInstance>,
// >;

// pub type EnsureRootOrThreeFourthsShuraCouncil = EnsureOneOf<
// 	AccountId,
// 	EnsureWeb3SettersClub,
// 	pallet_collective::EnsureProportionAtLeast<_3, _4, AccountId, ShuraCouncilInstance>,
// >;

// pub type EnsureRootOrTwoThirdsFinancialCouncil = EnsureOneOf<
// 	AccountId,
// 	EnsureWeb3SettersClub,
// 	pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, FinancialCouncilInstance>,
// >;

// pub type EnsureRootOrTwoThirdsTechnicalCommittee = EnsureOneOf<
// 	AccountId,
// 	EnsureWeb3SettersClub,
// 	pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalCommitteeInstance>,
// >;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub setbft: SetBFT,
			pub im_online: ImOnline,
			pub authority_discovery: AuthorityDiscovery,
		}
	}
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("setheum"),
	impl_name: create_runtime_str!("setheum"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

/// Aura slot duration configuration.
parameter_types! {
	pub const MaxAuthorities: u32 = 100_000;
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 4800; // 4hrs
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = (Indices);
/// The nonce type.
	type Nonce = Nonce;
/// The block type.
	type Block = Block;
/// The type for hashing blocks and tries.
	type Hash = Hash;
/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
/// The aggregated dispatch type.
	type RuntimeCall = RuntimeCall;
/// The aggregated task type.
	type RuntimeTask = RuntimeTask;
/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
/// Maximum weight of each block.
	type DbWeight = RocksDbWeight;
/// Version of the runtime.
	type Version = Version;
/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
/// What to do if a new account is created.
	type OnNewAccount = ();
/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
/// This is a hook that is use when setCode is called - not require unless using cumulus.
	type OnSetCode = ();
/// The maximum number of consumers.
	type MaxConsumers = frame_support::traits::ConstU32<16>;
/// Single block migrations.
	type SingleBlockMigrations = ();
/// Multi block migrator.
	type MultiBlockMigrator = ();
/// Hooks before inherents.
	type PreInherents = ();
/// Hooks after inherents.
	type PostInherents = ();
/// Hooks after transactions.
	type PostTransactions = ();
/// Weight information for system extensions.
	type ExtensionsWeightInfo = ();
}


parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = sp_runtime::traits::ConvertInto;
	type ShouldEndSession = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type SessionManager = SetBFT;
	type SessionHandler = (Aura, SetBFT);
	type Keys = opaque::SessionKeys;
	type DisablingStrategy = pallet_session::disabling::UpToLimitWithReEnablingDisablingStrategy;
	type WeightInfo = ();
}

parameter_types! {
	pub const SessionPeriod: u32 = EPOCH_DURATION_IN_SLOTS as u32;
	pub const Offset: u32 = 0;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * SessionPeriod::get() as u64;
}

impl pallet_session::historical::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}


pallet_staking_reward_curve::build! {
// 2.58% min, 25.8% max, 50% ideal stake
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_800,
		max_inflation: 0_258_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_500,
	);
}

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 2; // 2 hours (20 mins in test)
	pub const BondingDuration: primitives::EraIndex = 4; // 8 hours (80 mins in test)
	pub const SlashDeferDuration: primitives::EraIndex = 2; // 4 hours (40 mins in test)
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 64;
	pub const HistoryDepth: u32 = 84;
	pub const MaxControllersInDeprecationBatch: u32 = 512;
}

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = frame_support::traits::ConstU32<5000>;
	type MaxValidators = frame_support::traits::ConstU32<1000>;
}

parameter_types! {
	pub const MaxElectingVotersSolution: u32 = 1000;
	pub ElectionBoundsOnChain: frame_election_provider_support::bounds::ElectionBounds =
		frame_election_provider_support::bounds::ElectionBoundsBuilder::default()
			.voters_count(frame_election_provider_support::bounds::CountBound(500))
			.targets_count(frame_election_provider_support::bounds::CountBound(200))
			.build();
}

pub type OnChainSeqPhragmen = onchain::OnChainExecution<OnChainSeqPhragmenConfig>;

pub struct OnChainSeqPhragmenConfig;
impl onchain::Config for OnChainSeqPhragmenConfig {
	type System = Runtime;
	type Solver = frame_election_provider_support::SequentialPhragmen<AccountId, sp_runtime::Perbill>;
	type DataProvider = Staking;
	type WeightInfo = ();
	type MaxBackersPerWinner = MaxElectingVotersSolution;
	type MaxWinnersPerPage = frame_support::traits::ConstU32<100>;
	type Sort = frame_support::traits::ConstBool<true>;
	type Bounds = ElectionBoundsOnChain;
}

impl pallet_staking::Config for Runtime {
	type OldCurrency = Balances;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type RewardRemainder = frame_support::traits::tokens::imbalance::ResolveTo<TreasuryAccount, Balances>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type Slash = frame_support::traits::tokens::imbalance::ResolveTo<TreasuryAccount, Balances>;
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type AdminOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type SessionInterface = Self;
	type NextNewSession = Session;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type MaxExposurePageSize = frame_support::traits::ConstU32<256>;
	type ElectionProvider = OnChainSeqPhragmen;
	type GenesisElectionProvider = OnChainSeqPhragmen;
	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Self>;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type NominationsQuota = pallet_staking::FixedNominationsQuota<16>;
	type MaxUnlockingChunks = frame_support::traits::ConstU32<32>;
	type MaxControllersInDeprecationBatch = MaxControllersInDeprecationBatch;
	type HistoryDepth = HistoryDepth;
	type EventListeners = ();
	type WeightInfo = ();
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type Filter = frame_support::traits::Nothing;
	type MaxValidatorSet = frame_support::traits::ConstU32<1000>;
}


impl pallet_aura::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type AllowMultipleBlocksPerSlot = frame_support::traits::ConstBool<false>;
	type SlotDuration = frame_support::traits::ConstU64<SLOT_DURATION>;
}

// SetBFT pallet replaces Grandpa as the finality gadget

pub struct SessionInfoImpl;
impl primitives::setbft::SessionInfoProvider<BlockNumber> for SessionInfoImpl {
	fn current_session() -> primitives::setbft::SessionIndex {
		pallet_session::Pallet::<Runtime>::current_index()
	}
	fn next_session_block_number(current_block: BlockNumber) -> Option<BlockNumber> {
		let session_period = SessionPeriod::get();
		let current_session = pallet_session::Pallet::<Runtime>::current_index();
		let next_session = current_session + 1;
		let current_session_start = next_session * session_period;
		if current_block < current_session_start {
			Some(current_session_start)
		} else {
			Some(current_session_start + session_period)
		}
	}
}

impl module_setbft::Config for Runtime {
	type AuthorityId = primitives::AuthorityId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInfoProvider = SessionInfoImpl;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type NextSessionAuthorityProvider = module_traits::SessionNextSessionAuthorityProvider<Runtime>;
}

impl pallet_sheyth_vm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (Staking, ImOnline);
}

impl pallet_offences::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorSet = Historical;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = ();
	type MaxKeys = frame_support::traits::ConstU32<10_000>;
	type MaxPeerInHeartbeats = frame_support::traits::ConstU32<10_000>;
}

parameter_types! {
	pub BasicDeposit: Balance =      10 * dollar(SEU);
	pub ByteDeposit: Balance =        1 * dollar(SEU);
	pub UsernameDeposit: Balance =    5 * dollar(SEU);
	pub SubAccountDeposit: Balance =  20 * dollar(SEU);
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 19;
}

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type ByteDeposit = ByteDeposit;
	type UsernameDeposit = UsernameDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = ();
	type ForceOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type RegistrarOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as sp_runtime::traits::Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<Self::AccountId>;
	type PendingUsernameExpiration = frame_support::traits::ConstU32<{ 7 * DAYS }>;
	type UsernameGracePeriod = frame_support::traits::ConstU32<{ 30 * DAYS }>;
	type MaxSuffixLength = frame_support::traits::ConstU32<7>;
	type MaxUsernameLength = frame_support::traits::ConstU32<32>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type WeightInfo = ();
}


parameter_types! {
	pub IndexDeposit: Balance = 1 * dollar(SEU);
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type WeightInfo = ();
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SEU;
	pub const GetSetUSDId: CurrencyId = SEUSD;
	pub StableCurrencyIds: Vec<CurrencyId> = vec![
		SEUSD,
	];
}

impl module_currencies::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = weights::module_currencies::WeightInfo<Runtime>;
	type SweepOrigin = EnsureRootOrOneShuraCouncil;
	type OnDust = module_currencies::TransferDust<Runtime, TreasuryAccount>;
}

parameter_types! {
	pub const MinimumCount: u32 = 1;
	pub const ExpiresIn: Moment = 1000 * 60 * 60; // 60 mins
	pub ZeroAccountId: AccountId = AccountId::from([0u8; 32]);
	pub const MaxHasDispatchedSize: u32 = 40;
}

type SetheumDataProvider = module_oracle::Instance1;
impl module_oracle::Config<SetheumDataProvider> for Runtime {
	type OnNewData = ();
	type CombineData = module_oracle::DefaultCombineData<Runtime, MinimumCount, ExpiresIn, SetheumDataProvider>;
	type Time = Timestamp;
	type OracleKey = CurrencyId;
	type OracleValue = Price;
	type RootOperatorAccountId = ZeroAccountId;
	type Members = OperatorMembershipSetheum;
	type MaxHasDispatchedSize = MaxHasDispatchedSize;
	type MaxFeedValues = frame_support::traits::ConstU32<5>;
	type WeightInfo = weights::module_oracle::WeightInfo<Runtime>;
}

create_median_value_data_provider!(
	AggregatedDataProvider,
	CurrencyId,
	Price,
	TimeStampedPrice,
	[SetheumOracle]
);
// Aggregated data provider cannot feed.
impl DataFeeder<CurrencyId, Price, AccountId> for AggregatedDataProvider {
	fn feed_value(_: Option<AccountId>, _: CurrencyId, _: Price) -> DispatchResult {
		Err("Not supported".into())
	}
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		get_all_module_accounts().contains(a)
	}
}

parameter_type_with_key! {
	pub GetStableCurrencyMinimumSupply: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			&SEUSD => 1_000_000_000 * dollar(SEUSD),
			_ => 0,
		}
	};
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			CurrencyId::Token(symbol) => match symbol {
				TokenSymbol::SEUSD => 10 * cent(SEUSD), // 10 cents (0.1)
				TokenSymbol::SEU => 10 * cent(SEU), // 10 cents (0.1)
			},
			CurrencyId::DexShare(dex_share_0, _) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();

// initial dex share amount is calculated based on currency_id_0,
// use the ED of currency_id_0 as the ED of lp token.
				if currency_id_0 == GetNativeCurrencyId::get() {
					NativeTokenExistentialDeposit::get()
				} else if let CurrencyId::Erc20(_) = currency_id_0 {
// LP token with erc20
					1
				} else {
					Self::get(&currency_id_0)
				}
			},
			CurrencyId::Erc20(_) => Balance::max_value(), // not handled by module-tokens
			CurrencyId::ForeignAsset(_) => Balance::max_value(), // not handled by module-tokens
		}
	};
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
	pub CDPTreasuryAccount: AccountId = CDPTreasuryPalletId::get().into_account_truncating();
// pub SerpTreasuryAccount: AccountId = SerpTreasuryPalletId::get().into_account_truncating();
}

pub struct CurrencyHooks<T>(sp_std::marker::PhantomData<T>);
impl<T: module_tokens::Config> module_traits::currency::MutationHooks<T::AccountId, T::CurrencyId, T::Balance> for CurrencyHooks<T>
where
	T::AccountId: From<AccountId>,
{
	type OnDust = module_tokens::TransferDust<T, TreasuryAccount>;
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl module_tokens::Config for Runtime {
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::module_tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = ExistentialDeposits;
	type CurrencyHooks = CurrencyHooks<Runtime>;
	type MaxLocks = MaxLocks;
	type MaxReserves = frame_support::traits::ConstU32<50>;
	type ReserveIdentifier = ReserveIdentifier;
	type DustRemovalWhitelist = DustRemovalWhitelist;
}

parameter_types! {
	pub SetUSDFixedPrice: Price = Price::saturating_from_rational(1, 1); // $1
	pub SetterFixedPrice: Price = Price::saturating_from_rational(1, 4); // $0.25
}

impl module_prices::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Source = AggregatedDataProvider;
	type SEUSDFixedPrice = SetUSDFixedPrice;
	type GetSEUSDCurrencyId = GetSetUSDId;
	type GetSEECurrencyId = GetNativeCurrencyId;
	type LockOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type SwapManager = Dex;
	type Currency = Currencies;
	type PricingPegged = PricingPegged;
	type WeightInfo = weights::module_prices::WeightInfo<Runtime>;
}

/// No currency is pegged to another for pricing.
pub struct PricingPegged;
impl module_traits::GetByKey<CurrencyId, Option<CurrencyId>> for PricingPegged {
	fn get(_key: &CurrencyId) -> Option<CurrencyId> {
		None
	}
}

// impl dex_oracle::Config for Runtime {
// 	type DEX = Dex;
// 	type Time = Timestamp;
// 	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
// 	type WeightInfo = weights::dex_oracle::WeightInfo<Runtime>;
// }

impl module_transaction_pause::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UpdateOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type WeightInfo = weights::module_transaction_pause::WeightInfo<Runtime>;
}

parameter_types! {
	pub MinimumIncrementSize: Rate = Rate::saturating_from_rational(2, 100); // 2%
	pub const AuctionTimeToClose: BlockNumber = 15 * MINUTES;
	pub const AuctionDurationSoftCap: BlockNumber = 2 * HOURS;
}

// impl auction_manager::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Currencies;
// 	type Auction = Auction;
// 	type MinimumIncrementSize = MinimumIncrementSize;
// 	type AuctionTimeToClose = AuctionTimeToClose;
// 	type AuctionDurationSoftCap = AuctionDurationSoftCap;
// 	type GetSetUSDId = GetSetUSDId;
// 	type CDPTreasury = CdpTreasury;
// 	type DEX = Dex;
// 	type PriceSource = module_prices::PriorityLockedPriceProvider<Runtime>;
// 	type UnsignedPriority = runtime_common::AuctionManagerUnsignedPriority;
// 	type EmergencyShutdown = EmergencyShutdown;
// 	type WeightInfo = weights::module_auction_manager::WeightInfo<Runtime>;
// }

// impl module_loans::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Currencies;
// 	type RiskManager = CdpEngine;
// 	type CDPTreasury = CdpTreasury;
// 	type PalletId = LoansPalletId;
// }

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_signed_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: Nonce,
	) -> Option<UncheckedExtrinsic> {
// take the biggest period possible.
		let period = BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
// The `System::block_number` is initialized with `n+1`,
// so the actual block number is `n`.
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			sp_runtime::traits::transaction_extension::AsTransactionExtension(
				module_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip)
			),
			);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = Indices::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some(UncheckedExtrinsic::new_signed(call, address, signature, extra))
	}
}

impl<LocalCall> frame_system::offchain::CreateBare<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_bare(call: RuntimeCall) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_bare(call)
	}
}

impl<C> frame_system::offchain::CreateTransactionBase<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	pub AlternativeSwapPathJointList: Vec<Vec<CurrencyId>> = vec![
		vec![SEU],
		vec![SEU, SEUSD],
	];
	pub CollateralCurrencyIds: Vec<CurrencyId> = vec![SEU];
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(110, 100);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::saturating_from_rational(1, 10);
	pub DefaultLiquidationPenalty: Rate = Rate::saturating_from_rational(5, 100);
	pub MinimumDebitValue: Balance = 10 * dollar(SEUSD);
	pub MaxSwapSlippageComparedToOracle: Ratio = Ratio::saturating_from_rational(15, 100);
}

// impl cdp_engine::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type PriceSource = module_prices::PriorityLockedPriceProvider<Runtime>;
// 	type CollateralCurrencyIds = CollateralCurrencyIds;
// 	type DefaultLiquidationRatio = DefaultLiquidationRatio;
// 	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
// 	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
// 	type MinimumDebitValue = MinimumDebitValue;
// 	type GetSetUSDId = GetSetUSDId;
// 	type CDPTreasury = CdpTreasury;
// 	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
// 	type MaxSwapSlippageComparedToOracle = MaxSwapSlippageComparedToOracle;
// 	type UnsignedPriority = runtime_common::CdpEngineUnsignedPriority;
// 	type EmergencyShutdown = EmergencyShutdown;
// 	type Currency = Currencies;
// 	type AlternativeSwapPathJointList = AlternativeSwapPathJointList;
// 	type DEX = Dex;
// 	type WeightInfo = weights::module_cdp_engine::WeightInfo<Runtime>;
// }

parameter_types! {
	pub DepositPerAuthorization: Balance = deposit(1, 64);
}

// impl serp_setmint::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Balances;
// 	type DepositPerAuthorization = DepositPerAuthorization;
// 	type WeightInfo = weights::serp_setmint::WeightInfo<Runtime>;
// }

// impl emergency_shutdown::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type CollateralCurrencyIds = CollateralCurrencyIds;
// 	type PriceSource = Prices;
// 	type CDPTreasury = CdpTreasury;
// 	type AuctionManagerHandler = AuctionManager;
// 	type ShutdownOrigin = EnsureRootOrHalfShuraCouncil;
// 	type WeightInfo = weights::emergency_shutdown::WeightInfo<Runtime>;
// }

parameter_types! {
	pub const GetExchangeFee: (u32, u32) = (3, 1000);	// 0.3%
	pub const GetStableCurrencyExchangeFee: (u32, u32) = (1, 1000);	// 0.1%
	pub const TradingPathLimit: u32 = 4;
	pub EnabledTradingPairs: Vec<TradingPair> = vec![
		TradingPair::from_currency_ids(SEUSD, SEU).unwrap(),
	];
}

impl swap_legacy_module::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = TradingPathLimit;
	type PalletId = DEXPalletId;
	type Erc20InfoMapping = CurrencyIdMappingImpl;
	type WeightInfo = weights::module_dex::WeightInfo<Runtime>;
	type Incentives = ();
	type ListingOrigin = EnsureRootOrHalfFinancialCouncil;
	type ExtendedProvisioningBlocks = ExtendedProvisioningBlocks;
	type OnLiquidityPoolUpdated = ();
}

parameter_types! {
	pub const ExtendedProvisioningBlocks: BlockNumber = 0;
}

/// CurrencyId metadata mapping. EVM address encoding is unsupported (EVM removed).
pub struct CurrencyIdMappingImpl;
impl module_support::CurrencyIdMapping for CurrencyIdMappingImpl {
	fn name(currency_id: CurrencyId) -> Option<Vec<u8>> {
		use primitives::currency::TokenInfo;
		currency_id.name().map(|v| v.as_bytes().to_vec())
	}

	fn symbol(currency_id: CurrencyId) -> Option<Vec<u8>> {
		use primitives::currency::TokenInfo;
		currency_id.symbol().map(|v| v.as_bytes().to_vec())
	}

	fn decimals(currency_id: CurrencyId) -> Option<u8> {
		use primitives::currency::TokenInfo;
		currency_id.decimals()
	}

	fn encode_evm_address(_currency_id: CurrencyId) -> Option<primitives::currency::EvmAddress> {
		None
	}

	fn decode_evm_address(_address: primitives::currency::EvmAddress) -> Option<CurrencyId> {
		None
	}
}

impl dex_oracle::Config for Runtime {
	type DEX = Dex;
	type Time = Timestamp;
	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
	type WeightInfo = ();
}

// parameter_types! {
// 	pub const MaxAirdropListSize: usize = 250;
// }

// impl module_airdrop::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type MultiCurrency = Currencies;
// 	type MaxAirdropListSize = MaxAirdropListSize;
// 	type FundingOrigin = TreasuryAccount;
// 	type DropOrigin = EnsureRootOrTwoThirdsShuraCouncil;
// 	type PalletId = AirdropPalletId;
// }

parameter_types! {
    pub const StableCurrencyInflationPeriod: BlockNumber = MINUTES;
    
	pub SetDollarMinimumClaimableTransferAmounts: Balance = 4 * dollar(SEUSD);
	pub SetDollarMaximumClaimableTransferAmounts: Balance = 100_000 * dollar(SEUSD);
}

// impl serp_treasury::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Currencies;
// 	type StableCurrencyIds = StableCurrencyIds;
// 	type StableCurrencyInflationPeriod = StableCurrencyInflationPeriod;
// 	type GetStableCurrencyMinimumSupply = GetStableCurrencyMinimumSupply;
// 	type GetNativeCurrencyId = GetNativeCurrencyId;
// 	type GetSerpCurrencyId = GetSerpCurrencyId;
// 	type GetDinarCurrencyId = GetDinarCurrencyId;
// 	type GetHelpCurrencyId = GetHelpCurrencyId;
// 	type SetterCurrencyId = SetterCurrencyId;
// 	type GetSetUSDId = GetSetUSDId;
// 	type CDPTreasuryAccountId = CDPTreasuryAccount;
// 	type Dex = Dex;
// 	type MaxSwapSlippageComparedToOracle = MaxSwapSlippageComparedToOracle;
// 	type PriceSource = module_prices::RealTimePriceProvider<Runtime>;
// 	type AlternativeSwapPathJointList = AlternativeSwapPathJointList;
// 	type SetterMinimumClaimableTransferAmounts = SetterMinimumClaimableTransferAmounts;
// 	type SetterMaximumClaimableTransferAmounts = SetterMaximumClaimableTransferAmounts;
// 	type SetDollarMinimumClaimableTransferAmounts = SetDollarMinimumClaimableTransferAmounts;
// 	type SetDollarMaximumClaimableTransferAmounts = SetDollarMaximumClaimableTransferAmounts;
// 	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
// 	type PalletId = SerpTreasuryPalletId;
// 	type WeightInfo = ();
// }

parameter_types! {
	pub const MaxAuctionsCount: u32 = 100;
}

// impl cdp_treasury::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Currency = Currencies;
// 	type GetSetUSDId = GetSetUSDId;
// 	type AuctionManagerHandler = AuctionManager;
// 	type DEX = Dex;
// 	type MaxAuctionsCount = MaxAuctionsCount;
// 	type PalletId = CDPTreasuryPalletId;
// 	type SerpTreasury = SerpTreasury;
// 	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
// 	type AlternativeSwapPathJointList = AlternativeSwapPathJointList;
// 	type WeightInfo = weights::module_cdp_treasury::WeightInfo<Runtime>;
// }

parameter_types! {
// Sort by fee charge order
	pub DefaultFeeSwapPathList: Vec<Vec<CurrencyId>> = vec![
		vec![SEUSD, SEU],
	];
}

type NegativeImbalance = <Balances as PalletCurrency<AccountId>>::NegativeImbalance;
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
	fn on_unbalanceds(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
		if let Some(fees) = fees_then_tips.next() {
// for fees, 50% to treasury, 50% burn
            let mut split = fees.ration(50, 50);
            if let Some(tips) = fees_then_tips.next() {
// for tips, if any, 50% to treasury, 50% burn (though this can be anything)
                tips.ration_merge_into(50, 50, &mut split);
            }
            Treasury::on_unbalanced(split.0);
        }
	}
}

parameter_types! {
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
	pub MaximumMultiplier: Multiplier = Multiplier::saturating_from_integer(10);
	pub const OperationalFeeMultiplier: u64 = 5;
	pub TipPerWeightStep: Balance = 10 * millicent(SEU);
	pub MaxTipsOfPriority: Balance = 100 * dollar(SEU);
	pub AlternativeFeeSwapDeposit: Balance = 2 * dollar(SEU);
	pub CustomFeeSurplus: Percent = Percent::from_percent(5);
	pub AlternativeFeeSurplus: Percent = Percent::from_percent(5);
	pub DefaultFeeTokens: Vec<CurrencyId> = vec![SEU];
}

impl module_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type NativeCurrencyId = GetNativeCurrencyId;
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type OnTransactionPayment = DealWithFees;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type TipPerWeightStep = TipPerWeightStep;
	type MaxTipsOfPriority = MaxTipsOfPriority;
	type AlternativeFeeSwapDeposit = AlternativeFeeSwapDeposit;
	type WeightToFee = WeightToFee;
	type LengthToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier, MaximumMultiplier>;
	type Swap = module_support::swap_legacy::SpecificJointsSwap<Dex, DefaultFeeSwapPathList>;
	type MaxSwapSlippageComparedToOracle = MaxSwapSlippageComparedToOracle;
	type TradingPathLimit = TradingPathLimit;
	type PriceSource = module_prices::RealTimePriceProvider<Runtime>;
	type WeightInfo = weights::module_transaction_payment::WeightInfo<Runtime>;
	type PalletId = TransactionPaymentPalletId;
	type TreasuryAccount = TreasuryAccount;
	type CustomFeeSurplus = CustomFeeSurplus;
	type AlternativeFeeSurplus = AlternativeFeeSurplus;
	type DefaultFeeTokens = DefaultFeeTokens;
	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
}

parameter_types! {
	pub CreateClassDeposit: Balance = 11 * dollar(SEU);
	pub CreateTokenDeposit: Balance = 7 * dollar(SEU);
	pub MaxAttributesBytes: u32 = 2048;
}

impl module_nft::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CreateClassDeposit = CreateClassDeposit;
	type CreateTokenDeposit = CreateTokenDeposit;
	type DataDepositPerByte = DataDepositPerByte;
	type PalletId = NftPalletId;
	type MaxAttributesBytes = MaxAttributesBytes;
	type WeightInfo = weights::module_nft::WeightInfo<Runtime>;
	type ClassId = u32;
	type TokenId = u64;
	type MaxClassMetadata = MaxClassMetadata;
	type MaxTokenMetadata = MaxTokenMetadata;
}

parameter_types! {
	pub MaxClassMetadata: u32 = 1024;
	pub MaxTokenMetadata: u32 = 1024;
}

parameter_types! {
// One storage item; key size 32, value size 8; .
	pub ProxyDepositBase: Balance = deposit(1, 8);
// Additional storage item size of 33 bytes.
	pub ProxyDepositFactor: Balance = deposit(0, 33);
	pub const MaxProxies: u16 = 32;
	pub AnnouncementDepositBase: Balance = deposit(1, 8);
	pub AnnouncementDepositFactor: Balance = deposit(0, 66);
	pub const MaxPending: u16 = 32;
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
// Always allowed RuntimeCall::Utility no matter type.
// Only transactions allowed by Proxy.filter can be executed,
// otherwise `BadOrigin` will be returned in RuntimeCall::Utility.
			_ if matches!(c, RuntimeCall::Utility(..)) => true,
			ProxyType::Any => true,
			ProxyType::CancelProxy => matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. })),
			ProxyType::Governance => {
				matches!(
					c,
					RuntimeCall::Authority(..)
						| RuntimeCall::ShuraCouncil(..)
						| RuntimeCall::FinancialCouncil(..)
						| RuntimeCall::TechnicalCommittee(..)
						| RuntimeCall::Treasury(..)
						| RuntimeCall::Bounties(..)
						| RuntimeCall::Tips(..)
				)
			}
			ProxyType::Auction => false,
			ProxyType::Swap => {
				matches!(
					c,
					RuntimeCall::Dex(swap_legacy_module::Call::swap_with_exact_supply { .. })
						| RuntimeCall::Dex(swap_legacy_module::Call::swap_with_exact_target { .. })
				)
			}
			ProxyType::Loan => false,
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type WeightInfo = ();
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

parameter_types! {
// note: if we add other native tokens (SEUSD) we have to set native
// existential deposit to 0 or check for other tokens on account pruning
	pub NativeTokenExistentialDeposit: Balance = 1 * dollar(SEU); // 1 SEU
	pub MaxNativeTokenExistentialDeposit: Balance = 100 * dollar(SEU); // 100 SEU
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = ReserveIdentifier::Count as u32;
}

impl pallet_balances::Config for Runtime {
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = ();
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = ReserveIdentifier;
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = NativeTokenExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = frame_support::traits::ConstU32<0>;
	type DoneSlashHandler = ();
}

parameter_types! {
	pub MinVestedTransfer: Balance = 0;
	pub const MaxNativeVestingSchedules: u32 = 70;
	pub const MaxSetUSDVestingSchedules: u32 = 70;
}

impl module_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MultiCurrency = Currencies;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type GetSetUSDId = GetSetUSDId;
	type MinVestedTransfer = MinVestedTransfer;
	type TreasuryAccount = TreasuryAccount;
	type UpdateOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type MaxNativeVestingSchedules = MaxNativeVestingSchedules;
	type MaxSetUSDVestingSchedules = MaxSetUSDVestingSchedules;
	type WeightInfo = weights::module_vesting::WeightInfo<Runtime>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(10) * BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * BlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = ();
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type Preimages = ();
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

impl module_authority::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type Scheduler = Scheduler;
	type AsOriginId = AuthoritysOriginId;
	type AuthorityConfig = AuthorityConfigImpl;
	type WeightInfo = ();
}


impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
}

parameter_types! {
	pub const ShuraCouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const ShuraCouncilMaxProposals: u32 = 100;
	pub const ShuraCouncilMaxMembers: u32 = 100;
}

impl pallet_collective::Config<ShuraCouncilInstance> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = ShuraCouncilMotionDuration;
	type MaxProposals = ShuraCouncilMaxProposals;
	type MaxMembers = ShuraCouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type KillOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type Consideration = ();
}

impl pallet_membership::Config<ShuraCouncilMembershipInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type RemoveOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type SwapOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type ResetOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type PrimeOrigin = EnsureRootOrThreeFourthsShuraCouncil;
	type MembershipInitialized = ShuraCouncil;
	type MembershipChanged = ShuraCouncil;
	type MaxMembers = ShuraCouncilMaxMembers;
	type WeightInfo = ();
}

parameter_types! {
	pub const FinancialCouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const FinancialCouncilMaxProposals: u32 = 100;
	pub const FinancialCouncilMaxMembers: u32 = 100;
}

impl pallet_collective::Config<FinancialCouncilInstance> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = FinancialCouncilMotionDuration;
	type MaxProposals = FinancialCouncilMaxProposals;
	type MaxMembers = FinancialCouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type KillOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type Consideration = ();
}

impl pallet_membership::Config<FinancialCouncilMembershipInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type MembershipInitialized = FinancialCouncil;
	type MembershipChanged = FinancialCouncil;
	type MaxMembers = FinancialCouncilMaxMembers;
	type WeightInfo = ();
}

parameter_types! {
	pub const TechnicalCommitteeMotionDuration: BlockNumber = 7 * DAYS;
	pub const TechnicalCommitteeMaxProposals: u32 = 100;
	pub const TechnicalCouncilMaxMembers: u32 = 100;
}

impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = TechnicalCommitteeMotionDuration;
	type MaxProposals = TechnicalCommitteeMaxProposals;
	type MaxMembers = TechnicalCouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type KillOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type Consideration = ();
}

impl pallet_membership::Config<TechnicalCommitteeMembershipInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsShuraCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = TechnicalCouncilMaxMembers;
	type WeightInfo = ();
}

parameter_types! {
	pub const OracleMaxMembers: u32 = 100;
}

impl pallet_membership::Config<OperatorMembershipInstanceSetheum> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsFinancialCouncil;
	type MembershipInitialized = ();
	type MembershipChanged = SetheumOracle;
	type MaxMembers = OracleMaxMembers;
	type WeightInfo = ();
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub MultisigDepositBase: Balance = 500_000_000_000_000_000; // 500 millicents
	pub MultisigDepositFactor: Balance = 100_000_000_000_000_000; // 100 millicents
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = MultisigDepositBase;
	type DepositFactor = MultisigDepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = ();
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}

pub struct ShuraCouncilProvider;
impl SortedMembers<AccountId> for ShuraCouncilProvider {
	fn sorted_members() -> Vec<AccountId> {
		pallet_collective::Members::<Runtime, ShuraCouncilInstance>::get()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(_: &AccountId) {
		todo!()
	}
}

impl ContainsLengthBound for ShuraCouncilProvider {
	fn max_len() -> usize {
		100
	}
	fn min_len() -> usize {
		0
	}
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(3);
	pub ProposalBondMinimum: Balance = 1 * dollar(SEU); // 1 SEU
	pub const SpendPeriod: BlockNumber = 40 * DAYS;
	pub const Burn: Permill = Permill::from_perthousand(0); // 0.0%
	pub const MaxApprovals: u32 = 100;
	pub MaxBalance: Balance = Balance::max_value();
	pub const SpendPayoutPeriod: BlockNumber = 7 * DAYS;

	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(10);
	pub TipReportDepositBase: Balance = deposit(1, 0);
	pub const SevenDays: BlockNumber = 7 * DAYS;
	pub const ZeroDay: BlockNumber = 0;
	pub const OneDay: BlockNumber = DAYS;
	pub BountyDepositBase: Balance = deposit(1, 0);
	pub const BountyDepositPayoutDelay: BlockNumber = DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 21 * DAYS;
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub CuratorDepositMin: Option<Balance> = Some(1 * dollar(SEU));
	pub CuratorDepositMax: Option<Balance> = Some(100 * dollar(SEU));
	pub BountyValueMinimum: Balance = 1 * dollar(SEU); // 1 SEU
	pub DataDepositPerByte: Balance = deposit(0, 1);
	pub const MaximumReasonLength: u32 = 16384;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type RejectOrigin = EnsureRootOrHalfShuraCouncil;
	type RuntimeEvent = RuntimeEvent;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = ();
	type MaxApprovals = MaxApprovals;
	type SpendOrigin = frame_system::EnsureWithSuccess<EnsureRootOrHalfShuraCouncil, AccountId, MaxBalance>;
	type AssetKind = ();
	type Beneficiary = AccountId;
	type BeneficiaryLookup = Indices;
	type Paymaster = frame_support::traits::tokens::pay::PayFromAccount<Balances, TreasuryAccount>;
	type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
	type PayoutPeriod = SpendPayoutPeriod;
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = ();
	type ChildBountyManager = ();
	type OnSlash = Treasury;
}

impl pallet_tips::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = ShuraCouncilProvider;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type MaxTipAmount = frame_support::traits::ConstU128<{ 500 * 1_000_000_000_000_000_000 }>;
	type WeightInfo = ();
	type OnSlash = Treasury;
}

parameter_types! {
	pub ConfigDepositBase: Balance = 100_000_000_000_000; // 10 millicents
	pub FriendDepositFactor: Balance = 10_000_000_000_000_000; // 1 cent
	pub const MaxFriends: u16 = 9;
	pub RecoveryDeposit: Balance = 100_000_000_000_000_000; // 10 cent
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
	type WeightInfo = ();
}

// impl module_auction::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type Balance = Balance;
// 	type AuctionId = AuctionId;
// 	type Handler = AuctionManager;
// 	type WeightInfo = weights::module_auction::WeightInfo<Runtime>;
// }

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

// Create the runtime by composing the FRAME pallets that were previously configured.

// workaround for a weird bug in macro
use pallet_session::historical as pallet_session_historical;

// TODO: Implementation of `From` is preferred since it gives you `Into<_>` for free where the reverse isn't true.
// After this TODO will be resolved, remove the suppresion of `from-over-into` warnings in the Makefile.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
// Core
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>} = 0,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip::{Pallet, Storage} = 1,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 2,
		Sudo: pallet_sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 3,
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 4,
		Prices: module_prices::{Pallet, Storage, Call, Event<T>} = 5,
		Dex: swap_legacy_module::{Pallet, Storage, Call, Event<T>, Config<T>} = 6,

		Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 7,
		Recovery: pallet_recovery::{Pallet, Call, Storage, Event<T>} = 8,
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 9,

// MODULE Core
// Auction: module_auction::{Pallet, Storage, Call, Event<T>} = 10,
		

// Governance
		ShuraCouncil: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 12,
		ShuraCouncilMembership: pallet_membership::<Instance1>::{Pallet, Call, Storage, Event<T>, Config<T>} = 13,
		FinancialCouncil: pallet_collective::<Instance2>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 14,
		FinancialCouncilMembership: pallet_membership::<Instance2>::{Pallet, Call, Storage, Event<T>, Config<T>} = 15,
		TechnicalCommittee: pallet_collective::<Instance3>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 16,
		TechnicalCommitteeMembership: pallet_membership::<Instance3>::{Pallet, Call, Storage, Event<T>, Config<T>} = 17,

		Authority: module_authority::{Pallet, Call, Storage, Event<T>, Origin<T>} = 18,

		Utility: pallet_utility::{Pallet, Call, Event} = 19,

// Oracle
//
// NOTE: OperatorMembership must be placed after Oracle or else will have race condition on initialization
		DexOracle: dex_oracle::{Pallet, Storage, Call} = 20,
		SetheumOracle: module_oracle::<Instance1>::{Pallet, Storage, Call, Event<T>} = 21,
		OperatorMembershipSetheum: pallet_membership::<Instance4>::{Pallet, Call, Storage, Event<T>, Config<T>} = 22,

// AuctionManager: auction_manager::{Pallet, Storage, Call, Event<T>, ValidateUnsigned} = 23,
// Loans: module_loans::{Pallet, Storage, Call, Event<T>} = 24,
// Setmint: serp_setmint::{Pallet, Storage, Call, Event<T>} = 25,
// SerpTreasury: serp_treasury::{Pallet, Storage, Call, Config, Event<T>} = 26,
// CdpTreasury: cdp_treasury::{Pallet, Storage, Call, Config, Event<T>} = 27,
// CdpEngine: cdp_engine::{Pallet, Storage, Call, Event<T>, Config, ValidateUnsigned} = 28,
// EmergencyShutdown: emergency_shutdown::{Pallet, Storage, Call, Event<T>} = 29,

// Treasury
		Treasury: pallet_treasury::{Pallet, Call, Storage, Config<T>, Event<T>} = 30,
// Bounties
		Bounties: pallet_bounties::{Pallet, Call, Storage, Event<T>} = 31,
// Tips
		Tips: pallet_tips::{Pallet, Call, Storage, Event<T>} = 32,

// Extras
		NFT: module_nft::{Pallet, Storage, Call, Event<T>, Config<T>} = 33,
// AirDrop: module_airdrop::{Pallet, Call, Storage, Event<T>} = 34,

// Account lookup
		Indices: pallet_indices::{Pallet, Call, Storage, Config<T>, Event<T>} = 35,

// Tokens, Fees & Related
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 36,
		Currencies: module_currencies::{Pallet, Call, Event<T>} = 37,
		Tokens: module_tokens::{Pallet, Storage, Event<T>, Config<T>} = 38,
		TransactionPayment: module_transaction_payment::{Pallet, Call, Storage, Event<T>} = 39,
		TransactionPause: module_transaction_pause::{Pallet, Call, Storage, Event<T>} = 40,
		Vesting: module_vesting::{Pallet, Storage, Call, Event<T>, Config<T>} = 41,

// Identity
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 42,

// Smart contracts

// Consensus - Aura + SetBFT (replacing Babe + Grandpa)
		Authorship: pallet_authorship::{Pallet, Storage} = 47,
		Aura: pallet_aura::{Pallet, Config<T>, Storage} = 48,
		SetBFT: module_setbft::{Pallet, Call, Config<T>, Storage, Event<T>} = 49,
		Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>, HoldReason} = 50,
		Session: pallet_session::{Pallet, Call, Storage, Event<T>, Config<T>} = 51,
		Historical: pallet_session_historical::{Pallet, Event<T>} = 52,
		Offences: pallet_offences::{Pallet, Storage, Event} = 53,
		ImOnline: pallet_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>} = 54,
		AuthorityDiscovery: pallet_authority_discovery::{Pallet, Config<T>} = 55,

		// SetBFT consensus modules
		// Elections: module_elections::{Pallet, Call, Storage, Event<T>} = 56,
		// CommitteeManagement: module_committee_management::{Pallet, Call, Storage, Event<T>} = 57,
		// Operations: module_operations::{Pallet, Call, Storage, Event<T>} = 58,
		SheythVM: pallet_sheyth_vm::{Pallet, Storage, Event<T>} = 59,
	}
);

pub struct OnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for OnRuntimeUpgrade {
	fn on_runtime_upgrade() -> Weight {
// no migration
		Weight::zero()
	}
}


/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, AccountIndex>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	sp_runtime::traits::transaction_extension::AsTransactionExtension<
		module_transaction_payment::ChargeTransactionPayment<Runtime>
	>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	OnRuntimeUpgrade,
>;

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			pallet_aura::Authorities::<Runtime>::get().to_vec()
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl primitives::SetBFTSessionApi<Block> for Runtime {
		fn millisecs_per_block() -> u64 {
			primitives::setbft::MILLISECS_PER_BLOCK
		}

		fn session_period() -> u32 {
			primitives::setbft::DEFAULT_SESSION_PERIOD
		}

		fn authorities() -> Vec<primitives::setbft::AuthorityId> {
			SetBFT::authorities()
		}

		fn next_session_authorities() -> Result<Vec<primitives::setbft::AuthorityId>, primitives::setbft::ApiError> {
			let next_authorities = SetBFT::next_authorities();
			if next_authorities.is_empty() {
				return Err(primitives::setbft::ApiError::DecodeKey);
			}
			Ok(next_authorities)
		}

		fn authority_data() -> primitives::setbft::SessionAuthorityData {
			primitives::setbft::SessionAuthorityData::new(SetBFT::authorities(), SetBFT::emergency_finalizer())
		}

		fn next_session_authority_data() -> Result<primitives::setbft::SessionAuthorityData, primitives::setbft::ApiError> {
			Ok(primitives::setbft::SessionAuthorityData::new(
				Self::next_session_authorities()?,
				SetBFT::queued_emergency_finalizer(),
			))
		}

		fn finality_version() -> primitives::Version {
			SetBFT::finality_version()
		}

		fn next_session_finality_version() -> primitives::Version {
			SetBFT::next_session_finality_version()
		}

		fn predict_session_committee(
			session: primitives::setbft::SessionIndex,
		) -> Result<primitives::setbft::SessionCommittee<AccountId>, primitives::setbft::SessionValidatorError> {
			Err(primitives::setbft::SessionValidatorError::Other(b"CommitteeManagement pallet is disabled".to_vec()))
		}

		fn next_session_aura_authorities() -> Vec<(AccountId, AuraId)> {
			let queued_keys = pallet_session::QueuedKeys::<Runtime>::get();
			queued_keys
				.into_iter()
				.filter_map(|(account_id, keys)| {
					keys.get(sp_application_crypto::key_types::AURA).map(|key| (account_id, key))
				})
				.collect()
		}

		fn key_owner(key: primitives::setbft::AuthorityId) -> Option<AccountId> {
			Session::key_owner(primitives::setbft::KEY_TYPE, key.as_ref())
		}
	}

	impl module_oracle_rpc_runtime_api::OracleApi<
		Block,
		DataProviderId,
		CurrencyId,
		TimeStampedPrice,
	> for Runtime {
		fn get_value(provider_id: DataProviderId ,key: CurrencyId) -> Option<TimeStampedPrice> {
			match provider_id {
				DataProviderId::Setheum => SetheumOracle::get_no_op(&key),
				DataProviderId::Aggregated => <AggregatedDataProvider as DataProviderExtended<_, _>>::get_no_op(&key)
			}
		}

		fn get_all_values(provider_id: DataProviderId) -> Vec<(CurrencyId, Option<TimeStampedPrice>)> {
			match provider_id {
				DataProviderId::Setheum => SetheumOracle::get_all_values(),
				DataProviderId::Aggregated => <AggregatedDataProvider as DataProviderExtended<_, _>>::get_all_values()
			}
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}


	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use module_benchmarking::{list_benchmark as module_list_benchmark};

			use module_nft::benchmarking::Pallet as NftBench;

			let mut list = Vec::<BenchmarkList>::new();


			list_benchmark!(list, extra, module_nft, NftBench::<Runtime>);

// module_list_benchmark!(list, extra, swap_legacy_module, benchmarking::dex);
// module_list_benchmark!(list, extra, auction_manager, benchmarking::auction_manager);
// module_list_benchmark!(list, extra, cdp_engine, benchmarking::cdp_engine);
// module_list_benchmark!(list, extra, emergency_shutdown, benchmarking::emergency_shutdown);
// module_list_benchmark!(list, extra, module_evm, benchmarking::evm);
// module_list_benchmark!(list, extra, serp_setmint, benchmarking::serp_setmint);
// module_list_benchmark!(list, extra, serp_treasury, benchmarking::serp_treasury);
// module_list_benchmark!(list, extra, cdp_treasury, benchmarking::cdp_treasury);
			module_list_benchmark!(list, extra, module_transaction_pause, benchmarking::transaction_pause);
			module_list_benchmark!(list, extra, module_transaction_payment, benchmarking::transaction_payment);
			module_list_benchmark!(list, extra, module_prices, benchmarking::prices);
// module_list_benchmark!(list, extra, dex_oracle, benchmarking::dex_oracle);
			module_list_benchmark!(list, extra, module_currencies, benchmarking::currencies);
			module_list_benchmark!(list, extra, module_vesting, benchmarking::vesting);

			module_list_benchmark!(list, extra, module_tokens, benchmarking::tokens);

			module_list_benchmark!(list, extra, module_authority, benchmarking::authority);
			module_list_benchmark!(list, extra, module_oracle, benchmarking::oracle);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};
			use module_benchmarking::{add_benchmark as module_add_benchmark};

			impl frame_system_benchmarking::Config for Runtime {}

			use module_nft::benchmarking::Pallet as NftBench;


			let whitelist: Vec<TrackedStorageKey> = vec![
// Block Number
// frame_system::Number::<Runtime>::hashed_key().to_vec(),
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
// Treasury
				hex_literal::hex!("6d6f646c7365742f747273790000000000000000000000000000000000000000").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, module_nft, NftBench::<Runtime>);
// module_add_benchmark!(params, batches, swap_legacy_module, benchmarking::dex);
// module_add_benchmark!(params, batches, auction_manager, benchmarking::auction_manager);
// module_add_benchmark!(params, batches, cdp_engine, benchmarking::cdp_engine);
// module_add_benchmark!(params, batches, emergency_shutdown, benchmarking::emergency_shutdown);
// module_add_benchmark!(params, batches, module_evm, benchmarking::evm);
// module_add_benchmark!(params, batches, serp_setmint, benchmarking::serp_setmint);
// module_add_benchmark!(params, batches, serp_treasury, benchmarking::serp_treasury);
// module_add_benchmark!(params, batches, cdp_treasury, benchmarking::cdp_treasury);
			module_add_benchmark!(params, batches, module_transaction_pause, benchmarking::transaction_pause);
			module_add_benchmark!(params, batches, module_transaction_payment, benchmarking::transaction_payment);
// module_add_benchmark!(params, batches, dex_oracle, benchmarking::dex_oracle);
			module_add_benchmark!(params, batches, module_currencies, benchmarking::currencies);

			module_add_benchmark!(params, batches, module_tokens, benchmarking::tokens);
			module_add_benchmark!(params, batches, module_vesting, benchmarking::vesting);

			module_add_benchmark!(params, batches, module_authority, benchmarking::authority);
			module_add_benchmark!(params, batches, module_oracle, benchmarking::oracle);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

