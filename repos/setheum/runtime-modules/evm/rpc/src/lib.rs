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
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{HeaderMetadata, HeaderBackend};
use sp_runtime::traits::Block as BlockT;

pub use module_evm_rpc_runtime_api::EVMRuntimeRPCApi;

#[rpc]
pub trait EVMApiServer {
    #[rpc(name = "evm_placeholder")]
    fn placeholder(&self) -> Result<String>;
}

pub struct EVMApi<C, B> {
    client: Arc<C>,
    deny_unsafe: DenyUnsafe,
    _phantom: std::marker::PhantomData<B>,
}

impl<C, B> EVMApi<C, B> {
    pub fn new(client: Arc<C>, deny_unsafe: DenyUnsafe) -> Self {
        Self {
            client,
            deny_unsafe,
            _phantom: Default::default(),
        }
    }
}

impl<C, B> EVMApiServer for EVMApi<C, B> {
    fn placeholder(&self) -> Result<String> {
        Ok("placeholder".into())
    }
}
