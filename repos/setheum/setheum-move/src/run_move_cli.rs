//! A handler module for move_cli.

use crate::run_context::RunContext;
use anyhow::Result;
use move_cli::base::build::Build;
use move_cli::Command;

/// Execute `move_cli build`.
pub fn execute_build(ctx: &RunContext) -> Result<()> {
    let cmd = Command::Build(Build);

    run_command(ctx, cmd)
}

/// Execute move_cli subcommand.
pub fn run_command(ctx: &RunContext, command: Command) -> Result<()> {
    move_cli::run_cli(
        ctx.natives.clone(),
        &ctx.cost_table,
        &ctx.error_descriptions,
        &ctx.move_args,
        command,
    )
}
