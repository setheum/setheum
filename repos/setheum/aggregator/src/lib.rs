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
    fmt::{Debug, Display},
    hash::Hash as StdHash,
};

use aleph_bft_rmc::{Message as RmcMessage, Signable};
use aleph_bft_types::Recipient;
use parity_scale_codec::{Codec, Decode, Encode};

mod aggregator;

pub use crate::aggregator::{BlockSignatureAggregator, IO};

pub type RmcNetworkData<H, S, SS> = RmcMessage<SignableHash<H>, S, SS>;

/// A convenience trait for gathering all of the desired hash characteristics.
pub trait Hash: AsRef<[u8]> + StdHash + Eq + Clone + Codec + Debug + Display + Send + Sync {}

impl<T: AsRef<[u8]> + StdHash + Eq + Clone + Codec + Debug + Display + Send + Sync> Hash for T {}

/// A wrapper allowing block hashes to be signed.
#[derive(PartialEq, Eq, StdHash, Clone, Debug, Default, Encode, Decode)]
pub struct SignableHash<H: Hash> {
    hash: H,
}

impl<H: Hash> SignableHash<H> {
    pub fn new(hash: H) -> Self {
        Self { hash }
    }

    pub fn get_hash(&self) -> H {
        self.hash.clone()
    }
}

impl<H: Hash> Signable for SignableHash<H> {
    type Hash = H;
    fn hash(&self) -> Self::Hash {
        self.hash.clone()
    }
}

#[derive(Debug)]
pub enum NetworkError {
    SendFail,
}

#[async_trait::async_trait]
pub trait ProtocolSink<D>: Send + Sync {
    async fn next(&mut self) -> Option<D>;
    fn send(&self, data: D, recipient: Recipient) -> Result<(), NetworkError>;
}
