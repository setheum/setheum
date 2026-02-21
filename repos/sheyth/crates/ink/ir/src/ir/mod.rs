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

#![allow(dead_code)]

mod attrs;
mod blake2;
mod chain_extension;
mod config;
mod contract;
mod event;
mod idents_lint;
mod ink_test;
mod item;
mod item_impl;
mod item_mod;
mod selector;
mod storage_item;
mod trait_def;
pub mod utils;

const CFG_IDENT: &str = "cfg";

/// Marker types and definitions.
pub mod marker {
    pub use super::selector::{
        SelectorBytes,
        SelectorId,
    };
}

#[cfg(test)]
use self::attrs::Attribute;

use self::attrs::{
    contains_ink_attributes,
    first_ink_attribute,
    partition_attributes,
    sanitize_attributes,
    sanitize_optional_attributes,
    AttributeArg,
    AttributeArgKind,
    AttributeFrag,
    InkAttribute,
};
pub use self::{
    attrs::{
        IsDocAttribute,
        Namespace,
    },
    blake2::{
        blake2b_256,
        Blake2x256Macro,
    },
    chain_extension::{
        ChainExtension,
        ChainExtensionMethod,
        ExtensionId,
    },
    config::Config,
    contract::Contract,
    event::{
        Event,
        SignatureTopicArg,
    },
    ink_test::InkTest,
    item::{
        InkItem,
        Item,
        Storage,
    },
    item_impl::{
        Callable,
        CallableKind,
        CallableWithSelector,
        Constructor,
        ImplItem,
        InputsIter,
        ItemImpl,
        IterConstructors,
        IterMessages,
        Message,
        Receiver,
        Visibility,
    },
    item_mod::{
        ItemMod,
        IterEvents,
        IterItemImpls,
    },
    selector::{
        Selector,
        SelectorMacro,
        TraitPrefix,
    },
    storage_item::StorageItem,
    trait_def::{
        InkItemTrait,
        InkTraitDefinition,
        InkTraitItem,
        InkTraitMessage,
        IterInkTraitItems,
    },
};
