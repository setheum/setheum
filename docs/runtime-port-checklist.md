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

### `module_transaction_payment::Config` (lib.rs:999) - 12 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 613.

- [ ] lib.rs:1001 `E0437`: error[E0437]: type `DefaultFeeSwapPathList` is not a member of trait `module_transaction_payment::Config`
      source: `type DefaultFeeSwapPathList = DefaultFeeSwapPathList;`
- [ ] lib.rs:1005 `E0437`: error[E0437]: type `TransactionByteFee` is not a member of trait `module_transaction_payment::Config`
      source: `type TransactionByteFee = TransactionByteFee;`
- [ ] lib.rs:1008 `E0437`: error[E0437]: type `DEX` is not a member of trait `module_transaction_payment::Config`
      source: `type DEX = Dex;`
- [ ] lib.rs:1025 `E0412`: error[E0412]: cannot find type `AllPrecompiles` in this scope
      source: `impl sp_core::Get<AllPrecompiles> for PrecompilesValue {`
- [ ] lib.rs:1032 `E0425`: error[E0425]: cannot find value `CHAIN_ID_SETHEUM_MAINNET` in this scope
      source: `pub const ChainId: u64 = CHAIN_ID_SETHEUM_MAINNET;`
- [ ] lib.rs:1033 `E0412`: error[E0412]: cannot find type `U256` in this scope
      source: `pub BlockGasLimit: U256 = U256::from(u32::MAX);`
- [ ] lib.rs:1033 `E0433`: error[E0433]: failed to resolve: use of undeclared type `U256`
      source: `pub BlockGasLimit: U256 = U256::from(u32::MAX);`
- [ ] lib.rs:1007 `E0107`: error[E0107]: struct takes 5 generic arguments but 4 generic arguments were supplied
      source: `type FeeMultiplierUpdate = TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable,`
- [ ] lib.rs:999 `E0277`: error[E0277]: the trait bound `RuntimeEvent: From<module_transaction_payment::Event<Runtime>>` is not satisfied
      source: `impl module_transaction_payment::Config for Runtime {`
