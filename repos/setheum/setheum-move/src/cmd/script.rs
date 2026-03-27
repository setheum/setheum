use super::script_args::ScriptFunctionArguments;
use crate::run_context::RunContext;
use anyhow::{Error, Result};
use clap::Parser;
use move_vm_backend_common::bytecode::verify_script_integrity_and_check_signers;
use move_vm_backend_common::types::ScriptTransaction;
use std::fs;
use std::path::PathBuf;

/// Create a script transaction.
#[derive(Parser, Debug, Clone)]
#[clap(about = "setheum-move create-transaction")]
pub struct CreateTransaction {
    #[clap(short, long, help = "Path for the compiled Move script.")]
    compiled_script_path: PathBuf,

    /// Arguments for script functions.
    #[clap(flatten)]
    script_function_args: ScriptFunctionArguments,
}

impl CreateTransaction {
    /// Executes the command.
    pub fn execute(&mut self, ctx: &RunContext) -> Result<()> {
        let compiled_script = &self.compiled_script_path;
        let script_bc = fs::read(compiled_script)
            .map_err(|e| Error::msg(format!("Can't read '{}':\n{e}", compiled_script.display())))?;

        // Check the script bytecode and verify the parameter rules.
        // This is checked in the Substrate layer again for the safety reasons.
        let _signer_count = verify_script_integrity_and_check_signers(&script_bc)
            .map_err(|e| Error::msg(format!("Script parameters verification failure {e:?}")))?;

        let type_args = self.script_function_args.type_args()?;
        let args = self.script_function_args.args()?;

        let tx = ScriptTransaction {
            bytecode: script_bc,
            args,
            type_args,
        };

        // Path to the output file.
        let tx_name = compiled_script.file_name().unwrap(); // this can't fail in case `fs::read` succeeds above.
        let output_file_path = ctx.script_tx_output_path(&tx_name)?;
        if output_file_path.exists() {
            fs::remove_file(&output_file_path)?;
        }
        fs::write(&output_file_path, tx.encode()?)?;

        println!(
            "Script transaction is created at:\n{}",
            output_file_path.canonicalize()?.display()
        );

        Ok(())
    }
}
