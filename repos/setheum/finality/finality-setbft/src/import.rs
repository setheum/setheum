// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

use std::{
    error::Error,
    fmt::{Debug, Display, Error as FmtError, Formatter},
};

use futures::channel::mpsc::{self, TrySendError, UnboundedReceiver, UnboundedSender};
use log::{debug, warn};
use sc_consensus::{
    BlockCheckParams, BlockImport, BlockImportParams, ForkChoiceStrategy, ImportResult,
    JustificationImport,
};
use sp_consensus::{Error as ConsensusError, SelectChain};
use sp_runtime::{traits::Header as HeaderT, Justification as SubstrateJustification};

use crate::{
    setbft_primitives::{Block, BlockHash, BlockNumber, SETBFT_ENGINE_ID},
    block::substrate::{Justification, JustificationTranslator, TranslateError},
    justification::{backwards_compatible_decode, DecodeError},
    BlockId,
};

/// Constructs block import specific for setbft consensus.
pub fn get_setbft_block_import<I, SC>(
    inner: I,
    justification_tx: UnboundedSender<Justification>,
    translator: JustificationTranslator,
    select_chain: SC,
) -> impl BlockImport<Block, Error = I::Error> + JustificationImport<Block, Error = ConsensusError> + Clone
where
    I: BlockImport<Block> + Send + Sync + Clone,
    SC: SelectChain<Block> + Send + Sync,
{
    let favourite_marker_import = FavouriteMarkerBlockImport::new(inner, select_chain);

    SetBFTBlockImport::new(favourite_marker_import, justification_tx, translator)
}

/// A wrapper around a block import that also checks if the newly imported block is potentially
/// a new favourite block.
#[derive(Clone)]
struct FavouriteMarkerBlockImport<I, SC>
where
    I: BlockImport<Block> + Send + Sync,
    SC: SelectChain<Block> + Send + Sync,
{
    inner: I,
    select_chain: SC,
}

impl<I, SC> FavouriteMarkerBlockImport<I, SC>
where
    I: BlockImport<Block> + Send + Sync,
    SC: SelectChain<Block> + Send + Sync,
{
    pub fn new(inner: I, select_chain: SC) -> Self {
        Self {
            inner,
            select_chain,
        }
    }
}

#[async_trait::async_trait]
impl<I, SC> BlockImport<Block> for FavouriteMarkerBlockImport<I, SC>
where
    I: BlockImport<Block> + Send + Sync,
    SC: SelectChain<Block> + Send + Sync,
{
    type Error = I::Error;

    async fn check_block(
        &mut self,
        block: BlockCheckParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        self.inner.check_block(block).await
    }

    async fn import_block(
        &mut self,
        mut block: BlockImportParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        if let Ok(best) = self.select_chain.best_chain().await {
            block.fork_choice = Some(ForkChoiceStrategy::Custom(
                best.hash() == *block.header.parent_hash(),
            ));
        }

        self.inner.import_block(block).await
    }
}

/// A wrapper around a block import that also extracts any present justifications and sends them to
/// our components which will process them further and possibly finalize the block.
#[derive(Clone)]
pub struct SetBFTBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    inner: I,
    justification_tx: UnboundedSender<Justification>,
    translator: JustificationTranslator,
}

#[derive(Debug)]
enum SendJustificationError<TE: Debug> {
    Send(Box<TrySendError<Justification>>),
    Consensus(Box<ConsensusError>),
    Decode(DecodeError),
    Translate(TE),
}

impl<TE: Debug> From<DecodeError> for SendJustificationError<TE> {
    fn from(decode_error: DecodeError) -> Self {
        Self::Decode(decode_error)
    }
}

impl<I> SetBFTBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    pub fn new(
        inner: I,
        justification_tx: UnboundedSender<Justification>,
        translator: JustificationTranslator,
    ) -> SetBFTBlockImport<I> {
        SetBFTBlockImport {
            inner,
            justification_tx,
            translator,
        }
    }

    fn send_justification(
        &mut self,
        block_id: BlockId,
        justification: SubstrateJustification,
    ) -> Result<(), SendJustificationError<TranslateError>> {
        debug!(target: "setbft-justification", "Importing justification for block {}.", block_id);
        if justification.0 != SETBFT_ENGINE_ID {
            return Err(SendJustificationError::Consensus(Box::new(
                ConsensusError::ClientImport("SetBFT can import only SetBFT justifications.".into()),
            )));
        }
        let justification_raw = justification.1;
        let setbft_justification = backwards_compatible_decode(justification_raw)?;
        let justification = self
            .translator
            .translate(setbft_justification, block_id)
            .map_err(SendJustificationError::Translate)?;

        self.justification_tx
            .unbounded_send(justification)
            .map_err(|e| SendJustificationError::Send(Box::new(e)))
    }
}

