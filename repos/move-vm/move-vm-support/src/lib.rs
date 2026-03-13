//! Library with helper functions for using MoveVM in Substrate-based chains.

pub mod base58_address;
pub mod ss58_address;

// Public key length in bytes
pub(crate) const PUB_KEY_LEN: usize = 32;

// Checksum length in bytes
pub(crate) const CHECKSUM_LEN: usize = 2;

// Blake2b512 hash length in bytes
pub(crate) const HASH_LEN: usize = 64;

// Maximum supported address type length in bytes
pub(crate) const ADDR_TYPE_MAX_LEN: usize = 2;

// Minimum supported address type length in bytes
pub(crate) const ADDR_TYPE_MIN_LEN: usize = 1;
