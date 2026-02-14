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

use crate::contract_storage::{
    ContractStorageData,
    ContractStorageLayout,
};
use contract_transcode::ContractMessageTranscoder;

use ink::{
    metadata::{
        layout::{
            Layout::{
                self,
                Struct,
            },
            LayoutKey,
            RootLayout,
        },
        ConstructorSpec,
        ContractSpec,
        InkProject,
        LangError,
        MessageSpec,
        ReturnTypeSpec,
        TypeSpec,
    },
    storage::{
        traits::{
            ManualKey,
            Storable,
            StorageLayout,
        },
        Lazy,
        Mapping,
    },
    ConstructorResult,
    MessageResult,
};

use scale::Encode;
use std::collections::BTreeMap;
use subxt::backend::legacy::rpc_methods::Bytes;

const BASE_KEY_RAW: [u8; 16] = [0u8; 16];
const ROOT_KEY: u32 = 0;
const LAZY_TYPE_ROOT_KEY: u32 = 1;

fn contract_default_spec() -> ContractSpec {
    ContractSpec::new()
        .constructors(vec![ConstructorSpec::from_label("new")
            .selector([94u8, 189u8, 136u8, 214u8])
            .payable(true)
            .args(Vec::new())
            .returns(ReturnTypeSpec::new(TypeSpec::with_name_str::<
                ConstructorResult<()>,
            >(
                "ink_primitives::ConstructorResult"
            )))
            .docs(Vec::new())
            .done()])
        .messages(vec![MessageSpec::from_label("inc")
            .selector([231u8, 208u8, 89u8, 15u8])
            .mutates(true)
            .payable(true)
            .args(Vec::new())
            .returns(ReturnTypeSpec::new(TypeSpec::with_name_str::<
                MessageResult<()>,
            >(
                "ink_primitives::MessageResult"
            )))
            .default(true)
            .done()])
        .events(Vec::new())
        .lang_error(TypeSpec::with_name_segs::<LangError, _>(
            ::core::iter::Iterator::map(
                ::core::iter::IntoIterator::into_iter(["ink", "LangError"]),
                ::core::convert::AsRef::as_ref,
            ),
        ))
        .done()
}

fn encode_storage_value<T: Storable>(value: &T) -> Bytes {
    let mut value_encoded = Vec::new();
    Storable::encode(value, &mut value_encoded);
    Bytes::from(value_encoded)
}

#[test]
fn storage_decode_simple_type_works() {
    let root_key_encoded = Encode::encode(&ROOT_KEY);
    #[derive(scale_info::TypeInfo, StorageLayout, Storable)]
    struct Data {
        a: i32,
    }

    let Struct(data_layout) = <Data as StorageLayout>::layout(&ROOT_KEY) else {
        panic!("Layout shall be created");
    };
    let storage_layout: Layout = RootLayout::new(
        LayoutKey::from(ROOT_KEY),
        data_layout,
        scale_info::meta_type::<Data>(),
    )
    .into();

    let metadata = InkProject::new(storage_layout, contract_default_spec());
    let decoder = ContractMessageTranscoder::new(metadata);

    let key = [BASE_KEY_RAW.to_vec(), root_key_encoded].concat();
    let value = Data { a: 16 };

    let mut map = BTreeMap::new();
    map.insert(Bytes::from(key), encode_storage_value(&value));
    let data = ContractStorageData::new(map);
    let layout = ContractStorageLayout::new(data, &decoder)
        .expect("Contract storage layout shall be created");

    let cell = layout.iter().next().expect("Root cell shall be in layout");
    assert_eq!(cell.to_string(), format!("Data {{ a: {} }}", value.a));
}

