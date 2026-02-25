// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use libp2p::{core::muxing::StreamMuxer, PeerId, Transport};
use rate_limiter::{FuturesRateLimitedAsyncReadWrite, SharedRateLimiter};

struct RateLimitedStreamMuxer<SM> {
    rate_limiter: SharedRateLimiter,
    stream_muxer: SM,
}

impl<SM> RateLimitedStreamMuxer<SM> {
    pub fn new(stream_muxer: SM, rate_limiter: SharedRateLimiter) -> Self {
        Self {
            rate_limiter,
            stream_muxer,
        }
    }

    fn inner(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut SM>
    where
        SM: Unpin,
    {
        let this = self.get_mut();
        std::pin::Pin::new(&mut this.stream_muxer)
    }
}

impl<SM> StreamMuxer for RateLimitedStreamMuxer<SM>
where
    SM: StreamMuxer + Unpin,
    SM::Substream: Unpin,
{
    type Substream = FuturesRateLimitedAsyncReadWrite<SM::Substream>;

    type Error = SM::Error;

    fn poll_inbound(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Self::Substream, Self::Error>> {
        let rate_limiter = self.rate_limiter.share();
        self.inner().poll_inbound(cx).map(|result| {
            result.map(|substream| FuturesRateLimitedAsyncReadWrite::new(substream, rate_limiter))
        })
    }

    fn poll_outbound(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Self::Substream, Self::Error>> {
        let rate_limiter = self.rate_limiter.share();
        self.inner().poll_outbound(cx).map(|result| {
            result.map(|substream| FuturesRateLimitedAsyncReadWrite::new(substream, rate_limiter))
        })
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner().poll_close(cx)
    }

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<libp2p::core::muxing::StreamMuxerEvent, Self::Error>> {
        self.inner().poll(cx)
    }
}

/// Builds a rate-limited implementation of [libp2p::Transport].
/// Note: all of the `Send` constraints in the return type are put in order to satisfy constraints of the constructor of
/// [sc_network::NetworkWorker].
pub fn build_transport(
    rate_limiter: SharedRateLimiter,
    config: sc_network::transport::NetworkConfig,
) -> impl Transport<
    Output = (
        PeerId,
        impl StreamMuxer<Substream = impl Send, Error = impl Send> + Send,
    ),
    Dial = impl Send,
    ListenerUpgrade = impl Send,
    Error = impl Send,
> + Send {
    struct ClonableSharedRateLimiter(SharedRateLimiter);
    impl ClonableSharedRateLimiter {
        fn share(&self) -> SharedRateLimiter {
            self.0.share()
        }
    }
    impl Clone for ClonableSharedRateLimiter {
        fn clone(&self) -> Self {
            Self(self.share())
        }
    }
    let rate_limiter = ClonableSharedRateLimiter(rate_limiter);

    sc_network::transport::build_transport(
        config.keypair,
        config.memory_only,
        config.muxer_window_size,
        config.muxer_maximum_buffer_size,
    )
    .map(move |(peer_id, stream_muxer), _| {
        (
            peer_id,
            RateLimitedStreamMuxer::new(stream_muxer, rate_limiter.share()),
        )
    })
}