- [ ] lib.rs:999 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeEvent`, `RuntimeCall`, `OperationalFeeMultiplier`, 
      source: `impl module_transaction_payment::Config for Runtime {`
- [ ] lib.rs:1027 `E0425`: error[E0425]: cannot find function, tuple struct or tuple variant `AllPrecompiles` in this scope
      source: `AllPrecompiles(Default::default())`

### `construct_runtime! / macro-level (cross-cutting)` (lib.rs:0) - 11 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1451 `E0405`: error[E0405]: cannot find trait `TryInto` in this scope
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0405`: error[E0405]: cannot find trait `TryFrom` in this scope
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0107`: error[E0107]: missing generics for enum `pallet_session::Event`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0107`: error[E0107]: missing generics for struct `frame_system::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0107`: error[E0107]: missing generics for struct `pallet_treasury::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0107`: error[E0107]: missing generics for struct `pallet_aura::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0107`: error[E0107]: missing generics for struct `pallet_authority_discovery::GenesisConfig`
      source: `/ construct_runtime!(`
- [ ] lib.rs:1451 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `/ construct_runtime!(`

### `pallet_staking::Config` (lib.rs:398) - 8 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 731.

- [ ] lib.rs:399 `E0438`: error[E0438]: const `MAX_NOMINATIONS` is not a member of trait `pallet_staking::Config`
      source: `const MAX_NOMINATIONS: u32 = 16; // The maximum number of Validators a nominator can choose to nominate.`
- [ ] lib.rs:410 `E0437`: error[E0437]: type `SlashCancelOrigin` is not a member of trait `pallet_staking::Config`
      source: `type SlashCancelOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;`
- [ ] lib.rs:413 `E0437`: error[E0437]: type `MaxNominatorRewardedPerValidator` is not a member of trait `pallet_staking::Config`
      source: `type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;`
- [ ] lib.rs:415 `E0412`: error[E0412]: cannot find type `OnChainSequentialPhragmen` in module `onchain`
      source: `type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;`
- [ ] lib.rs:403 `E0271`: error[E0271]: type mismatch resolving `<Pallet<...> as Currency<...>>::NegativeImbalance == Imbalance<u128, ..., ...>`
      source: `type RewardRemainder = Treasury;`
- [ ] lib.rs:398 `E0046`: error[E0046]: not all trait items implemented, missing: `OldCurrency`, `RuntimeHoldReason`, `CurrencyBalance`, 
      source: `impl pallet_staking::Config for Runtime {`

### `?` (lib.rs:?) - 7 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:175 `E0412`: error[E0412]: cannot find type `Origin` in this scope
      source: `impl EnsureOrigin<Origin> for EnsureWeb3SettersClub {`
- [ ] lib.rs:184 `E0433`: error[E0433]: failed to resolve: use of undeclared type `Origin`
      source: `Err(Origin::from(Some(caller)))`
- [ ] lib.rs:91 `E0603`: error[E0603]: type alias `Multiplier` is private
      source: `use module_transaction_payment::{Multiplier, TargetedFeeAdjustment};`

### `pallet_treasury::Config` (lib.rs:1377) - 6 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1307.

- [ ] lib.rs:1380 `E0437`: error[E0437]: type `ApproveOrigin` is not a member of trait `pallet_treasury::Config`
      source: `type ApproveOrigin = EnsureRootOrHalfShuraCouncil;`
- [ ] lib.rs:1383 `E0437`: error[E0437]: type `OnSlash` is not a member of trait `pallet_treasury::Config`
      source: `type OnSlash = Treasury;`
- [ ] lib.rs:1384 `E0437`: error[E0437]: type `ProposalBond` is not a member of trait `pallet_treasury::Config`
      source: `type ProposalBond = ProposalBond;`
- [ ] lib.rs:1385 `E0437`: error[E0437]: type `ProposalBondMinimum` is not a member of trait `pallet_treasury::Config`
      source: `type ProposalBondMinimum = ProposalBondMinimum;`
- [ ] lib.rs:1381 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type RejectOrigin = EnsureRootOrHalfShuraCouncil;`
- [ ] lib.rs:1377 `E0046`: error[E0046]: not all trait items implemented, missing: `SpendOrigin`, `AssetKind`, `Beneficiary`, 
      source: `impl pallet_treasury::Config for Runtime {`

### `onchain::Config` (lib.rs:420) - 5 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:421 `E0437`: error[E0437]: type `BlockWeights` is not a member of trait `onchain::Config`
      source: `type BlockWeights = BlockWeights;`
- [ ] lib.rs:422 `E0437`: error[E0437]: type `AccountId` is not a member of trait `onchain::Config`
      source: `type AccountId = AccountId;`
- [ ] lib.rs:423 `E0437`: error[E0437]: type `BlockNumber` is not a member of trait `onchain::Config`
      source: `type BlockNumber = BlockNumber;`
- [ ] lib.rs:424 `E0437`: error[E0437]: type `Accuracy` is not a member of trait `onchain::Config`
      source: `type Accuracy = sp_runtime::Perbill;`
- [ ] lib.rs:420 `E0046`: error[E0046]: not all trait items implemented, missing: `Sort`, `System`, `Solver`, `MaxBackersPerWinner`, 
      source: `impl onchain::Config for Runtime {`

### `pallet_identity::Config` (lib.rs:523) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1666.

- [ ] lib.rs:527 `E0437`: error[E0437]: type `FieldDeposit` is not a member of trait `pallet_identity::Config`
      source: `type FieldDeposit = FieldDeposit;`
- [ ] lib.rs:530 `E0437`: error[E0437]: type `MaxAdditionalFields` is not a member of trait `pallet_identity::Config`
      source: `type MaxAdditionalFields = MaxAdditionalFields;`
- [ ] lib.rs:533 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance3>: From<&'a 
      source: `type ForceOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;`
- [ ] lib.rs:523 `E0046`: error[E0046]: not all trait items implemented, missing: `ByteDeposit`, `UsernameDeposit`, `IdentityInformation`, 
      source: `impl pallet_identity::Config for Runtime {`

### `pallet_membership::Config<ShuraCouncilMembershipInstance>` (lib.rs:1222) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1224 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type AddOrigin = EnsureRootOrThreeFourthsShuraCouncil;`

### `pallet_membership::Config<FinancialCouncilMembershipInstance>` (lib.rs:1252) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1254 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type AddOrigin = EnsureRootOrTwoThirdsShuraCouncil;`

### `pallet_membership::Config<TechnicalCommitteeMembershipInstance>` (lib.rs:1282) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1284 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type AddOrigin = EnsureRootOrTwoThirdsShuraCouncil;`

### `pallet_membership::Config<OperatorMembershipInstanceSetheum>` (lib.rs:1299) - 5 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1280.

- [ ] lib.rs:1301 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance2>: From<&'a 
      source: `type AddOrigin = EnsureRootOrTwoThirdsFinancialCouncil;`

### `dex_oracle::Config` (lib.rs:900) - 4 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:901 `E0437`: error[E0437]: type `RuntimeEvent` is not a member of trait `dex_oracle::Config`
      source: `type RuntimeEvent = RuntimeEvent;`
- [ ] lib.rs:902 `E0277`: error[E0277]: the trait bound `Pallet<Runtime>: SwapManager<AccountId32, u128, CurrencyId>` is not satisfied
      source: `type DEX = swap_legacy_module::Pallet<Runtime>;`
- [ ] lib.rs:900 `E0046`: error[E0046]: not all trait items implemented, missing: `Time`, `UpdateOrigin`, `WeightInfo`
      source: `impl dex_oracle::Config for Runtime {`
- [ ] lib.rs:979 `E0049`: error[E0049]: associated function `on_unbalanceds` has 2 type parameters but its trait declaration has 1 type parameter
      source: `fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {`

### `pallet_session::historical::Config` (lib.rs:371) - 4 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 694.

- [ ] lib.rs:392 `E0603`: error[E0603]: type alias `EraIndex` is private
      source: `pub const BondingDuration: pallet_staking::EraIndex = 4; // 8 hours (80 mins in test)`
- [ ] lib.rs:371 `E0277`: error[E0277]: the trait bound `RuntimeEvent: From<pallet_session::historical::Event<Runtime>>` is not satisfied
      source: `impl pallet_session::historical::Config for Runtime {`

### `sp_block_builder::BlockBuilder<Block>` (lib.rs:1618) - 4 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1618 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_block_builder::BlockBuilder<Block> for Runtime {`

### `module_oracle::Config<SetheumDataProvider>` (lib.rs:577) - 3 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:578 `E0437`: error[E0437]: type `RuntimeEvent` is not a member of trait `module_oracle::Config`
      source: `type RuntimeEvent = RuntimeEvent;`
- [ ] lib.rs:577 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxFeedValues`
      source: `impl module_oracle::Config<SetheumDataProvider> for Runtime {`
- [ ] lib.rs:599 `E0053`: error[E0053]: method `feed_value` has an incompatible type for trait
      source: `fn feed_value(_: AccountId, _: CurrencyId, _: Price) -> DispatchResult {`

### `module_tokens::Config` (lib.rs:652) - 3 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:653 `E0437`: error[E0437]: type `RuntimeEvent` is not a member of trait `module_tokens::Config`
      source: `type RuntimeEvent = RuntimeEvent;`
- [ ] lib.rs:659 `E0437`: error[E0437]: type `OnDust` is not a member of trait `module_tokens::Config`
      source: `type OnDust = module_tokens::TransferDust<Runtime, TreasuryAccount>;`
- [ ] lib.rs:652 `E0046`: error[E0046]: not all trait items implemented, missing: `CurrencyHooks`, `MaxReserves`, `ReserveIdentifier`
      source: `impl module_tokens::Config for Runtime {`

### `primitives::SetBFTSessionApi<Block>` (lib.rs:1683) - 3 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1683 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl primitives::SetBFTSessionApi<Block> for Runtime {`
- [ ] lib.rs:1726 `E0433`: error[E0433]: failed to resolve: use of undeclared type `CommitteeManagement`
      source: `CommitteeManagement::predict_session_committee_for_session(session)`

### `pallet_bounties::Config` (lib.rs:1394) - 2 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1356.

- [ ] lib.rs:1399 `E0437`: error[E0437]: type `BountyCuratorDeposit` is not a member of trait `pallet_bounties::Config`
      source: `type BountyCuratorDeposit = BountyCuratorDeposit;`
- [ ] lib.rs:1394 `E0046`: error[E0046]: not all trait items implemented, missing: `CuratorDepositMultiplier`, `CuratorDepositMax`, 
      source: `impl pallet_bounties::Config for Runtime {`

### `pallet_insecure_randomness_collective_flip::Config` (lib.rs:1442) - 2 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 369.

- [ ] lib.rs:1588 `E0412`: error[E0412]: cannot find type `AllPallets` in this scope
      source: `AllPallets,`
- [ ] lib.rs:1549 `E0053`: error[E0053]: method `on_runtime_upgrade` has an incompatible type for trait
      source: `fn on_runtime_upgrade() -> u64 {`

### `swap_legacy_module::Config` (lib.rs:855) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:862 `E0277`: error[E0277]: the trait bound `module_dex::WeightInfo<Runtime>: module_swap_legacy::WeightInfo` is not satisfied
      source: `type WeightInfo = weights::module_dex::WeightInfo<Runtime>;`
- [ ] lib.rs:864 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance2>: From<&'a 
      source: `type ListingOrigin = EnsureRootOrHalfFinancialCouncil;`

### `sp_api::Core<Block>` (lib.rs:1598) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1598 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_api::Core<Block> for Runtime {`
- [ ] lib.rs:1607 `E0053`: error[E0053]: method `initialize_block` has an incompatible type for trait
      source: `fn initialize_block(header: &<Block as BlockT>::Header) {`

### `sp_api::Metadata<Block>` (lib.rs:1612) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1612 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_api::Metadata<Block> for Runtime {`
- [ ] lib.rs:1612 `E0046`: error[E0046]: not all trait items implemented, missing: `metadata_at_version`, `metadata_versions`
      source: `impl sp_api::Metadata<Block> for Runtime {`

### `sp_offchain::OffchainWorkerApi<Block>` (lib.rs:1649) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1649 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_offchain::OffchainWorkerApi<Block> for Runtime {`

### `pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>` (lib.rs:1771) - 2 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1771 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {`
- [ ] lib.rs:1771 `E0046`: error[E0046]: not all trait items implemented, missing: `query_weight_to_fee`, `query_length_to_fee`
      source: `impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {`

### `module_authority::Config` (lib.rs:1188) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1189 `E0437`: error[E0437]: type `RuntimeEvent` is not a member of trait `module_authority::Config`
      source: `type RuntimeEvent = RuntimeEvent;`

### `frame_system::Config` (lib.rs:282) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 350.

- [ ] lib.rs:296 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `type Block = Block;`

### `pallet_im_online::Config` (lib.rs:504) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1617.

- [ ] lib.rs:504 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxKeys`, `MaxPeerInHeartbeats`
      source: `impl pallet_im_online::Config for Runtime {`

### `module_currencies::Config` (lib.rs:559) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:565 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type SweepOrigin = EnsureRootOrOneShuraCouncil;`

### `module_prices::Config` (lib.rs:669) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:675 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance2>: From<&'a 
      source: `type LockOrigin = EnsureRootOrTwoThirdsFinancialCouncil;`

### `module_transaction_pause::Config` (lib.rs:697) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:699 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type UpdateOrigin = EnsureRootOrThreeFourthsShuraCouncil;`

### `pallet_proxy::Config` (lib.rs:1116) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 476.

- [ ] lib.rs:1116 `E0046`: error[E0046]: not all trait items implemented, missing: `BlockNumberProvider`
      source: `impl pallet_proxy::Config for Runtime {`

### `pallet_balances::Config` (lib.rs:1140) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 584.

- [ ] lib.rs:1140 `E0046`: error[E0046]: not all trait items implemented, missing: `RuntimeHoldReason`, `RuntimeFreezeReason`, 
      source: `impl pallet_balances::Config for Runtime {`

### `module_vesting::Config` (lib.rs:1159) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1747.

- [ ] lib.rs:1166 `E0277`: error[E0277]: the trait bound `for<'a> &'a pallet_collective::RawOrigin<AccountId32, Instance1>: From<&'a 
      source: `type UpdateOrigin = EnsureRootOrTwoThirdsShuraCouncil;`

### `pallet_scheduler::Config` (lib.rs:1177) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 497.

- [ ] lib.rs:1177 `E0046`: error[E0046]: not all trait items implemented, missing: `OriginPrivilegeCmp`, `Preimages`, `BlockNumberProvider`
      source: `impl pallet_scheduler::Config for Runtime {`

### `pallet_sudo::Config` (lib.rs:1200) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1492.

- [ ] lib.rs:1200 `E0046`: error[E0046]: not all trait items implemented, missing: `WeightInfo`
      source: `impl pallet_sudo::Config for Runtime {`

### `pallet_collective::Config<ShuraCouncilInstance>` (lib.rs:1211) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1211 `E0046`: error[E0046]: not all trait items implemented, missing: `SetMembersOrigin`, `MaxProposalWeight`, `DisapproveOrigin`, 
      source: `impl pallet_collective::Config<ShuraCouncilInstance> for Runtime {`

### `pallet_collective::Config<FinancialCouncilInstance>` (lib.rs:1241) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1241 `E0046`: error[E0046]: not all trait items implemented, missing: `SetMembersOrigin`, `MaxProposalWeight`, `DisapproveOrigin`, 
      source: `impl pallet_collective::Config<FinancialCouncilInstance> for Runtime {`

### `pallet_collective::Config<TechnicalCommitteeInstance>` (lib.rs:1271) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1187.

- [ ] lib.rs:1271 `E0046`: error[E0046]: not all trait items implemented, missing: `SetMembersOrigin`, `MaxProposalWeight`, `DisapproveOrigin`, 
      source: `impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {`

### `pallet_utility::Config` (lib.rs:1312) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 378.

- [ ] lib.rs:1312 `E0046`: error[E0046]: not all trait items implemented, missing: `PalletsOrigin`
      source: `impl pallet_utility::Config for Runtime {`

### `pallet_multisig::Config` (lib.rs:1324) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 392.

- [ ] lib.rs:1324 `E0046`: error[E0046]: not all trait items implemented, missing: `BlockNumberProvider`
      source: `impl pallet_multisig::Config for Runtime {`

### `pallet_tips::Config` (lib.rs:1406) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1404.

- [ ] lib.rs:1406 `E0046`: error[E0046]: not all trait items implemented, missing: `MaxTipAmount`, `OnSlash`
      source: `impl pallet_tips::Config for Runtime {`

### `pallet_recovery::Config` (lib.rs:1424) - 1 errors

Reference: `substrate/bin/node/runtime/src/lib.rs` (kitchensink) line 1698.

- [ ] lib.rs:1424 `E0046`: error[E0046]: not all trait items implemented, missing: `WeightInfo`, `BlockNumberProvider`
      source: `impl pallet_recovery::Config for Runtime {`

### `sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>` (lib.rs:1639) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1639 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {`

### `sp_consensus_aura::AuraApi<Block, AuraId>` (lib.rs:1655) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1655 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {`

### `sp_authority_discovery::AuthorityDiscoveryApi<Block>` (lib.rs:1665) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1665 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {`

### `sp_session::SessionKeys<Block>` (lib.rs:1671) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1671 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl sp_session::SessionKeys<Block> for Runtime {`

### `frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>` (lib.rs:1765) - 1 errors

Reference: no kitchensink equivalent - read the pallet `Config` trait (our crate).

- [ ] lib.rs:1765 `E0277`: error[E0277]: the trait bound `SetheumMultiSignature: DecodeWithMemTracking` is not satisfied
      source: `impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {`

## Errors outside lib.rs

- [ ]  `macro`: error: could not compile `setheum-runtime` (lib) due to 138 previous errors; 13 warnings emitted
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