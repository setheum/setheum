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

use aleph_bft_types::{DataProvider as DataProviderT, FinalizationHandler as FinalizationHandlerT};
use async_trait::async_trait;
use codec::{Decode, Encode};
use futures::{channel::mpsc::unbounded, future::pending, AsyncWrite};
use log::error;
use parking_lot::Mutex;
use std::{
    io::{self},
    pin::Pin,
    sync::Arc,
    task::{self, Poll},
};

type Receiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;
type Sender<T> = futures::channel::mpsc::UnboundedSender<T>;

pub type Data = u32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DataProvider {
    counter: usize,
    n_data: Option<usize>,
}

impl DataProvider {
    pub fn new() -> Self {
        Self {
            counter: 0,
            n_data: None,
        }
    }

    pub fn new_finite(n_data: usize) -> Self {
        Self {
            counter: 0,
            n_data: Some(n_data),
        }
    }
    pub fn new_range(start: usize, end: usize) -> Self {
        Self {
            counter: start,
            n_data: Some(end),
        }
    }
}

#[async_trait]
impl DataProviderT for DataProvider {
    type Output = Data;

    async fn get_data(&mut self) -> Option<Data> {
        let result = self.counter as u32;
        self.counter += 1;
        if let Some(n_data) = self.n_data {
            if n_data < self.counter {
                return None;
            }
        }
        Some(result)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Decode, Encode)]
pub struct StalledDataProvider {}

impl StalledDataProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl DataProviderT for StalledDataProvider {
    type Output = Data;

    async fn get_data(&mut self) -> Option<Data> {
        pending().await
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Default)]
pub struct Saver {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Saver {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl AsyncWrite for Saver {
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut task::Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.data.lock().extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut task::Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

impl From<Arc<Mutex<Vec<u8>>>> for Saver {
    fn from(data: Arc<Mutex<Vec<u8>>>) -> Self {
        Self { data }
    }
}

pub type Loader = futures::io::Cursor<Vec<u8>>;
