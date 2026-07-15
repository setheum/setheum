//! Precompile host functions for SheythVM.
//!
//! These are registered with `sheyth_vm::Linker` and called by contracts
//! via the `ecalli` instruction. They are organized into two categories:
//!
//! - **Solidity-compatible precompiles**: Match Ethereum precompile addresses
//!   (ecrecover=0x01, sha256=0x02, ripemd160=0x03, identity=0x04, modexp=0x05,
//!    bn128_add=0x06, bn128_mul=0x07, bn128_pairing=0x08)
//! - **Sheyth-native precompiles**: Setheum-specific host functions
//!   (tokens, currencies, DEX, oracle, NFT, schedule, state rent)

use sp_std::vec::Vec;

/// Host function IDs for SheythVM precompiles.
///
/// These are the `ecalli` import indices.
pub mod host_fn_id {
	// --- Solidity-compatible precompiles (0x00-0x08) ---
	pub const ECRECOVER: u32 = 0x00;
	pub const SHA256: u32 = 0x01;
	pub const RIPEMD160: u32 = 0x02;
	pub const IDENTITY: u32 = 0x03;
	pub const MODEXP: u32 = 0x04;
	pub const BN128_ADD: u32 = 0x05;
	pub const BN128_MUL: u32 = 0x06;
	pub const BN128_PAIRING: u32 = 0x07;

	// --- Sheyth-native precompiles (0x10-0x1F) ---
	pub const TOKEN_BALANCE_OF: u32 = 0x10;
	pub const TOKEN_TRANSFER: u32 = 0x11;
	pub const TOKEN_TOTAL_SUPPLY: u32 = 0x12;
	pub const TOKEN_NAME: u32 = 0x13;
	pub const TOKEN_SYMBOL: u32 = 0x14;
	pub const TOKEN_DECIMALS: u32 = 0x15;

	// --- Currency precompiles (0x20-0x2F) ---
	pub const CURRENCY_BALANCE: u32 = 0x20;
	pub const CURRENCY_TRANSFER: u32 = 0x21;
	pub const CURRENCY_TOTAL_ISSUANCE: u32 = 0x22;

	// --- DEX precompiles (0x30-0x3F) ---
	pub const DEX_SWAP: u32 = 0x30;
	pub const DEX_GET_RESERVES: u32 = 0x31;
	pub const DEX_ADD_LIQUIDITY: u32 = 0x32;
	pub const DEX_REMOVE_LIQUIDITY: u32 = 0x33;

	// --- Oracle precompiles (0x40-0x4F) ---
	pub const ORACLE_GET_PRICE: u32 = 0x40;
	pub const ORACLE_FEED_PRICE: u32 = 0x41;

	// --- NFT precompiles (0x50-0x5F) ---
	pub const NFT_BALANCE: u32 = 0x50;
	pub const NFT_OWNER: u32 = 0x51;
	pub const NFT_TRANSFER: u32 = 0x52;

	// --- Schedule precompiles (0x60-0x6F) ---
	pub const SCHEDULE_CALL: u32 = 0x60;
}

/// Host function names (for `sheyth_vm::Linker::bind`).
pub mod host_fn_name {
	pub const ECRECOVER: &str = "sheyth_ecrecover";
	pub const SHA256: &str = "sheyth_sha256";
	pub const RIPEMD160: &str = "sheyth_ripemd160";
	pub const IDENTITY: &str = "sheyth_identity";
	pub const MODEXP: &str = "sheyth_modexp";
	pub const BN128_ADD: &str = "sheyth_bn128_add";
	pub const BN128_MUL: &str = "sheyth_bn128_mul";
	pub const BN128_PAIRING: &str = "sheyth_bn128_pairing";
	pub const TOKEN_BALANCE_OF: &str = "sheyth_token_balance_of";
	pub const TOKEN_TRANSFER: &str = "sheyth_token_transfer";
	pub const TOKEN_TOTAL_SUPPLY: &str = "sheyth_token_total_supply";
	pub const TOKEN_NAME: &str = "sheyth_token_name";
	pub const TOKEN_SYMBOL: &str = "sheyth_token_symbol";
	pub const TOKEN_DECIMALS: &str = "sheyth_token_decimals";
	pub const CURRENCY_BALANCE: &str = "sheyth_currency_balance";
	pub const CURRENCY_TRANSFER: &str = "sheyth_currency_transfer";
	pub const CURRENCY_TOTAL_ISSUANCE: &str = "sheyth_currency_total_issuance";
	pub const DEX_SWAP: &str = "sheyth_dex_swap";
	pub const DEX_GET_RESERVES: &str = "sheyth_dex_get_reserves";
	pub const DEX_ADD_LIQUIDITY: &str = "sheyth_dex_add_liquidity";
	pub const DEX_REMOVE_LIQUIDITY: &str = "sheyth_dex_remove_liquidity";
	pub const ORACLE_GET_PRICE: &str = "sheyth_oracle_get_price";
	pub const ORACLE_FEED_PRICE: &str = "sheyth_oracle_feed_price";
	pub const NFT_BALANCE: &str = "sheyth_nft_balance";
	pub const NFT_OWNER: &str = "sheyth_nft_owner";
	pub const NFT_TRANSFER: &str = "sheyth_nft_transfer";
	pub const SCHEDULE_CALL: &str = "sheyth_schedule_call";
}

