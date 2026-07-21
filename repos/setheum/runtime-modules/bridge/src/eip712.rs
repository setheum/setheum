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

/// Port from https://github.com/gakonst/ethers-rs/blob/master/ethers-core/src/types/transaction/eip712.rs
/// Replace hash provided by `sp_io`
use ethabi::{
	encode as abi_encode,
	ethereum_types::{Address, U256},
	token::Token,
};
use sp_io::hashing::keccak_256;
use sp_std::{vec, vec::Vec};

/// Pre-computed value of the following statement:
///
/// `keccak_256("EIP712Domain(string name,string version,uint256 chainId,address
/// verifyingContract)")`
pub const EIP712_DOMAIN_TYPE_HASH: [u8; 32] = [
	139, 115, 195, 198, 155, 184, 254, 61, 81, 46, 204, 76, 247, 89, 204, 121, 35, 159, 123, 23, 155, 15, 250, 202,
	169, 167, 93, 82, 43, 57, 64, 15,
];

/// Pre-computed value of the following statement:
///
/// `keccak_256("EIP712Domain(string name,string version,uint256 chainId,address
/// verifyingContract,bytes32 salt)")`
pub const EIP712_DOMAIN_TYPE_HASH_WITH_SALT: [u8; 32] = [
	216, 124, 214, 239, 121, 212, 226, 185, 94, 21, 206, 138, 191, 115, 45, 181, 30, 199, 113, 241, 202, 46, 220, 207,
	34, 164, 108, 114, 154, 197, 100, 114,
];

/// Eip712 Domain attributes used in determining the domain separator;
/// Unused fields are left out of the struct type.
#[derive(Debug, Default, Clone)]
pub struct EIP712Domain {
	///  The user readable name of signing domain, i.e. the name of the DApp or the protocol.
	pub name: Vec<u8>,

	/// The current major version of the signing domain. Signatures from different versions are not
	/// compatible.
	pub version: Vec<u8>,

	/// The EIP-155 chain id. The user-agent should refuse signing if it does not match the
	/// currently active chain.
	pub chain_id: U256,

	/// The address of the contract that will verify the signature.
	pub verifying_contract: Address,

	/// A disambiguating salt for the protocol. This can be used as a domain separator of last
	/// resort.
	pub salt: Option<[u8; 32]>,
}

impl EIP712Domain {
	// Compute the domain separator;
	// See: https://github.com/gakonst/ethers-rs/blob/master/examples/permit_hash.rs#L41
	pub fn separator(&self) -> [u8; 32] {
		let domain_type_hash =
			if self.salt.is_some() { EIP712_DOMAIN_TYPE_HASH_WITH_SALT } else { EIP712_DOMAIN_TYPE_HASH };

		let mut tokens = vec![
			Token::Uint(U256::from(domain_type_hash)),
			Token::Uint(U256::from(keccak_256(&self.name))),
			Token::Uint(U256::from(keccak_256(&self.version))),
			Token::Uint(self.chain_id),
			Token::Address(self.verifying_contract),
		];

		// Add the salt to the struct to be hashed if it exists;
		if let Some(salt) = &self.salt {
			tokens.push(Token::Uint(U256::from(salt)));
		}

		keccak_256(&abi_encode(&tokens))
	}
}
