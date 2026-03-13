// The Licensed Work is (c) 2022 Sygma
// SPDX-License-Identifier: LGPL-3.0-only

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
	fn deposit_asset(what: &Asset, who: &Location, context: &XcmContext) -> XcmResult {
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
