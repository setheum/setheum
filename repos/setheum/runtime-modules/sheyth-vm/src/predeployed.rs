//! Predeployed SheythVM contracts.
//!
//! These are contracts deployed at genesis that provide standard
//! interfaces for Setheum's native functionality. They are written
//! in Rust and compiled to SheythVM bytecode.
//!
//! Each predeployed contract has a fixed address derived from its name.

use sp_std::vec::Vec;

/// A predeployed contract definition.
pub struct PredeployedContract {
	/// The contract address (32 bytes).
	pub address: [u8; 32],
	/// The compiled SheythVM bytecode.
	pub code: Vec<u8>,
	/// Human-readable name.
	pub name: &'static str,
}

/// Derive a predeployed address from a name.
/// Uses blake2s(concat("predeploy", name)) truncated to 20 bytes,
/// then padded to 32 bytes (matching SheythVM account format).
const fn predeploy_address(name: &[u8]) -> [u8; 32] {
	// Placeholder: in practice this uses blake2s at runtime
	let mut addr = [0u8; 32];
	addr[0] = 0xFF; // predeploy prefix
	let mut i = 0;
	while i < name.len() && i < 31 {
		addr[i + 1] = name[i];
		i += 1;
	}
	addr
}

/// All predeployed contracts.
pub fn all_predeployed() -> Vec<PredeployedContract> {
	vec![
		// Standard ERC20 interface for SEU
		PredeployedContract {
			address: predeploy_address(b"SEU"),
			code: predeploy_seu_code(),
			name: "SEU",
		},
		// Standard ERC20 interface for SEUSD
		PredeployedContract {
			address: predeploy_address(b"SEUSD"),
			code: predeploy_seusd_code(),
			name: "SEUSD",
		},
		// DEX router contract
		PredeployedContract {
			address: predeploy_address(b"DEX"),
			code: predeploy_dex_code(),
			name: "DEX",
		},
	]
}

/// SEU predeployed contract bytecode.
pub fn predeploy_seu_code() -> Vec<u8> {
	// This would be the compiled SheythVM bytecode for a contract that
	// implements the ERC20 interface by calling the token precompile host functions.
	// For now, returns empty (to be compiled from Rust source).
	Vec::new()
}

/// SEUSD predeployed contract bytecode.
pub fn predeploy_seusd_code() -> Vec<u8> {
	Vec::new()
}

/// DEX predeployed contract bytecode.
pub fn predeploy_dex_code() -> Vec<u8> {
	Vec::new()
}

/// Test helper: create an address for a named predeployed contract.
pub fn test_address(name: &str) -> [u8; 32] {
	let mut addr = [0u8; 32];
	addr[0] = 0xFF;
	let bytes = name.as_bytes();
	let mut i = 0;
	while i < bytes.len() && i < 31 {
		addr[i + 1] = bytes[i];
		i += 1;
	}
	addr
}
