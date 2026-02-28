
use pallet_evm::{Precompile, PrecompileHandle, PrecompileResult, PrecompileSet};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::{Sha3FIPS256, Sha3FIPS512};
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

pub struct SetheumPrecompiles<R>(PhantomData<R>);

impl<R> SetheumPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<R> PrecompileSet for SetheumPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let address = handle.code_address();

		if let Some(_) = crate::currency_precompile::get_currency_id(address) {
			return crate::currency_precompile::CurrencyPrecompile::<R>::execute(handle);
		}

		if address == H160::from_low_u64_be(1) {
			return Some(ECRecover::execute(handle));
		}
		if address == H160::from_low_u64_be(2) {
			return Some(Sha256::execute(handle));
		}
		if address == H160::from_low_u64_be(3) {
			return Some(Ripemd160::execute(handle));
		}
		if address == H160::from_low_u64_be(4) {
			return Some(Identity::execute(handle));
		}
		if address == H160::from_low_u64_be(5) {
			return Some(Modexp::execute(handle));
		}
		if address == H160::from_low_u64_be(1024) {
			return Some(Sha3FIPS256::execute(handle));
		}
		if address == H160::from_low_u64_be(1025) {
			return Some(Sha3FIPS512::execute(handle));
		}
		if address == H160::from_low_u64_be(1026) {
			return Some(ECRecoverPublicKey::execute(handle));
		}

		None
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> fp_evm::IsPrecompileResult {
		use fp_evm::IsPrecompileResult;
		
		if let Some(_) = crate::currency_precompile::get_currency_id(address) {
			return IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 };
		}

		if address > H160::zero() && address <= H160::from_low_u64_be(5) {
			return IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 };
		}
		if address >= H160::from_low_u64_be(1024) && address <= H160::from_low_u64_be(1026) {
			return IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 };
		}
		IsPrecompileResult::Answer { is_precompile: false, extra_cost: 0 }
	}
}
