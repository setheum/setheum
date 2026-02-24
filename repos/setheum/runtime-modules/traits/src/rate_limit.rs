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

use frame_support::Parameter;
use parity_scale_codec::Encode;
use sp_runtime::{traits::Member, RuntimeDebug};

#[derive(PartialEq, Eq, RuntimeDebug)]
pub enum RateLimiterError {
	ExceedLimit,
}

/// Rate Limiter
pub trait RateLimiter {
	/// The type for the rate limiter.
	type RateLimiterId: Parameter + Member + Copy;

	/// Check whether the rate limiter of can be bypassed according to the
	/// `key`.
	fn is_whitelist(limiter_id: Self::RateLimiterId, key: impl Encode) -> bool;

	/// Check whether the `value` can be consumed under the limit of
	/// `limit_key`.
	fn can_consume(
		limiter_id: Self::RateLimiterId,
		limit_key: impl Encode,
		value: u128,
	) -> Result<(), RateLimiterError>;

	/// The handler function to consume quota.
	fn consume(limiter_id: Self::RateLimiterId, limit_key: impl Encode, value: u128);

	/// Try consume quota.
	fn try_consume(
		limiter_id: Self::RateLimiterId,
		limit_key: impl Encode + Clone,
		value: u128,
		whitelist_check: Option<impl Encode>,
	) -> Result<(), RateLimiterError> {
		let need_consume = match whitelist_check {
			Some(whitelist_key) => !Self::is_whitelist(limiter_id, whitelist_key),
			None => true,
		};

		if need_consume {
			Self::can_consume(limiter_id, limit_key.clone(), value)?;
			Self::consume(limiter_id, limit_key, value);
		}

		Ok(())
	}
}

impl RateLimiter for () {
	type RateLimiterId = ();

	fn is_whitelist(_: Self::RateLimiterId, _: impl Encode) -> bool {
		true
	}

	fn can_consume(_: Self::RateLimiterId, _: impl Encode, _: u128) -> Result<(), RateLimiterError> {
		Ok(())
	}

	fn consume(_: Self::RateLimiterId, _: impl Encode, _: u128) {}
}
