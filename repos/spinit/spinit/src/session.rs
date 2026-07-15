//! This module provides a context-aware interface for interacting with SheythVM contracts.

use std::{
	fmt::Debug,
	mem,
	sync::{Arc, Mutex},
};

pub use contract_transcode;
use contract_transcode::ContractMessageTranscoder;
use error::SessionError;
use sheyth_vm::{Module, ModuleConfig};

use crate::SheythSandbox;
use parity_scale_codec::Decode;
pub use record::{EventBatch, Record};

pub mod bundle;
pub mod error;
pub mod mock;
pub mod mocking_api;
mod record;
mod transcoding;

pub use bundle::ContractBundle;

use self::mocking_api::MockingApi;
use crate::session::transcoding::TranscoderRegistry;

/// Convenient value for an empty sequence of call/instantiation arguments.
pub const NO_ARGS: &[String] = &[];
/// Convenient value for an empty salt.
pub const NO_SALT: Option<[u8; 32]> = None;

/// Wrapper around `SheythSandbox` that provides a convenient API for interacting with multiple contracts.
pub struct Session {
	sandbox: SheythSandbox,

	actor: [u8; 32],
	gas_limit: u64,

	transcoders: TranscoderRegistry,
	record: Record,
	mocks: Arc<Mutex<MockRegistry>>,
}

impl Default for Session {
	fn default() -> Self {
		let mocks = Arc::new(Mutex::new(MockRegistry::new()));

		Self {
			sandbox: SheythSandbox::new(),
			mocks,
			actor: [0u8; 32],
			gas_limit: 500_000_000,
			transcoders: TranscoderRegistry::new(),
			record: Default::default(),
		}
	}
}

impl Session {
	/// Sets a new actor and returns updated `self`.
	pub fn with_actor(self, actor: [u8; 32]) -> Self {
		Self { actor, ..self }
	}

	/// Returns currently set actor.
	pub fn get_actor(&self) -> [u8; 32] {
		self.actor
	}

	/// Sets a new actor and returns the old one.
	pub fn set_actor(&mut self, actor: [u8; 32]) -> [u8; 32] {
		mem::replace(&mut self.actor, actor)
	}

	/// Sets a new gas limit and returns updated `self`.
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		Self { gas_limit, ..self }
	}

	/// The underlying `SheythSandbox` instance.
	pub fn sandbox(&mut self) -> &mut SheythSandbox {
		&mut self.sandbox
	}

	/// Returns a reference to the record of the session.
	pub fn record(&self) -> &Record {
		&self.record
	}

	/// Deploys a contract from raw SheythVM bytecode.
	pub fn deploy<S: AsRef<str> + Debug>(
		&mut self,
		contract_bytes: Vec<u8>,
		constructor: &str,
		args: &[S],
		_salt: Option<[u8; 32]>,
		transcoder: &Arc<ContractMessageTranscoder>,
	) -> Result<[u8; 32], SessionError> {
		let _data = transcoder
			.encode(constructor, args)
			.map_err(|err| SessionError::Encoding(err.to_string()))?;

		let module = Module::new(ModuleConfig::default(), &contract_bytes)
			.map_err(|e| SessionError::DeploymentFailed(format!("{e:?}")))?;

		let address = blake3::hash(&contract_bytes).into();
		let _ = (&self.sandbox, module);

		self.record.push_deploy_return(address);
		Ok(address)
	}

	/// Similar to `deploy` but takes the parsed contract file (`ContractBundle`) as a first argument.
	pub fn deploy_bundle<S: AsRef<str> + Debug>(
		&mut self,
		contract_file: ContractBundle,
		constructor: &str,
		args: &[S],
		salt: Option<[u8; 32]>,
	) -> Result<[u8; 32], SessionError> {
		self.deploy(contract_file.binary, constructor, args, salt, &contract_file.transcoder)
	}

	/// Deploys a contract and returns `self` (builder pattern).
	pub fn deploy_and<S: AsRef<str> + Debug>(
		mut self,
		contract_bytes: Vec<u8>,
		constructor: &str,
		args: &[S],
		salt: Option<[u8; 32]>,
		transcoder: &Arc<ContractMessageTranscoder>,
	) -> Result<Self, SessionError> {
		self.deploy(contract_bytes, constructor, args, salt, transcoder)
			.map(|_| self)
	}

	/// Deploys a contract from a bundle and returns `self`.
	pub fn deploy_bundle_and<S: AsRef<str> + Debug>(
		mut self,
		contract_file: ContractBundle,
		constructor: &str,
		args: &[S],
		salt: Option<[u8; 32]>,
	) -> Result<Self, SessionError> {
		self.deploy_bundle(contract_file, constructor, args, salt)
			.map(|_| self)
	}

	/// Calls a contract and returns `self`.
	pub fn call_and<S: AsRef<str> + Debug>(
		mut self,
		message: &str,
		args: &[S],
	) -> Result<Self, SessionError> {
		self.call_internal::<_, ()>(None, message, args)
			.map(|_| self)
	}

	/// Calls a contract with a given address and returns `self`.
	pub fn call_with_address_and<S: AsRef<str> + Debug>(
		mut self,
		address: [u8; 32],
		message: &str,
		args: &[S],
	) -> Result<Self, SessionError> {
		self.call_internal::<_, ()>(Some(address), message, args)
			.map(|_| self)
	}

	/// Calls the last deployed contract and returns the decoded result.
	pub fn call<S: AsRef<str> + Debug, V: Decode>(
		&mut self,
		message: &str,
		args: &[S],
	) -> Result<V, SessionError> {
		self.call_internal::<_, V>(None, message, args)
	}

	/// Calls a contract with a given address and returns the decoded result.
	pub fn call_with_address<S: AsRef<str> + Debug, V: Decode>(
		&mut self,
		address: [u8; 32],
		message: &str,
		args: &[S],
	) -> Result<V, SessionError> {
		self.call_internal(Some(address), message, args)
	}

	/// Uploads raw contract code.
	pub fn upload_and(mut self, contract_bytes: Vec<u8>) -> Result<Self, SessionError> {
		self.upload(contract_bytes).map(|_| self)
	}

	/// Uploads raw contract code and returns its hash.
	pub fn upload(&mut self, contract_bytes: Vec<u8>) -> Result<[u8; 32], SessionError> {
		let hash = blake3::hash(&contract_bytes).into();
		Ok(hash)
	}

	/// Uploads a contract bundle and returns `self`.
	pub fn upload_bundle_and(self, contract_file: ContractBundle) -> Result<Self, SessionError> {
		self.upload_and(contract_file.binary)
	}

	/// Uploads a contract bundle and returns its hash.
	pub fn upload_bundle(&mut self, contract_file: ContractBundle) -> Result<[u8; 32], SessionError> {
		self.upload(contract_file.binary)
	}

	fn call_internal<S: AsRef<str> + Debug, V: Decode>(
		&mut self,
		_address: Option<[u8; 32]>,
		message: &str,
		args: &[S],
	) -> Result<V, SessionError> {
		let _ = (&self.actor, self.gas_limit, message, args);
		Err(SessionError::CallFailed("SheythVM execution not yet implemented in spinit".into()))
	}
}

