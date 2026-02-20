use frame_support::{
    dispatch::DispatchErrorWithPostInfo,
    pallet_prelude::{DispatchError, DispatchResultWithPostInfo, Weight},
    traits::{OnFinalize, OnIdle, OnInitialize},
};
use frame_support::{
    parameter_types,
    traits::{ConstU128, ConstU16, ConstU32, ConstU64},
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_runtime::BuildStorage;

use crate as pallet_move;

pub use move_core_types::account_address::AccountAddress;
pub use move_vm_backend_common::types::ScriptTransaction;
pub use sp_runtime::AccountId32;

// Internally, within move-vm-backend, the gas is of type u64 and the maximum value is even greater.
pub const MAX_GAS_AMOUNT: u32 = u32::MAX;

// Primitive type definitions for this mockup.
pub type Balance = u128;
pub type Block = frame_system::mocking::MockBlock<Test>;

pub const EXISTENTIAL_DEPOSIT: Balance = 100;

mod constants {
    // Key constants or frequently used ones.
    pub const EMPTY_CHEQUE: super::Balance = 0;
    pub const COIN_PRICE: u128 = 1_000_000_000_000;
}

pub use constants::*;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        MoveModule: pallet_move,
    }
);

impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId32;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type MultiBlockMigrator = ();
    type Nonce = u64;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeTask = RuntimeTask;
    type SingleBlockMigrations = ();
    type SS58Prefix = ConstU16<42>;
    type SystemWeightInfo = ();
    type Version = ();
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

parameter_types! {
    pub const MultisigReqExpireTime: BlockNumberFor<Test> = 5;
    pub const MaxScriptSigners: u32 = 8;
}

impl pallet_move::Config for Test {
    type Currency = Balances;
    type CurrencyBalance = Balance;
    type MultisigReqExpireTime = MultisigReqExpireTime;
    type MaxScriptSigners = MaxScriptSigners;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_move::weights::SubstrateWeight<Test>;
}

/// Test Externalities Builder for an easier test setup.
#[derive(Default)]
pub(crate) struct ExtBuilder {
    /// Overwrite default accounts with balances.
    balances: Vec<(AccountId32, Balance)>,
    /// Overwrite default Move-stdlib setup.
    move_stdlib: Option<Vec<u8>>,
    /// Overwrite default Substrate-stdlib.
    substrate_stdlib: Option<Vec<u8>>,
}

impl ExtBuilder {
    /// Overwrites default balances on dev-test setup.
    pub(crate) fn with_balances(mut self, balances: Vec<(AccountId32, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    /// Overwrites default Move-stdlib dev-test setup.
    pub(crate) fn with_move_stdlib(mut self, move_stdlib: Option<Vec<u8>>) -> Self {
        self.move_stdlib = move_stdlib;
        self
    }

    /// Overwrites default Substrate-stdlib on dev-test setup.
    pub(crate) fn with_substrate_stdlib(mut self, sub_stdlib: Option<Vec<u8>>) -> Self {
        self.substrate_stdlib = sub_stdlib;
        self
    }

    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut ext = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances.clone(),
        }
        .assimilate_storage(&mut ext)
        .expect("Pallet balances storage cannot be assimilated");

        pallet_move::GenesisConfig::<Test> {
            _phantom: core::marker::PhantomData,
            change_default_move_stdlib_bundle_to: self.move_stdlib.clone(),
            change_default_substrate_stdlib_bundle_to: self.substrate_stdlib.clone(),
        }
        .assimilate_storage(&mut ext)
        .expect("Pallet Move storage cannot be assimilated");

        ext.into()
    }
}

/// Rolls forward in future to the given block height.
pub(crate) fn roll_to(n: BlockNumberFor<Test>) {
    let weight = Weight::from_parts(100_000_000_000, 1);
    while System::block_number() < n {
        <AllPalletsWithSystem as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <AllPalletsWithSystem as OnIdle<u64>>::on_idle(System::block_number(), weight);
        <AllPalletsWithSystem as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

/// Returns the last emitted event by the blockchain.
pub(crate) fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

/// In case of an error returned from MoveVM, this method compares the encapsuled error string at
/// the level of the returned `DispatchResultWithPostInfo`.
pub(crate) fn verify_module_error_with_msg(
    res: DispatchResultWithPostInfo,
    e_msg: &str,
) -> Result<bool, String> {
    if let Err(DispatchErrorWithPostInfo {
        error: DispatchError::Module(moderr),
        ..
    }) = res
    {
        if let Some(msg) = moderr.message {
            return Ok(msg == e_msg);
        }
    }
    Err(format!("{res:?} does not match '{e_msg}'"))
}
