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

use crate::GenerateCode;
use derive_more::From;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code for the `[ink::test]` macro.
#[derive(From)]
pub struct InkTest<'a> {
    /// The test function to generate code for.
    test: &'a ir::InkTest,
}

impl GenerateCode for InkTest<'_> {
    /// Generates the code for `#[ink:test]`.
    fn generate_code(&self) -> TokenStream2 {
        let item_fn = &self.test.item_fn;
        let attrs = &item_fn.attrs;
        let sig = &item_fn.sig;
        let fn_name = &sig.ident;
        let fn_return_type = &sig.output;
        let fn_block = &item_fn.block;
        let vis = &item_fn.vis;
        let fn_args = &sig.inputs;
        let expect_msg = format!(
            "{}: the off-chain testing environment returned an error",
            stringify!(#fn_name)
        );
        match fn_return_type {
            syn::ReturnType::Default => {
                quote! {
                    #( #attrs )*
                    #[test]
                    #vis fn #fn_name( #fn_args ) {
                        ::ink::env::test::run_test::<::ink::env::DefaultEnvironment, _>(|_| {
                            {
                                {
                                    #fn_block
                                };
                                ::core::result::Result::Ok(())
                            }
                        })
                        .unwrap_or_else(|error| ::core::panic!("{}: {:?}", #expect_msg, error));
                    }
                }
            }
            syn::ReturnType::Type(rarrow, ret_type) => {
                quote! {
                    #( #attrs )*
                    #[test]
                    #vis fn #fn_name( #fn_args ) #rarrow #ret_type {
                        ::ink::env::test::run_test::<::ink::env::DefaultEnvironment, _>(|_| {
                            #fn_block
                        })
                    }
                }
            }
        }
    }
}

impl GenerateCode for ir::InkTest {
    fn generate_code(&self) -> TokenStream2 {
        InkTest::from(self).generate_code()
    }
}
