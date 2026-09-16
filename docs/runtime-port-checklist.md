# setheum-runtime: stable2506 port checklist

Baseline: `SKIP_WASM_BUILD=1 cargo check -p setheum-runtime` -> **250 errors** (commit `34e27160`).
Reference runtime (pinned polkadot-sdk `3785760`): `substrate/bin/node/runtime/src/lib.rs` (kitchensink).
Verify with: `SKIP_WASM_BUILD=1 cargo check -p setheum-runtime 2>&1 | Select-String "^error"`.
Fix order is independent per Config; each block below can be parallelised to one workstream.

NOTE: many `E0412 Call/Event/AllPallets` and `E0107 GenesisConfig` errors are cascades from `construct_runtime!` failing to expand. Fix the macro-level/cross-cutting section first, then re-run before assigning the rest.

## Parallel workstreams

| Stream | Crates/pallets |
|---|---|
| A - core system | `frame_system` (+offchain), `pallet_session` (+historical), `pallet_authorship`, `pallet_aura`, `pallet_authority_discovery` |
| B - governance | `pallet_collective` x3, `pallet_membership` x4, `pallet_democracy`, `pallet_treasury`, `pallet_bounties`, `pallet_tips`, `pallet_scheduler`, `pallet_proxy`, `pallet_multisig`, `pallet_recovery`, `pallet_indices`, `pallet_identity`, `pallet_sudo`, `pallet_utility`, `pallet_insecure_randomness_collective_flip` |
| C - staking/consensus | `pallet_staking`, `frame_election_provider_support::onchain`, `pallet_offences`, `pallet_im_online` |
| D - custom Setheum pallets | `module_nft`, `module_prices`, `module_transaction_payment`, `module-transaction-payment`, `swap_legacy_module`, `module_currencies`, `module_authority`, `module_oracle`, `module_tokens`, `module_vesting`, `module_transaction_pause`, `module_setbft`, `pallet_sheyth_vm`, `dex_oracle`, `module_airdrop` |

## lib.rs errors by Config

### `module_nft::Config` (lib.rs:973) - 18 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:974 `E0437`: error[E0437]: type `Event` is not a member of trait `module_nft::Config`
      source: `type Event = Event;`
- [ ] lib.rs:984 `E0437`: error[E0437]: type `ClassData` is not a member of trait `module_nft::Config`
      source: `type ClassData = module_nft::ClassData<Balance>;`
- [ ] lib.rs:985 `E0437`: error[E0437]: type `TokenData` is not a member of trait `module_nft::Config`
      source: `type TokenData = module_nft::TokenData<Balance>;`
- [ ] lib.rs:974 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1006 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `impl InstanceFilter<Call> for ProxyType {`
- [ ] lib.rs:1012 `E0433`: error[E0433]: failed to resolve: use of undeclared type `Call`
      source: `_ if matches!(c, Call::Utility(..)) => true,`
