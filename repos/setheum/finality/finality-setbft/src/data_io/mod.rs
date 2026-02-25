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

use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    num::NonZeroUsize,
};

use parity_scale_codec::{Decode, Encode};

use crate::block::UnverifiedHeader;

mod chain_info;
mod data_interpreter;
mod data_provider;
mod data_store;
mod proposal;
mod status_provider;

pub use chain_info::{ChainInfoProvider, SubstrateChainInfoProvider};
pub use data_interpreter::OrderedDataInterpreter;
pub use data_provider::{ChainTracker, DataProvider};
pub use data_store::DataStore;
#[cfg(test)]
pub use data_store::DataStoreConfig;
pub use proposal::UnvalidatedSetBFTProposal;

// Maximum number of blocks above the last finalized allowed in an SetBFT proposal.
pub const MAX_DATA_BRANCH_LEN: usize = 7;

/// The data ordered by the SetBFT consensus.
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct SetBFTData<UH: UnverifiedHeader> {
    pub head_proposal: UnvalidatedSetBFTProposal<UH>,
}

impl<UH: UnverifiedHeader> Hash for SetBFTData<UH> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.head_proposal.hash(state);
    }
}

/// A trait allowing to check the data contained in an SetBFT network message, for the purpose of
/// data availability checks.
pub trait SetBFTNetworkMessage<UH: UnverifiedHeader>: Clone + Debug {
    fn included_data(&self) -> Vec<SetBFTData<UH>>;
}

#[derive(Clone, Debug)]
pub struct ChainInfoCacheConfig {
    pub block_cache_capacity: NonZeroUsize,
}

impl Default for ChainInfoCacheConfig {
    fn default() -> ChainInfoCacheConfig {
        ChainInfoCacheConfig {
            block_cache_capacity: NonZeroUsize::new(2000).unwrap(),
        }
    }
}
