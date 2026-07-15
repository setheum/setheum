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

//! Implementations and definitions of traits used in current sbft
use crate::{
    block::{Header, HeaderVerifier, UnverifiedHeader},
    data_io::{SetBFTData, ChainInfoProvider, DataProvider, OrderedDataInterpreter},
};

#[async_trait::async_trait]
impl<UH: UnverifiedHeader> set_bft::DataProvider for DataProvider<UH> {
    type Output = SetBFTData<UH>;

    async fn get_data(&mut self) -> Option<SetBFTData<UH>> {
        DataProvider::get_data(self).await
    }
}

impl<CIP, H, V> set_bft::FinalizationHandler<SetBFTData<H::Unverified>>
    for OrderedDataInterpreter<CIP, H, V>
where
    CIP: ChainInfoProvider,
    H: Header,
    V: HeaderVerifier<H>,
{
    fn data_finalized(&mut self, data: SetBFTData<H::Unverified>) {
        OrderedDataInterpreter::data_finalized(self, data)
    }
}
