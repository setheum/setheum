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

use std::{
    collections::{HashSet, VecDeque},
    time::{Duration, Instant},
};

use crate::{
    testing::{init_log, spawn_honest_member, HonestMember, NetworkData},
    NodeCount, NodeIndex, SpawnHandle,
};
use aleph_bft_mock::{DataProvider, NetworkHook, Router, Spawner};
use futures::StreamExt;
use log::info;

struct Latency {
    who: NodeIndex,
    buffer: VecDeque<(Instant, (NetworkData, NodeIndex, NodeIndex))>,
}

const LATENCY: Duration = Duration::from_millis(300);

impl Latency {
    pub fn new(who: NodeIndex) -> Self {
        Latency {
            who,
            buffer: VecDeque::new(),
        }
    }

    fn add_message(
        &mut self,
        data: NetworkData,
        sender: NodeIndex,
        recipient: NodeIndex,
    ) -> Vec<(NetworkData, NodeIndex, NodeIndex)> {
        match sender == self.who || recipient == self.who {
            true => {
                self.buffer
                    .push_back((Instant::now(), (data, sender, recipient)));
                Vec::new()
            }
            false => vec![(data, sender, recipient)],
        }
    }

    fn messages_to_send(&mut self) -> Vec<(NetworkData, NodeIndex, NodeIndex)> {
        let mut result = Vec::new();
        while !self.buffer.is_empty() {
            let (when, msg) = self
                .buffer
                .pop_front()
                .expect("just checked it is not empty");
            if Instant::now().duration_since(when) < LATENCY {
                self.buffer.push_front((when, msg));
                break;
            }
            result.push(msg);
        }
        result
    }
}

impl NetworkHook<NetworkData> for Latency {
    fn process_message(
        &mut self,
        data: NetworkData,
        sender: NodeIndex,
        recipient: NodeIndex,
    ) -> Vec<(NetworkData, NodeIndex, NodeIndex)> {
        let mut result = self.add_message(data, sender, recipient);
        result.append(&mut self.messages_to_send());
        result
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn delayed_finalized() {
    let n_members = NodeCount(7);
    let australian = NodeIndex(0);
    init_log();
    let spawner = Spawner::new();
    let mut batch_rxs = Vec::new();
    let mut exits = Vec::new();
    let mut handles = Vec::new();
    let (mut net_hub, networks) = Router::new(n_members);

    net_hub.add_hook(Latency::new(australian));

    spawner.spawn("network-hub", net_hub);

    for (network, _) in networks {
        let ix = network.index();
        let HonestMember {
            finalization_rx,
            exit_tx,
            handle,
            ..
        } = spawn_honest_member(
            spawner,
            ix,
            n_members,
            vec![],
            DataProvider::new_range(ix.0 * 50, (ix.0 + 1) * 50),
            network,
        );
        batch_rxs.push(finalization_rx);
        exits.push(exit_tx);
        handles.push(handle);
    }
    let to_finalize: HashSet<u32> = (0..((n_members.0) * 50))
        .map(|number| number as u32)
        .collect();

    for mut rx in batch_rxs.drain(..) {
        let mut to_finalize_local = to_finalize.clone();
        while !to_finalize_local.is_empty() {
            let number = rx.next().await.unwrap();
            info!("finalizing {}", number);
            assert!(to_finalize_local.remove(&number));
        }
        info!("finished one node");
    }

    for exit in exits {
        let _ = exit.send(());
    }
    for handle in handles {
        let _ = handle.await;
    }
}
