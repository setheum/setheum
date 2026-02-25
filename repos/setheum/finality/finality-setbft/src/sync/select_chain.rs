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

use std::fmt::Debug;

use futures::channel::{mpsc, oneshot};
use log::debug;
use sp_consensus::{Error, SelectChain};
use sp_runtime::traits::Block as BlockT;

use crate::{Block, BlockHash};

const LOG_TARGET: &str = "setbft-select-chain";

#[derive(Clone)]
pub struct FavouriteSelectChainInner<B: Block> {
    favourite_block_request: mpsc::UnboundedSender<oneshot::Sender<B::Header>>,
}

pub struct FavouriteSelectChainProvider<B: Block> {
    sc: FavouriteSelectChainInner<B>,
    rx: mpsc::UnboundedReceiver<oneshot::Sender<B::Header>>,
}

impl<B: Block, H> FavouriteSelectChainProvider<B>
where
    B: BlockT<Header = H, Hash = BlockHash>,
    B: Block<Header = H, Hash = BlockHash>,
    H: Sync + Send + Clone + Debug + 'static,
{
    pub fn new() -> Self {
        let (sc, rx) = FavouriteSelectChainInner::new();

        Self { sc, rx }
    }

    pub fn favourite_block_user_requests(
        self,
    ) -> mpsc::UnboundedReceiver<oneshot::Sender<B::Header>> {
        self.rx
    }

    pub fn select_chain(&self) -> impl SelectChain<B> {
        self.sc.clone()
    }
}

impl<B: Block, H> Default for FavouriteSelectChainProvider<B>
where
    B: BlockT<Header = H, Hash = BlockHash>,
    B: Block<Header = H, Hash = BlockHash>,
    H: Sync + Send + Clone + Debug + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Block> FavouriteSelectChainInner<B> {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<oneshot::Sender<B::Header>>) {
        let (rx, tx) = mpsc::unbounded();

        (
            Self {
                favourite_block_request: rx,
            },
            tx,
        )
    }
}

#[async_trait::async_trait]
impl<B, H> SelectChain<B> for FavouriteSelectChainInner<B>
where
    B: BlockT<Header = H, Hash = BlockHash>,
    B: Block<Header = H, Hash = BlockHash>,
    H: Sync + Send + Clone + Debug + 'static,
{
    async fn leaves(&self) -> Result<Vec<<B as BlockT>::Hash>, Error> {
        // this is never used in the current version
        Ok(Vec::new())
    }

    async fn best_chain(&self) -> Result<<B as BlockT>::Header, Error> {
        let (rx, tx) = oneshot::channel();

        self.favourite_block_request
            .unbounded_send(rx)
            .map_err(|e| Error::Other(Box::new(e)))?;
        let best = tx.await.map_err(|e| Error::Other(Box::new(e)))?;

        debug!(target: LOG_TARGET, "Best chain: {:?}", best);

        Ok(best)
    }
}