#[async_trait::async_trait]
impl<I> BlockImport<Block> for SetBFTBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    type Error = I::Error;

    async fn check_block(
        &mut self,
        block: BlockCheckParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        self.inner.check_block(block).await
    }

    async fn import_block(
        &mut self,
        mut block: BlockImportParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        let number = *block.header.number();
        let post_hash = block.post_hash();

        let justifications = block.justifications.take();

        debug!(target: "setbft-justification", "Importing block {:?} {:?} {:?}", number, block.header.hash(), block.post_hash());
        let result = self.inner.import_block(block).await;

        if let Ok(ImportResult::Imported(_)) = result {
            if let Some(justification) =
                justifications.and_then(|just| just.into_justification(SETBFT_ENGINE_ID))
            {
                debug!(target: "setbft-justification", "Got justification along imported block {:?}", number);

                if let Err(e) = self.send_justification(
                    BlockId::new(post_hash, number),
                    (SETBFT_ENGINE_ID, justification),
                ) {
                    warn!(target: "setbft-justification", "Error while receiving justification for block {:?}: {:?}", post_hash, e);
                }
            }
        }

        result
    }
}

#[async_trait::async_trait]
impl<I> JustificationImport<Block> for SetBFTBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    type Error = ConsensusError;

    async fn on_start(&mut self) -> Vec<(BlockHash, BlockNumber)> {
        debug!(target: "setbft-justification", "On start called");
        Vec::new()
    }

    async fn import_justification(
        &mut self,
        hash: BlockHash,
        number: BlockNumber,
        justification: SubstrateJustification,
    ) -> Result<(), Self::Error> {
        use SendJustificationError::*;
        debug!(target: "setbft-justification", "import_justification called on {:?}", justification);
        self.send_justification(BlockId::new(hash, number), justification)
            .map_err(|error| match error {
                Send(e) => ConsensusError::ClientImport(format!(
                    "Could not send justification {:?} to ConsensusParty ",
                    (*e).into_inner()
                )),
                Consensus(e) => *e,
                Decode(e) => ConsensusError::ClientImport(format!(
                    "Justification for block {number:?} decoded incorrectly: {e}"
                )),
                Translate(e) => {
                    ConsensusError::ClientImport(format!("Could not translate justification: {e}"))
                }
            })
    }
}

/// A wrapper around a block import that actually sends all the blocks elsewhere through a channel.
/// Very barebones, e.g. does not work with justifications, but sufficient for passing to Aura.
#[derive(Clone)]
pub struct RedirectingBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    inner: I,
    blocks_tx: UnboundedSender<Block>,
}

impl<I> RedirectingBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    pub fn new(inner: I) -> (Self, UnboundedReceiver<Block>) {
        let (blocks_tx, blocks_rx) = mpsc::unbounded();
        (Self { inner, blocks_tx }, blocks_rx)
    }
}

/// What can go wrong when redirecting a block import.
#[derive(Debug)]
pub enum RedirectingImportError<E> {
    Inner(E),
    MissingBody,
    ChannelClosed,
}

impl<E: Display> Display for RedirectingImportError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        use RedirectingImportError::*;
        match self {
            Inner(e) => write!(f, "{}", e),
            MissingBody => write!(
                f,
                "redirecting block import does not support importing blocks without a body"
            ),
            ChannelClosed => write!(f, "channel closed, cannot redirect import"),
        }
    }
}

impl<E: Display + Debug> Error for RedirectingImportError<E> {}

#[async_trait::async_trait]
impl<I> BlockImport<Block> for RedirectingBlockImport<I>
where
    I: BlockImport<Block> + Clone + Send,
{
    type Error = RedirectingImportError<I::Error>;

    async fn check_block(
        &mut self,
        block: BlockCheckParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        self.inner
            .check_block(block)
            .await
            .map_err(RedirectingImportError::Inner)
    }

    async fn import_block(
        &mut self,
        block: BlockImportParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        let header = block.post_header();
        let BlockImportParams { body, .. } = block;

        let extrinsics = body.ok_or(RedirectingImportError::MissingBody)?;

        self.blocks_tx
            .unbounded_send(Block { header, extrinsics })
            .map_err(|_| RedirectingImportError::ChannelClosed)?;

        // We claim it was successfully imported and no further action is necessary.
        // This is likely inaccurate, but again, should be enough for Aura.
        Ok(ImportResult::Imported(Default::default()))
    }
}
