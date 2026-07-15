//! The spinit crate provides a sandboxed runtime for testing SheythVM smart contracts
//! without a need for a running node.

#![warn(missing_docs)]

pub mod errors;
#[cfg(feature = "session")]
pub mod session;

#[cfg(feature = "macros")]
pub use spinit_test_macro::{contract_bundle_provider, test};
pub use errors::Error;
pub use sp_runtime::{AccountId32, DispatchError};

/// Main result type for the spinit crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

