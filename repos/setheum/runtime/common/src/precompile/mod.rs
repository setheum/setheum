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

#![allow(clippy::upper_case_acronyms)]

mod mock;
mod tests;

use fp_evm::{
	Context, ExitError, ExitRevert, ExitSucceed, Precompile, PrecompileFailure, PrecompileOutput, PrecompileSet,
};
use pallet_evm::{IsPrecompileResult, PrecompileHandle};
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_ecrecover::ECRecover;
use pallet_evm_precompile_identity::Identity;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_ripemd160::Ripemd160;
use pallet_evm_precompile_sha2::Sha256;
use module_support::PrecompileCallerFilter as PrecompileCallerFilterT;
use primitives::PRECOMPILE_ADDRESS_START;
use sp_core::H160;
use sp_std::marker::PhantomData;

pub mod dex;
pub mod input;
pub mod multicurrency;
pub mod nft;
pub mod oracle;
pub mod schedule_call;
pub mod state_rent;

pub use dex::DexPrecompile;
pub use multicurrency::MultiCurrencyPrecompile;
pub use nft::NFTPrecompile;
pub use oracle::OraclePrecompile;
pub use schedule_call::ScheduleCallPrecompile;
pub use state_rent::StateRentPrecompile;

pub struct AllPrecompiles<
	PrecompileCallerFilter,
	MultiCurrencyPrecompile,
	NFTPrecompile,
	StateRentPrecompile,
	OraclePrecompile,
	ScheduleCallPrecompile,
	DexPrecompile,
>(
	PhantomData<(
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	)>,
);

impl<
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	> PrecompileSet
	for AllPrecompiles<
		PrecompileCallerFilter,
		MultiCurrencyPrecompile,
		NFTPrecompile,
		StateRentPrecompile,
		OraclePrecompile,
		ScheduleCallPrecompile,
		DexPrecompile,
	>
where
	MultiCurrencyPrecompile: Precompile,
	NFTPrecompile: Precompile,
	StateRentPrecompile: Precompile,
	OraclePrecompile: Precompile,
	ScheduleCallPrecompile: Precompile,
	PrecompileCallerFilter: PrecompileCallerFilterT,
	DexPrecompile: Precompile,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<core::result::Result<PrecompileOutput, PrecompileFailure>> {
		let address = handle.code_address();

		if address == H160::from_low_u64_be(1) {
			Some(ECRecover::execute(handle))
		} else if address == H160::from_low_u64_be(2) {
			Some(Sha256::execute(handle))
		} else if address == H160::from_low_u64_be(3) {
			Some(Ripemd160::execute(handle))
		} else if address == H160::from_low_u64_be(4) {
			Some(Identity::execute(handle))
		} else if address == H160::from_low_u64_be(5) {
			Some(Modexp::execute(handle))
		} else if address == H160::from_low_u64_be(6) {
			Some(Bn128Add::execute(handle))
		} else if address == H160::from_low_u64_be(7) {
			Some(Bn128Mul::execute(handle))
		} else if address == H160::from_low_u64_be(8) {
			Some(Bn128Pairing::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START) {
			Some(MultiCurrencyPrecompile::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 1) {
			Some(NFTPrecompile::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 2) {
			Some(StateRentPrecompile::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 3) {
			Some(OraclePrecompile::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 4) {
			Some(ScheduleCallPrecompile::execute(handle))
		} else if address == H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 5) {
			Some(DexPrecompile::execute(handle))
		} else {
			None
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: (address >= H160::from_low_u64_be(1) && address <= H160::from_low_u64_be(9))
				|| (address >= H160::from_low_u64_be(PRECOMPILE_ADDRESS_START)
					&& address <= H160::from_low_u64_be(PRECOMPILE_ADDRESS_START + 5)),
			extra_cost: 0,
		}
	}
}