- [ ] lib.rs:973 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl module_nft::Config for Runtime {`

### `module_prices::Config` (lib.rs:644) - 11 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:645 `E0437`: error[E0437]: type `Event` is not a member of trait `module_prices::Config`
      source: `type Event = Event;`
- [ ] lib.rs:647 `E0437`: error[E0437]: type `GetSetUSDId` is not a member of trait `module_prices::Config`
      source: `type GetSetUSDId = GetSetUSDId;`
- [ ] lib.rs:648 `E0437`: error[E0437]: type `SetterCurrencyId` is not a member of trait `module_prices::Config`
      source: `type SetterCurrencyId = SetterCurrencyId;`
- [ ] lib.rs:649 `E0437`: error[E0437]: type `SetUSDFixedPrice` is not a member of trait `module_prices::Config`
      source: `type SetUSDFixedPrice = SetUSDFixedPrice;`
- [ ] lib.rs:650 `E0437`: error[E0437]: type `SetterFixedPrice` is not a member of trait `module_prices::Config`
      source: `type SetterFixedPrice = SetterFixedPrice;`
- [ ] lib.rs:652 `E0437`: error[E0437]: type `DEX` is not a member of trait `module_prices::Config`
      source: `type DEX = Dex;`
- [ ] lib.rs:654 `E0437`: error[E0437]: type `CurrencyIdMapping` is not a member of trait `module_prices::Config`
      source: `type CurrencyIdMapping = module_asset_registry::EvmCurrencyIdMapping<Runtime>;`
- [ ] lib.rs:645 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:648 `E0412`: error[E0412]: cannot find type `SetterCurrencyId` in this scope
      source: `type SetterCurrencyId = SetterCurrencyId;`
- [ ] lib.rs:644 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `SEUSDFixedPrice`, `GetSEUSDCurrencyId`, 
      source: `impl module_prices::Config for Runtime {`
- [ ] lib.rs:654 `E0433`: error[E0433]: failed to resolve: use of unresolved module or unlinked crate `module_asset_registry`
      source: `type CurrencyIdMapping = module_asset_registry::EvmCurrencyIdMapping<Runtime>;`

### `module_transaction_payment::Config` (lib.rs:929) - 11 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 613.

- [ ] lib.rs:931 `E0437`: error[E0437]: type `DefaultFeeSwapPathList` is not a member of trait `module_transaction_payment::Config`
      source: `type DefaultFeeSwapPathList = DefaultFeeSwapPathList;`
- [ ] lib.rs:935 `E0437`: error[E0437]: type `TransactionByteFee` is not a member of trait `module_transaction_payment::Config`
      source: `type TransactionByteFee = TransactionByteFee;`
- [ ] lib.rs:938 `E0437`: error[E0437]: type `DEX` is not a member of trait `module_transaction_payment::Config`
      source: `type DEX = Dex;`
- [ ] lib.rs:955 `E0412`: error[E0412]: cannot find type `AllPrecompiles` in this scope
      source: `impl sp_core::Get<AllPrecompiles> for PrecompilesValue {`
- [ ] lib.rs:962 `E0425`: error[E0425]: cannot find value `CHAIN_ID_SETHEUM_MAINNET` in this scope
      source: `pub const ChainId: u64 = CHAIN_ID_SETHEUM_MAINNET;`
- [ ] lib.rs:963 `E0412`: error[E0412]: cannot find type `U256` in this scope
      source: `pub BlockGasLimit: U256 = U256::from(u32::MAX);`
- [ ] lib.rs:963 `E0433`: error[E0433]: failed to resolve: use of undeclared type `U256`
      source: `pub BlockGasLimit: U256 = U256::from(u32::MAX);`
- [ ] lib.rs:937 `E0107`: error[E0107]: struct takes 5 generic arguments but 4 generic arguments were supplied
      source: `type FeeMultiplierUpdate = TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable,`
- [ ] lib.rs:929 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `OperationalFeeMultiplier`, 
      source: `impl module_transaction_payment::Config for Runtime {`
- [ ] lib.rs:957 `E0425`: error[E0425]: cannot find function, tuple struct or tuple variant `AllPrecompiles` in this scope
      source: `AllPrecompiles(Default::default())`

### `pallet_staking::Config` (lib.rs:372) - 10 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 731.

- [ ] lib.rs:373 `E0438`: error[E0438]: const `MAX_NOMINATIONS` is not a member of trait `pallet_staking::Config`
      source: `const MAX_NOMINATIONS: u32 = 16; // The maximum number of Validators a nominator can choose to nominate.`
- [ ] lib.rs:378 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_staking::Config`
      source: `type Event = Event;`
- [ ] lib.rs:384 `E0437`: error[E0437]: type `SlashCancelOrigin` is not a member of trait `pallet_staking::Config`
      source: `type SlashCancelOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;`
- [ ] lib.rs:387 `E0437`: error[E0437]: type `MaxNominatorRewardedPerValidator` is not a member of trait `pallet_staking::Config`
      source: `type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;`
- [ ] lib.rs:378 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:389 `E0412`: error[E0412]: cannot find type `OnChainSequentialPhragmen` in module `onchain`
      source: `type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;`
- [ ] lib.rs:377 `E0271`: error[E0271]: type mismatch resolving `<Pallet<...> as Currency<...>>::NegativeImbalance == Imbalance<u128, ..., ...>`
      source: `type RewardRemainder = Treasury;`
- [ ] lib.rs:372 `E0046`: error[E0046]: not all trait items implemented, missing: `OldCurrency`, `RuntimeHoldReason`, `CurrencyBalance`, 
      source: `impl pallet_staking::Config for Runtime {`

### `construct_runtime! / macro-level (cross-cutting)` (lib.rs:0) - 9 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:705 `E0407`: error[E0407]: method `create_transaction` is not a member of trait `frame_system::offchain::CreateSignedTransaction`
      source: `/     fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(`
- [ ] lib.rs:1383 `E0405`: error[E0405]: cannot find trait `TryInto` in this scope
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0405`: error[E0405]: cannot find trait `TryFrom` in this scope
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0107`: error[E0107]: missing generics for enum `pallet_session::Event`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0107`: error[E0107]: missing generics for struct `frame_system::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0107`: error[E0107]: missing generics for struct `pallet_treasury::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0107`: error[E0107]: missing generics for struct `pallet_aura::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1383 `E0107`: error[E0107]: missing generics for struct `pallet_authority_discovery::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:701 `E0046`: error[E0046]: not all trait items implemented, missing: `create_signed_transaction`
      source: `/ impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime`

### `swap_legacy_module::Config` (lib.rs:817) - 8 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:818 `E0437`: error[E0437]: type `Event` is not a member of trait `swap_legacy_module::Config`
      source: `type Event = Event;`
- [ ] lib.rs:820 `E0437`: error[E0437]: type `StableCurrencyIds` is not a member of trait `swap_legacy_module::Config`
      source: `type StableCurrencyIds = StableCurrencyIds;`
- [ ] lib.rs:822 `E0437`: error[E0437]: type `GetStableCurrencyExchangeFee` is not a member of trait `swap_legacy_module::Config`
      source: `type GetStableCurrencyExchangeFee = GetStableCurrencyExchangeFee;`
- [ ] lib.rs:825 `E0437`: error[E0437]: type `CurrencyIdMapping` is not a member of trait `swap_legacy_module::Config`
      source: `type CurrencyIdMapping = EvmCurrencyIdMapping<Runtime>;`
- [ ] lib.rs:818 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:825 `E0412`: error[E0412]: cannot find type `EvmCurrencyIdMapping` in this scope
      source: `type CurrencyIdMapping = EvmCurrencyIdMapping<Runtime>;`
- [ ] lib.rs:826 `E0277`: error[E0277]: the trait bound `module_dex::WeightInfo<Runtime>: module_swap_legacy::WeightInfo` is not satisfied
      source: `type WeightInfo = weights::module_dex::WeightInfo<Runtime>;`
- [ ] lib.rs:817 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `Erc20InfoMapping`, `Incentives`, 
      source: `impl swap_legacy_module::Config for Runtime {`

### `frame_system::Config` (lib.rs:275) - 7 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 350.

- [ ] lib.rs:287 `E0437`: error[E0437]: type `BlockNumber` is not a member of trait `frame_system::Config`
      source: `type BlockNumber = BlockNumber;`
- [ ] lib.rs:293 `E0437`: error[E0437]: type `Header` is not a member of trait `frame_system::Config`
      source: `type Header = generic::Header<BlockNumber, BlakeTwo256>;`
- [ ] lib.rs:295 `E0437`: error[E0437]: type `Event` is not a member of trait `frame_system::Config`
      source: `type Event = Event;`
- [ ] lib.rs:297 `E0437`: error[E0437]: type `Origin` is not a member of trait `frame_system::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:295 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:297 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:275 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeOrigin`, `RuntimeCall`, `RuntimeTask`, 
      source: `impl frame_system::Config for Runtime {`

### `pallet_session::Config` (lib.rs:325) - 7 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 680.

- [ ] lib.rs:326 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_session::Config`
      source: `type Event = Event;`
- [ ] lib.rs:334 `E0437`: error[E0437]: type `DisabledValidatorsThreshold` is not a member of trait `pallet_session::Config`
      source: `type DisabledValidatorsThreshold = DisabledValidatorsThreshold;`
- [ ] lib.rs:326 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:328 `E0412`: error[E0412]: cannot find type `StashOf` in crate `pallet_staking`
      source: `type ValidatorIdOf = pallet_staking::StashOf<Self>;`
- [ ] lib.rs:339 `E0425`: error[E0425]: cannot find value `EPOCH_DURATION_IN_SLOTS` in this scope
      source: `pub const SessionPeriod: u32 = EPOCH_DURATION_IN_SLOTS as u32;`
- [ ] lib.rs:341 `E0425`: error[E0425]: cannot find value `MILLISECS_PER_BLOCK` in this scope
      source: `pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;`
- [ ] lib.rs:325 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `DisablingStrategy`
      source: `impl pallet_session::Config for Runtime {`

### `module_currencies::Config` (lib.rs:531) - 7 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:532 `E0437`: error[E0437]: type `Event` is not a member of trait `module_currencies::Config`
      source: `type Event = Event;`
- [ ] lib.rs:536 `E0437`: error[E0437]: type `StableCurrencyIds` is not a member of trait `module_currencies::Config`
      source: `type StableCurrencyIds = StableCurrencyIds;`
- [ ] lib.rs:537 `E0437`: error[E0437]: type `SerpTreasury` is not a member of trait `module_currencies::Config`
      source: `type SerpTreasury = SerpTreasury;`
- [ ] lib.rs:532 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:537 `E0412`: error[E0412]: cannot find type `SerpTreasury` in this scope
      source: `type SerpTreasury = SerpTreasury;`
- [ ] lib.rs:539 `E0412`: error[E0412]: cannot find type `EvmAddressMapping` in this scope
      source: `type AddressMapping = EvmAddressMapping<Runtime>;`
- [ ] lib.rs:531 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl module_currencies::Config for Runtime {`

### `pallet_scheduler::Config` (lib.rs:1109) - 7 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 497.

- [ ] lib.rs:1110 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_scheduler::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1111 `E0437`: error[E0437]: type `Origin` is not a member of trait `pallet_scheduler::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:1113 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_scheduler::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1110 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1111 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:1113 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1109 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeOrigin`, `RuntimeCall`, 
      source: `impl pallet_scheduler::Config for Runtime {`

### `module_authority::Config` (lib.rs:1120) - 7 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1121 `E0437`: error[E0437]: type `Event` is not a member of trait `module_authority::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1122 `E0437`: error[E0437]: type `Origin` is not a member of trait `module_authority::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:1124 `E0437`: error[E0437]: type `Call` is not a member of trait `module_authority::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1121 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1122 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:1124 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1120 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeOrigin`, `RuntimeCall`
      source: `impl module_authority::Config for Runtime {`

### `pallet_treasury::Config` (lib.rs:1309) - 7 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1307.

- [ ] lib.rs:1312 `E0437`: error[E0437]: type `ApproveOrigin` is not a member of trait `pallet_treasury::Config`
      source: `type ApproveOrigin = EnsureRootOrHalfShuraCouncil;`
- [ ] lib.rs:1314 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_treasury::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1315 `E0437`: error[E0437]: type `OnSlash` is not a member of trait `pallet_treasury::Config`
      source: `type OnSlash = Treasury;`
- [ ] lib.rs:1316 `E0437`: error[E0437]: type `ProposalBond` is not a member of trait `pallet_treasury::Config`
      source: `type ProposalBond = ProposalBond;`
- [ ] lib.rs:1317 `E0437`: error[E0437]: type `ProposalBondMinimum` is not a member of trait `pallet_treasury::Config`
      source: `type ProposalBondMinimum = ProposalBondMinimum;`
- [ ] lib.rs:1314 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1309 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `SpendOrigin`, `AssetKind`, `Beneficiary`, 
      source: `impl pallet_treasury::Config for Runtime {`

### `?` (lib.rs:?) - 7 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:168 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `impl EnsureOrigin<Origin> for EnsureWeb3SettersClub {`
- [ ] lib.rs:177 `E0433`: error[E0433]: failed to resolve: use of undeclared type `Origin`
      source: `Err(Origin::from(Some(caller)))`
- [ ] lib.rs:91 `E0603`: error[E0603]: type alias `Multiplier` is private
      source: `use module_transaction_payment::{Multiplier, TargetedFeeAdjustment};`

### `pallet_collective::Config<ShuraCouncilInstance>` (lib.rs:1143) - 6 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1144 `E0437`: error[E0437]: type `Origin` is not a member of trait `pallet_collective::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:1146 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_collective::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1144 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:1145 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Proposal = Call;`
- [ ] lib.rs:1146 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1143 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeOrigin`, `RuntimeEvent`, `SetMembersOrigin`, 
      source: `impl pallet_collective::Config<ShuraCouncilInstance> for Runtime {`

### `pallet_collective::Config<FinancialCouncilInstance>` (lib.rs:1173) - 6 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1174 `E0437`: error[E0437]: type `Origin` is not a member of trait `pallet_collective::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:1176 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_collective::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1174 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:1175 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Proposal = Call;`
- [ ] lib.rs:1176 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1173 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeOrigin`, `RuntimeEvent`, `SetMembersOrigin`, 
      source: `impl pallet_collective::Config<FinancialCouncilInstance> for Runtime {`

### `pallet_collective::Config<TechnicalCommitteeInstance>` (lib.rs:1203) - 6 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1204 `E0437`: error[E0437]: type `Origin` is not a member of trait `pallet_collective::Config`
      source: `type Origin = Origin;`
- [ ] lib.rs:1206 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_collective::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1204 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `type Origin = Origin;`
- [ ] lib.rs:1205 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Proposal = Call;`
- [ ] lib.rs:1206 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1203 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeOrigin`, `RuntimeEvent`, `SetMembersOrigin`, 
      source: `impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {`

### `onchain::Config` (lib.rs:394) - 5 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:395 `E0437`: error[E0437]: type `BlockWeights` is not a member of trait `onchain::Config`
      source: `type BlockWeights = BlockWeights;`
- [ ] lib.rs:396 `E0437`: error[E0437]: type `AccountId` is not a member of trait `onchain::Config`
      source: `type AccountId = AccountId;`
- [ ] lib.rs:397 `E0437`: error[E0437]: type `BlockNumber` is not a member of trait `onchain::Config`
      source: `type BlockNumber = BlockNumber;`
- [ ] lib.rs:398 `E0437`: error[E0437]: type `Accuracy` is not a member of trait `onchain::Config`
      source: `type Accuracy = sp_runtime::Perbill;`
- [ ] lib.rs:394 `E0046`: error[E0046]: not all trait items implemented, missing: `Sort`, `System`, `Solver`, `MaxBackersPerWinner`, 
      source: `impl onchain::Config for Runtime {`

### `pallet_identity::Config` (lib.rs:495) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1666.

- [ ] lib.rs:496 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_identity::Config`
      source: `type Event = Event;`
- [ ] lib.rs:499 `E0437`: error[E0437]: type `FieldDeposit` is not a member of trait `pallet_identity::Config`
      source: `type FieldDeposit = FieldDeposit;`
- [ ] lib.rs:502 `E0437`: error[E0437]: type `MaxAdditionalFields` is not a member of trait `pallet_identity::Config`
      source: `type MaxAdditionalFields = MaxAdditionalFields;`
- [ ] lib.rs:496 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:495 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `ByteDeposit`, `UsernameDeposit`, 
      source: `impl pallet_identity::Config for Runtime {`

### `dex_oracle::Config` (lib.rs:830) - 5 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:831 `E0437`: error[E0437]: type `Event` is not a member of trait `dex_oracle::Config`
      source: `type Event = Event;`
- [ ] lib.rs:831 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:832 `E0277`: error[E0277]: the trait bound `Pallet<Runtime>: SwapManager<AccountId32, u128, CurrencyId>` is not satisfied
      source: `type DEX = swap_legacy_module::Pallet<Runtime>;`
- [ ] lib.rs:830 `E0046`: error[E0046]: not all trait items implemented, missing: `Time`, `UpdateOrigin`, `WeightInfo`
      source: `impl dex_oracle::Config for Runtime {`
- [ ] lib.rs:909 `E0049`: error[E0049]: associated function `on_unbalanceds` has 2 type parameters but its trait declaration has 1 type parameter
      source: `fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {`

### `pallet_proxy::Config` (lib.rs:1048) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 476.

- [ ] lib.rs:1049 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_proxy::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1050 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_proxy::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1049 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1050 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1048 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `BlockNumberProvider`
      source: `impl pallet_proxy::Config for Runtime {`

### `pallet_sudo::Config` (lib.rs:1132) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1492.

- [ ] lib.rs:1133 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_sudo::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1134 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_sudo::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1133 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1134 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1132 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `WeightInfo`
      source: `impl pallet_sudo::Config for Runtime {`

### `pallet_utility::Config` (lib.rs:1244) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 378.

- [ ] lib.rs:1245 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_utility::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1246 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_utility::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1245 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1246 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1244 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `PalletsOrigin`
      source: `impl pallet_utility::Config for Runtime {`

### `pallet_multisig::Config` (lib.rs:1256) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 392.

- [ ] lib.rs:1257 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_multisig::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1258 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_multisig::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1257 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1258 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1256 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `BlockNumberProvider`
      source: `impl pallet_multisig::Config for Runtime {`

### `pallet_recovery::Config` (lib.rs:1356) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1698.

- [ ] lib.rs:1357 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_recovery::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1358 `E0437`: error[E0437]: type `Call` is not a member of trait `pallet_recovery::Config`
      source: `type Call = Call;`
- [ ] lib.rs:1357 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1358 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `type Call = Call;`
- [ ] lib.rs:1356 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `WeightInfo`, `RuntimeCall`, 
      source: `impl pallet_recovery::Config for Runtime {`

### `pallet_insecure_randomness_collective_flip::Config` (lib.rs:1374) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 369.

- [ ] lib.rs:1509 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;`
- [ ] lib.rs:1520 `E0412`: error[E0412]: cannot find type `AllPallets` in this scope
      source: `AllPallets,`
- [ ] lib.rs:1481 `E0053`: error[E0053]: method `on_runtime_upgrade` has an incompatible type for trait
      source: `fn on_runtime_upgrade() -> u64 {`

### `pallet_im_online::Config` (lib.rs:476) - 4 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1617.

- [ ] lib.rs:478 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_im_online::Config`
      source: `type Event = Event;`
- [ ] lib.rs:478 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:476 `E0277`: error[E0277]: the trait bound `Runtime: CreateBare<pallet_im_online::Call<Runtime>>` is not satisfied
      source: `impl pallet_im_online::Config for Runtime {`
- [ ] lib.rs:476 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxKeys`, `MaxPeerInHeartbeats`, `RuntimeEvent`
      source: `impl pallet_im_online::Config for Runtime {`

### `module_oracle::Config<SetheumDataProvider>` (lib.rs:552) - 4 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:553 `E0437`: error[E0437]: type `Event` is not a member of trait `module_oracle::Config`
      source: `type Event = Event;`
- [ ] lib.rs:553 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:552 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxFeedValues`
      source: `impl module_oracle::Config<SetheumDataProvider> for Runtime {`
- [ ] lib.rs:574 `E0053`: error[E0053]: method `feed_value` has an incompatible type for trait
      source: `fn feed_value(_: AccountId, _: CurrencyId, _: Price) -> DispatchResult {`

### `module_tokens::Config` (lib.rs:627) - 4 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:628 `E0437`: error[E0437]: type `Event` is not a member of trait `module_tokens::Config`
      source: `type Event = Event;`
- [ ] lib.rs:634 `E0437`: error[E0437]: type `OnDust` is not a member of trait `module_tokens::Config`
      source: `type OnDust = module_tokens::TransferDust<Runtime, TreasuryAccount>;`
- [ ] lib.rs:628 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:627 `E0046`: error[E0046]: not all trait items implemented, missing: `CurrencyHooks`, `MaxReserves`, `ReserveIdentifier`
      source: `impl module_tokens::Config for Runtime {`

### `pallet_bounties::Config` (lib.rs:1326) - 4 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1356.

- [ ] lib.rs:1327 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_bounties::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1331 `E0437`: error[E0437]: type `BountyCuratorDeposit` is not a member of trait `pallet_bounties::Config`
      source: `type BountyCuratorDeposit = BountyCuratorDeposit;`
- [ ] lib.rs:1327 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1326 `E0046`: error[E0046]: not all trait items implemented, missing: `CuratorDepositMultiplier`, `CuratorDepositMax`, 
      source: `impl pallet_bounties::Config for Runtime {`

### `frame_system::offchain::CreateSignedTransaction<LocalCall>` (lib.rs:701) - 4 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:703 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `Call: From<LocalCall>,`
- [ ] lib.rs:701 `E0277`: error[E0277]: the trait bound `Runtime: CreateTransactionBase<LocalCall>` is not satisfied
      source: `impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime`

### `pallet_offences::Config` (lib.rs:464) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1629.

- [ ] lib.rs:465 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_offences::Config`
      source: `type Event = Event;`
- [ ] lib.rs:465 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:464 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_offences::Config for Runtime {`

### `pallet_indices::Config` (lib.rs:515) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 568.

- [ ] lib.rs:517 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_indices::Config`
      source: `type Event = Event;`
- [ ] lib.rs:517 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:515 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_indices::Config for Runtime {`

### `module_transaction_pause::Config` (lib.rs:665) - 3 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:666 `E0437`: error[E0437]: type `Event` is not a member of trait `module_transaction_pause::Config`
      source: `type Event = Event;`
- [ ] lib.rs:666 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:665 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl module_transaction_pause::Config for Runtime {`

### `pallet_balances::Config` (lib.rs:1072) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 584.

- [ ] lib.rs:1073 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_balances::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1073 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1072 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeHoldReason`, `RuntimeFreezeReason`, 
      source: `impl pallet_balances::Config for Runtime {`

### `module_vesting::Config` (lib.rs:1091) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1747.

- [ ] lib.rs:1092 `E0437`: error[E0437]: type `Event` is not a member of trait `module_vesting::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1092 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1091 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl module_vesting::Config for Runtime {`

### `pallet_membership::Config<ShuraCouncilMembershipInstance>` (lib.rs:1154) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1155 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_membership::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1155 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1154 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_membership::Config<ShuraCouncilMembershipInstance> for Runtime {`

### `pallet_membership::Config<FinancialCouncilMembershipInstance>` (lib.rs:1184) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1185 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_membership::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1185 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1184 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_membership::Config<FinancialCouncilMembershipInstance> for Runtime {`

### `pallet_membership::Config<TechnicalCommitteeMembershipInstance>` (lib.rs:1214) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1215 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_membership::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1215 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1214 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_membership::Config<TechnicalCommitteeMembershipInstance> for Runtime {`

### `pallet_membership::Config<OperatorMembershipInstanceSetheum>` (lib.rs:1231) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1232 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_membership::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1232 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1231 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_membership::Config<OperatorMembershipInstanceSetheum> for Runtime {`

### `pallet_tips::Config` (lib.rs:1338) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1404.

- [ ] lib.rs:1339 `E0437`: error[E0437]: type `Event` is not a member of trait `pallet_tips::Config`
      source: `type Event = Event;`
- [ ] lib.rs:1339 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type Event = Event;`
- [ ] lib.rs:1338 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `MaxTipAmount`, `OnSlash`
      source: `impl pallet_tips::Config for Runtime {`

### `frame_system::offchain::SendTransactionTypes<C>` (lib.rs:746) - 3 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:746 `E0405`: error[E0405]: cannot find trait `SendTransactionTypes` in module `frame_system::offchain`
      source: `impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime`
- [ ] lib.rs:748 `E0412`: error[E0412]: cannot find type `Call` in this scope
      source: `Call: From<C>,`

### `pallet_session::historical::Config` (lib.rs:346) - 3 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 694.

- [ ] lib.rs:366 `E0603`: error[E0603]: type alias `EraIndex` is private
      source: `pub const BondingDuration: pallet_staking::EraIndex = 4; // 8 hours (80 mins in test)`
- [ ] lib.rs:346 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`
      source: `impl pallet_session::historical::Config for Runtime {`

### `pallet_authorship::Config` (lib.rs:457) - 2 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 664.

- [ ] lib.rs:459 `E0437`: error[E0437]: type `UncleGenerations` is not a member of trait `pallet_authorship::Config`
      source: `type UncleGenerations = UncleGenerations;`
- [ ] lib.rs:460 `E0437`: error[E0437]: type `FilterUncle` is not a member of trait `pallet_authorship::Config`
      source: `type FilterUncle = ();`

### `pallet_sheyth_vm::Config` (lib.rs:437) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:438 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type RuntimeEvent = Event;`
- [ ] lib.rs:442 `E0425`: error[E0425]: cannot find value `SLOT_DURATION` in this scope
      source: `pub const MinimumPeriod: u64 = SLOT_DURATION / 2;`

### `module_setbft::Config` (lib.rs:429) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:431 `E0412`: error[E0412]: cannot find type `Event` in this scope
      source: `type RuntimeEvent = Event;`

### `pallet_aura::Config` (lib.rs:403) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:403 `E0046`: error[E0046]: not all trait items implemented, missing: `AllowMultipleBlocksPerSlot`, `SlotDuration`
      source: `impl pallet_aura::Config for Runtime {`

### `pallet_authority_discovery::Config` (lib.rs:470) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1635.

- [ ] lib.rs:470 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxAuthorities`
      source: `impl pallet_authority_discovery::Config for Runtime {}`

### `primitives::SetBFTSessionApi<Block>` (lib.rs:1615) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1658 `E0433`: error[E0433]: failed to resolve: use of undeclared type `CommitteeManagement`
      source: `CommitteeManagement::predict_session_committee_for_session(session)`

## Errors outside lib.rs

- [ ]  `macro`: error: could not compile `setheum-runtime` (lib) due to 249 previous errors; 14 warnings emitted
- [ ] repos\setheum\runtime\src\authority.rs:24 `E0432`: error[E0432]: unresolved import `crate::Origin`
      source: `EnsureRootOrOneThirdsTechnicalCommittee, EnsureRootOrTwoThirdsTechnicalCommittee, OneDay, Origin, SevenDays,`
- [ ] repos\setheum\runtime\src\constants.rs:43 `E0603`: error[E0603]: constant `SEU` is private
      source: `use runtime_common::{cent, SEU};`
- [ ] repos\setheum\runtime\src\weights\module_currencies.rs:30 `E0046`: error[E0046]: not all trait items implemented, missing: `force_set_lock`, `force_remove_lock`
      source: `impl<T: frame_system::Config> module_currencies::WeightInfo for WeightInfo<T> {`
- [ ] repos\setheum\runtime\src\weights\module_dex.rs:30 `E0433`: error[E0433]: failed to resolve: use of unresolved module or unlinked crate `swap_legacy_module`
      source: `impl<T: frame_system::Config> swap_legacy_module::WeightInfo for WeightInfo<T> {`
- [ ] repos\setheum\runtime\src\weights\module_transaction_pause.rs:30 `E0046`: error[E0046]: not all trait items implemented, missing: `pause_evm_precompile`, `unpause_evm_precompile`
      source: `impl<T: frame_system::Config> module_transaction_pause::WeightInfo for WeightInfo<T> {`
- [ ] repos\setheum\runtime\src\weights\module_transaction_payment.rs:30 `E0046`: error[E0046]: not all trait items implemented, missing: `enable_charge_fee_pool`, `disable_charge_fee_pool`, 
      source: `impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {`