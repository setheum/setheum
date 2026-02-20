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
    alerts::AlertMessage,
    network::{NetworkData, NetworkDataInner, UnitMessage},
    Data, Hasher, Network, PartialMultisignature, Receiver, Recipient, Sender, Signature,
    Terminator,
};
use futures::{FutureExt, StreamExt};
use log::{debug, error, warn};

pub struct Hub<
    H: Hasher,
    D: Data,
    S: Signature,
    MS: PartialMultisignature,
    N: Network<NetworkData<H, D, S, MS>>,
> {
    network: N,
    units_to_send: Receiver<(UnitMessage<H, D, S>, Recipient)>,
    units_received: Sender<UnitMessage<H, D, S>>,
    alerts_to_send: Receiver<(AlertMessage<H, D, S, MS>, Recipient)>,
    alerts_received: Sender<AlertMessage<H, D, S, MS>>,
}

impl<
        H: Hasher,
        D: Data,
        S: Signature,
        MS: PartialMultisignature,
        N: Network<NetworkData<H, D, S, MS>>,
    > Hub<H, D, S, MS, N>
{
    pub fn new(
        network: N,
        units_to_send: Receiver<(UnitMessage<H, D, S>, Recipient)>,
        units_received: Sender<UnitMessage<H, D, S>>,
        alerts_to_send: Receiver<(AlertMessage<H, D, S, MS>, Recipient)>,
        alerts_received: Sender<AlertMessage<H, D, S, MS>>,
    ) -> Self {
        Hub {
            network,
            units_to_send,
            units_received,
            alerts_to_send,
            alerts_received,
        }
    }

    fn send(&self, data: NetworkData<H, D, S, MS>, recipient: Recipient) {
        self.network.send(data, recipient);
    }

    fn handle_incoming(&self, network_data: NetworkData<H, D, S, MS>) {
        let NetworkData(network_data) = network_data;
        use NetworkDataInner::*;
        match network_data {
            Units(unit_message) => {
                if let Err(e) = self.units_received.unbounded_send(unit_message) {
                    warn!(target: "AlephBFT-network-hub", "Error when sending units to consensus {:?}", e);
                }
            }

            Alert(alert_message) => {
                if let Err(e) = self.alerts_received.unbounded_send(alert_message) {
                    warn!(target: "AlephBFT-network-hub", "Error when sending alerts to consensus {:?}", e);
                }
            }
        }
    }

    pub async fn run(mut self, mut terminator: Terminator) {
        loop {
            use NetworkDataInner::*;
            futures::select! {
                unit_message = self.units_to_send.next() => match unit_message {
                    Some((unit_message, recipient)) => self.send(NetworkData(Units(unit_message)), recipient),
                    None => {
                        error!(target: "AlephBFT-network-hub", "Outgoing units stream closed.");
                        break;
                    }
                },
                alert_message = self.alerts_to_send.next() => match alert_message {
                    Some((alert_message, recipient)) => self.send(NetworkData(Alert(alert_message)), recipient),
                    None => {
                        error!(target: "AlephBFT-network-hub", "Outgoing alerts stream closed.");
                        break;
                    }
                },
                incoming_message = self.network.next_event().fuse() => match incoming_message {
                    Some(incoming_message) => self.handle_incoming(incoming_message),
                    None => {
                        error!(target: "AlephBFT-network-hub", "Network stopped working.");
                        break;
                    }
                },
                _ = terminator.get_exit().fuse() => {
                    terminator.terminate_sync().await;
                    break;
                }
            }
        }

        debug!(target: "AlephBFT-network-hub", "Network ended.");
    }
}