#[test]
fn storage_decode_lazy_type_works() {
    let root_key_encoded = Encode::encode(&ROOT_KEY);
    let lazy_type_root_encoded = Encode::encode(&LAZY_TYPE_ROOT_KEY);
    #[derive(scale_info::TypeInfo, StorageLayout, Storable)]
    struct Data {
        a: Lazy<i32, ManualKey<LAZY_TYPE_ROOT_KEY>>,
    }

    let Struct(data_layout) = <Data as StorageLayout>::layout(&ROOT_KEY) else {
        panic!("Layout shall be created");
    };
    let storage_layout: Layout = RootLayout::new(
        LayoutKey::from(ROOT_KEY),
        data_layout,
        scale_info::meta_type::<Data>(),
    )
    .into();

    let metadata = InkProject::new(storage_layout, contract_default_spec());
    let decoder = ContractMessageTranscoder::new(metadata);

    let key = [BASE_KEY_RAW.to_vec(), root_key_encoded.clone()].concat();
    let lazy_type_key = [BASE_KEY_RAW.to_vec(), lazy_type_root_encoded.clone()].concat();

    let value = Data { a: Lazy::new() };
    // Cannot be set on struct directly because it issues storage calls
    let a = 8i32;

    let mut map = BTreeMap::new();
    map.insert(Bytes::from(key), encode_storage_value(&value));
    map.insert(Bytes::from(lazy_type_key), encode_storage_value(&a));

    let data = ContractStorageData::new(map);
    let layout = ContractStorageLayout::new(data, &decoder)
        .expect("Contract storage layout shall be created");
    let mut iter = layout.iter();
    let cell = iter.next().expect("Root cell shall be in layout");
    assert_eq!(cell.to_string(), "Data { a: Lazy }".to_string());
    assert_eq!(cell.root_key(), hex::encode(root_key_encoded));

    let cell = iter.next().expect("Lazy type cell shall be in layout");
    assert_eq!(cell.to_string(), format!("Lazy {{ {a} }}"));
    assert_eq!(cell.root_key(), hex::encode(lazy_type_root_encoded));
}

#[test]
fn storage_decode_mapping_type_works() {
    let root_key_encoded = Encode::encode(&ROOT_KEY);
    let lazy_type_root_encoded = Encode::encode(&LAZY_TYPE_ROOT_KEY);
    #[derive(scale_info::TypeInfo, StorageLayout, Storable)]
    struct Data {
        a: Mapping<u8, u8, ManualKey<LAZY_TYPE_ROOT_KEY>>,
    }

    let Struct(data_layout) = <Data as StorageLayout>::layout(&ROOT_KEY) else {
        panic!("Layout shall be created");
    };
    let storage_layout: Layout = RootLayout::new(
        LayoutKey::from(ROOT_KEY),
        data_layout,
        scale_info::meta_type::<Data>(),
    )
    .into();

    let metadata = InkProject::new(storage_layout, contract_default_spec());
    let decoder = ContractMessageTranscoder::new(metadata);

    let value = Data { a: Mapping::new() };
    // Cannot be set on struct directly because it issues storage calls
    let mapping_item = (4u8, 8u8);

    let key = [BASE_KEY_RAW.to_vec(), root_key_encoded.clone()].concat();
    let lazy_type_key = [
        BASE_KEY_RAW.to_vec(),
        lazy_type_root_encoded.clone(),
        Encode::encode(&mapping_item.0),
    ]
    .concat();

    let mut map = BTreeMap::new();
    map.insert(Bytes::from(key), encode_storage_value(&value));
    map.insert(
        Bytes::from(lazy_type_key),
        encode_storage_value(&mapping_item.1),
    );

    let data = ContractStorageData::new(map);
    let layout = ContractStorageLayout::new(data, &decoder)
        .expect("Contract storage layout shall be created");
    let mut iter = layout.iter();
    let cell = iter.next().expect("Root cell shall be in layout");
    assert_eq!(cell.to_string(), "Data { a: Mapping }".to_string());
    assert_eq!(cell.root_key(), hex::encode(root_key_encoded));

    let cell = iter.next().expect("Mapping type cell shall be in layout");
    assert_eq!(
        cell.to_string(),
        format!("Mapping {{ {} => {} }}", mapping_item.0, mapping_item.1)
    );
    assert_eq!(cell.root_key(), hex::encode(lazy_type_root_encoded));
}
