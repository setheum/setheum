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

use crate::{
    Data, DataProvider, FinalizationHandler, Hasher, OrderedUnit, UnitFinalizationHandler,
};
use futures::{AsyncRead, AsyncWrite};
use std::marker::PhantomData;

/// This adapter allows to map an implementation of [`FinalizationHandler`] onto implementation of [`UnitFinalizationHandler`].
pub struct FinalizationHandlerAdapter<FH, D, H> {
    finalization_handler: FH,
    _phantom: PhantomData<(D, H)>,
}

impl<FH, D, H> From<FH> for FinalizationHandlerAdapter<FH, D, H> {
    fn from(value: FH) -> Self {
        Self {
            finalization_handler: value,
            _phantom: PhantomData,
        }
    }
}

impl<D: Data, H: Hasher, FH: FinalizationHandler<D>> UnitFinalizationHandler
    for FinalizationHandlerAdapter<FH, D, H>
{
    type Data = D;
    type Hasher = H;

    fn batch_finalized(&mut self, batch: Vec<OrderedUnit<Self::Data, Self::Hasher>>) {
        for unit in batch {
            if let Some(data) = unit.data {
                self.finalization_handler.data_finalized(data)
            }
        }
    }
}

/// The local interface of the consensus algorithm. Contains a [`DataProvider`] as a source of data
/// to order, a [`UnitFinalizationHandler`] for handling ordered units, and a pair of read/write
/// structs intended for saving and restorin the state of the algorithm within the session, as a
/// contingency in the case of a crash.
#[derive(Clone)]
pub struct LocalIO<DP: DataProvider, UFH: UnitFinalizationHandler, US: AsyncWrite, UL: AsyncRead> {
    data_provider: DP,
    finalization_handler: UFH,
    unit_saver: US,
    unit_loader: UL,
}

impl<
        H: Hasher,
        DP: DataProvider,
        FH: FinalizationHandler<DP::Output>,
        US: AsyncWrite,
        UL: AsyncRead,
    > LocalIO<DP, FinalizationHandlerAdapter<FH, DP::Output, H>, US, UL>
{
    /// Create a new local interface. Note that this uses the simplified, and recommended,
    /// finalization handler that only deals with ordered data.
    pub fn new(
        data_provider: DP,
        finalization_handler: FH,
        unit_saver: US,
        unit_loader: UL,
    ) -> Self {
        Self {
            data_provider,
            finalization_handler: finalization_handler.into(),
            unit_saver,
            unit_loader,
        }
    }
}

impl<DP: DataProvider, UFH: UnitFinalizationHandler, US: AsyncWrite, UL: AsyncRead>
    LocalIO<DP, UFH, US, UL>
{
    /// Create a new local interface, providing a full implementation of a
    /// [`UnitFinalizationHandler`].Implementing [`UnitFinalizationHandler`] directly is more
    /// complex, and should be unnecessary for most usecases. Implement [`FinalizationHandler`]
    /// and use `new` instead, unless you absolutely know what you are doing.
    pub fn new_with_unit_finalization_handler(
        data_provider: DP,
        finalization_handler: UFH,
        unit_saver: US,
        unit_loader: UL,
    ) -> Self {
        Self {
            data_provider,
            finalization_handler,
            unit_saver,
            unit_loader,
        }
    }

    /// Disassemble the interface into components.
    pub fn into_components(self) -> (DP, UFH, US, UL) {
        let LocalIO {
            data_provider,
            finalization_handler,
            unit_saver,
            unit_loader,
        } = self;
        (data_provider, finalization_handler, unit_saver, unit_loader)
    }
}
