use crate::run_context::RunContext;
use anyhow::Result;
use clap::Parser;
use move_cli::Move;
use std::path::PathBuf;

mod cmd;
mod run_context;
mod run_move_cli;

/// CLI frontend for the Move compiler and VM in Setheum.
#[derive(Parser, Debug, Clone)]
#[clap(name = "setheum-move", author, about, long_about = None, version)]
pub struct SetheumMoveArgs {
    /// Native move-cli arguments.
    #[clap(flatten)]
    pub move_args: Move,

    /// Commands.
    #[clap(subcommand)]
    pub cmd: SetheumMoveCommand,
}

/// Move CLI and setheum-move subcommands.
#[derive(clap::Subcommand, Debug, Clone)]
pub enum SetheumMoveCommand {
    /// Native move-cli commands.
    #[clap(flatten)]
    MoveCommand(move_cli::Command),

    /// Create a package bundle.
    #[clap(about = "Create a package bundle")]
    Bundle {
        #[clap(flatten)]
        cmd: cmd::bundle::Bundle,
    },

    /// Generate call hash for a script transaction.
    #[clap(about = "Generate call hash for a script transaction")]
    CallHash {
        #[clap(flatten)]
        cmd: cmd::call_hash::CallHash,
    },

    /// Create a script transaction.
    #[clap(about = "Create a script transaction")]
    CreateTransaction {
        #[clap(flatten)]
        cmd: cmd::script::CreateTransaction,
    },

    /// Commands for accessing the node.
    #[clap(about = "Commands for accessing the node")]
    Node {
        #[clap(flatten)]
        cmd: cmd::node::Node,
    },
}

/// Run the setheum-move CLI.
pub fn setheum_move_cli(cwd: PathBuf) -> Result<()> {
    let args = SetheumMoveArgs::parse();
    run_with_args(cwd, args)
}

/// Run setheum-move with provided arguments.
pub fn run_with_args(cwd: PathBuf, args: SetheumMoveArgs) -> Result<()> {
    let SetheumMoveArgs { move_args, cmd } = args;

    let project_root_dir = if let Some(ref project_path) = move_args.package_path {
        project_path.canonicalize()?
    } else {
        cwd
    };

    let ctx = RunContext::new(project_root_dir, move_args)?;

    match cmd {
        SetheumMoveCommand::MoveCommand(cmd) => run_move_cli::run_command(&ctx, cmd),
        SetheumMoveCommand::Bundle { mut cmd } => cmd.execute(&ctx),
        SetheumMoveCommand::Node { mut cmd } => cmd.execute(),
        SetheumMoveCommand::CreateTransaction { mut cmd } => cmd.execute(&ctx),
        SetheumMoveCommand::CallHash { cmd } => cmd.execute(),
    }
}
