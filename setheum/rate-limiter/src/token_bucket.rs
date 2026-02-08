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

use std::{
    cmp::min,
    time::{Duration, Instant},
};

use log::trace;

use crate::LOG_TARGET;

/// Implementation of the `Token Bucket` algorithm for the purpose of rate-limiting access to some abstract resource.
#[derive(Clone, Debug)]
pub struct TokenBucket {
    rate_per_second: usize,
    available: usize,
    requested: usize,
    last_update: Instant,
}

impl TokenBucket {
/// Constructs a instance of [TokenBucket] with given target rate-per-second.
    pub fn new(rate_per_second: usize) -> Self {
        Self {
            rate_per_second,
            available: rate_per_second,
            requested: 0,
            last_update: Instant::now(),
        }
    }

    #[cfg(test)]
    pub fn new_with_now(rate_per_second: usize, now: Instant) -> Self {
        Self {
            last_update: now,
            ..Self::new(rate_per_second)
        }
    }

    fn calculate_delay(&self) -> Duration {
        let delay_micros = (self.requested - self.available)
            .saturating_mul(1_000_000)
            .saturating_div(self.rate_per_second);
        Duration::from_micros(delay_micros.try_into().unwrap_or(u64::MAX))
    }

    fn update_units(&mut self, now: Instant) -> usize {
        let time_since_last_update = now.duration_since(self.last_update);
        let new_units = time_since_last_update
            .as_micros()
            .saturating_mul(self.rate_per_second as u128)
            .saturating_div(1_000_000)
            .try_into()
            .unwrap_or(usize::MAX);
        self.available = self.available.saturating_add(new_units);
        self.last_update = now;

        let used = min(self.available, self.requested);
        self.available -= used;
        self.requested -= used;
        self.available = min(self.available, self.token_limit());
        self.available
    }

/// Calculates [Duration](time::Duration) by which we should delay next call to some governed resource in order to satisfy
/// configured rate limit.
    pub fn rate_limit(&mut self, requested: usize, now: Instant) -> Option<Duration> {
        trace!(
            target: LOG_TARGET,
            "TokenBucket called for {} of requested bytes. Internal state: {:?}.",
            requested,
            self
        );
        if self.requested > 0 || self.available < requested {
            assert!(
                now >= self.last_update,
                "Provided value for `now` should be at least equal to `self.last_update`: now = {:#?} self.last_update = {:#?}.",
                now,
                self.last_update
            );
            if self.update_units(now) < requested {
                self.requested = self.requested.saturating_add(requested);
                let required_delay = self.calculate_delay();
                return Some(required_delay);
            }
        }
        self.available -= requested;
        self.available = min(self.available, self.token_limit());
        None
    }

    fn token_limit(&self) -> usize {
        self.rate_per_second
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::TokenBucket;

    #[test]
    fn token_bucket_sanity_check() {
        let limit_per_second = 10;
        let now = Instant::now();
        let mut rate_limiter = TokenBucket::new_with_now(limit_per_second, now);

        assert_eq!(
            rate_limiter.rate_limit(9, now + Duration::from_secs(1)),
            None
        );

        assert!(rate_limiter
            .rate_limit(12, now + Duration::from_secs(1))
            .is_some());

        assert_eq!(
            rate_limiter.rate_limit(8, now + Duration::from_secs(3)),
            None
        );
    }

    #[test]
    fn no_slowdown_while_within_rate_limit() {
        let limit_per_second = 10;
        let now = Instant::now();
        let mut rate_limiter = TokenBucket::new_with_now(limit_per_second, now);

        assert_eq!(
            rate_limiter.rate_limit(9, now + Duration::from_secs(1)),
            None
        );
        assert_eq!(
            rate_limiter.rate_limit(5, now + Duration::from_secs(2)),
            None
        );
        assert_eq!(
            rate_limiter.rate_limit(1, now + Duration::from_secs(3)),
            None
        );
        assert_eq!(
            rate_limiter.rate_limit(9, now + Duration::from_secs(3)),
            None
        );
    }

    #[test]
    fn slowdown_when_limit_reached() {
        let limit_per_second = 10;
        let now = Instant::now();
        let mut rate_limiter = TokenBucket::new_with_now(limit_per_second, now);

        assert_eq!(rate_limiter.rate_limit(10, now), None);

// we should wait some time after reaching the limit
        assert!(rate_limiter.rate_limit(1, now).is_some());

        assert_eq!(
            rate_limiter.rate_limit(19, now),
            Some(Duration::from_secs(2)),
            "we should wait exactly 2 seconds"
        );
    }

    #[test]
    fn buildup_tokens_but_no_more_than_limit() {
        let limit_per_second = 10;
        let now = Instant::now();
        let mut rate_limiter = TokenBucket::new_with_now(limit_per_second, now);

        assert_eq!(
            rate_limiter.rate_limit(10, now + Duration::from_secs(2)),
            None
        );

        assert_eq!(
            rate_limiter.rate_limit(40, now + Duration::from_secs(10)),
            Some(Duration::from_secs(3)),
        );
        assert_eq!(
            rate_limiter.rate_limit(40, now + Duration::from_secs(11)),
            Some(Duration::from_secs(6))
        );
    }

    #[test]
    fn multiple_calls_buildup_wait_time() {
        let limit_per_second = 10;
        let now = Instant::now();
        let mut rate_limiter = TokenBucket::new_with_now(limit_per_second, now);

        assert_eq!(
            rate_limiter.rate_limit(10, now + Duration::from_secs(3)),
            None
        );

        assert_eq!(
            rate_limiter.rate_limit(10, now + Duration::from_secs(3)),
            None
        );

        assert_eq!(
            rate_limiter.rate_limit(10, now + Duration::from_secs(3)),
            Some(Duration::from_secs(1))
        );

        assert_eq!(
            rate_limiter.rate_limit(50, now + Duration::from_secs(3)),
            Some(Duration::from_secs(6))
        );
    }
}
