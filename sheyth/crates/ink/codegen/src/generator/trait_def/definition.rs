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

use super::TraitDefinition;
use heck::ToLowerCamelCase as _;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
    quote_spanned,
};

impl<'a> TraitDefinition<'a> {
    fn generate_for_message(message: ir::InkTraitMessage<'a>) -> TokenStream2 {
        let span = message.span();
        let attrs = message.attrs();
        let sig = message.sig();
        let ident = &sig.ident;
        let inputs = &sig.inputs;
        let cfg_attrs = message.get_cfg_attrs(span);
        let output = match &sig.output {
            syn::ReturnType::Default => quote! { () },
            syn::ReturnType::Type(_, ty) => quote! { #ty },
        };
        let output_ident =
            format_ident!("{}Output", ident.to_string().to_lower_camel_case());
        quote_spanned!(span =>
            /// Output type of the respective trait message.
            #(#cfg_attrs)*
            type #output_ident: ::ink::codegen::ImpliesReturn<#output>;

            #(#attrs)*
            fn #ident(#inputs) -> Self::#output_ident;
        )
    }
}

impl TraitDefinition<'_> {
    pub(super) fn generate_trait_definition(&self) -> TokenStream2 {
        let item = self.trait_def.item();
        let span = item.span();
        let attrs = item.attrs();
        let ident = item.ident();
        let messages = item
            .iter_items()
            .map(|(item, _)| item)
            .flat_map(ir::InkTraitItem::filter_map_message)
            .map(Self::generate_for_message);
        quote_spanned!(span =>
            #(#attrs)*
            pub trait #ident: ::ink::env::ContractEnv {
                /// Holds general and global information about the trait.
                #[doc(hidden)]
                #[allow(non_camel_case_types)]
                type __ink_TraitInfo: ::ink::codegen::TraitCallForwarder;

                #(#messages)*
            }
        )
    }
}
