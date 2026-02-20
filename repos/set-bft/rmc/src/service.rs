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

//! Reliable MultiCast - a primitive for Reliable Broadcast protocol.
use crate::{
    handler::{Handler, OnStartRmcResponse},
    scheduler::TaskScheduler,
    Message,
};
pub use aleph_bft_crypto::{MultiKeychain, Multisigned, Signable};
use core::fmt::Debug;
use log::{debug, warn};
use std::hash::Hash;

const LOG_TARGET: &str = "AlephBFT-rmc";

/// Reliable Multicast Box
///
/// The instance of [`Service<H, MK, SCH>`] is used to reliably broadcasts hashes of type `H`.
/// It collects the signed hashes and upon receiving a large enough number of them it yields
/// the multisigned hash.
///
/// A node with an instance of [`Service<H, MK, SCH>`] can initiate broadcasting a message `msg: H`
/// by calling [`Service::start_rmc`]. As a result, the node signs `msg` and starts scheduling
/// messages for broadcast which can be obtained by awaiting on [`Service::next_message`]. When
/// sufficiently many nodes initiate rmc with the same message `msg` and a node collects enough
/// signatures to form a complete multisignature under the message, [`Service::process_message`]
/// will return the multisigned hash.
///
/// We refer to the documentation https://cardinal-cryptography.github.io/AlephBFT/reliable_broadcast.html
/// for a high-level description of this protocol and how it is used for fork alerts.
pub struct Service<H, MK, SCH>
where
    H: Signable + Hash,
    MK: MultiKeychain,
    SCH: TaskScheduler<Message<H, MK::Signature, MK::PartialMultisignature>>,
{
    scheduler: SCH,
    handler: Handler<H, MK>,
}

impl<H, MK, SCH> Service<H, MK, SCH>
where
    H: Signable + Hash + Eq + Clone + Debug,
    MK: MultiKeychain,
    SCH: TaskScheduler<Message<H, MK::Signature, MK::PartialMultisignature>>,
{
    pub fn new(scheduler: SCH, handler: Handler<H, MK>) -> Self {
        Service { scheduler, handler }
    }

    /// Signs the given `hash` and adds the signature to the collection. If the given `hash`
    /// completes the multisignature, it is scheduled for the broadcasts and then returned.
    /// If the multisignature is not completed, `None` is returned. If the multisignature was
    /// already completed when starting rmc, no tasks are scheduled. Otherwise the signed hash
    /// is scheduled for the broadcasts.
    pub fn start_rmc(&mut self, hash: H) -> Option<Multisigned<H, MK>> {
        debug!(target: LOG_TARGET, "starting rmc for {:?}", hash);
        match self.handler.on_start_rmc(hash) {
            OnStartRmcResponse::SignedHash(signed_hash) => {
                self.scheduler
                    .add_task(Message::SignedHash(signed_hash.into_unchecked()));
            }
            OnStartRmcResponse::MultisignedHash(multisigned) => {
                self.scheduler.add_task(Message::MultisignedHash(
                    multisigned.clone().into_unchecked(),
                ));
                return Some(multisigned);
            }
            OnStartRmcResponse::Noop => {}
        }
        None
    }

    /// Processes a message which can be of two types. If the message is a hash signed by one
    /// person, it adds it to the collective signature. If it completes the multisignature, it is
    /// returned. Otherwise `None` is returned. If the message is a multisigned hash, it returns
    /// the multisignature, if we haven't seen it before. Otherwise `None` is returned.
    pub fn process_message(
        &mut self,
        message: Message<H, MK::Signature, MK::PartialMultisignature>,
    ) -> Option<Multisigned<H, MK>> {
        match message {
            Message::SignedHash(unchecked) => match self.handler.on_signed_hash(unchecked) {
                Ok(Some(multisigned)) => {
                    self.scheduler.add_task(Message::MultisignedHash(
                        multisigned.clone().into_unchecked(),
                    ));
                    return Some(multisigned);
                }
                Ok(None) => {}
                Err(error) => {
                    warn!(target: LOG_TARGET, "failed handling multisigned hash: {}", error);
                }
            },
            Message::MultisignedHash(unchecked) => {
                match self.handler.on_multisigned_hash(unchecked) {
                    Ok(Some(multisigned)) => {
                        self.scheduler.add_task(Message::MultisignedHash(
                            multisigned.clone().into_unchecked(),
                        ));
                        return Some(multisigned);
                    }
                    Ok(None) => {}
                    Err(error) => {
                        warn!(target: LOG_TARGET, "failed handling signed hash: {}", error);
                    }
                }
            }
        }
        None
    }

    /// Obtain the next message scheduled for broadcast.
    pub async fn next_message(&mut self) -> Message<H, MK::Signature, MK::PartialMultisignature> {
        self.scheduler.next_task().await
    }
}

