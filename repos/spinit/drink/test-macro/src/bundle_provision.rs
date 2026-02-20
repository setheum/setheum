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

use std::{collections::HashMap, path::PathBuf};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::ItemEnum;

pub struct BundleProviderGenerator {
    root_contract_name: Option<String>,
    bundles: HashMap<String, PathBuf>,
}

impl BundleProviderGenerator {
    pub fn new<I: Iterator<Item = (String, PathBuf)>>(
        bundles: I,
        root_contract_name: Option<String>,
    ) -> Self {
        let root_contract_name = root_contract_name.map(|name| name.to_case(Case::Pascal));
        let bundles = HashMap::from_iter(bundles.map(|(name, path)| {
            let name = name.to_case(Case::Pascal);
            (name, path)
        }));

        if let Some(root_contract_name) = &root_contract_name {
            assert!(
                bundles.contains_key(root_contract_name),
                "Root contract must be part of the bundles"
            );
        }

        Self {
            root_contract_name,
            bundles,
        }
    }

    pub fn generate_bundle_provision(&self, enum_item: ItemEnum) -> TokenStream2 {
        let enum_name = &enum_item.ident;
        let enum_vis = &enum_item.vis;
        let enum_attrs = &enum_item.attrs;

        let local = match &self.root_contract_name {
            None => quote! {},
            Some(root_name) => {
                let local_bundle = self.bundles[root_name].to_str().expect("Invalid path");
                quote! {
                    pub fn local() -> ::drink::DrinkResult<::drink::session::ContractBundle> {
                        ::drink::session::ContractBundle::load(#local_bundle)
                    }
                }
            }
        };

        let (contract_names, matches): (Vec<_>, Vec<_>) = self
            .bundles
            .keys()
            .map(|name| {
                let name_ident = Ident::new(name, Span::call_site());
                let path = self.bundles[name].to_str().expect("Invalid path");
                let matcher = quote! {
                    #enum_name::#name_ident => ::drink::session::ContractBundle::load(#path),
                };
                (name_ident, matcher)
            })
            .unzip();

        quote! {
            #(#enum_attrs)*
            #[derive(Copy, Clone, PartialEq, Eq, Debug)]
            #enum_vis enum #enum_name {
                #(#contract_names,)*
            }

            impl #enum_name {
                #local

                pub fn bundle(self) -> ::drink::DrinkResult<::drink::session::ContractBundle> {
                    match self {
                        #(#matches)*
                    }
                }
            }
        }
    }
}
