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

pub use crate::setheum::api::runtime_types::*;
use crate::{
    setheum_runtime::SessionKeys,
    api::runtime_types::{
        primitives::app::Public as AlephPublic,
        sp_consensus_aura::sr25519::app_sr25519::Public as AuraPublic,
        sp_core::{ed25519::Public as EdPublic, sr25519::Public as SrPublic},
    },
    pallet_staking::EraRewardPoints,
    sp_weights::weight_v2::Weight,
};

impl<AccountId> Default for EraRewardPoints<AccountId> {
    fn default() -> Self {
        Self {
            total: 0,
            individual: vec![],
        }
    }
}

// Manually implementing decoding
impl From<Vec<u8>> for SessionKeys {
    fn from(bytes: Vec<u8>) -> Self {
        assert_eq!(bytes.len(), 64);
        Self {
            aura: AuraPublic(SrPublic(
                bytes[..32]
                    .try_into()
                    .expect("Failed to convert bytes slice to an Aura key!"),
            )),
            aleph: AlephPublic(EdPublic(
                bytes[32..64]
                    .try_into()
                    .expect("Failed to convert bytes slice to an Aleph key!"),
            )),
        }
    }
}

impl TryFrom<String> for SessionKeys {
    type Error = ();

    fn try_from(keys: String) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> = match hex::FromHex::from_hex(keys) {
            Ok(bytes) => bytes,
            Err(_) => return Err(()),
        };
        Ok(SessionKeys::from(bytes))
    }
}

impl Weight {
/// Returns new instance of weight v2 object.
    pub const fn new(ref_time: u64, proof_size: u64) -> Self {
        Self {
            ref_time,
            proof_size,
        }
    }
}
