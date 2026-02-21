// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

use crate::{PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult};
use module_evm_utility::evm::{ExitError, ExitSucceed};
use sp_std::vec::Vec;

mod blake2;
mod bn128;
mod ecrecover;
mod ecrecover_publickey;
mod identity;
mod modexp;
mod ripemd;
mod sha256;
mod sha3fips;

pub use self::ripemd::Ripemd160;
pub use blake2::Blake2F;
pub use bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
pub use ecrecover::ECRecover;
pub use ecrecover_publickey::ECRecoverPublicKey;
pub use identity::Identity;
pub use modexp::{IstanbulModexp, Modexp};
pub use sha256::Sha256;
pub use sha3fips::{Sha3FIPS256, Sha3FIPS512};

/// One single precompile used by EVM engine.
pub trait Precompile {
/// Try to execute the precompile. Calculate the amount of gas needed with given `input` and
/// `target_gas`. Return `Ok(status, output, gas_used)` if the execution is
/// successful. Otherwise return `Err(_)`.
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult;

	#[cfg(feature = "evm-tests")]
	fn execute_ext(
		input: &[u8],
		target_gas: Option<u64>,
		context: &crate::Context,
		is_static: bool,
	) -> Result<(PrecompileOutput, u64), PrecompileFailure>;
}

pub trait LinearCostPrecompile {
	const BASE: u64;
	const WORD: u64;

	fn execute(input: &[u8], cost: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), PrecompileFailure>;
}

impl<T: LinearCostPrecompile> Precompile for T {
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let target_gas = handle.gas_limit();
		let cost = ensure_linear_cost(target_gas, handle.input().len() as u64, T::BASE, T::WORD)?;

		handle.record_cost(cost)?;
		let (exit_status, output) = T::execute(handle.input(), cost)?;
		Ok(PrecompileOutput { exit_status, output })
	}

	#[cfg(feature = "evm-tests")]
	fn execute_ext(
		input: &[u8],
		target_gas: Option<u64>,
		context: &crate::Context,
		is_static: bool,
	) -> Result<(PrecompileOutput, u64), PrecompileFailure> {
		let cost = ensure_linear_cost(target_gas, input.len() as u64, T::BASE, T::WORD)?;
		let (exit_status, output) = T::execute(input, cost)?;
		Ok((PrecompileOutput { exit_status, output }, cost))
	}
}

/// Linear gas cost
fn ensure_linear_cost(target_gas: Option<u64>, len: u64, base: u64, word: u64) -> Result<u64, PrecompileFailure> {
	let cost = base
		.checked_add(
			word.checked_mul(len.saturating_add(31) // 32)
				.ok_or(PrecompileFailure::Error {
					exit_status: ExitError::OutOfGas,
				})?,
		)
		.ok_or(PrecompileFailure::Error {
			exit_status: ExitError::OutOfGas,
		})?;

	if let Some(target_gas) = target_gas {
		if cost > target_gas {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::OutOfGas,
			});
		}
	}

	Ok(cost)
}

pub mod tests {
	use crate::{ExitError, ExitReason, PrecompileHandle};
	use module_evm_utility::evm::{Context, Transfer};
	use sp_core::{H160, H256};
	use sp_std::vec::Vec;

	pub struct MockPrecompileHandle<'inner> {
		pub input: &'inner [u8],
		pub code_address: H160,
		pub gas_limit: Option<u64>,
		pub gas_used: u64,
		pub context: &'inner Context,
		pub is_static: bool,
	}

	impl<'inner> MockPrecompileHandle<'inner> {
		pub fn new(input: &'inner [u8], gas_limit: Option<u64>, context: &'inner Context, is_static: bool) -> Self {
			Self {
				input,
				code_address: H160::default(),
				gas_limit,
				gas_used: 0,
				context,
				is_static,
			}
		}
	}

	impl<'inner> PrecompileHandle for MockPrecompileHandle<'inner> {
		fn call(
			&mut self,
			_: H160,
			_: Option<Transfer>,
			_: Vec<u8>,
			_: Option<u64>,
			_: bool,
			_: &Context,
		) -> (ExitReason, Vec<u8>) {
			unimplemented!()
		}

		fn record_cost(&mut self, cost: u64) -> Result<(), ExitError> {
			self.gas_used += cost;

			if let Some(gas_limit) = self.gas_limit {
				if self.gas_used > gas_limit {
					Err(ExitError::OutOfGas)
				} else {
					Ok(())
				}
			} else {
				Ok(())
			}
		}

		fn record_external_cost(
			&mut self,
			_ref_time: Option<u64>,
			_proof_size: Option<u64>,
			_storage_growth: Option<u64>,
		) -> Result<(), ExitError> {
			unimplemented!()
		}

		fn refund_external_cost(&mut self, _ref_time: Option<u64>, _proof_size: Option<u64>) {
			unimplemented!()
		}

		fn remaining_gas(&self) -> u64 {
			unimplemented!()
		}

		fn log(&mut self, _address: H160, _topics: Vec<H256>, _data: Vec<u8>) -> Result<(), ExitError> {
			unimplemented!()
		}

		fn code_address(&self) -> H160 {
			self.code_address
		}

		fn input(&self) -> &[u8] {
			self.input
		}

		fn context(&self) -> &Context {
			self.context
		}

		fn is_static(&self) -> bool {
			self.is_static
		}

		fn gas_limit(&self) -> Option<u64> {
			self.gas_limit
		}
	}
}
