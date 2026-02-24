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

pub use num_traits::{
	Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedShl, CheckedShr, CheckedSub, One, Signed, Zero,
};
use sp_std::{
	self,
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, Shr, Sub, SubAssign},
};

/// A meta trait for arithmetic.
///
/// Arithmetic types do all the usual stuff you'd expect numbers to do. They are
/// guaranteed to be able to represent at least `u32` values without loss, hence
/// the trait implies `From<u32>` and smaller ints. All other conversions are
/// fallible.
pub trait SimpleArithmetic:
	Zero
	+ One
	+ From<u8>
	+ From<u16>
	+ From<u32>
	+ TryInto<u8>
	+ TryInto<u16>
	+ TryInto<u32>
	+ TryFrom<u64>
	+ TryInto<u64>
	+ TryFrom<u128>
	+ TryInto<u128>
	+ Add<Self, Output = Self>
	+ AddAssign<Self>
	+ Sub<Self, Output = Self>
	+ SubAssign<Self>
	+ Mul<Self, Output = Self>
	+ MulAssign<Self>
	+ Div<Self, Output = Self>
	+ DivAssign<Self>
	+ Rem<Self, Output = Self>
	+ RemAssign<Self>
	+ Shl<u32, Output = Self>
	+ Shr<u32, Output = Self>
	+ CheckedShl
	+ CheckedShr
	+ CheckedAdd
	+ CheckedSub
	+ CheckedMul
	+ CheckedDiv
	+ PartialOrd<Self>
	+ Ord
	+ Bounded
	+ Sized
{
}

impl<
		T: Zero
			+ One
			+ From<u8>
			+ From<u16>
			+ From<u32>
			+ TryInto<u8>
			+ TryInto<u16>
			+ TryInto<u32>
			+ TryFrom<u64>
			+ TryInto<u64>
			+ TryFrom<u128>
			+ TryInto<u128>
			+ Add<Self, Output = Self>
			+ AddAssign<Self>
			+ Sub<Self, Output = Self>
			+ SubAssign<Self>
			+ Mul<Self, Output = Self>
			+ MulAssign<Self>
			+ Div<Self, Output = Self>
			+ DivAssign<Self>
			+ Rem<Self, Output = Self>
			+ RemAssign<Self>
			+ Shl<u32, Output = Self>
			+ Shr<u32, Output = Self>
			+ CheckedShl
			+ CheckedShr
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ CheckedDiv
			+ PartialOrd<Self>
			+ Ord
			+ Bounded
			+ Sized,
	> SimpleArithmetic for T
{
}
