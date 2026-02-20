extern crate alloc;

use alloc::{string::String, vec::Vec};
use codec::{Decode, Encode};
use frame_support::weights::Weight;
use sp_runtime::{scale_info::TypeInfo, DispatchError};

pub use move_vm_backend_common::abi::ModuleAbi;

/// Gas estimation information.
#[derive(Clone, PartialEq, Debug, Encode, Decode, TypeInfo)]
pub struct MoveApiEstimation {
    /// Gas used.
    pub gas_used: u64,
    /// Status code for the MoveVM execution.
    pub vm_status_code: u64,
    /// Substrate weight required for the complete extrinsic cost combined with the variable gas
    /// indicated in the [`Estimation`] struct.
    pub total_weight_including_gas_used: Weight,
}

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs` of the node)
sp_api::decl_runtime_apis! {
    pub trait MoveApi<AccountId> where      // AccountID is already here for the next API calls.
        AccountId: codec::Codec,
    {
        // Estimate gas for publishing a module.
        fn estimate_gas_publish_module(account: AccountId, bytecode: Vec<u8>) -> Result<MoveApiEstimation, DispatchError>;

        // Estimate gas for publishing a bundle.
        fn estimate_gas_publish_bundle(account: AccountId, bytecode: Vec<u8>) -> Result<MoveApiEstimation, DispatchError>;

        // Estimate gas for script execution.
        fn estimate_gas_execute_script(transaction: Vec<u8>) -> Result<MoveApiEstimation, DispatchError>;

        // Get module binary by its address.
        fn get_module(address: AccountId, name: String) -> Result<Option<Vec<u8>>, Vec<u8>>;

        // Get module ABI by its address.
        fn get_module_abi(address: AccountId, name: String) -> Result<Option<ModuleAbi>, Vec<u8>>;

        // Get resource.
        fn get_resource(account: AccountId, tag: Vec<u8>) -> Result<Option<Vec<u8>>, Vec<u8>>;
    }
}
