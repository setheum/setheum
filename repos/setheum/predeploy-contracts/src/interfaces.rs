// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Standard SheythVM predeployed contract interfaces.
//!
//! These are Rust source files compiled to SheythVM bytecode
//! using `sheyth-vm-linker` and registered at genesis.

/// Predeployed contract addresses (prefix 0xFF, then name).
pub mod addresses {
	const fn addr(name: &[u8]) -> [u8; 32] {
		let mut a = [0u8; 32];
		a[0] = 0xFF;
		let mut i = 0;
		while i < name.len() && i < 31 { a[i+1] = name[i]; i += 1; }
		a
	}
	pub const SEU: [u8; 32] = addr(b"SEU");
	pub const SEUSD: [u8; 32] = addr(b"SEUSD");
	pub const DEX: [u8; 32] = addr(b"DEX");
	pub const NFT: [u8; 32] = addr(b"NFT");
	pub const ORACLE: [u8; 32] = addr(b"ORACLE");
}

/// Standard ERC20 interface as SheythVM imports.
///
/// These call the native precompile host functions.
#[sheyth_vm_derive::sheyth_vm_import]
extern "C" {
	#[sheyth_vm_import(symbol = b"sheyth_token_balance_of")]
	fn native_balance_of(address: [u8; 32], currency_id: [u8; 32]) -> [u8; 32];

	#[sheyth_vm_import(symbol = b"sheyth_token_transfer")]
	fn native_transfer(from: [u8; 32], to: [u8; 32], currency_id: [u8; 32], amount: u128) -> u32;

	#[sheyth_vm_import(symbol = b"sheyth_token_total_supply")]
	fn native_total_supply(currency_id: [u8; 32]) -> [u8; 32];

	#[sheyth_vm_import(symbol = b"sheyth_token_name")]
	fn native_name(currency_id: [u8; 32]) -> u32;

	#[sheyth_vm_import(symbol = b"sheyth_token_symbol")]
	fn native_symbol(currency_id: [u8; 32]) -> u32;

	#[sheyth_vm_import(symbol = b"sheyth_token_decimals")]
	fn native_decimals(currency_id: [u8; 32]) -> u32;
}

/// Standard DEX interface as SheythVM imports.
#[sheyth_vm_derive::sheyth_vm_import]
extern "C" {
	#[sheyth_vm_import(symbol = b"sheyth_dex_get_reserves")]
	fn dex_reserves(token_a: [u8; 32], token_b: [u8; 32]) -> [u8; 64];

	#[sheyth_vm_import(symbol = b"sheyth_dex_swap")]
	fn dex_swap(amount_in: u128, amount_out_min: u128, path: u32, to: [u8; 32], deadline: u64) -> [u8; 32];
}

/// Standard Oracle interface as SheythVM imports.
#[sheyth_vm_derive::sheyth_vm_import]
extern "C" {
	#[sheyth_vm_import(symbol = b"sheyth_oracle_get_price")]
	fn oracle_price(base: [u8; 32], quote: [u8; 32]) -> [u8; 32];
}
