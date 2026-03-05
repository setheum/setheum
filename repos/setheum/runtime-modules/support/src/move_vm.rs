#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::prelude::*;

/// Precompiles and helper functions for MoveVM integration
/// Native Tokens (SEU, SEUSD) unwrapped integration traits
pub trait MoveVmManager<AccountId> {
    fn get_native_balance(who: &AccountId) -> u128;
}
