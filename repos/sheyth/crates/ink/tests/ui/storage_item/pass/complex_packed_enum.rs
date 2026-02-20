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

use ink_prelude::{
    collections::{
        BTreeMap,
        BTreeSet,
    },
    vec::Vec,
};
use ink::storage::traits::Storable;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
enum Deep2 {
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

#[derive(Default)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
enum Deep1 {
    #[default]
    None,
    A(Deep2),
    B((Deep2, Deep2)),
    C(Vec<Deep2>),
    D(BTreeMap<Deep2, Deep2>),
    E(BTreeSet<Deep2>),
}

#[derive(Default)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
#[ink::scale_derive(Encode, Decode)]
struct Contract {
    a: Deep1,
    b: Deep2,
    c: (Deep1, Deep2),
}

fn main() {
    let _: Result<Contract, _> = Storable::decode(&mut &[][..]);
}
