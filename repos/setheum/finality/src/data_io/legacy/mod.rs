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

use std::{fmt::Debug, hash::Hash};

use parity_scale_codec::{Decode, Encode};

mod data_interpreter;
mod data_provider;
mod data_store;
mod proposal;
mod status_provider;

pub use data_interpreter::OrderedDataInterpreter;
pub use data_provider::{ChainTracker, DataProvider};
pub use data_store::{DataStore, DataStoreConfig};
pub use proposal::UnvalidatedSetBFTProposal;

pub use super::ChainInfoCacheConfig;

// Maximum number of blocks above the last finalized allowed in an SetBFT proposal.
pub const MAX_DATA_BRANCH_LEN: usize = 7;

/// The data ordered by the SetBFT consensus.
#[derive(Clone, Debug, Encode, Decode, Hash, PartialEq, Eq)]
pub struct SetBFTData {
    pub head_proposal: UnvalidatedSetBFTProposal,
}

/// A trait allowing to check the data contained in an SetBFT network message, for the purpose of
/// data availability checks.
pub trait SetBFTNetworkMessage: Clone + Debug {
    fn included_data(&self) -> Vec<SetBFTData>;
}

#[cfg(test)]
mod test {
    use crate::{
        data_io::legacy::{SetBFTData, UnvalidatedSetBFTProposal},
        testing::mocks::{TBlock, THeader},
    };

    pub fn unvalidated_proposal_from_headers(headers: Vec<THeader>) -> UnvalidatedSetBFTProposal {
        let num = headers.last().unwrap().number;
        let hashes = headers.into_iter().map(|header| header.hash()).collect();
        UnvalidatedSetBFTProposal::new(hashes, num)
    }

    pub fn setbft_data_from_blocks(blocks: Vec<TBlock>) -> SetBFTData {
        let headers = blocks.into_iter().map(|b| b.header).collect();
        setbft_data_from_headers(headers)
    }

    pub fn setbft_data_from_headers(headers: Vec<THeader>) -> SetBFTData {
        SetBFTData {
            head_proposal: unvalidated_proposal_from_headers(headers),
        }
    }
}
