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

/// Generates code for the ink! environment of the contract.
#[derive(From)]
pub struct Env<'a> {
    contract: &'a ir::Contract,
}

impl GenerateCode for Env<'_> {
    fn generate_code(&self) -> TokenStream2 {
        let env = self.contract.config().env();
        let storage_ident = self.contract.module().storage().ident();
        quote! {
            impl ::ink::env::ContractEnv for #storage_ident {
                type Env = #env;
            }

            type Environment = <#storage_ident as ::ink::env::ContractEnv>::Env;

            type AccountId = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
            type Balance = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
            type Hash = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
            type Timestamp = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
            type BlockNumber = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
            type ChainExtension = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::ChainExtension;
            const MAX_EVENT_TOPICS: usize = <<#storage_ident as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS;
        }
    }
}
