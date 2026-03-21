//! List of setheum-move subcommands.

pub(super) mod bundle;
pub(super) mod call_hash;
pub(super) mod node;
pub(super) mod script;
pub(super) mod script_args;

use anyhow::{Context, Result};
use std::path::Path;

/// Reads bytes from a file for the given path.
pub(crate) fn read_bytes(file_path: &Path) -> Result<Vec<u8>> {
    std::fs::read(file_path)
        .map_err(anyhow::Error::from)
        .with_context(|| format!("Failure to read filename {}", file_path.display()))
}