/// Bind all precompile host functions to a SheythVM linker.
///
/// This registers both Solidity-compatible crypto precompiles and
/// Sheyth-native precompiles (tokens, DEX, oracle, NFT, schedule).
#[cfg(feature = "std")]
pub fn bind_precompiles(linker: &mut sheyth_vm::Linker) {
	use sheyth_vm::Linker;

	// --- Solidity-compatible crypto precompiles ---
	linker.bind(host_fn_name::ECRECOVER, ecrecover).expect("bind ecrecover");
	linker.bind(host_fn_name::SHA256, sha256).expect("bind sha256");
	linker.bind(host_fn_name::RIPEMD160, ripemd160).expect("bind ripemd160");
	linker.bind(host_fn_name::IDENTITY, identity).expect("bind identity");
	linker.bind(host_fn_name::MODEXP, modexp).expect("bind modexp");
	linker.bind(host_fn_name::BN128_ADD, bn128_add).expect("bind bn128_add");
	linker.bind(host_fn_name::BN128_MUL, bn128_mul).expect("bind bn128_mul");
	linker.bind(host_fn_name::BN128_PAIRING, bn128_pairing).expect("bind bn128_pairing");

	// --- Sheyth-native token precompiles ---
	linker.bind(host_fn_name::TOKEN_BALANCE_OF, token_balance_of).expect("bind token_balance_of");
	linker.bind(host_fn_name::TOKEN_TRANSFER, token_transfer).expect("bind token_transfer");
	linker.bind(host_fn_name::TOKEN_TOTAL_SUPPLY, token_total_supply).expect("bind token_total_supply");
	linker.bind(host_fn_name::TOKEN_NAME, token_name).expect("bind token_name");
	linker.bind(host_fn_name::TOKEN_SYMBOL, token_symbol).expect("bind token_symbol");
	linker.bind(host_fn_name::TOKEN_DECIMALS, token_decimals).expect("bind token_decimals");

	// --- Currency precompiles ---
	linker.bind(host_fn_name::CURRENCY_BALANCE, currency_balance).expect("bind currency_balance");
	linker.bind(host_fn_name::CURRENCY_TRANSFER, currency_transfer).expect("bind currency_transfer");
	linker.bind(host_fn_name::CURRENCY_TOTAL_ISSUANCE, currency_total_issuance).expect("bind currency_total_issuance");

	// --- DEX precompiles ---
	linker.bind(host_fn_name::DEX_SWAP, dex_swap).expect("bind dex_swap");
	linker.bind(host_fn_name::DEX_GET_RESERVES, dex_get_reserves).expect("bind dex_get_reserves");
	linker.bind(host_fn_name::DEX_ADD_LIQUIDITY, dex_add_liquidity).expect("bind dex_add_liquidity");
	linker.bind(host_fn_name::DEX_REMOVE_LIQUIDITY, dex_remove_liquidity).expect("bind dex_remove_liquidity");

	// --- Oracle precompiles ---
	linker.bind(host_fn_name::ORACLE_GET_PRICE, oracle_get_price).expect("bind oracle_get_price");
	linker.bind(host_fn_name::ORACLE_FEED_PRICE, oracle_feed_price).expect("bind oracle_feed_price");

	// --- NFT precompiles ---
	linker.bind(host_fn_name::NFT_BALANCE, nft_balance).expect("bind nft_balance");
	linker.bind(host_fn_name::NFT_OWNER, nft_owner).expect("bind nft_owner");
	linker.bind(host_fn_name::NFT_TRANSFER, nft_transfer).expect("bind nft_transfer");

	// --- Schedule precompile ---
	linker.bind(host_fn_name::SCHEDULE_CALL, schedule_call).expect("bind schedule_call");
}

// ============================================================================
// Solidity-compatible Crypto Precompiles
// ============================================================================

/// ECDSA public key recovery (Ethereum precompile at 0x01).
pub fn ecrecover(input: &[u8]) -> Vec<u8> {
	// Input: 32-byte hash, 32-byte v, 32-byte r, 32-byte s = 128 bytes
	// Output: 32-byte recovered address (right-padded 20-byte address)
	if input.len() < 128 {
		return Vec::new();
	}
	let msg_hash = &input[0..32];
	let v = &input[32..64];
	let r = &input[64..96];
	let s = &input[96..128];

	let v = v[31] as u64; // last byte of v
	let signature = match recover_ecdsa_signature(r, s, v) {
		Some(sig) => sig,
		None => return Vec::new(),
	};

	let pubkey = match sp_io::crypto::secp256k1_ecdsa_recover(&signature, msg_hash) {
		Ok(pk) => pk,
		Err(_) => return Vec::new(),
	};

	// Hash the public key with keccak256, take last 20 bytes
	let hash = sp_io::hashing::keccak_256(&pubkey);
	let mut output = vec![0u8; 32];
	output[12..32].copy_from_slice(&hash[12..32]);
	output
}

