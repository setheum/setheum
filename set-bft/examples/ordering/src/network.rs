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

use crate::Data;
use aleph_bft::{NodeIndex, Recipient};
use aleph_bft_mock::{Hasher64, PartialMultisignature, Signature};
use codec::{Decode, Encode};
use log::error;
use std::net::SocketAddr;
use tokio::{
    io,
    net::UdpSocket,
    time::{sleep, Duration},
};

const MAX_UDP_DATAGRAM_BYTES: usize = 65536;

pub type NetworkData = aleph_bft::NetworkData<Hasher64, Data, Signature, PartialMultisignature>;

#[derive(Debug)]
pub struct Network {
    my_id: usize,
    addresses: Vec<SocketAddr>,
    socket: UdpSocket,
    /// Buffer for incoming data.
    ///
    /// It's allocated on the heap, because otherwise it overflows the stack when used inside a future.
    buffer: Box<[u8; MAX_UDP_DATAGRAM_BYTES]>,
}

impl Network {
    pub async fn new(
        my_id: NodeIndex,
        ports: &[usize],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let my_id = my_id.0;
        assert!(my_id < ports.len());

        let addresses = ports
            .iter()
            .map(|p| format!("127.0.0.1:{}", p).parse::<SocketAddr>())
            .collect::<Result<Vec<_>, _>>()?;

        let socket = Self::bind_socket(addresses[my_id]).await;
        Ok(Network {
            my_id,
            addresses,
            socket,
            buffer: Box::new([0; MAX_UDP_DATAGRAM_BYTES]),
        })
    }

    async fn bind_socket(address: SocketAddr) -> UdpSocket {
        loop {
            match UdpSocket::bind(address).await {
                Ok(socket) => {
                    return socket;
                }
                Err(e) => {
                    error!("{}", e);
                    error!("Waiting 10 seconds before the next attempt...");
                    sleep(Duration::from_secs(10)).await;
                }
            };
        }
    }

    fn send_to_peer(&self, data: NetworkData, recipient: usize) {
        if let Err(e) = self.try_send_to_peer(data, recipient) {
            error!("Sending failed, recipient: {:?}, error: {:?}", recipient, e);
        }
    }

    fn try_send_to_peer(&self, data: NetworkData, recipient: usize) -> io::Result<()> {
        let encoded = data.encode();
        assert!(encoded.len() <= MAX_UDP_DATAGRAM_BYTES);

        self.socket
            .try_send_to(&encoded, self.addresses[recipient])?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl aleph_bft::Network<NetworkData> for Network {
    fn send(&self, data: NetworkData, recipient: Recipient) {
        match recipient {
            Recipient::Everyone => {
                for r in 0..self.addresses.len() {
                    if r != self.my_id {
                        self.send_to_peer(data.clone(), r);
                    }
                }
            }
            Recipient::Node(r) => {
                if r.0 < self.addresses.len() {
                    self.send_to_peer(data, r.0);
                } else {
                    error!("Recipient unknown: {}", r.0);
                }
            }
        }
    }

    async fn next_event(&mut self) -> Option<NetworkData> {
        match self.socket.recv_from(self.buffer.as_mut()).await {
            Ok((_len, _addr)) => NetworkData::decode(&mut &self.buffer[..]).ok(),
            Err(e) => {
                error!("Couldn't receive datagram: {:?}", e);
                None
            }
        }
    }
}
