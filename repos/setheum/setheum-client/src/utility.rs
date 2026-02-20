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

use anyhow::anyhow;
use log::debug;
use primitives::Balance;
use subxt::{blocks::ExtrinsicEvents, config::Hasher, Config};

use crate::{
    api::transaction_payment::events::TransactionFeePaid,
    connections::{AsConnection, TxInfo},
    pallets::{committee_management::CommitteeManagementApi, staking::StakingApi},
    AlephConfig, BlockHash, BlockNumber, EraIndex, SessionIndex,
};

/// Block info API.
#[async_trait::async_trait]
pub trait BlocksApi {
/// Returns the first block of a session.
/// * `session` - number of the session to query the first block from
    async fn first_block_of_session(
        &self,
        session: SessionIndex,
    ) -> anyhow::Result<Option<BlockHash>>;

/// Returns hash of a given block if the given block exists, otherwise `None`
/// * `block` - number of the block
    async fn get_block_hash(&self, block: BlockNumber) -> anyhow::Result<Option<BlockHash>>;

/// Returns the most recent block from the current best chain.
    async fn get_best_block(&self) -> anyhow::Result<Option<BlockNumber>>;

/// Returns the most recent block from the finalized chain.
    async fn get_finalized_block_hash(&self) -> anyhow::Result<BlockHash>;

/// Returns number of a given block hash, if the given block exists, otherwise `None`
/// This is version that returns `Result`
/// * `block` - hash of the block to query its number
    async fn get_block_number(&self, block: BlockHash) -> anyhow::Result<Option<BlockNumber>>;

/// Returns number of a given block hash, if the given block exists, otherwise `None`
/// * `block` - hash of the block to query its number
    async fn get_block_number_opt(
        &self,
        block: Option<BlockHash>,
    ) -> anyhow::Result<Option<BlockNumber>>;

/// Fetch all events that corresponds to the transaction identified by `tx_info`.
    async fn get_tx_events(&self, tx_info: TxInfo) -> anyhow::Result<ExtrinsicEvents<AlephConfig>>;

/// Returns the fee that was paid for the transaction identified by `tx_info`.
    async fn get_tx_fee(&self, tx_info: TxInfo) -> anyhow::Result<Balance>;
}

/// Interaction logic between pallet session and pallet staking.
#[async_trait::async_trait]
pub trait SessionEraApi {
/// Returns which era given session is.
/// * `session` - session index
    async fn get_active_era_for_session(&self, session: SessionIndex) -> anyhow::Result<EraIndex>;
}

#[async_trait::async_trait]
impl<C: AsConnection + Sync> BlocksApi for C {
    async fn first_block_of_session(
        &self,
        session: SessionIndex,
    ) -> anyhow::Result<Option<BlockHash>> {
        let period = self.get_session_period().await?;
        let block_num = period * session;

        self.get_block_hash(block_num).await
    }

    async fn get_block_hash(&self, block: BlockNumber) -> anyhow::Result<Option<BlockHash>> {
        debug!(target: "setheum-client", "querying block hash for number #{}", block);
        self.as_connection()
            .as_client()
            .rpc()
            .block_hash(Some(block.into()))
            .await
            .map_err(|e| e.into())
    }

    async fn get_best_block(&self) -> anyhow::Result<Option<BlockNumber>> {
        self.get_block_number_opt(None).await
    }

    async fn get_finalized_block_hash(&self) -> anyhow::Result<BlockHash> {
        self.as_connection()
            .as_client()
            .rpc()
            .finalized_head()
            .await
            .map_err(|e| e.into())
    }

    async fn get_block_number(&self, block: BlockHash) -> anyhow::Result<Option<BlockNumber>> {
        self.get_block_number_opt(Some(block)).await
    }

    async fn get_block_number_opt(
        &self,
        block: Option<BlockHash>,
    ) -> anyhow::Result<Option<BlockNumber>> {
        self.as_connection()
            .as_client()
            .rpc()
            .header(block)
            .await
            .map(|maybe_header| maybe_header.map(|header| header.number))
            .map_err(|e| e.into())
    }

    async fn get_tx_events(&self, tx_info: TxInfo) -> anyhow::Result<ExtrinsicEvents<AlephConfig>> {
        let block_body = self
            .as_connection()
            .as_client()
            .blocks()
            .at(tx_info.block_hash)
            .await?
            .body()
            .await?;

        let extrinsic_events = block_body
            .extrinsics()
            .iter()
            .find(|tx| match tx {
                Ok(tx) => tx_info.tx_hash == <AlephConfig as Config>::Hasher::hash_of(&tx.bytes()),
                _ => false,
            })
            .ok_or_else(|| anyhow!("Couldn't find the transaction in the block."))??
            .events()
            .await
            .map_err(|e| anyhow!("Couldn't fetch events for the transaction: {e:?}"))?;

        Ok(extrinsic_events)
    }

    async fn get_tx_fee(&self, tx_info: TxInfo) -> anyhow::Result<Balance> {
        let events = self.get_tx_events(tx_info).await?;
        events
            .find_first::<TransactionFeePaid>()?
            .ok_or_else(|| anyhow!("TransactionFeePaid event not found"))
            .map(|tfp| tfp.actual_fee)
    }
}

#[async_trait::async_trait]
impl<C: AsConnection + Sync> SessionEraApi for C {
    async fn get_active_era_for_session(&self, session: SessionIndex) -> anyhow::Result<EraIndex> {
        let block = self.first_block_of_session(session).await?;
        Ok(self.get_active_era(block).await)
    }
}
