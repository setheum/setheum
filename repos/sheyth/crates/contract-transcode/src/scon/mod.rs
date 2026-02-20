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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

// Copyright (C) Use Ink (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

//! SCALE Object Notation (SCON)

mod display;
mod parse;

use indexmap::IndexMap;

use crate::util;
use std::{
    cmp::{
        Eq,
        Ordering,
    },
    hash::{
        Hash,
        Hasher,
    },
    iter::FromIterator,
    ops::{
        Index,
        IndexMut,
    },
    str::FromStr,
};

use serde::{
    ser::SerializeMap,
    Serialize,
};

pub use self::parse::parse_value;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Value {
    Bool(bool),
    Char(char),
    UInt(u128),
    Int(i128),
    Map(Map),
    Tuple(Tuple),
    String(String),
    Seq(Seq),
    Hex(Hex),
    Literal(String),
    Unit,
}

#[derive(Clone, Debug)]
pub struct Map {
    ident: Option<String>,
    map: IndexMap<Value, Value>,
}

// `IndexMap` is defined outside and can not be made serializable.
// Therefore, we implement custom implementation for the wrapping `Map`
impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            // we need to convert the key to a string
            // because serde_json disallows non-string keys
            map.serialize_entry(&k.to_string(), v)?;
        }
        map.end()
    }
}

impl Eq for Map {}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|x| x.hash(state));
    }
}

impl Index<&Value> for Map {
    type Output = Value;

    fn index(&self, index: &Value) -> &Self::Output {
        &self.map[index]
    }
}

impl IndexMut<&Value> for Map {
    fn index_mut(&mut self, index: &Value) -> &mut Self::Output {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Map) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

/// Note: equality is only given if both values and order of values match
impl PartialEq for Map {
    fn eq(&self, other: &Map) -> bool {
        if self.map.len() != other.map.len() {
            return false
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Map) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromIterator<(Value, Value)> for Map {
    fn from_iter<T: IntoIterator<Item = (Value, Value)>>(iter: T) -> Self {
        Map::new(None, IndexMap::from_iter(iter))
    }
}

impl Map {
    /// Creates a new, empty `Map`.
    pub fn new(ident: Option<&str>, map: IndexMap<Value, Value>) -> Map {
        Map {
            ident: ident.map(|s| s.to_string()),
            map,
        }
    }

    /// Return the identifier of the [`Map`].
    pub fn ident(&self) -> Option<String> {
        self.ident.clone()
    }

    /// Iterate all key-value pairs.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&Value, &Value)> {
        self.map.iter()
    }

    /// Return an iterator over the map's values
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.map.values()
    }

    /// Return a reference to the value stored for string key, if it is present, else
    /// None.
    pub fn get_by_str(&self, key: &str) -> Option<&Value> {
        self.map.get(&Value::String(key.to_string()))
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tuple {
    ident: Option<String>,
    values: Vec<Value>,
}

impl From<Vec<Value>> for Tuple {
    fn from(values: Vec<Value>) -> Self {
        Tuple {
            ident: None,
            values,
        }
    }
}

impl Tuple {
    pub fn new(ident: Option<&str>, values: Vec<Value>) -> Self {
        Tuple {
            ident: ident.map(|s| s.into()),
            values,
        }
    }

    pub fn ident(&self) -> Option<String> {
        self.ident.clone()
    }

    /// Returns an iterator over the tuple's values
    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Seq {
    elems: Vec<Value>,
}

impl From<Vec<Value>> for Seq {
    fn from(elems: Vec<Value>) -> Self {
        Self::new(elems)
    }
}

impl Seq {
    pub fn new(elems: Vec<Value>) -> Self {
        Seq { elems }
    }

    pub fn elems(&self) -> &[Value] {
        &self.elems
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Hex {
    s: String,
    #[serde(skip_serializing)]
    bytes: Vec<u8>,
}

impl FromStr for Hex {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("0x").to_string();
        let bytes = util::decode_hex(&s)?;
        Ok(Self { s, bytes })
    }
}

impl Hex {
    pub fn as_str(&self) -> &str {
        &self.s
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
