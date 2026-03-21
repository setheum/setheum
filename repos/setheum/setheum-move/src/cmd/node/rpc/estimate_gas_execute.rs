use crate::cmd::{node::rpc::Estimation, read_bytes};
use anyhow::{Context, Result};
use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use std::path::PathBuf;
use url::Url;

/// Estimate gas for publishing modules.
#[derive(Parser, Debug, Clone)]
#[clap(about = "Estimate gas for executing script")]
pub struct EstimateGasExecuteScript {
    /// Path to the script transaction (compiled by the setheum-move create-transaction).
    #[clap(short, long)]
    script_transaction_path: PathBuf,
}

impl EstimateGasExecuteScript {
    /// Executes the command.
    pub fn execute(&self, url: &Url) -> Result<()> {
        let script_tx = read_bytes(&self.script_transaction_path)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(url)?;
        let params = rpc_params![script_tx];
        let response: Result<Estimation, _> =
            rt.block_on(async { client.request("mvm_estimateGasExecuteScript", params).await });

        let estimated_gas = response.with_context(|| "RPC result failure")?;

        println!("{estimated_gas}");

        Ok(())
    }
}
