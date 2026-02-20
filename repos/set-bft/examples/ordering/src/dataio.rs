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

use aleph_bft_types::{
    DataProvider as DataProviderT, FinalizationHandler as FinalizationHandlerT, NodeIndex,
};
use async_trait::async_trait;
use codec::{Decode, Encode};
use futures::{channel::mpsc::unbounded, future::pending};
use log::{error, info};

type Receiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;
type Sender<T> = futures::channel::mpsc::UnboundedSender<T>;

pub type Data = (NodeIndex, u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Decode, Encode)]
pub struct DataProvider {
    id: NodeIndex,
    starting_data_item: u32,
    data_items: u32,
    current_data: u32,
    stalled: bool,
}

impl DataProvider {
    pub fn new(id: NodeIndex, starting_data_item: u32, data_items: u32, stalled: bool) -> Self {
        Self {
            id,
            starting_data_item,
            current_data: starting_data_item,
            data_items,
            stalled,
        }
    }
}

#[async_trait]
impl DataProviderT for DataProvider {
    type Output = Data;

    async fn get_data(&mut self) -> Option<Data> {
        if self.starting_data_item + self.data_items == self.current_data {
            if self.stalled {
                info!("Awaiting DataProvider::get_data forever");
                pending::<()>().await;
            }
            info!("Providing None");
            None
        } else {
            let data = (self.id, self.current_data);
            info!("Providing data: {}", self.current_data);
            self.current_data += 1;
            Some(data)
        }
    }
}

#[derive(Clone)]
pub struct FinalizationHandler {
    tx: Sender<Data>,
}

impl FinalizationHandlerT<Data> for FinalizationHandler {
    fn data_finalized(&mut self, data: Data) {
        if let Err(e) = self.tx.unbounded_send(data) {
            error!(target: "finalization-handler", "Error when sending data from FinalizationHandler {:?}.", e);
        }
    }
}

impl FinalizationHandler {
    pub fn new() -> (Self, Receiver<Data>) {
        let (tx, rx) = unbounded();
        (Self { tx }, rx)
    }
}