#[cfg(test)]
mod tests {
    use crate::{DoublingDelayScheduler, Handler, Message, Service};
    use aleph_bft_crypto::{Multisigned, NodeCount, NodeIndex, Signed};
    use aleph_bft_mock::{BadSigning, Keychain, PartialMultisignature, Signable, Signature};
    use futures::{
        channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        future, StreamExt,
    };
    use rand::Rng;
    use std::{collections::HashMap, time::Duration};

    type TestMessage = Message<Signable, Signature, PartialMultisignature>;

    struct TestEnvironment {
        rmc_services: Vec<Service<Signable, Keychain, DoublingDelayScheduler<TestMessage>>>,
        rmc_start_tx: UnboundedSender<(Signable, NodeIndex)>,
        rmc_start_rx: UnboundedReceiver<(Signable, NodeIndex)>,
        broadcast_tx: UnboundedSender<(TestMessage, NodeIndex)>,
        broadcast_rx: UnboundedReceiver<(TestMessage, NodeIndex)>,
        hashes: HashMap<NodeIndex, Multisigned<Signable, Keychain>>,
        message_filter: Box<dyn FnMut(NodeIndex, TestMessage) -> bool>,
    }

    enum EnvironmentEvent {
        NetworkMessage(TestMessage, NodeIndex),
        ManualBroadcast(TestMessage, NodeIndex),
        StartRmc(Signable, NodeIndex),
    }

    impl TestEnvironment {
        fn new(
            node_count: NodeCount,
            message_filter: impl FnMut(NodeIndex, TestMessage) -> bool + 'static,
        ) -> Self {
            let mut rmc_services = vec![];
            let (rmc_start_tx, rmc_start_rx) = unbounded();
            let (broadcast_tx, broadcast_rx) = unbounded();

            for i in 0..node_count.0 {
                let service = Service::new(
                    DoublingDelayScheduler::new(Duration::from_millis(1)),
                    Handler::new(Keychain::new(node_count, NodeIndex(i))),
                );
                rmc_services.push(service);
            }

            TestEnvironment {
                rmc_services,
                rmc_start_tx,
                rmc_start_rx,
                broadcast_tx,
                broadcast_rx,
                hashes: HashMap::new(),
                message_filter: Box::new(message_filter),
            }
        }

        fn start_rmc(&self, hash: Signable, node_index: NodeIndex) {
            self.rmc_start_tx
                .unbounded_send((hash, node_index))
                .expect("our channel should be open");
        }

        fn broadcast_message(&self, message: TestMessage, node_index: NodeIndex) {
            self.broadcast_tx
                .unbounded_send((message, node_index))
                .expect("our channel should be open");
        }

        fn handle_message(
            &mut self,
            message: TestMessage,
            node_index: NodeIndex,
            use_filter: bool,
        ) {
            for (j, service) in self.rmc_services.iter_mut().enumerate() {
                if j == node_index.0
                    || (use_filter && !(self.message_filter)(j.into(), message.clone()))
                {
                    continue;
                }
                if let Some(multisigned) = service.process_message(message.clone()) {
                    assert_eq!(self.hashes.insert(j.into(), multisigned), None);
                    // there should be only one multisig per node
                }
            }
        }

        async fn next_event(&mut self) -> EnvironmentEvent {
            let message_futures = self
                .rmc_services
                .iter_mut()
                .map(|serv| Box::pin(serv.next_message()));
            tokio::select! {
                (message, i, _) = future::select_all(message_futures) => {
                    EnvironmentEvent::NetworkMessage(message, NodeIndex(i))
                }
                maybe_message = self.broadcast_rx.next() => {
                    let (message, node_index) = maybe_message.expect("our channel should be open");
                    EnvironmentEvent::ManualBroadcast(message, node_index)
                }
                maybe_start_rmc = self.rmc_start_rx.next() => {
                    let (hash, node_index) = maybe_start_rmc.expect("our channel should be open");
                    EnvironmentEvent::StartRmc(hash, node_index)
                }
            }
        }

