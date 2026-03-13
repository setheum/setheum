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
use frame_support::{
	construct_runtime, derive_impl, ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, Nothing},
	PalletId,
};
use frame_system::EnsureSignedBy;
use module_support::{uctionManager, EmergencyShutdown, SpecificJointsSwap};
use module_traits::parameter_type_with_key;
use primitives::{evm::convert_decimals_to_evm, DexShare, Moment, ReserveIdentifier, TokenSymbol, TradingPair};
use sp_core::crypto::AccountId32;
use sp_runtime::{
	testing::TestXt,
	traits::{AccountIdConversion, IdentityLookup, One as OneT},
	BuildStorage,
};
use sp_std::{cell::RefCell, str::FromStr};

pub type AccountId = AccountId32;
pub type BlockNumber = u64;
pub type AuctionId = u32;

pub const ALICE: AccountId = AccountId32::new([1u8; 32]);
pub const BOB: AccountId = AccountId32::new([2u8; 32]);
pub const CAROL: AccountId = AccountId32::new([3u8; 32]);
pub const SEU: CurrencyId = CurrencyId::Token(TokenSymbol::SEU);
pub const SEUSD: CurrencyId = CurrencyId::Token(TokenSymbol::SEUSD);
pub const BTC: CurrencyId = CurrencyId::ForeignAsset(255);

mod seusd_engine {
	pub use super::super::*;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

impl module_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type CurrencyHooks = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ReserveIdentifier;
	type DustRemovalWhitelist = Nothing;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = frame_system::Pallet<Runtime>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ReserveIdentifier;
	type WeightInfo = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}
pub type AdaptedBasicCurrency = module_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SEU;
}

impl module_currencies::Config for Runtime {
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
	pub const LoansPalletId: PalletId = PalletId(*b"set/seusdloan");
}

impl module_loans::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Currencies;
	type UssdRiskManager = UssdEngineModule;
	type UssdTreasury = UssdTreasuryModule;
	type PalletId = LoansPalletId;
	type OnUpdateLoan = ();
}

thread_local! {
	static BTC_PRICE: RefCell<Option<Price>> = RefCell::new(Some(Price::one()));
}

pub struct MockPriceSource;
impl MockPriceSource {
	pub fn set_price(currency_id: CurrencyId, price: Option<Price>) {
		match currency_id {
			BTC => BTC_PRICE.with(|v| *v.borrow_mut() = price),
			_ => {}
		}
	}
}
impl PriceProvider<CurrencyId> for MockPriceSource {
	fn get_price(currency_id: CurrencyId) -> Option<Price> {
		match currency_id {
			BTC => BTC_PRICE.with(|v| *v.borrow()),
			SEUSD => Some(Price::one()),
			_ => None,
		}
	}
}

thread_local! {
	pub static AUCTION: RefCell<Option<(AccountId, CurrencyId, Balance, Balance)>> = RefCell::new(None);
}

pub struct MockAuctionsManager;
impl MockAuctionsManager {
	pub fn auction() -> Option<(AccountId, CurrencyId, Balance, Balance)> {
		AUCTION.with(|v| {
			let cloned = v.borrow().clone();
			cloned
		})
	}
}
impl AuctionsManager<AccountId> for MockAuctionsManager {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type AuctionId = AuctionId;

	fn new_collateral_auction(
		refund_recipient: &AccountId,
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
	) -> DispatchResult {
		AUCTION.with(|v| *v.borrow_mut() = Some((refund_recipient.clone(), currency_id, amount, target)));
		Ok(())
	}

	fn cancel_auction(_id: Self::AuctionId) -> DispatchResult {
		AUCTION.with(|v| *v.borrow_mut() = None);
		Ok(())
	}

	fn get_total_target_in_auction() -> Self::Balance {
		Self::auction().map(|auction| auction.3).unwrap_or_default()
	}

	fn get_total_collateral_in_auction(_id: Self::CurrencyId) -> Self::Balance {
		Self::auction().map(|auction| auction.2).unwrap_or_default()
	}
}

parameter_types! {
	pub const GetSEUSDCurrencyId: CurrencyId = SEUSD;
	pub const UssdTreasuryPalletId: PalletId = PalletId(*b"set/seusdtrsymod");
	pub TreasuryAccount: AccountId = PalletId(*b"set/seusdtrsyacc").into_account_truncating();
	pub AlternativeSwapPathJointList: Vec<Vec<CurrencyId>> = vec![
		vec![SEU],
	];
}

impl module_seusd_treasury::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Currencies;
	type GetSEUSDCurrencyId = GetSEUSDCurrencyId;
	type AuctionsManagerHandler = MockAuctionsManager;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type Swap = SwapModule;
	type Swap = SpecificJointsSwap<SwapModule, AlternativeSwapPathJointList>;
	type MaxAuctionsCount = ConstU32<10_000>;
	type PalletId = UssdTreasuryPalletId;
	type TreasuryAccount = TreasuryAccount;
	type WeightInfo = ();
}

