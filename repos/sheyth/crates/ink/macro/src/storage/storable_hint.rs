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

use ink_ir::utils::find_storage_key_salt;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    parse2,
    GenericParam,
};

fn storable_hint_inner(s: synstructure::Structure) -> TokenStream2 {
    let ident = s.ast().ident.clone();
    let salt_ident = format_ident!("__ink_generic_salt");

    let mut generics = s.ast().generics.clone();
    generics.params.push(
        parse2(quote! { #salt_ident : ::ink::storage::traits::StorageKey }).unwrap(),
    );

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics_original, _) = s.ast().generics.split_for_impl();

    if let Some(inner_salt_ident) = find_storage_key_salt(s.ast()) {
        let inner_salt_ident = inner_salt_ident.ident.to_token_stream();
        let ty_generics: Vec<_> = s
            .ast()
            .generics
            .params
            .clone()
            .into_iter()
            .map(|param| {
                let ident = match param {
                    GenericParam::Type(t) => t.ident.to_token_stream(),
                    GenericParam::Lifetime(l) => l.lifetime.to_token_stream(),
                    GenericParam::Const(c) => c.ident.to_token_stream(),
                };
                if inner_salt_ident.to_string() == ident.to_string() {
                    Some(quote! {
                        #salt_ident
                    })
                } else {
                    Some(ident)
                }
            })
            .collect();

        quote! {
            impl #impl_generics ::ink::storage::traits::StorableHint<#salt_ident> for #ident #ty_generics_original #where_clause {
                type Type = #ident <#(#ty_generics),*>;
                type PreferredKey = #inner_salt_ident;
            }
        }
    } else {
        quote! {
            impl #impl_generics ::ink::storage::traits::StorableHint<#salt_ident> for #ident #ty_generics_original #where_clause {
                type Type = #ident #ty_generics_original;
                type PreferredKey = ::ink::storage::traits::AutoKey;
            }
        }
    }
}

pub fn storable_hint_derive(s: synstructure::Structure) -> TokenStream2 {
    let derive = storable_hint_inner(s);

    quote! {
        const _ : () = {
            #derive
        };
    }
}
