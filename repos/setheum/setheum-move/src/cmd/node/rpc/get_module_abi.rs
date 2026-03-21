use anyhow::{Context, Result};
use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use move_vm_backend_common::abi::ModuleAbi;
use url::Url;

/// Estimate gas for publishing modules.
#[derive(Parser, Debug, Clone)]
#[clap(about = "Get a move module's ABI")]
pub struct GetModuleAbi {
    #[clap(short, long, help = "Address of the module")]
    address: String,
    #[clap(short, long, help = "Name of the module")]
    name: String,
}

impl GetModuleAbi {
    /// Executes the command.
    pub fn execute(&self, url: &Url) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(url)?;
        let params = rpc_params![&self.address, &self.name];
        let response: Result<Option<ModuleAbi>, _> =
            rt.block_on(async { client.request("mvm_getModuleABI", params).await });

        let module_abi = response.with_context(|| "RPC result failure")?;

        println!("Module ABI: {module_abi:?}");

        Ok(())
    }
}
