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

use rate_limiter::{RateLimiter, SleepingRateLimiter};
use tokio::io::AsyncRead;

use crate::{ConnectionInfo, Data, Dialer, Listener, PeerAddressInfo, Splittable, Splitted};

pub struct RateLimitedAsyncRead<Read> {
    rate_limiter: RateLimiter,
    read: Read,
}

impl<Read> RateLimitedAsyncRead<Read> {
    pub fn new(read: Read, rate_limiter: RateLimiter) -> Self {
        Self { rate_limiter, read }
    }
}

impl<Read: AsyncRead + Unpin> AsyncRead for RateLimitedAsyncRead<Read> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let read = std::pin::Pin::new(&mut this.read);
        this.rate_limiter.rate_limit(read, cx, buf)
    }
}

impl<Read: ConnectionInfo> ConnectionInfo for RateLimitedAsyncRead<Read> {
    fn peer_address_info(&self) -> PeerAddressInfo {
        self.read.peer_address_info()
    }
}

/// Implementation of the [Dialer] trait governing all returned [Dialer::Connection] instances by a rate-limiting wrapper.
#[derive(Clone)]
pub struct RateLimitingDialer<D> {
    dialer: D,
    rate_limiter: SleepingRateLimiter,
}

impl<D> RateLimitingDialer<D> {
    pub fn new(dialer: D, rate_limiter: SleepingRateLimiter) -> Self {
        Self {
            dialer,
            rate_limiter,
        }
    }
}

#[async_trait::async_trait]
impl<A, D> Dialer<A> for RateLimitingDialer<D>
where
    A: Data,
    D: Dialer<A>,
    <D::Connection as Splittable>::Sender: Unpin,
    <D::Connection as Splittable>::Receiver: Unpin,
{
    type Connection = Splitted<
        RateLimitedAsyncRead<<D::Connection as Splittable>::Receiver>,
        <D::Connection as Splittable>::Sender,
    >;
    type Error = D::Error;

    async fn connect(&mut self, address: A) -> Result<Self::Connection, Self::Error> {
        let connection = self.dialer.connect(address).await?;
        let (sender, receiver) = connection.split();
        Ok(Splitted(
            RateLimitedAsyncRead::new(receiver, RateLimiter::new(self.rate_limiter.clone())),
            sender,
        ))
    }
}

/// Implementation of the [Listener] trait governing all returned [Listener::Connection] instances by a rate-limiting wrapper.
pub struct RateLimitingListener<L> {
    listener: L,
    rate_limiter: SleepingRateLimiter,
}

impl<L> RateLimitingListener<L> {
    pub fn new(listener: L, rate_limiter: SleepingRateLimiter) -> Self {
        Self {
            listener,
            rate_limiter,
        }
    }
}

#[async_trait::async_trait]
impl<L: Listener + Send> Listener for RateLimitingListener<L> {
    type Connection = Splitted<
        RateLimitedAsyncRead<<L::Connection as Splittable>::Receiver>,
        <L::Connection as Splittable>::Sender,
    >;
    type Error = L::Error;

    async fn accept(&mut self) -> Result<Self::Connection, Self::Error> {
        let connection = self.listener.accept().await?;
        let (sender, receiver) = connection.split();
        Ok(Splitted(
            RateLimitedAsyncRead::new(receiver, RateLimiter::new(self.rate_limiter.clone())),
            sender,
        ))
    }
}
