// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
