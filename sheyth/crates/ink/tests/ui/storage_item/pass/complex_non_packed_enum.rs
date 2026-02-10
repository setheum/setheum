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

use ink_prelude::vec::Vec;
use ink_primitives::KeyComposer;
use ink_storage::{
    traits::{
        AutoKey,
        StorageKey,
    },
    Lazy,
    Mapping,
};

#[derive(Default)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
enum Packed {
    #[default]
    None,
    A(u8),
    B(u16),
    C(u32),
    D(u64),
    E(u128),
    F(String),
    G {
        a: u8,
        b: String,
    },
    H((u16, u32)),
}

#[ink::storage_item]
#[derive(Default)]
enum NonPacked<KEY: StorageKey = AutoKey> {
    #[default]
    None,
    A(Mapping<u128, Packed>),
    B(Lazy<u128>),
    C(Lazy<Packed>),
    D(Lazy<Vec<Packed>>),
    E(Mapping<String, Packed>),
    F {
        a: Mapping<String, Packed>,
    },
}

#[ink::storage_item]
#[derive(Default)]
struct Contract {
    a: Lazy<NonPacked>,
}

fn main() {
    ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
        let mut contract = Contract::default();
        assert_eq!(contract.key(), 0);

        // contract.a
        assert_eq!(contract.a.key(), KeyComposer::from_str("Contract::a"));
        assert_eq!(
            contract.a.get_or_default().key(),
            KeyComposer::from_str("Contract::a"),
        );

        contract.a.set(&NonPacked::<_>::A(Default::default()));
        let variant = if let Some(NonPacked::<_>::A(variant)) = contract.a.get() {
            variant
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::A::0"),
                KeyComposer::from_str("Contract::a")
            ),
        );

        contract.a.set(&NonPacked::<_>::B(Default::default()));
        let variant = if let Some(NonPacked::<_>::B(variant)) = contract.a.get() {
            variant
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::B::0"),
                KeyComposer::from_str("Contract::a")
            ),
        );

        contract.a.set(&NonPacked::<_>::C(Default::default()));
        let variant = if let Some(NonPacked::<_>::C(variant)) = contract.a.get() {
            variant
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::C::0"),
                KeyComposer::from_str("Contract::a")
            ),
        );

        contract.a.set(&NonPacked::<_>::D(Default::default()));
        let variant = if let Some(NonPacked::<_>::D(variant)) = contract.a.get() {
            variant
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::D::0"),
                KeyComposer::from_str("Contract::a")
            ),
        );

        contract.a.set(&NonPacked::<_>::E(Default::default()));
        let variant = if let Some(NonPacked::<_>::E(variant)) = contract.a.get() {
            variant
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::E::0"),
                KeyComposer::from_str("Contract::a")
            )
        );

        contract.a.set(&NonPacked::<_>::F {
            a: Default::default(),
        });
        let variant = if let Some(NonPacked::<_>::F { a }) = contract.a.get() {
            a
        } else {
            panic!("Wrong variant")
        };
        assert_eq!(
            variant.key(),
            KeyComposer::concat(
                KeyComposer::from_str("NonPacked::F::a"),
                KeyComposer::from_str("Contract::a")
            )
        );
        Ok(())
    })
    .unwrap()
}
