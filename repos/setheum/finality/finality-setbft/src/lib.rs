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

extern crate set_bft as current_set_bft;
extern crate set_bft as legacy_set_bft;
extern crate set_bft_rmc as current_set_bft_rmc;
extern crate set_bft_rmc as legacy_set_bft_rmc;

use std::{fmt::Debug, hash::Hash, path::PathBuf, sync::Arc};

use derive_more::Display;
use futures::{
    channel::{
        mpsc::{self, unbounded, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    Future,
};
use parity_scale_codec::{Decode, Encode, Output};
use primitives as setbft_primitives;
use primitives::{AuthorityId, Block as SetBFTBlock, BlockHash, BlockNumber};
use sc_client_api::{
    Backend, BlockBackend, BlockchainEvents, Finalizer, LockImportRun, ProofProvider,
    StorageProvider,
};
use sc_consensus::BlockImport;
use sc_keystore::LocalKeystore;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_runtime::traits::{BlakeTwo256, Block};
use substrate_prometheus_endpoint::Registry;
use tokio::time::Duration;

use crate::{
    abft::{
        CurrentNetworkData, Keychain, LegacyNetworkData, NodeCount, NodeIndex, Recipient,
        SignatureSet, SpawnHandle, CURRENT_VERSION, LEGACY_VERSION,
    },
    aggregation::RmcNetworkData,
    block::UnverifiedHeader,
    network::data::split::Split,
    session::{SessionBoundaries, SessionBoundaryInfo, SessionId},
};

mod abft;
mod aggregation;
mod block;
mod compatibility;
mod crypto;
mod data_io;
mod finalization;
mod idx_to_account;
mod import;
mod justification;
mod metrics;
mod network;
mod nodes;
mod party;
mod runtime_api;
mod session;
mod session_map;
mod sync;
mod sync_oracle;
#[cfg(test)]
pub mod testing;

pub use crate::{
    block::{
        substrate::{BlockImporter, Justification, JustificationTranslator, SubstrateChainStatus},
        BlockId,
    },
    import::{get_setbft_block_import, SetBFTBlockImport, RedirectingBlockImport},
    justification::SetBFTJustification,
    network::{
        address_cache::{ValidatorAddressCache, ValidatorAddressingInfo},
        build_network, BuildNetworkOutput, ProtocolNetwork, SubstrateNetworkConfig,
        SubstratePeerId,
    },
    nodes::run_validator_node,
    session::SessionPeriod,
    sync::FavouriteSelectChainProvider,
    sync_oracle::SyncOracle,
};

/// Constant defining how often components of finality-setbft should report their state
const STATUS_REPORT_INTERVAL: Duration = Duration::from_secs(20);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Encode, Decode)]
pub struct MillisecsPerBlock(pub u64);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Encode, Decode)]
pub struct UnitCreationDelay(pub u64);

pub type SplitData<UH> = Split<NetworkData<UH>, RmcNetworkData>;

pub trait ClientForSetBFT<B, BE>:
    LockImportRun<B, BE>
    + Finalizer<B, BE>
    + ProvideRuntimeApi<B>
    + BlockImport<B, Error = sp_consensus::Error>
    + HeaderBackend<B>
    + HeaderMetadata<B, Error = sp_blockchain::Error>
    + BlockchainEvents<B>
    + BlockBackend<B>
    + StorageProvider<B, BE>
    + ProofProvider<B>
    + 'static
where
    BE: Backend<B>,
    B: Block,
{
}

impl<B, BE, T> ClientForSetBFT<B, BE> for T
where
    BE: Backend<B>,
    B: Block,
    T: LockImportRun<B, BE>
        + Finalizer<B, BE>
        + ProvideRuntimeApi<B>
        + HeaderBackend<B>
        + HeaderMetadata<B, Error = sp_blockchain::Error>
        + BlockchainEvents<B>
        + BlockImport<B, Error = sp_consensus::Error>
        + BlockBackend<B>
        + StorageProvider<B, BE>
        + ProofProvider<B>
        + 'static,
{
}

pub struct ChannelProvider<T> {
    sender: UnboundedSender<T>,
    receiver: UnboundedReceiver<T>,
}

impl<T> ChannelProvider<T> {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        ChannelProvider { sender, receiver }
    }

    pub fn get_sender(&self) -> UnboundedSender<T> {
        self.sender.clone()
    }

    pub fn into_receiver(self) -> UnboundedReceiver<T> {
        self.receiver
    }
}

impl<T> Default for ChannelProvider<T> {
    fn default() -> Self {
        Self::new()
    }
}

type Hasher = abft::HashWrapper<BlakeTwo256>;

#[derive(Clone)]
pub struct RateLimiterConfig {
    /// Maximum bit-rate in bits per second of the setbft validator network.
    pub setbft_network_bit_rate: u64,
    /// Maximum bit-rate in bits per second of the substrate network (shared by sync, gossip, etc.).
    pub substrate_network_bit_rate: u64,
}

pub struct SetBFTConfig<C, T> {
    pub authentication_network: ProtocolNetwork,
    pub block_sync_network: ProtocolNetwork,
    pub client: Arc<C>,
    pub chain_status: SubstrateChainStatus,
    pub import_queue_handle: BlockImporter,
    pub select_chain_provider: FavouriteSelectChainProvider<SetBFTBlock>,
    pub spawn_handle: SpawnHandle,
    pub keystore: Arc<LocalKeystore>,
    pub justification_channel_provider: ChannelProvider<Justification>,
    pub block_rx: mpsc::UnboundedReceiver<SetBFTBlock>,
    pub registry: Option<Registry>,
    pub session_period: SessionPeriod,
    pub millisecs_per_block: MillisecsPerBlock,
    pub score_submission_period: u32,
    pub unit_creation_delay: UnitCreationDelay,
    pub backup_saving_path: Option<PathBuf>,
    pub external_addresses: Vec<String>,
    pub validator_port: u16,
    pub rate_limiter_config: RateLimiterConfig,
    pub sync_oracle: SyncOracle,
    pub validator_address_cache: Option<ValidatorAddressCache>,
    pub transaction_pool: Arc<T>,
}
