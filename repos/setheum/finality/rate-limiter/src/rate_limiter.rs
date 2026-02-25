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

use std::time::Instant;

use futures::future::pending;

use crate::{token_bucket::SharedTokenBucket, RatePerSecond};

pub type SharedRateLimiter = RateLimiterFacade;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Deadline {
    Never,
    Instant(Instant),
}

impl From<Deadline> for Option<Instant> {
    fn from(value: Deadline) -> Self {
        match value {
            Deadline::Never => None,
            Deadline::Instant(value) => Some(value),
        }
    }
}

pub enum RateLimiterFacade {
    NoTraffic,
    RateLimiter(SharedTokenBucket),
}

impl RateLimiterFacade {
    pub fn new(rate: RatePerSecond) -> Self {
        match rate {
            RatePerSecond::Block => Self::NoTraffic,
            RatePerSecond::Rate(rate) => Self::RateLimiter(SharedTokenBucket::new(rate)),
        }
    }

    pub async fn rate_limit(self, read_size: usize) -> Self {
        match self {
            RateLimiterFacade::NoTraffic => pending().await,
            RateLimiterFacade::RateLimiter(rate_limiter) => RateLimiterFacade::RateLimiter(
                rate_limiter
                    .rate_limit(read_size.try_into().unwrap_or(u64::MAX))
                    .await,
            ),
        }
    }

    pub fn share(&self) -> Self {
        match self {
            RateLimiterFacade::NoTraffic => RateLimiterFacade::NoTraffic,
            RateLimiterFacade::RateLimiter(shared_token_bucket) => {
                RateLimiterFacade::RateLimiter(shared_token_bucket.share())
            }
        }
    }
}
