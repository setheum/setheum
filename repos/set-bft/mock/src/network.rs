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

use aleph_bft_types::{Network as NetworkT, NodeCount, NodeIndex, Recipient};
use futures::{
    channel::{
        mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    Future, StreamExt,
};
use log::debug;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
};

pub type NetworkReceiver<D> = UnboundedReceiver<(D, NodeIndex)>;
pub type NetworkSender<D> = UnboundedSender<(D, NodeIndex)>;

#[derive(Debug)]
pub struct Network<D: Debug> {
    rx: NetworkReceiver<D>,
    tx: NetworkSender<D>,
    peers: Vec<NodeIndex>,
    index: NodeIndex,
}

impl<D: Debug> Network<D> {
    pub fn new(
        rx: NetworkReceiver<D>,
        tx: NetworkSender<D>,
        peers: Vec<NodeIndex>,
        index: NodeIndex,
    ) -> Self {
        Network {
            rx,
            tx,
            peers,
            index,
        }
    }

    pub fn index(&self) -> NodeIndex {
        self.index
    }

    pub fn peers(&self) -> Vec<NodeIndex> {
        self.peers.clone()
    }
}

#[async_trait::async_trait]
impl<D: Clone + Send + Debug + 'static> NetworkT<D> for Network<D> {
    fn send(&self, data: D, recipient: Recipient) {
        use Recipient::*;
        match recipient {
            Node(node) => self
                .tx
                .unbounded_send((data, node))
                .expect("send on channel should work"),
            Everyone => {
                for peer in self.peers.iter() {
                    if *peer != self.index {
                        self.send(data.clone(), Node(*peer));
                    }
                }
            }
        }
    }

    async fn next_event(&mut self) -> Option<D> {
        Some(self.rx.next().await?.0)
    }
}

pub struct Peer<D> {
    tx: NetworkSender<D>,
    rx: NetworkReceiver<D>,
}

pub trait NetworkHook<D>: Send {
    fn process_message(
        &mut self,
        data: D,
        sender: NodeIndex,
        recipient: NodeIndex,
    ) -> Vec<(D, NodeIndex, NodeIndex)>;
}

pub struct UnreliableHook {
    reliability: f64,
}

impl UnreliableHook {
    // reliability - a number in the range [0, 1], 1.0 means perfect reliability, 0.0 means no message gets through
    pub fn new(reliability: f64) -> Self {
        UnreliableHook { reliability }
    }
}

impl<D> NetworkHook<D> for UnreliableHook {
    fn process_message(
        &mut self,
        data: D,
        sender: NodeIndex,
        recipient: NodeIndex,
    ) -> Vec<(D, NodeIndex, NodeIndex)> {
        let rand_sample = rand::random::<f64>();
        if rand_sample > self.reliability {
            debug!("Simulated network fail.");
            Vec::new()
        } else {
            vec![(data, sender, recipient)]
        }
    }
}

type ReconnectReceiver<D> = UnboundedReceiver<(NodeIndex, oneshot::Sender<Network<D>>)>;
pub type ReconnectSender<D> = UnboundedSender<(NodeIndex, oneshot::Sender<Network<D>>)>;

pub struct Router<D: Debug> {
    peers: RefCell<HashMap<NodeIndex, Peer<D>>>,
    peer_list: Vec<NodeIndex>,
    hook_list: RefCell<Vec<Box<dyn NetworkHook<D>>>>,
    peer_reconnect_rx: ReconnectReceiver<D>,
}

impl<D: Debug> Debug for Router<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("peers", &self.peer_list)
            .field("hook count", &self.hook_list.borrow().len())
            .finish()
    }
}

type RouterWithNetworks<D> = (Router<D>, Vec<(Network<D>, ReconnectSender<D>)>);

impl<D: Debug> Router<D> {
    pub fn new(n_members: NodeCount) -> RouterWithNetworks<D> {
        let peer_list = n_members.into_iterator().collect();
        let (reconnect_tx, peer_reconnect_rx) = unbounded();
        let mut router = Router {
            peers: RefCell::new(HashMap::new()),
            peer_list,
            hook_list: RefCell::new(Vec::new()),
            peer_reconnect_rx,
        };
        let mut networks = Vec::new();
        for ix in n_members.into_iterator() {
            let network = router.connect_peer(ix);
            networks.push((network, reconnect_tx.clone()));
        }
        (router, networks)
    }

    pub fn add_hook<HK: NetworkHook<D> + 'static>(&mut self, hook: HK) {
        self.hook_list.borrow_mut().push(Box::new(hook));
    }

    pub fn connect_peer(&mut self, peer: NodeIndex) -> Network<D> {
        assert!(
            self.peer_list.iter().any(|p| *p == peer),
            "Must connect a peer in the list."
        );
        assert!(
            !self.peers.borrow().contains_key(&peer),
            "Cannot connect a peer twice."
        );
        let (tx_in_hub, rx_in_hub) = unbounded();
        let (tx_out_hub, rx_out_hub) = unbounded();
        let peer_entry = Peer {
            tx: tx_out_hub,
            rx: rx_in_hub,
        };
        self.peers.borrow_mut().insert(peer, peer_entry);
        Network::new(rx_out_hub, tx_in_hub, self.peer_list.clone(), peer)
    }

    pub fn peer_list(&self) -> Vec<NodeIndex> {
        self.peer_list.clone()
    }
}

impl<D: Debug> Future for Router<D> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut self;
        let mut disconnected_peers: Vec<NodeIndex> = Vec::new();
        let mut buffer = Vec::new();
        for (peer_id, peer) in this.peers.borrow_mut().iter_mut() {
            loop {
                // this call is responsible for waking this Future
                match peer.rx.poll_next_unpin(cx) {
                    Poll::Ready(Some((data, recipient))) => {
                        buffer.push((data, *peer_id, recipient));
                    }
                    Poll::Ready(None) => {
                        disconnected_peers.push(*peer_id);
                        break;
                    }
                    Poll::Pending => {
                        break;
                    }
                }
            }
        }
        for peer_id in disconnected_peers {
            this.peers.borrow_mut().remove(&peer_id);
        }
        loop {
            // this call is responsible for waking this Future
            match this.peer_reconnect_rx.poll_next_unpin(cx) {
                Poll::Ready(Some((node_id, sender))) => {
                    sender
                        .send(this.connect_peer(node_id))
                        .expect("channel should be open");
                }
                Poll::Ready(None) => {
                    break;
                }
                Poll::Pending => {
                    break;
                }
            }
        }
        let mut new_buffer = Vec::new();
        for hook in this.hook_list.borrow_mut().iter_mut() {
            for (data, sender, recipient) in buffer {
                new_buffer.append(&mut hook.process_message(data, sender, recipient));
            }
            buffer = new_buffer;
            new_buffer = Vec::new();
        }
        for (data, sender, recipient) in buffer {
            if let Some(peer) = this.peers.borrow().get(&recipient) {
                peer.tx.unbounded_send((data, sender)).ok();
            }
        }
        if this.peers.borrow().is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
