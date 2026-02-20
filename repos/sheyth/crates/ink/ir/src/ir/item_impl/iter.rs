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

// Copyright (C) Use Ink (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{
    CallableWithSelector,
    ImplItem,
    ItemImpl,
};
use crate::ir;

/// Iterator yielding all ink! constructor within a source ink!
/// [`ir::ItemImpl`](`crate::ir::ItemImpl`).
pub struct IterConstructors<'a> {
    item_impl: &'a ir::ItemImpl,
    impl_items: core::slice::Iter<'a, ImplItem>,
}

impl<'a> IterConstructors<'a> {
    /// Creates a new ink! messages iterator.
    pub(super) fn new(item_impl: &'a ItemImpl) -> Self {
        Self {
            item_impl,
            impl_items: item_impl.items.iter(),
        }
    }
}

impl<'a> Iterator for IterConstructors<'a> {
    type Item = CallableWithSelector<'a, ir::Constructor>;

    fn next(&mut self) -> Option<Self::Item> {
        'repeat: loop {
            match self.impl_items.next() {
                None => return None,
                Some(impl_item) => {
                    if let Some(constructor) = impl_item.filter_map_constructor() {
                        return Some(CallableWithSelector::new(
                            self.item_impl,
                            constructor,
                        ))
                    }
                    continue 'repeat
                }
            }
        }
    }
}

/// Iterator yielding all ink! messages within a source ink!
/// [`ir::ItemImpl`](`crate::ir::ItemImpl`).
pub struct IterMessages<'a> {
    item_impl: &'a ir::ItemImpl,
    impl_items: core::slice::Iter<'a, ImplItem>,
}

impl<'a> IterMessages<'a> {
    /// Creates a new ink! messages iterator.
    pub(super) fn new(item_impl: &'a ItemImpl) -> Self {
        Self {
            item_impl,
            impl_items: item_impl.items.iter(),
        }
    }
}

impl<'a> Iterator for IterMessages<'a> {
    type Item = CallableWithSelector<'a, ir::Message>;

    fn next(&mut self) -> Option<Self::Item> {
        'repeat: loop {
            match self.impl_items.next() {
                None => return None,
                Some(impl_item) => {
                    if let Some(message) = impl_item.filter_map_message() {
                        return Some(CallableWithSelector::new(self.item_impl, message))
                    }
                    continue 'repeat
                }
            }
        }
    }
}
