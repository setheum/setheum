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

macro_rules! impl_as_ref_for_generator {
    ( $generator_name:ident ) => {
        impl ::core::convert::AsRef<ir::Contract> for $generator_name<'_> {
            fn as_ref(&self) -> &ir::Contract {
                self.contract
            }
        }
    };
}

mod arg_list;
mod as_dependency;
mod blake2b;
mod chain_extension;
mod contract;
mod dispatch;
mod env;
mod event;
mod ink_test;
mod item_impls;
mod metadata;
mod selector;
mod storage;
mod storage_item;
mod trait_def;

pub use self::{
    arg_list::{
        generate_argument_list,
        generate_reference_to_trait_info,
        input_bindings,
        input_bindings_tuple,
        input_message_idents,
        input_types,
        input_types_tuple,
        output_ident,
    },
    as_dependency::ContractReference,
    blake2b::Blake2x256,
    chain_extension::ChainExtension,
    contract::Contract,
    dispatch::Dispatch,
    env::Env,
    event::Event,
    ink_test::InkTest,
    item_impls::ItemImpls,
    metadata::{
        generate_type_spec,
        Metadata,
    },
    selector::{
        SelectorBytes,
        SelectorId,
    },
    storage::Storage,
    storage_item::StorageItem,
    trait_def::TraitDefinition,
};
