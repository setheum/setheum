use std::sync::Arc;

use codec::Codec;
use frame_support::weights::Weight;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    types::{error::ErrorCode, ErrorObjectOwned},
};
pub use pallet_move::api::{ModuleAbi, MoveApi as MoveRuntimeApi, MoveApiEstimation};
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;

/// Gas estimation information.
#[derive(Clone, Serialize, Deserialize)]
pub struct Estimation {
    /// Gas used.
    pub gas_used: u64,
    /// Status code for the MoveVM execution.
    pub vm_status_code: u64,
    /// Substrate weight required for the complete extrinsic cost combined with the variable
    /// used gas indicated in the [`Estimation`] struct.
    pub total_weight_including_gas_used: Weight,
}

impl From<MoveApiEstimation> for Estimation {
    fn from(estimate: MoveApiEstimation) -> Self {
        Self {
            gas_used: estimate.gas_used,
            vm_status_code: estimate.vm_status_code,
            total_weight_including_gas_used: estimate.total_weight_including_gas_used,
        }
    }
}

/// Public RPC API of the Move pallet.
#[rpc(client, server)]
pub trait MoveApi<BlockHash, AccountId> {
    /// Estimate gas for publishing module.
    #[method(name = "mvm_estimateGasPublishModule")]
    fn estimate_gas_publish_module(
        &self,
        account: AccountId,
        bytecode: Vec<u8>,
        at: Option<BlockHash>,
    ) -> RpcResult<Estimation>;

    /// Estimate gas for publishing bundle.
    #[method(name = "mvm_estimateGasPublishBundle")]
    fn estimate_gas_publish_bundle(
        &self,
        account: AccountId,
        bytecode: Vec<u8>,
        at: Option<BlockHash>,
    ) -> RpcResult<Estimation>;

    /// Estimate gas for executing Move script.
    #[method(name = "mvm_estimateGasExecuteScript")]
    fn estimate_gas_execute_script(
        &self,
        transaction: Vec<u8>,
        at: Option<BlockHash>,
    ) -> RpcResult<Estimation>;

    /// Get resource.
    #[method(name = "mvm_getResource")]
    fn get_resource(
        &self,
        account: AccountId,
        tag: Vec<u8>,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u8>>>;

    /// Get module ABI using address.
    #[method(name = "mvm_getModuleABI")]
    fn get_module_abi(
        &self,
        address: AccountId,
        name: &str,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<ModuleAbi>>;

    /// Get module binary using address.
    #[method(name = "mvm_getModule")]
    fn get_module(
        &self,
        address: AccountId,
        name: &str,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Vec<u8>>>;
}

/// A struct that implements the `MoveApi`.
pub struct MovePallet<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> MovePallet<C, Block> {
    /// Create new `MovePallet` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> MoveApiServer<<Block as BlockT>::Hash, AccountId> for MovePallet<C, Block>
where
    Block: BlockT,
    AccountId: Clone + std::fmt::Display + Codec,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: MoveRuntimeApi<Block, AccountId>,
{
    fn estimate_gas_publish_module(
        &self,
        account: AccountId,
        bytecode: Vec<u8>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Estimation> {
        let api = self.client.runtime_api();
        let res = api
            .estimate_gas_publish_module(
                at.unwrap_or_else(|| self.client.info().best_hash),
                account,
                bytecode,
            )
            .map_err(runtime_error_into_rpc_err)?;

        let move_api_estimation = res.map_err(runtime_error_into_rpc_err)?;

        Ok(Estimation::from(move_api_estimation))
    }

    fn estimate_gas_publish_bundle(
        &self,
        account: AccountId,
        bytecode: Vec<u8>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Estimation> {
        let api = self.client.runtime_api();
        let res = api
            .estimate_gas_publish_bundle(
                at.unwrap_or_else(|| self.client.info().best_hash),
                account,
                bytecode,
            )
            .map_err(runtime_error_into_rpc_err)?;

        let move_api_estimation = res.map_err(runtime_error_into_rpc_err)?;

        Ok(Estimation::from(move_api_estimation))
    }

    fn estimate_gas_execute_script(
        &self,
        transaction: Vec<u8>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Estimation> {
        let api = self.client.runtime_api();
        let res = api
            .estimate_gas_execute_script(
                at.unwrap_or_else(|| self.client.info().best_hash),
                transaction,
            )
            .map_err(runtime_error_into_rpc_err)?;

        let move_api_estimation = res.map_err(runtime_error_into_rpc_err)?;

        Ok(Estimation::from(move_api_estimation))
    }

    fn get_resource(
        &self,
        account: AccountId,
        tag: Vec<u8>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<Vec<u8>>> {
        let api = self.client.runtime_api();
        let res = api.get_resource(
            at.unwrap_or_else(|| self.client.info().best_hash),
            account,
            tag,
        );

        // Currently, there is always correct value returned so it's safe to unwrap here.
        res.unwrap().map_err(runtime_error_into_rpc_err)
    }

    fn get_module_abi(
        &self,
        address: AccountId,
        name: &str,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<ModuleAbi>> {
        let api = self.client.runtime_api();
        let res = api.get_module_abi(
            at.unwrap_or_else(|| self.client.info().best_hash),
            address,
            name.to_string(),
        );

        // Currently, there is always correct value returned so it's safe to unwrap here.
        res.unwrap().map_err(runtime_error_into_rpc_err)
    }

    fn get_module(
        &self,
        address: AccountId,
        name: &str,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<Vec<u8>>> {
        let api = self.client.runtime_api();
        let res = api.get_module(
            at.unwrap_or_else(|| self.client.info().best_hash),
            address,
            name.to_string(),
        );

        // Currently, there is always correct value returned so it's safe to unwrap here.
        res.unwrap().map_err(runtime_error_into_rpc_err)
    }
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(_: impl std::fmt::Debug) -> ErrorObjectOwned {
    ErrorCode::InternalError.into()
}
