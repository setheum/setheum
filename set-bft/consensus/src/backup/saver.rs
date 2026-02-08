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

use std::pin::Pin;

use crate::{
    dag::DagUnit,
    units::{UncheckedSignedUnit, WrappedUnit},
    Data, Hasher, MultiKeychain, Receiver, Sender, Terminator,
};
use codec::Encode;
use futures::{AsyncWrite, AsyncWriteExt, FutureExt, StreamExt};
use log::{debug, error};

const LOG_TARGET: &str = "AlephBFT-backup-saver";

/// Component responsible for saving units into backup.
/// It waits for items to appear on its receivers, and writes them to backup.
/// It announces a successful write through an appropriate response sender.
pub struct BackupSaver<H: Hasher, D: Data, MK: MultiKeychain, W: AsyncWrite> {
    units_from_consensus: Receiver<DagUnit<H, D, MK>>,
    responses_for_consensus: Sender<DagUnit<H, D, MK>>,
    backup: Pin<Box<W>>,
}

impl<H: Hasher, D: Data, MK: MultiKeychain, W: AsyncWrite> BackupSaver<H, D, MK, W> {
    pub fn new(
        units_from_consensus: Receiver<DagUnit<H, D, MK>>,
        responses_for_consensus: Sender<DagUnit<H, D, MK>>,
        backup: W,
    ) -> BackupSaver<H, D, MK, W> {
        BackupSaver {
            units_from_consensus,
            responses_for_consensus,
            backup: Box::pin(backup),
        }
    }

    pub async fn save_unit(&mut self, unit: &DagUnit<H, D, MK>) -> Result<(), std::io::Error> {
        let unit: UncheckedSignedUnit<_, _, _> = unit.clone().unpack().into();
        self.backup.write_all(&unit.encode()).await?;
        self.backup.flush().await
    }

    pub async fn run(&mut self, mut terminator: Terminator) {
        let mut terminator_exit = false;
        loop {
            futures::select! {
                unit = self.units_from_consensus.next() => {
                    let item = match unit {
                        Some(unit) => unit,
                        None => {
                            error!(target: LOG_TARGET, "receiver of units to save closed early");
                            break;
                        },
                    };
                    if let Err(e) = self.save_unit(&item).await {
                        error!(target: LOG_TARGET, "couldn't save item to backup: {:?}", e);
                        break;
                    }
                    if self.responses_for_consensus.unbounded_send(item).is_err() {
                        error!(target: LOG_TARGET, "couldn't respond with saved unit to consensus");
                        break;
                    }
                },
                _ = terminator.get_exit().fuse() => {
                    debug!(target: LOG_TARGET, "backup saver received exit signal.");
                    terminator_exit = true;
                }
            }

            if terminator_exit {
                debug!(target: LOG_TARGET, "backup saver decided to exit.");
                terminator.terminate_sync().await;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::{
        channel::{mpsc, oneshot},
        StreamExt,
    };

    use aleph_bft_mock::{Data, Hasher64, Keychain, Saver};

    use crate::{
        backup::BackupSaver,
        dag::ReconstructedUnit,
        units::{creator_set, preunit_to_signed_unit, TestingSignedUnit},
        NodeCount, Terminator,
    };

    type TestUnit = ReconstructedUnit<TestingSignedUnit>;
    type TestBackupSaver = BackupSaver<Hasher64, Data, Keychain, Saver>;
    struct PrepareSaverResponse<F: futures::Future> {
        task: F,
        units_for_saver: mpsc::UnboundedSender<TestUnit>,
        units_from_saver: mpsc::UnboundedReceiver<TestUnit>,
        exit_tx: oneshot::Sender<()>,
    }

    fn prepare_saver() -> PrepareSaverResponse<impl futures::Future> {
        let (units_for_saver, units_from_consensus) = mpsc::unbounded();
        let (units_for_consensus, units_from_saver) = mpsc::unbounded();
        let (exit_tx, exit_rx) = oneshot::channel();
        let backup = Saver::new();

        let task = {
            let mut saver: TestBackupSaver =
                BackupSaver::new(units_from_consensus, units_for_consensus, backup);

            async move {
                saver.run(Terminator::create_root(exit_rx, "saver")).await;
            }
        };

        PrepareSaverResponse {
            task,
            units_for_saver,
            units_from_saver,
            exit_tx,
        }
    }

    #[tokio::test]
    async fn test_proper_relative_responses_ordering() {
        let node_count = NodeCount(5);
        let PrepareSaverResponse {
            task,
            units_for_saver,
            mut units_from_saver,
            exit_tx,
        } = prepare_saver();

        let handle = tokio::spawn(async {
            task.await;
        });

        let creators = creator_set(node_count);
        let keychains: Vec<_> = node_count
            .into_iterator()
            .map(|id| Keychain::new(node_count, id))
            .collect();
        let units: Vec<TestUnit> = node_count
            .into_iterator()
            .map(|id| {
                ReconstructedUnit::initial(preunit_to_signed_unit(
                    creators[id.0].create_unit(0).unwrap(),
                    0,
                    &keychains[id.0],
                ))
            })
            .collect();

        for u in units.iter() {
            units_for_saver.unbounded_send(u.clone()).unwrap();
        }

        for u in units {
            let u_backup = units_from_saver.next().await.unwrap();
            assert_eq!(u, u_backup);
        }

        exit_tx.send(()).unwrap();
        handle.await.unwrap();
    }
}