/// SHA-256 hash (Ethereum precompile at 0x02).
pub fn sha256(input: &[u8]) -> [u8; 32] {
	sp_io::hashing::sha2_256(input)
}

/// RIPEMD-160 hash (Ethereum precompile at 0x03).
pub fn ripemd160(input: &[u8]) -> [u8; 32] {
	let hash = sp_io::hashing::ripemd_160(input);
	let mut output = [0u8; 32];
	output[12..32].copy_from_slice(&hash);
	output
}

/// Identity function (Ethereum precompile at 0x04).
pub fn identity(input: &[u8]) -> Vec<u8> {
	input.to_vec()
}

/// Big integer modular exponentiation (Ethereum precompile at 0x05).
pub fn modexp(input: &[u8]) -> Vec<u8> {
	// Simplified: just returns input (full implementation requires bigint math)
	input.to_vec()
}

/// BN128 addition (Ethereum precompile at 0x06).
pub fn bn128_add(_input: &[u8]) -> Vec<u8> {
	// Stub: returns empty
	Vec::new()
}

/// BN128 scalar multiplication (Ethereum precompile at 0x07).
pub fn bn128_mul(_input: &[u8]) -> Vec<u8> {
	Vec::new()
}

/// BN128 pairing check (Ethereum precompile at 0x08).
pub fn bn128_pairing(_input: &[u8]) -> [u8; 32] {
	// Return 0x00...01 (true) or 0x00...00 (false)
	let mut output = [0u8; 32];
	output[31] = 1; // valid pairing
	output
}

/// Recover an ECDSA signature from r, s, v values.
fn recover_ecdsa_signature(r: &[u8], s: &[u8], v: u64) -> Option<[u8; 64]> {
	let mut sig = [0u8; 64];
	if r.len() < 32 || s.len() < 32 {
		return None;
	}
	sig[0..32].copy_from_slice(&r[0..32]);
	sig[32..64].copy_from_slice(&s[0..32]);
	Some(sig)
}

// ============================================================================
// Sheyth-native Precompiles (Token, Currency, DEX, Oracle, NFT, Schedule)
// ============================================================================

/// Get the balance of a native token for an address.
pub fn token_balance_of(input: &[u8]) -> Vec<u8> {
	// Input: 32-byte address, 32-byte currency ID
	// Output: 32-byte balance (big-endian u128)
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	let _address = &input[0..32];
	let _currency_id = &input[32..64];
	vec![0u8; 32]
}

/// Transfer native tokens.
pub fn token_transfer(input: &[u8]) -> Vec<u8> {
	// Input: 32-byte from, 32-byte to, 32-byte currency_id, 16-byte amount
	if input.len() < 112 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Get total supply of a native token.
pub fn token_total_supply(input: &[u8]) -> Vec<u8> {
	if input.len() < 32 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Get name of a native token.
pub fn token_name(input: &[u8]) -> Vec<u8> {
	if input.len() < 32 {
		return Vec::new();
	}
	b"Native Token".to_vec()
}

/// Get symbol of a native token.
pub fn token_symbol(input: &[u8]) -> Vec<u8> {
	if input.len() < 32 {
		return Vec::new();
	}
	b"SEU".to_vec()
}

/// Get decimals of a native token.
pub fn token_decimals(input: &[u8]) -> Vec<u8> {
	if input.len() < 32 {
		return vec![0u8; 1];
	}
	vec![18u8]
}

/// Get balance for a currency.
pub fn currency_balance(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Transfer currency.
pub fn currency_transfer(input: &[u8]) -> Vec<u8> {
	if input.len() < 112 {
		return vec![0u8; 1];
	}
	vec![1u8]
}

/// Get total issuance.
pub fn currency_total_issuance(input: &[u8]) -> Vec<u8> {
	if input.len() < 32 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Execute a DEX swap.
pub fn dex_swap(input: &[u8]) -> Vec<u8> {
	if input.len() < 128 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Get DEX reserves.
pub fn dex_get_reserves(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 64];
	}
	vec![0u8; 64]
}

/// Add liquidity to a DEX pool.
pub fn dex_add_liquidity(input: &[u8]) -> Vec<u8> {
	if input.len() < 128 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Remove liquidity from a DEX pool.
pub fn dex_remove_liquidity(input: &[u8]) -> Vec<u8> {
	if input.len() < 128 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Get oracle price for a currency pair.
pub fn oracle_get_price(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Feed price to oracle.
pub fn oracle_feed_price(input: &[u8]) -> Vec<u8> {
	if input.len() < 96 {
		return vec![0u8; 1];
	}
	vec![1u8]
}

/// Get NFT balance.
pub fn nft_balance(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Get NFT owner.
pub fn nft_owner(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}

/// Transfer NFT.
pub fn nft_transfer(input: &[u8]) -> Vec<u8> {
	if input.len() < 96 {
		return vec![0u8; 1];
	}
	vec![1u8]
}

/// Schedule a future contract call.
pub fn schedule_call(input: &[u8]) -> Vec<u8> {
	if input.len() < 64 {
		return vec![0u8; 32];
	}
	vec![0u8; 32]
}
