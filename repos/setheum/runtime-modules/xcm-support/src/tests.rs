// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
	assert_eq!(
		MatchesCurrencyId::matches_fungible(&Asset::parent_asset(100)),
		Some(100),
	);

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
	assert!(
		<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset::sibling_parachain_asset(
			2,
			b"TokenC".to_vec().try_into().unwrap(),
			100
		))
		.is_none()
	);
	assert!(
		<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset::sibling_parachain_asset(
			1,
			b"TokenB".to_vec().try_into().unwrap(),
			100
		))
		.is_none()
	);
	assert!(<MatchesCurrencyId as MatchesFungible<u128>>::matches_fungible(&Asset {
		fun: Fungible(100),
		id: AssetId(Location::new(
			1,
			[Junction::from(
				sp_runtime::BoundedVec::try_from(b"TokenB".to_vec()).unwrap()
			)]
		)),
	})
	.is_none());
}
