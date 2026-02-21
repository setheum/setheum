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

use crate::config::E2EConfig;
use darling::{
    ast::NestedMeta,
    FromMeta,
};
use proc_macro2::TokenStream as TokenStream2;

/// The End-to-End test with all required information.
pub struct InkE2ETest {
    /// The function which was annotated.
    pub item_fn: E2EFn,
    /// The specified configuration.
    pub config: E2EConfig,
}

/// The End-to-End test with all required information.
#[derive(derive_more::From)]
pub struct E2EFn {
    /// The function which was annotated.
    pub item_fn: syn::ItemFn,
}

impl InkE2ETest {
    /// Returns `Ok` if the test matches all requirements for an
    /// ink! E2E test definition.
    pub fn new(attrs: TokenStream2, input: TokenStream2) -> Result<Self, syn::Error> {
        let e2e_config = E2EConfig::from_list(&NestedMeta::parse_meta_list(attrs)?)?;
        let item_fn = syn::parse2::<syn::ItemFn>(input)?;
        let e2e_fn = E2EFn::from(item_fn);
        Ok(Self {
            item_fn: e2e_fn,
            config: e2e_config,
        })
    }
}
