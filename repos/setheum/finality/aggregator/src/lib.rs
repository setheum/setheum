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

use std::fmt::Debug;

use set_bft_rmc::Message as RmcMessage;
use set_bft_types::Recipient;

const LOG_TARGET: &str = "setbft-aggregator";

mod aggregator;

pub use crate::aggregator::{HashSignatureAggregator, IO};

pub type RmcNetworkData<H, S, SS> = RmcMessage<H, S, SS>;

#[derive(Debug)]
pub enum NetworkError {
	SendFail,
}

#[async_trait::async_trait]
pub trait ProtocolSink<D>: Send + Sync {
	async fn next(&mut self) -> Option<D>;
	fn send(&self, data: D, recipient: Recipient) -> Result<(), NetworkError>;
}
