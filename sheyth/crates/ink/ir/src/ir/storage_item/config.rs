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

use crate::{
    ast,
    utils::duplicate_config_err,
};

/// The ink! configuration.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct StorageItemConfig {
    /// If set to `true`, all storage related traits are implemented automatically,
    /// this is the default value.
    /// If set to `false`, implementing all storage traits is disabled. In some cases
    /// this can be helpful to override the default implementation of the trait.
    derive: bool,
}

impl TryFrom<ast::AttributeArgs> for StorageItemConfig {
    type Error = syn::Error;

    fn try_from(args: ast::AttributeArgs) -> Result<Self, Self::Error> {
        let mut derive: Option<syn::LitBool> = None;
        for arg in args.into_iter() {
            if arg.name().is_ident("derive") {
                if let Some(lit_bool) = derive {
                    return Err(duplicate_config_err(
                        lit_bool,
                        arg,
                        "derive",
                        "storage item",
                    ));
                }
                if let Some(lit_bool) = arg.value().and_then(ast::MetaValue::as_lit_bool)
                {
                    derive = Some(lit_bool.clone())
                } else {
                    return Err(format_err_spanned!(
                        arg,
                        "expected a bool literal value for `derive` ink! storage item configuration argument",
                    ));
                }
            } else {
                return Err(format_err_spanned!(
                    arg,
                    "encountered unknown or unsupported ink! storage item configuration argument",
                ));
            }
        }
        Ok(StorageItemConfig {
            derive: derive.map(|lit_bool| lit_bool.value).unwrap_or(true),
        })
    }
}

impl StorageItemConfig {
    /// Returns the derive configuration argument.
    pub fn derive(&self) -> bool {
        self.derive
    }
}
