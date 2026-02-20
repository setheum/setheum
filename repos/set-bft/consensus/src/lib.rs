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

//! Implements the Aleph BFT Consensus protocol as a "finality gadget". The [run_session] function
//! requires access to a network layer, a cryptographic primitive, and a data provider that
//! gives appropriate access to the set of available data that we need to make consensus on.

mod alerts;
mod collection;
mod config;
mod consensus;
mod creation;
mod dag;
mod dissemination;
mod extension;
mod interface;
mod network;
mod terminator;
mod units;

mod backup;
mod task_queue;
#[cfg(test)]
mod testing;

pub use aleph_bft_types::{
    Data, DataProvider, FinalizationHandler, Hasher, IncompleteMultisignatureError, Index, Indexed,
    Keychain, MultiKeychain, Multisigned, Network, NodeCount, NodeIndex, NodeMap, NodeSubset,
    OrderedUnit, PartialMultisignature, PartiallyMultisigned, Recipient, Round, SessionId,
    Signable, Signature, SignatureError, SignatureSet, Signed, SpawnHandle, TaskHandle,
    UncheckedSigned, UnitFinalizationHandler,
};
pub use config::{
    create_config, default_config, default_delay_config, exponential_slowdown, Config, DelayConfig,
};
pub use consensus::run_session;
pub use interface::LocalIO;
pub use network::NetworkData;
pub use terminator::{handle_task_termination, Terminator};

type Receiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;
type Sender<T> = futures::channel::mpsc::UnboundedSender<T>;
