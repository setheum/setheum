use anyhow::Result;
use args::ArgWithTypeVec;
use clap::Parser;
use move_core_types::language_storage::TypeTag;
use type_args::TypeArgVec;

pub(crate) mod args;
mod type_args;

/// Arguments for script functions.
#[derive(Debug, Parser, Clone)]
pub struct ScriptFunctionArguments {
    /// Type args.
    #[clap(flatten)]
    type_arg_vec: TypeArgVec,

    /// Function args.
    #[clap(flatten)]
    arg_vec: ArgWithTypeVec,
}

impl ScriptFunctionArguments {
    /// Get type arguments.
    pub fn type_args(&self) -> Result<Vec<TypeTag>> {
        let mut type_args = vec![];

        for arg in self.type_arg_vec.type_args.iter() {
            let type_arg: TypeTag = arg.try_into()?;
            type_args.push(type_arg);
        }

        Ok(type_args)
    }

    /// Get function arguments.
    pub fn args(&self) -> Result<Vec<Vec<u8>>> {
        Ok(self
            .arg_vec
            .args
            .iter()
            .map(|arg_with_type| arg_with_type.arg.clone())
            .collect())
    }
}
