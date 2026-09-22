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

use core::marker::PhantomData;

use codec::Encode;
use hex_literal::hex;
use module_bridge_traits::{AssetTypeIdentifier, TransactorForwarder};
use xcm::latest::prelude::*;
use xcm_executor::traits::TransactAsset;

pub struct XCMAssetTransactor<CurrencyTransactor, FungiblesTransactor, AssetTypeChecker, Forwarder>(
	PhantomData<(CurrencyTransactor, FungiblesTransactor, AssetTypeChecker, Forwarder)>,
);
impl<
		CurrencyTransactor: TransactAsset,
		FungiblesTransactor: TransactAsset,
		AssetTypeChecker: AssetTypeIdentifier,
		Forwarder: TransactorForwarder,
	> TransactAsset for XCMAssetTransactor<CurrencyTransactor, FungiblesTransactor, AssetTypeChecker, Forwarder>
{
	// deposit_asset implements the TransactAsset deposit_asset method and contains the logic to classify
	// the asset recipient location:
	// 1. recipient is on the local parachain
	// 2. recipient is on non-substrate chain(evm, cosmos, etc.)
	// 3. recipient is on the remote parachain
	fn deposit_asset(what: &Asset, who: &Location, context: Option<&XcmContext>) -> XcmResult {
		let interior = who.interior();
		let parents = who.parent_count();
		match (parents, interior.len()) {
			// 1. recipient is on the local parachain
			(0, 1) => {
				// check if the asset is native, and call the corresponding deposit_asset()
				if AssetTypeChecker::is_native_asset(what) {
					CurrencyTransactor::deposit_asset(what, who, context)?;
				} else {
					FungiblesTransactor::deposit_asset(what, who, context)?
				}
			},
			// recipient is on the remote chain
			_ => {
				// Check the interior junctions for sygma-bridge pattern
				let is_sygma_dest = interior.len() == 4;

				if is_sygma_dest {
					// 2. recipient is on non-substrate chain(evm, cosmos, etc.), will forward to sygma bridge pallet
					let tmp_account = sp_io::hashing::blake2_256(
						&Location::new(0, [GeneralKey { length: 8, data: [1u8; 32] }]).encode(),
					);
					if AssetTypeChecker::is_native_asset(what) {
						CurrencyTransactor::deposit_asset(
							&what.clone(),
							&Junction::AccountId32 { network: None, id: tmp_account }.into(),
							context,
						)?;
					} else {
						FungiblesTransactor::deposit_asset(
							&what.clone(),
							&Junction::AccountId32 { network: None, id: tmp_account }.into(),
							context,
						)?
					}

					Forwarder::other_world_transactor_forwarder(tmp_account, what.clone(), who.clone())
						.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
				} else {
					// 3. recipient is on remote parachain, will forward to xcm bridge pallet
					// xcm message must have a sender(origin), so a tmp account derived from pallet would be necessary here
					let tmp_account = sp_io::hashing::blake2_256(
						&Location::new(0, [GeneralKey { length: 8, data: [2u8; 32] }]).encode(),
					);

					// check if the asset is native or foreign, and call the corresponding deposit_asset(), recipient will be the derived tmp account
					// xcm message execution
					if AssetTypeChecker::is_native_asset(what) {
						CurrencyTransactor::deposit_asset(
							&what.clone(),
							&Junction::AccountId32 { network: None, id: tmp_account }.into(),
							context,
						)?;
					} else {
						FungiblesTransactor::deposit_asset(
							&what.clone(),
							&Junction::AccountId32 { network: None, id: tmp_account }.into(),
							context,
						)?
					}

					Forwarder::xcm_transactor_forwarder(tmp_account, what.clone(), who.clone())
						.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
				}
			},
		}

		Ok(())
	}

	fn withdraw_asset(
		what: &Asset,
		who: &Location,
		maybe_context: Option<&XcmContext>,
	) -> Result<xcm_executor::AssetsInHolding, XcmError> {
		let assets = if AssetTypeChecker::is_native_asset(what) {
			CurrencyTransactor::withdraw_asset(what, who, maybe_context)?
		} else {
			FungiblesTransactor::withdraw_asset(what, who, maybe_context)?
		};

		Ok(assets)
	}
}
