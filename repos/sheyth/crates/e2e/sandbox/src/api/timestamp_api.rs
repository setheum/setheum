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

use crate::Sandbox;

/// Generic Time type.
type MomentOf<R> = <R as pallet_timestamp::Config>::Moment;

/// Timestamp API used to interact with the timestamp pallet.
pub trait TimestampAPI {
    /// The runtime timestamp config.
    type T: pallet_timestamp::Config;

    /// Return the timestamp of the current block.
    fn get_timestamp(&mut self) -> MomentOf<Self::T>;

    /// Set the timestamp of the current block.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The new timestamp to be set.
    fn set_timestamp(&mut self, timestamp: MomentOf<Self::T>);
}

impl<T> TimestampAPI for T
where
    T: Sandbox,
    T::Runtime: pallet_timestamp::Config,
{
    type T = T::Runtime;

    fn get_timestamp(&mut self) -> MomentOf<Self::T> {
        self.execute_with(pallet_timestamp::Pallet::<T::Runtime>::get)
    }

    fn set_timestamp(&mut self, timestamp: MomentOf<Self::T>) {
        self.execute_with(|| {
            pallet_timestamp::Pallet::<T::Runtime>::set_timestamp(timestamp)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::prelude::*,
        DefaultSandbox,
    };

    #[test]
    fn getting_and_setting_timestamp_works() {
        let mut sandbox = DefaultSandbox::default();
        for timestamp in 0..10 {
            assert_ne!(sandbox.get_timestamp(), timestamp);
            sandbox.set_timestamp(timestamp);
            assert_eq!(sandbox.get_timestamp(), timestamp);

            sandbox.build_block();
        }
    }
}
