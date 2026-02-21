// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

use std::sync::Arc;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
pub use orml_oracle_runtime_api::OracleApi as OracleRuntimeApi;

#[rpc]
pub trait OracleApi<BlockHash, DataProviderId, CurrencyId, TimeStampedPrice> {
	#[rpc(name = "oracle_getValue")]
	fn get_value(
		&self,
		provider_id: DataProviderId,
		key: CurrencyId,
		at: Option<BlockHash>,
	) -> Result<Option<TimeStampedPrice>>;

	#[rpc(name = "oracle_getAllValues")]
	fn get_all_values(
		&self,
		provider_id: DataProviderId,
		at: Option<BlockHash>,
	) -> Result<Vec<(CurrencyId, Option<TimeStampedPrice>)>>;
}

pub struct Oracle<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> Oracle<C, B> {
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, Block, DataProviderId, CurrencyId, TimeStampedPrice>
	OracleApi<Block::Hash, DataProviderId, CurrencyId, TimeStampedPrice> for Oracle<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: OracleRuntimeApi<Block, DataProviderId, CurrencyId, TimeStampedPrice>,
	DataProviderId: codec::Codec + Send + Sync + 'static,
	CurrencyId: codec::Codec + Send + Sync + 'static,
	TimeStampedPrice: codec::Codec + Send + Sync + 'static,
{
	fn get_value(
		&self,
		provider_id: DataProviderId,
		key: CurrencyId,
		at: Option<Block::Hash>,
	) -> Result<Option<TimeStampedPrice>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		api.get_value(&at, provider_id, key).map_err(|e| Error {
			code: ErrorCode::ServerError(1),
			message: "Runtime error".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn get_all_values(
		&self,
		provider_id: DataProviderId,
		at: Option<Block::Hash>,
	) -> Result<Vec<(CurrencyId, Option<TimeStampedPrice>)>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		api.get_all_values(&at, provider_id).map_err(|e| Error {
			code: ErrorCode::ServerError(1),
			message: "Runtime error".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
