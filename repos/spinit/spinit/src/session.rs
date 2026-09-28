// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! This module provides a context-aware interface for interacting with SheythVM contracts.

use std::sync::Arc;

use parity_scale_codec::Decode;
pub use record::Record;

pub mod bundle;
pub mod error;
pub mod mock;
pub mod mocking_api;
mod record;
mod transcoding;

pub use bundle::ContractBundle;

use crate::session::transcoding::TranscoderRegistry;

/// Convenient value for an empty sequence of call/instantiation arguments.
pub const NO_ARGS: &[String] = &[];
/// Convenient value for an empty salt.
pub const NO_SALT: Option<[u8; 32]> = None;

/// Wrapper that provides a convenient API for interacting with SheythVM contracts.
pub struct Session {
	actor: [u8; 32],
	gas_limit: u64,
	transcoders: TranscoderRegistry,
	record: Record,
	last_deploy: Option<[u8; 32]>,
}

impl Default for Session {
	fn default() -> Self {
		Self {
			actor: [0u8; 32],
			gas_limit: 500_000_000,
			transcoders: TranscoderRegistry::new(),
			record: Default::default(),
			last_deploy: None,
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
		std::mem::replace(&mut self.actor, actor)
	}

	/// Sets a new gas limit and returns updated `self`.
	pub fn with_gas_limit(self, gas_limit: u64) -> Self {
		Self { gas_limit, ..self }
	}

	/// Returns a reference to the record of the session.
	pub fn record(&self) -> &Record {
		&self.record
	}

	/// Deploys a contract from raw SheythVM bytecode.
	pub fn deploy(
		&mut self,
		contract_bytes: Vec<u8>,
		constructor: &str,
		args: &[String],
		_salt: Option<[u8; 32]>,
		transcoder: &Arc<ContractMessageTranscoder>,
	) -> Result<[u8; 32], SessionError> {
		let _data = transcoder
			.encode(constructor, args)
			.map_err(|err| SessionError::Encoding(err.to_string()))?;

		// Compute a deterministic address from the contract code hash
		let address = sheyth_vm::hash(&contract_bytes);
		self.record.push_deploy_return(address);
		self.last_deploy = Some(address);
		Ok(address)
	}

	/// Similar to `deploy` but takes the parsed contract file (`ContractBundle`) as a first argument.
	pub fn deploy_bundle(
		&mut self,
		contract_file: ContractBundle,
		constructor: &str,
		args: &[String],
		salt: Option<[u8; 32]>,
	) -> Result<[u8; 32], SessionError> {
		self.deploy(contract_file.binary, constructor, args, salt, &contract_file.transcoder)
	}

	/// Deploys a contract and returns `self` (builder pattern).
	pub fn deploy_and(
		mut self,
		contract_bytes: Vec<u8>,
		constructor: &str,
		args: &[String],
		salt: Option<[u8; 32]>,
		transcoder: &Arc<ContractMessageTranscoder>,
	) -> Result<Self, SessionError> {
		self.deploy(contract_bytes, constructor, args, salt, transcoder)
			.map(|_| self)
	}

	/// Deploys a contract from a bundle and returns `self`.
	pub fn deploy_bundle_and(
		mut self,
		contract_file: ContractBundle,
		constructor: &str,
		args: &[String],
		salt: Option<[u8; 32]>,
	) -> Result<Self, SessionError> {
		self.deploy_bundle(contract_file, constructor, args, salt)
			.map(|_| self)
	}

	/// Calls a contract and returns `self`.
	pub fn call_and(
		mut self,
		message: &str,
		args: &[String],
	) -> Result<Self, SessionError> {
		self.call_internal::<()>(None, message, args)
			.map(|_| self)
	}

	/// Calls a contract with a given address and returns `self`.
	pub fn call_with_address_and(
		mut self,
		address: [u8; 32],
		message: &str,
		args: &[String],
	) -> Result<Self, SessionError> {
		self.call_internal::<()>(Some(address), message, args)
			.map(|_| self)
	}

	/// Calls the last deployed contract and returns the decoded result.
	pub fn call<V: Decode>(
		&mut self,
		message: &str,
		args: &[String],
	) -> Result<V, SessionError> {
		self.call_internal(None, message, args)
	}

	/// Calls a contract with a given address and returns the decoded result.
	pub fn call_with_address<V: Decode>(
		&mut self,
		address: [u8; 32],
		message: &str,
		args: &[String],
	) -> Result<V, SessionError> {
		self.call_internal(Some(address), message, args)
	}

	fn call_internal<V: Decode>(
		&mut self,
		_address: Option<[u8; 32]>,
		message: &str,
		args: &[String],
	) -> Result<V, SessionError> {
		let _ = (&self.actor, self.gas_limit, message, args);
		Err(SessionError::CallFailed("SheythVM execution not yet implemented in spinit".into()))
	}
}

