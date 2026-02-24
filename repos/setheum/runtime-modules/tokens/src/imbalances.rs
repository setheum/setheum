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

use crate::{Config, TotalIssuance};
use frame_support::traits::{tokens::imbalance::TryMerge, Get, Imbalance, SameOrOther, TryDrop};
use sp_runtime::traits::{Saturating, Zero};
use sp_std::{marker, mem, result};

/// Opaque, move-only struct with private fields that serves as a token
/// denoting that funds have been created without any equal and opposite
/// accounting.
#[must_use]
pub struct PositiveImbalance<T: Config, GetCurrencyId: Get<T::CurrencyId>>(
	T::Balance,
	marker::PhantomData<GetCurrencyId>,
);

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> PositiveImbalance<T, GetCurrencyId> {
	/// Create a new positive imbalance from a balance.
	pub fn new(amount: T::Balance) -> Self {
		PositiveImbalance(amount, marker::PhantomData::<GetCurrencyId>)
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Default for PositiveImbalance<T, GetCurrencyId> {
	fn default() -> Self {
		Self::zero()
	}
}

/// Opaque, move-only struct with private fields that serves as a token
/// denoting that funds have been destroyed without any equal and opposite
/// accounting.
#[must_use]
pub struct NegativeImbalance<T: Config, GetCurrencyId: Get<T::CurrencyId>>(
	T::Balance,
	marker::PhantomData<GetCurrencyId>,
);

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> NegativeImbalance<T, GetCurrencyId> {
	/// Create a new negative imbalance from a balance.
	pub fn new(amount: T::Balance) -> Self {
		NegativeImbalance(amount, marker::PhantomData::<GetCurrencyId>)
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Default for NegativeImbalance<T, GetCurrencyId> {
	fn default() -> Self {
		Self::zero()
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryDrop for PositiveImbalance<T, GetCurrencyId> {
	fn try_drop(self) -> result::Result<(), Self> {
		self.drop_zero()
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Imbalance<T::Balance> for PositiveImbalance<T, GetCurrencyId> {
	type Opposite = NegativeImbalance<T, GetCurrencyId>;

	fn zero() -> Self {
		Self::new(Zero::zero())
	}
	fn drop_zero(self) -> result::Result<(), Self> {
		if self.0.is_zero() {
			Ok(())
		} else {
			Err(self)
		}
	}
	fn split(self, amount: T::Balance) -> (Self, Self) {
		let first = self.0.min(amount);
		let second = self.0.saturating_sub(first);

		mem::forget(self);
		(Self::new(first), Self::new(second))
	}
	fn merge(mut self, other: Self) -> Self {
		self.0 = self.0.saturating_add(other.0);
		mem::forget(other);

		self
	}
	fn subsume(&mut self, other: Self) {
		self.0 = self.0.saturating_add(other.0);
		mem::forget(other);
	}
	// allow to make the impl same with `pallet-balances`
	#[allow(clippy::comparison_chain)]
	fn offset(self, other: Self::Opposite) -> SameOrOther<Self, Self::Opposite> {
		let (a, b) = (self.0, other.0);
		mem::forget((self, other));

		if a > b {
			SameOrOther::Same(Self::new(a.saturating_sub(b)))
		} else if b > a {
			SameOrOther::Other(NegativeImbalance::new(b.saturating_sub(a)))
		} else {
			SameOrOther::None
		}
	}
	fn peek(&self) -> T::Balance {
		self.0
	}

	fn extract(&mut self, amount: T::Balance) -> Self {
		let new: T::Balance = self.0.min(amount);
		self.0 -= new;
		Self::new(new)
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryDrop for NegativeImbalance<T, GetCurrencyId> {
	fn try_drop(self) -> result::Result<(), Self> {
		self.drop_zero()
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Imbalance<T::Balance> for NegativeImbalance<T, GetCurrencyId> {
	type Opposite = PositiveImbalance<T, GetCurrencyId>;

	fn zero() -> Self {
		Self::new(Zero::zero())
	}
	fn drop_zero(self) -> result::Result<(), Self> {
		if self.0.is_zero() {
			Ok(())
		} else {
			Err(self)
		}
	}
	fn split(self, amount: T::Balance) -> (Self, Self) {
		let first = self.0.min(amount);
		let second = self.0.saturating_sub(first);

		mem::forget(self);
		(Self::new(first), Self::new(second))
	}
	fn merge(mut self, other: Self) -> Self {
		self.0 = self.0.saturating_add(other.0);
		mem::forget(other);

		self
	}
	fn subsume(&mut self, other: Self) {
		self.0 = self.0.saturating_add(other.0);
		mem::forget(other);
	}
	// allow to make the impl same with `pallet-balances`
	#[allow(clippy::comparison_chain)]
	fn offset(self, other: Self::Opposite) -> SameOrOther<Self, Self::Opposite> {
		let (a, b) = (self.0, other.0);
		mem::forget((self, other));

		if a > b {
			SameOrOther::Same(Self::new(a.saturating_sub(b)))
		} else if b > a {
			SameOrOther::Other(PositiveImbalance::new(b.saturating_sub(a)))
		} else {
			SameOrOther::None
		}
	}
	fn peek(&self) -> T::Balance {
		self.0
	}

	fn extract(&mut self, amount: T::Balance) -> Self {
		let new: T::Balance = self.0.min(amount);
		self.0 -= new;
		Self::new(new)
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Drop for PositiveImbalance<T, GetCurrencyId> {
	/// Basic drop handler will just square up the total issuance.
	fn drop(&mut self) {
		TotalIssuance::<T>::mutate(GetCurrencyId::get(), |v| *v = v.saturating_add(self.0));
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> Drop for NegativeImbalance<T, GetCurrencyId> {
	/// Basic drop handler will just square up the total issuance.
	fn drop(&mut self) {
		TotalIssuance::<T>::mutate(GetCurrencyId::get(), |v| *v = v.saturating_sub(self.0));
	}
}

impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryMerge for PositiveImbalance<T, GetCurrencyId> {
	fn try_merge(self, other: Self) -> Result<Self, (Self, Self)> {
		Ok(self.merge(other))
	}
}
impl<T: Config, GetCurrencyId: Get<T::CurrencyId>> TryMerge for NegativeImbalance<T, GetCurrencyId> {
	fn try_merge(self, other: Self) -> Result<Self, (Self, Self)> {
		Ok(self.merge(other))
	}
}
