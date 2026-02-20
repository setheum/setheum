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
    AccountId32,
    Hex,
    Value,
};
use anyhow::{
    Context,
    Result,
};
use scale::{
    Decode,
    Encode,
    Output,
};
use scale_info::{
    form::PortableForm,
    IntoPortable,
    Path,
    TypeInfo,
};
use std::{
    boxed::Box,
    collections::HashMap,
    convert::TryFrom,
    str::FromStr,
};

/// Provides custom encoding and decoding for predefined environment types.
#[derive(Default)]
pub struct EnvTypesTranscoder {
    encoders: HashMap<u32, Box<dyn CustomTypeEncoder>>,
    decoders: HashMap<u32, Box<dyn CustomTypeDecoder>>,
}

impl EnvTypesTranscoder {
    /// Construct an `EnvTypesTranscoder` from the given type registry.
    pub fn new(
        encoders: HashMap<u32, Box<dyn CustomTypeEncoder>>,
        decoders: HashMap<u32, Box<dyn CustomTypeDecoder>>,
    ) -> Self {
        Self { encoders, decoders }
    }

    /// If the given type id is for a type with custom encoding, encodes the given value
    /// with the custom encoder and returns `true`. Otherwise returns `false`.
    ///
    /// # Errors
    ///
    /// - If the custom encoding fails.
    pub fn try_encode<O>(
        &self,
        type_id: u32,
        value: &Value,
        output: &mut O,
    ) -> Result<bool>
    where
        O: Output,
    {
        match self.encoders.get(&type_id) {
            Some(encoder) => {
                tracing::debug!("Encoding type {:?} with custom encoder", type_id);
                let encoded_env_type = encoder
                    .encode_value(value)
                    .context("Error encoding custom type")?;
                output.write(&encoded_env_type);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// If the given type lookup id is for an environment type with custom
    /// decoding, decodes the given input with the custom decoder and returns
    /// `Some(value)`. Otherwise returns `None`.
    ///
    /// # Errors
    ///
    /// - If the custom decoding fails.
    pub fn try_decode(&self, type_id: u32, input: &mut &[u8]) -> Result<Option<Value>> {
        match self.decoders.get(&type_id) {
            Some(decoder) => {
                tracing::debug!("Decoding type {:?} with custom decoder", type_id);
                let decoded = decoder.decode_value(input)?;
                Ok(Some(decoded))
            }
            None => {
                tracing::debug!("No custom decoder found for type {:?}", type_id);
                Ok(None)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PathKey(Vec<String>);

impl PathKey {
    pub fn from_type<T>() -> Self
    where
        T: TypeInfo,
    {
        let type_info = T::type_info();
        let path = type_info.path.into_portable(&mut Default::default());
        PathKey::from(&path)
    }
}

impl From<&Path<PortableForm>> for PathKey {
    fn from(path: &Path<PortableForm>) -> Self {
        PathKey(path.segments.to_vec())
    }
}

pub type TypesByPath = HashMap<PathKey, u32>;

/// Implement this trait to define custom encoding for a type in a `scale-info` type
/// registry.
pub trait CustomTypeEncoder: Send + Sync {
    fn encode_value(&self, value: &Value) -> Result<Vec<u8>>;
}

/// Implement this trait to define custom decoding for a type in a `scale-info` type
/// registry.
pub trait CustomTypeDecoder: Send + Sync {
    fn decode_value(&self, input: &mut &[u8]) -> Result<Value>;
}

/// Custom encoding/decoding for the Substrate `AccountId` type.
///
/// Enables an `AccountId` to be input/ouput as an SS58 Encoded literal e.g.
/// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[derive(Clone)]
pub struct AccountId;

impl CustomTypeEncoder for AccountId {
    fn encode_value(&self, value: &Value) -> Result<Vec<u8>> {
        let account_id = match value {
            Value::Literal(literal) => {
                AccountId32::from_str(literal).map_err(|e| {
                    anyhow::anyhow!(
                        "Error parsing AccountId from literal `{}`: {}",
                        literal,
                        e
                    )
                })?
            }
            Value::String(string) => {
                AccountId32::from_str(string).map_err(|e| {
                    anyhow::anyhow!(
                        "Error parsing AccountId from string '{}': {}",
                        string,
                        e
                    )
                })?
            }
            Value::Hex(hex) => {
                AccountId32::try_from(hex.bytes()).map_err(|_| {
                    anyhow::anyhow!(
                        "Error converting hex bytes `{:?}` to AccountId",
                        hex.bytes()
                    )
                })?
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected a string or a literal for an AccountId"
                ))
            }
        };
        Ok(account_id.encode())
    }
}

impl CustomTypeDecoder for AccountId {
    fn decode_value(&self, input: &mut &[u8]) -> Result<Value> {
        let account_id = AccountId32::decode(input)?;
        Ok(Value::Literal(account_id.to_ss58check()))
    }
}

/// Custom decoding for the `Hash` or `[u8; 32]` type so that it is displayed as a hex
/// encoded string.
pub struct Hash;

impl CustomTypeDecoder for Hash {
    fn decode_value(&self, input: &mut &[u8]) -> Result<Value> {
        let hash = primitive_types::H256::decode(input)?;
        Ok(Value::Hex(Hex::from_str(&format!("{hash:?}"))?))
    }
}
