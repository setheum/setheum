use alloc::vec::Vec;
use anyhow::{Error, Result};
use core::convert::TryFrom;
use move_core_types::language_storage::TypeTag;
use serde::{Deserialize, Serialize};

/// Bundle contains a list of module bytecodes.
#[derive(Serialize, Deserialize)]
pub struct ModuleBundle {
    /// Module bytecodes.
    modules: Vec<Vec<u8>>,
}

impl ModuleBundle {
    /// Create a new ModuleBundle.
    pub fn new(modules: Vec<Vec<u8>>) -> Self {
        Self { modules }
    }

    /// Gets module bytecodes.
    pub fn into_inner(self) -> Vec<Vec<u8>> {
        self.modules
    }

    /// Serializes data.
    pub fn encode(self) -> Result<Vec<u8>> {
        bcs::to_bytes(&self).map_err(Error::msg)
    }
}

impl TryFrom<&[u8]> for ModuleBundle {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(blob).map_err(Error::msg)
    }
}

/// Transaction representation used in execute call.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptTransaction {
    /// Script bytecode.
    pub bytecode: Vec<u8>,
    /// All script arguments that are fed to the MoveVM.
    ///
    /// Signers in this list are represented by their address - and are actually signed by the
    /// account that executes the extrinsic in the pallet layer.
    pub args: Vec<Vec<u8>>,
    /// Script type arguments.
    pub type_args: Vec<TypeTag>,
}

impl TryFrom<&[u8]> for ScriptTransaction {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(blob).map_err(Error::msg)
    }
}

impl ScriptTransaction {
    /// Serializes data.
    pub fn encode(self) -> Result<Vec<u8>> {
        bcs::to_bytes(&self).map_err(Error::msg)
    }
}
