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

use super::{
    Hex,
    Map,
    Seq,
    Tuple,
    Value,
};
use std::fmt::{
    Debug,
    Display,
    Formatter,
    LowerHex,
    Result,
};

/// Wraps Value for custom Debug impl to provide pretty-printed Display
struct DisplayValue<'a>(&'a Value);

impl Debug for DisplayValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.0 {
            Value::Bool(boolean) => <bool as Debug>::fmt(boolean, f),
            Value::Char(character) => <char as Debug>::fmt(character, f),
            Value::UInt(uint) => <u128 as Display>::fmt(uint, f),
            Value::Int(integer) => <i128 as Display>::fmt(integer, f),
            Value::Map(map) => <DisplayMap as Debug>::fmt(&DisplayMap(map), f),
            Value::Tuple(tuple) => <DisplayTuple as Debug>::fmt(&DisplayTuple(tuple), f),
            Value::String(string) => <String as Display>::fmt(string, f),
            Value::Seq(seq) => <DisplaySeq as Debug>::fmt(&DisplaySeq(seq), f),
            Value::Hex(hex) => <Hex as Debug>::fmt(hex, f),
            Value::Literal(literal) => <String as Display>::fmt(literal, f),
            Value::Unit => write!(f, "()"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::String(string) => <String as Display>::fmt(string, f),
            value => <DisplayValue as Debug>::fmt(&DisplayValue(value), f),
        }
    }
}

struct DisplayMap<'a>(&'a Map);

impl Debug for DisplayMap<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.0.ident {
            Some(ref name) => {
                let mut builder = f.debug_struct(name);
                for (name, value) in self.0.map.iter() {
                    builder.field(&format!("{name}"), &DisplayValue(value));
                }
                builder.finish()
            }
            None => {
                let mut builder = f.debug_map();
                for (name, value) in self.0.map.iter() {
                    builder.entry(name, &DisplayValue(value));
                }
                builder.finish()
            }
        }
    }
}

struct DisplayTuple<'a>(&'a Tuple);

impl Debug for DisplayTuple<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = self.0.ident.as_ref().map_or("", |s| s.as_str());
        let mut builder = f.debug_tuple(name);
        for value in self.0.values.iter() {
            builder.field(&DisplayValue(value));
        }
        builder.finish()
    }
}

struct DisplaySeq<'a>(&'a Seq);

impl Debug for DisplaySeq<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut builder = f.debug_list();
        for elem in &self.0.elems {
            builder.entry(&DisplayValue(elem));
        }
        builder.finish()
    }
}

impl Debug for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:#x}")
    }
}

impl LowerHex for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "0x{}", hex::encode(&self.bytes))
        } else {
            write!(f, "{}", hex::encode(&self.bytes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_map() {
        let map = Value::Map(Map::new(
            Some("M"),
            vec![(Value::String("a".into()), Value::UInt(1))]
                .into_iter()
                .collect(),
        ));
        assert_eq!(r#"M { a: 1 }"#, format!("{map}"), "non-alternate same line");
        assert_eq!(
            "M {\n    a: 1,\n}",
            format!("{map:#}"),
            "alternate indented (pretty)"
        );
    }
}
