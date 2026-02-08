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
    generator,
    GenerateCode,
    GenerateCodeUsing,
};
use derive_more::From;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code for the entirety of the ink! contract.
#[derive(From)]
pub struct Contract<'a> {
    /// The contract to generate code for.
    contract: &'a ir::Contract,
}
impl_as_ref_for_generator!(Contract);

impl GenerateCode for Contract<'_> {
    /// Generates ink! contract code.
    fn generate_code(&self) -> TokenStream2 {
        let module = self.contract.module();
        let ident = module.ident();
        let attrs = module.attrs();
        let vis = module.vis();
        let env = self.generate_code_using::<generator::Env>();
        let storage = self.generate_code_using::<generator::Storage>();
        let events = module.events().map(move |event| {
            let event_generator = generator::Event::from(event);
            event_generator.generate_code()
        });
        let dispatch2 = self.generate_code_using::<generator::Dispatch>();
        let item_impls = self.generate_code_using::<generator::ItemImpls>();
        let metadata = self.generate_code_using::<generator::Metadata>();
        let contract_reference =
            self.generate_code_using::<generator::ContractReference>();
        let non_ink_items = self
            .contract
            .module()
            .items()
            .iter()
            .filter_map(ir::Item::map_rust_item);
        quote! {
            #( #attrs )*
            #vis mod #ident {
                #env
                #storage
                #( #events )*
                #dispatch2
                #item_impls
                #contract_reference
                #metadata
                #( #non_ink_items )*
            }
        }
    }
}
