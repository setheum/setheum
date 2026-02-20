use std::sync::Arc;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{HeaderMetadata, HeaderBackend};
use sp_runtime::traits::Block as BlockT;

pub use module_evm_rpc_runtime_api::EVMRuntimeRPCApi;

#[rpc]
pub trait EVMApiServer {
    #[rpc(name = "evm_placeholder")]
    fn placeholder(&self) -> Result<String>;
}

pub struct EVMApi<C, B> {
    client: Arc<C>,
    deny_unsafe: DenyUnsafe,
    _phantom: std::marker::PhantomData<B>,
}

impl<C, B> EVMApi<C, B> {
    pub fn new(client: Arc<C>, deny_unsafe: DenyUnsafe) -> Self {
        Self {
            client,
            deny_unsafe,
            _phantom: Default::default(),
        }
    }
}

impl<C, B> EVMApiServer for EVMApi<C, B> {
    fn placeholder(&self) -> Result<String> {
        Ok("placeholder".into())
    }
}
