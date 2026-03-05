#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::prelude::*;

/// Precompiles/Chain Extensions and helper functions for WASM (pallet-contracts) integration
/// Native Tokens unwrapped integration traits
pub trait WasmContractsManager<AccountId> {
    fn get_native_balance(who: &AccountId) -> u128;
}
