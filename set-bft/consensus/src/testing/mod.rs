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

mod alerts;
mod behind;
mod byzantine;
mod crash;
mod crash_recovery;
mod creation;
mod dag;
mod unreliable;

use crate::{
    create_config, run_session, Config, DelayConfig, LocalIO, Network as NetworkT, NodeCount,
    NodeIndex, SpawnHandle, TaskHandle, Terminator,
};
use aleph_bft_mock::{
    Data, DataProvider, FinalizationHandler, Hasher64, Keychain, Loader, Network as MockNetwork,
    PartialMultisignature, ReconnectSender as ReconnectSenderGeneric, Saver, Signature, Spawner,
};
use futures::channel::{mpsc::UnboundedReceiver, oneshot};
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

pub type NetworkData = crate::NetworkData<Hasher64, Data, Signature, PartialMultisignature>;

pub type Network = MockNetwork<NetworkData>;
pub type ReconnectSender = ReconnectSenderGeneric<NetworkData>;

pub fn init_log() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .is_test(true)
        .try_init();
}

pub fn gen_delay_config() -> DelayConfig {
    DelayConfig {
        tick_interval: Duration::from_millis(5),
        unit_rebroadcast_interval_min: Duration::from_millis(400),
        unit_rebroadcast_interval_max: Duration::from_millis(500),
        //50, 50, 50, 50, ...
        unit_creation_delay: Arc::new(|_| Duration::from_millis(50)),
        //100, 100, 100, ...
        coord_request_delay: Arc::new(|_| Duration::from_millis(100)),
        //3, 1, 1, 1, ...
        coord_request_recipients: Arc::new(|t| if t == 0 { 3 } else { 1 }),
        // 50, 50, 50, 50, ...
        parent_request_delay: Arc::new(|_| Duration::from_millis(50)),
        // 1, 1, 1, ...
        parent_request_recipients: Arc::new(|_| 1),
        // 50, 50, 50, 50, ...
        newest_request_delay: Arc::new(|_| Duration::from_millis(50)),
    }
}

pub fn gen_config(node_ix: NodeIndex, n_members: NodeCount, delay_config: DelayConfig) -> Config {
    create_config(n_members, node_ix, 0, 5000, delay_config, Duration::ZERO)
        .expect("Should always succeed with Duration::ZERO")
}

pub struct HonestMember {
    finalization_rx: UnboundedReceiver<Data>,
    saved_state: Arc<Mutex<Vec<u8>>>,
    exit_tx: oneshot::Sender<()>,
    handle: TaskHandle,
}

pub fn spawn_honest_member(
    spawner: Spawner,
    node_index: NodeIndex,
    n_members: NodeCount,
    units: Vec<u8>,
    data_provider: DataProvider,
    network: impl 'static + NetworkT<NetworkData>,
) -> HonestMember {
    let (finalization_handler, finalization_rx) = FinalizationHandler::new();
    let config = gen_config(node_index, n_members, gen_delay_config());
    let (exit_tx, exit_rx) = oneshot::channel();
    let spawner_inner = spawner;
    let unit_loader = Loader::new(units);
    let saved_state = Arc::new(Mutex::new(vec![]));
    let unit_saver: Saver = saved_state.clone().into();
    let local_io = LocalIO::new(data_provider, finalization_handler, unit_saver, unit_loader);
    let member_task = async move {
        let keychain = Keychain::new(n_members, node_index);
        run_session(
            config,
            local_io,
            network,
            keychain,
            spawner_inner,
            Terminator::create_root(exit_rx, "AlephBFT-member"),
        )
        .await
    };
    let handle = spawner.spawn_essential("member", member_task);
    HonestMember {
        finalization_rx,
        saved_state,
        exit_tx,
        handle,
    }
}
