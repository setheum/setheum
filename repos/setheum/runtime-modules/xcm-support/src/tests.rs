// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

//! Unit tests for xcm-support implementations.

#![cfg(test)]

use super::*;

use module_traits::{location::RelativeLocations, ConcreteFungibleAsset};

#[derive(Debug, PartialEq, Eq)]
pub enum TestCurrencyId {
	TokenA,
	TokenB,
	RelayChainToken,
}

pub struct CurrencyIdConvert;
impl Convert<Location, Option<TestCurrencyId>> for CurrencyIdConvert {
	fn convert(l: Location) -> Option<TestCurrencyId> {
		use TestCurrencyId::*;

		if l == Location::parent() {
			return Some(RelayChainToken);
		}
		if l == Location::sibling_parachain_general_key(1, b"TokenA".to_vec().try_into().unwrap()) {
			return Some(TokenA);
		}
		if l == Location::sibling_parachain_general_key(2, b"TokenB".to_vec().try_into().unwrap()) {
			return Some(TokenB);
		}
		None
	}
}

type MatchesCurrencyId = IsNativeConcrete<TestCurrencyId, CurrencyIdConvert>;

#[test]
fn is_native_concrete_matches_native_currencies() {
	assert_eq!(MatchesCurrencyId::matches_fungible(&Asset::parent_asset(100)), Some(100),);

	assert_eq!(
		MatchesCurrencyId::matches_fungible(&Asset::sibling_parachain_asset(
			1,
			b"TokenA".to_vec().try_into().unwrap(),
			100
		)),
		Some(100),
	);

	assert_eq!(
		MatchesCurrencyId::matches_fungible(&Asset::sibling_parachain_asset(
			2,
			b"TokenB".to_vec().try_into().unwrap(),
			100
		)),
		Some(100),
	);
}

#[test]
fn is_native_concrete_does_not_matches_non_native_currencies() {
	assert!(<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset::sibling_parachain_asset(
		2,
		b"TokenC".to_vec().try_into().unwrap(),
		100
	))
	.is_none());
	assert!(<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset::sibling_parachain_asset(
		1,
		b"TokenB".to_vec().try_into().unwrap(),
		100
	))
	.is_none());
	assert!(<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset {
		fun: Fungible(100),
		id: AssetId(Location::new(1, [Junction::from(sp_runtime::BoundedVec::try_from(b"TokenB".to_vec()).unwrap())])),
	})
	.is_none());
}
