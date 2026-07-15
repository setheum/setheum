//! The spinit crate provides a sandboxed runtime for testing SheythVM smart contracts
//! without a need for a running node.

#![warn(missing_docs)]

pub mod errors;
#[cfg(feature = "session")]
pub mod session;

#[cfg(feature = "macros")]
pub use spinit_test_macro::{contract_bundle_provider, test};
pub use errors::Error;
pub use sheyth_vm::{
	Config as SheythVmConfig, Engine, Instance, Linker, Module, ModuleConfig, BackendKind,
};
pub use sp_runtime::{AccountId32, DispatchError};

/// Main result type for the spinit crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// A minimal sandbox engine for testing SheythVM contracts.
pub struct SheythSandbox {
	/// The SheythVM engine instance.
	pub engine: Engine,
}

impl SheythSandbox {
	/// Create a new sandbox with default configuration.
	pub fn new() -> Self {
		let engine = Engine::new(BackendKind::Interpreter);
		Self { engine }
	}

	/// Execute a contract module.
	pub fn execute(
		&mut self,
		module: &Module,
		linker: &Linker,
		function: &str,
		args: &[u8],
	) -> Result<Vec<u8>, String> {
		let mut instance = linker.instantiate(module).map_err(|e| format!("{e:?}"))?;
		let result = instance
			.call(function, args)
			.map_err(|e| format!("{e:?}"))?;
		Ok(result)
	}
}

impl Default for SheythSandbox {
	fn default() -> Self {
		Self::new()
	}
}