parameter_types! {
	pub const SwapPalletId: PalletId = PalletId(*b"set/swap");
	pub const GetExchangeFee: (u32, u32) = (0, 100);
	pub EnabledTradingPairs: Vec<TradingPair> = vec![
		TradingPair::from_currency_ids(SEUSD, BTC).unwrap(),
		TradingPair::from_currency_ids(SEU, BTC).unwrap(),
		TradingPair::from_currency_ids(SEU, SEUSD).unwrap(),
	];
}

impl module_swap_legacy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = ConstU32<4>;
	type PalletId = SwapPalletId;
	type Erc20InfoMapping = ();
	type Incentives = ();
	type WeightInfo = ();
	type ListingOrigin = EnsureSignedBy<One, AccountId>;
	type ExtendedProvisioningBlocks = ConstU64<0>;
	type OnLiquidityPoolUpdated = ();
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1_000>;
	type WeightInfo = ();
}

impl module_evm_accounts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = PalletBalances;
	type ChainId = ();
	type AddressMapping = module_evm_accounts::EvmAddressMapping<Runtime>;
	type TransferAll = Currencies;
	type WeightInfo = ();
}

parameter_types! {
	pub NetworkContractSource: EvmAddress = EvmAddress::from_str("1000000000000000000000000000000000000001").unwrap();
}

ord_parameter_types! {
	pub const CouncilAccount: AccountId = AccountId::from([1u8; 32]);
	pub const NetworkContractAccount: AccountId = AccountId::from([0u8; 32]);
	pub const StorageDepositPerByte: u128 = convert_decimals_to_evm(10);
}

impl module_evm::Config for Runtime {
	type AddressMapping = module_evm_accounts::EvmAddressMapping<Runtime>;
	type Currency = PalletBalances;
	type TransferAll = ();
	type NewContractExtraBytes = ConstU32<1>;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = ConstU128<10>;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type GasToWeight = ();
	type ChargeTransactionPayment = module_support::mocks::MockReservedTransactionPayment<PalletBalances>;
	type NetworkContractOrigin = EnsureSignedBy<NetworkContractAccount, AccountId>;
	type NetworkContractSource = NetworkContractSource;

	type DeveloperDeposit = ConstU128<1000>;
	type PublicationFee = ConstU128<200>;
	type TreasuryAccount = TreasuryAccount;
	type FreePublicationOrigin = EnsureSignedBy<CouncilAccount, AccountId>;

	type Runner = module_evm::runner::stack::Runner<Self>;
	type FindAuthor = ();
	type WeightInfo = ();
}

impl module_evm_bridge::Config for Runtime {
	type EVM = EVM;
}

thread_local! {
	static IS_SHUTDOWN: RefCell<bool> = RefCell::new(false);
}

pub fn mock_shutdown() {
	IS_SHUTDOWN.with(|v| *v.borrow_mut() = true)
}

pub fn liquidation_contract_addr() -> EvmAddress {
	EvmAddress::from_str(&"0x1000000000000000000000000000000000000000").unwrap()
}

pub struct MockEmergencyShutdown;
impl EmergencyShutdown for MockEmergencyShutdown {
	fn is_shutdown() -> bool {
		IS_SHUTDOWN.with(|v| *v.borrow_mut())
	}
}

thread_local! {
	static LIQUIDATED: RefCell<(EvmAddress, EvmAddress, Balance, Balance)> = RefCell::new((EvmAddress::default(), EvmAddress::default(), 0, 0));
	static TRANSFERRED: RefCell<(EvmAddress, Balance)> = RefCell::new((EvmAddress::default(), 0));
	static REFUNDED: RefCell<(EvmAddress, Balance)> = RefCell::new((EvmAddress::default(), 0));
	static LIQUIDATION_RESULT: RefCell<DispatchResult> = RefCell::new(Err(Error::<Runtime>::LiquidationFailed.into()));
	static REPAYMENT: RefCell<Option<Balance>> = RefCell::new(None);
}

