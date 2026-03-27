use crate::cmd::{read_bytes, script_args::args::HexEncodedBytes};

use anyhow::Result;
use blake2::{Blake2s256, Digest};
use clap::Parser;
use std::path::PathBuf;

/// Calculates the script transaction hash value.
#[derive(Parser, Debug, Clone)]
#[clap(about = "Generate call hash for script transaction")]
pub struct CallHash {
    /// Path to script transaction file (*.mvt) for a script execution.
    #[clap(short, long)]
    script_transaction_path: PathBuf,
}

impl CallHash {
    /// Executes the command.
    pub fn execute(&self) -> Result<()> {
        let script_tx = read_bytes(&self.script_transaction_path)?;

        let mut hasher = Blake2s256::new();
        hasher.update(&script_tx[..]);
        let call_hash: [u8; 32] = hasher.finalize().into();

        let call_hash_hex = HexEncodedBytes::from(call_hash);

        println!("Call hash: {call_hash_hex}");

        Ok(())
    }
}
