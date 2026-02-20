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

use crate::{
    upload::Determinism,
    WasmCode,
};
use subxt::{
    ext::{
        codec::Compact,
        scale_encode::EncodeAsType,
    },
    utils::MultiAddress,
};

/// Copied from `sp_weight` to additionally implement `scale_encode::EncodeAsType`.
#[derive(Debug, EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub(crate) struct Weight {
    #[codec(compact)]
    /// The weight of computational time used based on some reference hardware.
    ref_time: u64,
    #[codec(compact)]
    /// The weight of storage space used by proof of validity.
    proof_size: u64,
}

impl From<sp_weights::Weight> for Weight {
    fn from(weight: sp_weights::Weight) -> Self {
        Self {
            ref_time: weight.ref_time(),
            proof_size: weight.proof_size(),
        }
    }
}

impl core::fmt::Display for Weight {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Weight(ref_time: {}, proof_size: {})",
            self.ref_time, self.proof_size
        )
    }
}

/// A raw call to `pallet-contracts`'s `remove_code`.
#[derive(EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub(crate) struct RemoveCode<Hash> {
    code_hash: Hash,
}

impl<Hash> RemoveCode<Hash> {
    pub fn new(code_hash: Hash) -> Self {
        Self { code_hash }
    }

    pub fn build(self) -> subxt::tx::DefaultPayload<Self> {
        subxt::tx::DefaultPayload::new("Contracts", "remove_code", self)
    }
}

/// A raw call to `pallet-contracts`'s `upload_code`.
#[derive(Debug, EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub struct UploadCode<Balance> {
    code: Vec<u8>,
    storage_deposit_limit: Option<Compact<Balance>>,
    determinism: Determinism,
}

impl<Balance> UploadCode<Balance> {
    pub fn new(
        code: WasmCode,
        storage_deposit_limit: Option<Balance>,
        determinism: Determinism,
    ) -> Self {
        Self {
            code: code.0,
            storage_deposit_limit: storage_deposit_limit.map(Into::into),
            determinism,
        }
    }

    pub fn build(self) -> subxt::tx::DefaultPayload<Self> {
        subxt::tx::DefaultPayload::new("Contracts", "upload_code", self)
    }
}

/// A raw call to `pallet-contracts`'s `instantiate_with_code`.
#[derive(Debug, EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub struct InstantiateWithCode<Balance> {
    #[codec(compact)]
    value: Balance,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<Balance>>,
    code: Vec<u8>,
    data: Vec<u8>,
    salt: Vec<u8>,
}

impl<Balance> InstantiateWithCode<Balance> {
    pub fn new(
        value: Balance,
        gas_limit: sp_weights::Weight,
        storage_deposit_limit: Option<Balance>,
        code: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> Self {
        Self {
            value,
            gas_limit: gas_limit.into(),
            storage_deposit_limit: storage_deposit_limit.map(Into::into),
            code,
            data,
            salt,
        }
    }

    pub fn build(self) -> subxt::tx::DefaultPayload<Self> {
        subxt::tx::DefaultPayload::new("Contracts", "instantiate_with_code", self)
    }
}

/// A raw call to `pallet-contracts`'s `instantiate_with_code_hash`.
#[derive(Debug, EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub struct Instantiate<Hash, Balance>
where
    Hash: EncodeAsType,
{
    #[codec(compact)]
    value: Balance,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<Balance>>,
    code_hash: Hash,
    data: Vec<u8>,
    salt: Vec<u8>,
}

impl<Hash, Balance> Instantiate<Hash, Balance>
where
    Hash: EncodeAsType,
{
    pub fn new(
        value: Balance,
        gas_limit: sp_weights::Weight,
        storage_deposit_limit: Option<Balance>,
        code_hash: Hash,
        data: Vec<u8>,
        salt: Vec<u8>,
    ) -> Self {
        Self {
            value,
            gas_limit: gas_limit.into(),
            storage_deposit_limit: storage_deposit_limit.map(Into::into),
            code_hash,
            data,
            salt,
        }
    }

    pub fn build(self) -> subxt::tx::DefaultPayload<Self> {
        subxt::tx::DefaultPayload::new("Contracts", "instantiate", self)
    }
}

/// A raw call to `pallet-contracts`'s `call`.
#[derive(EncodeAsType)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
pub struct Call<AccountId, Balance> {
    dest: MultiAddress<AccountId, ()>,
    #[codec(compact)]
    value: Balance,
    gas_limit: Weight,
    storage_deposit_limit: Option<Compact<Balance>>,
    data: Vec<u8>,
}

impl<AccountId, Balance> Call<AccountId, Balance> {
    pub fn new(
        dest: MultiAddress<AccountId, ()>,
        value: Balance,
        gas_limit: sp_weights::Weight,
        storage_deposit_limit: Option<Balance>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            dest,
            value,
            gas_limit: gas_limit.into(),
            storage_deposit_limit: storage_deposit_limit.map(Into::into),
            data,
        }
    }

    pub fn build(self) -> subxt::tx::DefaultPayload<Self> {
        subxt::tx::DefaultPayload::new("Contracts", "call", self)
    }
}
