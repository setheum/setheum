// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
