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

use async_trait::async_trait;

use crate::{Data, Hasher, NodeIndex, Round};

/// The source of data items that consensus should order.
///
/// AlephBFT internally calls [`DataProvider::get_data`] whenever a new unit is created and data
/// needs to be placed inside.
///
/// We refer to the documentation
/// https://cardinal-cryptography.github.io/AlephBFT/aleph_bft_api.html for a discussion and
/// examples of how this trait can be implemented.
#[async_trait]
pub trait DataProvider: Sync + Send + 'static {
    /// Type of data returned by this provider.
    type Output: Data;
    /// Outputs a new data item to be ordered.
    async fn get_data(&mut self) -> Option<Self::Output>;
}

/// The source of finalization of the units that consensus produces.
///
/// The [`FinalizationHandler::data_finalized`] method is called whenever a piece of data input
/// to the algorithm using [`DataProvider::get_data`] has been finalized, in order of finalization.
pub trait FinalizationHandler<D: Data>: Sync + Send + 'static {
    /// Data, provided by [DataProvider::get_data], has been finalized.
    /// The calls to this function follow the order of finalization.
    fn data_finalized(&mut self, data: D);
}

/// Represents state of the main internal data structure of AlephBFT (i.e. direct acyclic graph) used for
/// achieving consensus.
///
/// Instances of this type are returned indirectly by [`member::run_session`] method using the
/// [`UnitFinalizationHandler`] trait. This way it allows to reconstruct the DAG's structure used by AlephBFT,
/// which can be then used for example for the purpose of node's performance evaluation.
pub struct OrderedUnit<D: Data, H: Hasher> {
    pub data: Option<D>,
    pub parents: Vec<H::Hash>,
    pub hash: H::Hash,
    pub creator: NodeIndex,
    pub round: Round,
}

/// The source of finalization of the units that consensus produces.
///
/// The [`UnitFinalizationHandler::batch_finalized`] method is called whenever a batch of units
/// has been finalized, in order of finalization.
pub trait UnitFinalizationHandler: Sync + Send + 'static {
    type Data: Data;
    type Hasher: Hasher;

    /// A batch of units, that contains data provided by [DataProvider::get_data], has been finalized.
    /// The calls to this function follow the order of finalization.
    fn batch_finalized(&mut self, batch: Vec<OrderedUnit<Self::Data, Self::Hasher>>);
}