        async fn collect_multisigned_hashes(
            mut self,
            expected_multisigs: usize,
        ) -> HashMap<NodeIndex, Multisigned<Signable, Keychain>> {
            while self.hashes.len() < expected_multisigs {
                match self.next_event().await {
                    EnvironmentEvent::StartRmc(hash, node_index) => {
                        let service = self
                            .rmc_services
                            .get_mut(node_index.0)
                            .expect("service should exist");
                        if let Some(multisigned) = service.start_rmc(hash) {
                            assert_eq!(self.hashes.insert(node_index, multisigned), None);
                            // there should be only one multisig per node
                        }
                    }
                    EnvironmentEvent::NetworkMessage(message, node_index) => {
                        self.handle_message(message, node_index, true);
                    }
                    EnvironmentEvent::ManualBroadcast(message, node_index) => {
                        self.handle_message(message, node_index, false);
                    }
                }
            }
            self.hashes
        }
    }

    /// Create 10 honest nodes and let each of them start rmc for the same hash.
    #[tokio::test]
    async fn simple_scenario() {
        let node_count = NodeCount(10);
        let environment = TestEnvironment::new(node_count, |_, _| true);
        let hash: Signable = "56".into();
        for i in 0..node_count.0 {
            environment.start_rmc(hash.clone(), NodeIndex(i));
        }

        let hashes = environment.collect_multisigned_hashes(node_count.0).await;
        assert_eq!(hashes.len(), node_count.0);
        for i in 0..node_count.0 {
            let multisignature = &hashes[&i.into()];
            assert_eq!(multisignature.as_signable(), &hash);
        }
    }

    /// Each message is delivered with 20% probability
    #[tokio::test]
    async fn faulty_network() {
        let node_count = NodeCount(10);
        let mut rng = rand::thread_rng();
        let environment = TestEnvironment::new(node_count, move |_, _| rng.gen_range(0..5) == 0);

        let hash: Signable = "56".into();
        for i in 0..node_count.0 {
            environment.start_rmc(hash.clone(), NodeIndex(i));
        }

        let hashes = environment.collect_multisigned_hashes(node_count.0).await;
        assert_eq!(hashes.len(), node_count.0);
        for i in 0..node_count.0 {
            let multisignature = &hashes[&i.into()];
            assert_eq!(multisignature.as_signable(), &hash);
        }
    }

    /// Only 7 nodes start rmc and one of the nodes which didn't start rmc
    /// is delivered only messages with complete multisignatures
    #[tokio::test]
    async fn node_hearing_only_multisignatures() {
        let node_count = NodeCount(10);
        let environment = TestEnvironment::new(node_count, move |node_ix, message| {
            !matches!((node_ix.0, message), (0, TestMessage::SignedHash(_)))
        });

        let threshold = node_count.consensus_threshold();
        let hash: Signable = "56".into();
        for i in 0..threshold.0 {
            environment.start_rmc(hash.clone(), NodeIndex(i));
        }

        let hashes = environment.collect_multisigned_hashes(node_count.0).await;
        assert_eq!(hashes.len(), node_count.0);
        for i in 0..node_count.0 {
            let multisignature = &hashes[&i.into()];
            assert_eq!(multisignature.as_signable(), &hash);
        }
    }

    /// 7 honest nodes and 3 dishonest nodes which emit bad signatures and multisignatures
    #[tokio::test]
    async fn bad_signatures_and_multisignatures_are_ignored() {
        let node_count = NodeCount(10);
        let _keychains = Keychain::new_vec(node_count);
        let environment = TestEnvironment::new(node_count, |_, _| true);

        let bad_hash: Signable = "65".into();
        let bad_keychain: BadSigning<Keychain> = Keychain::new(node_count, 0.into()).into();
        let bad_msg = TestMessage::SignedHash(
            Signed::sign_with_index(bad_hash.clone(), &bad_keychain).into(),
        );
        environment.broadcast_message(bad_msg, NodeIndex(0));
        let bad_msg = TestMessage::MultisignedHash(
            Signed::sign_with_index(bad_hash.clone(), &bad_keychain)
                .into_partially_multisigned(&bad_keychain)
                .into_unchecked(),
        );
        environment.broadcast_message(bad_msg, NodeIndex(0));

        let hash: Signable = "56".into();
        for i in 0..node_count.0 {
            environment.start_rmc(hash.clone(), NodeIndex(i));
        }

        let hashes = environment.collect_multisigned_hashes(node_count.0).await;
        assert_eq!(hashes.len(), node_count.0);
        for i in 0..node_count.0 {
            let multisignature = &hashes[&i.into()];
            assert_eq!(multisignature.as_signable(), &hash);
        }
    }
}
