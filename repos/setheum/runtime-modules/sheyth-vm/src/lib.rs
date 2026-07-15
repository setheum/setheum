#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

pub use pallet::*;
pub mod precompiles;
pub mod predeployed;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ContractDeployed { deployer: T::AccountId, contract: [u8; 32] },
		ContractCalled { caller: T::AccountId, contract: [u8; 32] },
	}

	#[pallet::error]
	pub enum Error<T> {
		ContractNotFound,
		ExecutionFailed,
		OutOfGas,
	}

	#[pallet::storage]
	pub type ContractCode<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], Vec<u8>, OptionQuery>;

	#[pallet::storage]
	pub type ContractNonce<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 32], u64, ValueQuery>;
}
