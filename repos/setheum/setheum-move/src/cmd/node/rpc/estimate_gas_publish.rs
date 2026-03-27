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
#[clap(about = "Estimate gas for publishing modules")]
pub struct EstimateGasPublishModule {
    #[clap(short, long, help = "Account ID in the SS58 format")]
    account_id: String,

    #[clap(short, long, help = "Path to the module (compiled by the setheum-move)")]
    module_path: PathBuf,
}

impl EstimateGasPublishModule {
    /// Executes the command.
    pub fn execute(&self, url: &Url) -> Result<()> {
        let script_tx = read_bytes(&self.module_path)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(url)?;
        let params = rpc_params![&self.account_id, script_tx];
        let response: Result<Estimation, _> =
            rt.block_on(async { client.request("mvm_estimateGasPublishModule", params).await });

        let estimated_gas = response.with_context(|| "RPC result failure")?;

        println!("{estimated_gas}");

        Ok(())
    }
}

/// Estimate gas for publishing a bunlde.
#[derive(Parser, Debug, Clone)]
#[clap(about = "Estimate gas for publishing a bundle")]
pub struct EstimateGasPublishBundle {
    #[clap(short, long, help = "Account ID in the SS58 format")]
    account_id: String,

    #[clap(short, long, help = "Path to the bundle (compiled by the setheum-move)")]
    bundle_path: PathBuf,
}

impl EstimateGasPublishBundle {
    /// Executes the command.
    pub fn execute(&self, url: &Url) -> Result<()> {
        let script_tx = read_bytes(&self.bundle_path)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(url)?;
        let params = rpc_params![&self.account_id, script_tx];
        let response: Result<Estimation, _> =
            rt.block_on(async { client.request("mvm_estimateGasPublishBundle", params).await });

        let estimated_gas = response.with_context(|| "RPC result failure")?;

        println!("Estimated gas: {estimated_gas}");

        Ok(())
    }
}
