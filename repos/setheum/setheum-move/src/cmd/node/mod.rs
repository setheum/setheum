use anyhow::Result;
use clap::Parser;
use url::Url;

pub(crate) mod rpc;

/// Commands for accessing the node.
#[derive(Parser, Debug, Clone)]
pub struct Node {
    /// Command option.
    #[clap(subcommand)]
    cmd: NodeCmd,

    /// URL for the node's endpoint depending on the chosen option.
    #[clap(
        short,
        long,
        help = "Node's URL (by default using local RPC's URL)",
        default_value = "http://localhost:9944/"
    )]
    url: Url,
}

/// List of possible node access commands.
#[derive(Parser, Debug, Clone)]
pub enum NodeCmd {
    /// Access node's RPC requests.
    #[clap(subcommand, about = "Access node's RPC requests")]
    Rpc(Rpc),
}

impl Node {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        match &self.cmd {
            NodeCmd::Rpc(rpc) => rpc.execute(&self.url),
        }
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Rpc {
    /// Estimate gas for publishing modules.
    #[clap(about = "Estimate gas for publishing modules")]
    EstimateGasPublishModule {
        #[clap(flatten)]
        cmd: rpc::estimate_gas_publish::EstimateGasPublishModule,
    },

    /// Estimate gas for publishing a bundle.
    #[clap(about = "Estimate gas for publishing a bundle")]
    EstimateGasPublishBundle {
        #[clap(flatten)]
        cmd: rpc::estimate_gas_publish::EstimateGasPublishBundle,
    },

    /// Estimate gas for executing a script.
    #[clap(about = "Estimate gas for executing a script")]
    EstimateGasExecuteScript {
        #[clap(flatten)]
        cmd: rpc::estimate_gas_execute::EstimateGasExecuteScript,
    },

    /// Get a module's ABI.
    #[clap(about = "Get a module's ABI")]
    GetModuleAbi {
        #[clap(flatten)]
        cmd: rpc::get_module_abi::GetModuleAbi,
    },
}

impl Rpc {
    /// Executes the command.
    pub fn execute(&self, url: &Url) -> Result<()> {
        match self {
            Self::EstimateGasPublishModule { cmd } => cmd.execute(url),
            Self::EstimateGasPublishBundle { cmd } => cmd.execute(url),
            Self::EstimateGasExecuteScript { cmd } => cmd.execute(url),
            Self::GetModuleAbi { cmd } => cmd.execute(url),
        }
    }
}
