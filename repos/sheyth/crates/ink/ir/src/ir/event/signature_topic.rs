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

// Copyright (C) Use Ink (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use impl_serde::serialize as serde_hex;
use syn::spanned::Spanned;

use crate::ast;

/// The signature topic argument of an event variant.
///
/// Used as part of `ink::event` macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureTopicArg {
    topic: [u8; 32],
}

impl SignatureTopicArg {
    pub fn signature_topic(&self) -> [u8; 32] {
        self.topic
    }
}

impl From<&[u8; 32]> for SignatureTopicArg {
    fn from(value: &[u8; 32]) -> Self {
        Self { topic: *value }
    }
}

impl TryFrom<&syn::Lit> for SignatureTopicArg {
    type Error = syn::Error;

    fn try_from(lit: &syn::Lit) -> Result<Self, Self::Error> {
        if let syn::Lit::Str(s) = lit {
            let bytes: [u8; 32] = serde_hex::from_hex(&s.value())
                    .map_err(|_| {
                        format_err_spanned!(
                            lit,
                            "`signature_topic` has invalid hex string",
                        )
                    })?
                    .try_into()
                    .map_err(|e: Vec<u8>| {
                        format_err_spanned!(
                            lit,
                            "`signature_topic` is expected to be 32-byte hex string. Found {} bytes",
                            e.len()
                        )
                    })?;

            Ok(Self { topic: bytes })
        } else {
            Err(format_err_spanned!(
                lit,
                "Expected string literal argument for the `signature_topic`"
            ))
        }
    }
}

impl TryFrom<&ast::MetaValue> for SignatureTopicArg {
    type Error = syn::Error;

    fn try_from(value: &ast::MetaValue) -> Result<Self, Self::Error> {
        if let ast::MetaValue::Lit(lit) = value {
            Self::try_from(lit)
        } else {
            Err(format_err_spanned!(
                value,
                "Expected string argument for the `signature_topic`"
            ))
        }
    }
}

impl TryFrom<ast::AttributeArgs> for Option<SignatureTopicArg> {
    type Error = syn::Error;

    fn try_from(args: ast::AttributeArgs) -> Result<Self, Self::Error> {
        let mut signature_topic: Option<SignatureTopicArg> = None;
        for arg in args.into_iter() {
            if arg.name().is_ident("hash") {
                if signature_topic.is_some() {
                    return Err(format_err!(
                        arg.span(),
                        "encountered duplicate ink! event configuration argument"
                    ));
                }
                signature_topic =
                    arg.value().map(SignatureTopicArg::try_from).transpose()?;
            } else {
                return Err(format_err_spanned!(
                    arg,
                    "encountered unknown or unsupported ink! event item configuration argument",
                ));
            }
        }
        Ok(signature_topic)
    }
}

impl TryFrom<&syn::MetaNameValue> for SignatureTopicArg {
    type Error = syn::Error;

    fn try_from(nv: &syn::MetaNameValue) -> Result<Self, Self::Error> {
        if nv.path.is_ident("signature_topic") {
            if let syn::Expr::Lit(lit_expr) = &nv.value {
                Self::try_from(&lit_expr.lit)
            } else {
                Err(format_err_spanned!(&nv.value, "Expected literal argument"))
            }
        } else {
            Err(format_err_spanned!(nv, "Expected `signature_topic` ident"))
        }
    }
}