pub struct MockLiquidationEvmBridge;
impl MockLiquidationEvmBridge {
	pub fn liquidated() -> (EvmAddress, EvmAddress, Balance, Balance) {
		LIQUIDATED.with(|v| v.borrow().clone())
	}
	pub fn transferred() -> (EvmAddress, Balance) {
		TRANSFERRED.with(|v| v.borrow().clone())
	}
	pub fn refunded() -> (EvmAddress, Balance) {
		REFUNDED.with(|v| v.borrow().clone())
	}
	pub fn reset() {
		LIQUIDATION_RESULT.with(|v| *v.borrow_mut() = Err(Error::<Runtime>::LiquidationFailed.into()));
		REPAYMENT.with(|v| *v.borrow_mut() = None);
	}
	pub fn set_liquidation_result(r: DispatchResult) {
		LIQUIDATION_RESULT.with(|v| *v.borrow_mut() = r);
	}
	pub fn set_repayment(repayment: Balance) {
		REPAYMENT.with(|v| *v.borrow_mut() = Some(repayment));
	}
}
impl LiquidationEvmBridge for MockLiquidationEvmBridge {
	fn liquidate(
		_context: InvokeContext,
		collateral: EvmAddress,
		repay_dest: EvmAddress,
		amount: Balance,
		min_repayment: Balance,
	) -> DispatchResult {
		let result = LIQUIDATION_RESULT.with(|v| v.borrow().clone());
		if result.is_ok() {
			let repayment = if let Some(r) = REPAYMENT.with(|v| v.borrow().clone()) {
				r
			} else {
				min_repayment
			};
			let _ = Currencies::deposit(GetSEUSDCurrencyId::get(), &UssdEngineModule::account_id(), repayment);
		}
		LIQUIDATED.with(|v| *v.borrow_mut() = (collateral, repay_dest, amount, min_repayment));
		result
	}
	fn on_collateral_transfer(_context: InvokeContext, collateral: EvmAddress, amount: Balance) {
		TRANSFERRED.with(|v| *v.borrow_mut() = (collateral, amount));
	}
	fn on_repayment_refund(_context: InvokeContext, collateral: EvmAddress, repayment: Balance) {
		REFUNDED.with(|v| *v.borrow_mut() = (collateral, repayment));
	}
}

ord_parameter_types! {
	pub const One: AccountId = ALICE;
}

parameter_type_with_key! {
	pub MinimumCollateralAmount: |_currency_id: CurrencyId| -> Balance {
		10
	};
}

parameter_types! {
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(3, 2);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::saturating_from_rational(1, 10);
	pub DefaultLiquidationPenalty: FractionalRate = FractionalRate::try_from(Rate::saturating_from_rational(10, 100)).unwrap();
	pub MaxSwapSlippageCompareToOracle: Ratio = Ratio::saturating_from_rational(50, 100);
	pub MaxLiquidationContractSlippage: Ratio = Ratio::saturating_from_rational(80, 100);
	pub const UssdEnginePalletId: PalletId = PalletId(*b"set/seusde");
	pub const SettleErc20EvmOrigin: AccountId = AccountId32::new([255u8; 32]);
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PriceSource = MockPriceSource;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
	type MinimumDebitValue = ConstU128<2>;
	type MinimumCollateralAmount = MinimumCollateralAmount;
	type GetSEUSDCurrencyId = GetSEUSDCurrencyId;
	type UssdTreasury = UssdTreasuryModule;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type MaxSwapSlippageCompareToOracle = MaxSwapSlippageCompareToOracle;
	type UnsignedPriority = ConstU64<1048576>; // 1 << 20
	type EmergencyShutdown = MockEmergencyShutdown;
	type UnixTime = Timestamp;
	type Currency = Currencies;
	type Swap = SwapModule;
	type LiquidationContractsUpdateOrigin = EnsureSignedBy<One, AccountId>;
	type MaxLiquidationContractSlippage = MaxLiquidationContractSlippage;
	type MaxLiquidationContracts = ConstU32<10>;
	type LiquidationEvmBridge = MockLiquidationEvmBridge;
	type PalletId = UssdEnginePalletId;
	type EvmAddressMapping = module_evm_accounts::EvmAddressMapping<Runtime>;
	type Swap = SpecificJointsSwap<SwapModule, AlternativeSwapPathJointList>;
	type EVMBridge = module_evm_bridge::EVMBridge<Runtime>;
	type SettleErc20EvmOrigin = SettleErc20EvmOrigin;
	type WeightInfo = ();
}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		UssdEngineModule: seusd_engine,
		UssdTreasuryModule: module_seusd_treasury,
		Currencies: module_currencies,
		Tokens: module_tokens,
		LoansModule: module_loans,
		PalletBalances: pallet_balances,
		SwapModule: module_swap_legacy,
		Timestamp: pallet_timestamp,
		EvmAccounts: module_evm_accounts,
		EVM: module_evm,
		EVMBridge: module_evm_bridge,
	}
);

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<RuntimeCall, ()>;

impl<LocalCall> SendTransactionTypes<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

pub struct ExtBuilder {
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balances: vec![
				(ALICE, BTC, 1000),
				(BOB, BTC, 1000),
				(CAROL, BTC, 10000),
				(CAROL, SEUSD, 10000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Runtime>::default()
			.build_storage()
			.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: vec![(CAROL, 10000)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		module_tokens::GenesisConfig::<Runtime> {
			balances: self.balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		module_swap_legacy::GenesisConfig::<Runtime> {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: EnabledTradingPairs::get(),
			initial_added_liquidity_pools: vec![],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		MockLiquidationEvmBridge::reset();

		t.into()
	}

	pub fn lots_of_accounts() -> Self {
		let mut balances = Vec::new();
		for i in 0..1001u32 {
			balances.push((account_id_from_u32(i), BTC, 1000));
		}
		Self { balances }
	}
}

pub fn account_id_from_u32(num: u32) -> AccountId {
	let mut data = [0u8; 32];
	let index = num.to_le_bytes();
	data[0..4].copy_from_slice(&index[..]);
	AccountId::new(data)
}
