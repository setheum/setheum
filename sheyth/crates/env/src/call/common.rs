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

use core::marker::PhantomData;

/// Represents a return type.
///
/// Used as a marker type to define the return type of an ink! message in call builders.
#[derive(Debug)]
pub struct ReturnType<T>(PhantomData<fn() -> T>);

impl<T> Clone for ReturnType<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ReturnType<T> {}

impl<T> Default for ReturnType<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

/// A parameter that has been set to some value.
#[derive(Debug, Copy, Clone)]
pub struct Set<T>(pub T);

impl<T> Set<T> {
    /// Returns the set value.
    #[inline]
    pub fn value(self) -> T {
        self.0
    }
}

/// A parameter that has not been set, yet.
#[derive(Debug)]
pub struct Unset<T>(PhantomData<fn() -> T>);

impl<T> Clone for Unset<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Unset<T> {}

impl<T> Default for Unset<T> {
    #[inline]
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Implemented by [`Set`] and [`Unset`] in order to unwrap their value.
///
/// This is useful in case the use-site does not know if it is working with
/// a set or an unset value generically unwrap it using a closure for fallback.
pub trait Unwrap {
    /// The output type of the `unwrap_or_else` operation.
    type Output;

    /// Returns the set value or evaluates the given closure.
    fn unwrap_or_else<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Self::Output;
}

impl<T> Unwrap for Unset<T> {
    type Output = T;

    #[inline]
    fn unwrap_or_else<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Self::Output,
    {
        f()
    }
}

impl<T> Unwrap for Set<T> {
    type Output = T;

    #[inline]
    fn unwrap_or_else<F>(self, _: F) -> Self::Output
    where
        F: FnOnce() -> Self::Output,
    {
        self.value()
    }
}
