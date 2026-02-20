// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

use sp_runtime::DispatchError;
use sp_std::vec::Vec;
use xcm::{
	v5::{prelude::*, Weight},
	VersionedAsset, VersionedAssets, VersionedLocation,
};
pub struct Transferred<AccountId> {
	pub sender: AccountId,
	pub assets: Assets,
	pub fee: Asset,
	pub dest: Location,
}

/// Abstraction over cross-chain token transfers.
pub trait XcmTransfer<AccountId, Balance, CurrencyId> {
	/// Transfer local assets with given `CurrencyId` and `Amount`.
	fn transfer(
		who: AccountId,
		currency_id: CurrencyId,
		amount: Balance,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;

	/// Transfer `Asset` assets.
	fn transfer_multiasset(
		who: AccountId,
		asset: Asset,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;

	/// Transfer native currencies specifying the fee and amount as separate.
	fn transfer_with_fee(
		who: AccountId,
		currency_id: CurrencyId,
		amount: Balance,
		fee: Balance,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;

	/// Transfer `Asset` specifying the fee and amount as separate.
	fn transfer_multiasset_with_fee(
		who: AccountId,
		asset: Asset,
		fee: Asset,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;

	/// Transfer several currencies specifying the item to be used as fee.
	fn transfer_multicurrencies(
		who: AccountId,
		currencies: Vec<(CurrencyId, Balance)>,
		fee_item: u32,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;

	/// Transfer several `Asset` specifying the item to be used as fee.
	fn transfer_multiassets(
		who: AccountId,
		assets: Assets,
		fee: Asset,
		dest: Location,
		dest_weight_limit: WeightLimit,
	) -> Result<Transferred<AccountId>, DispatchError>;
}

pub trait XtokensWeightInfo<AccountId, Balance, CurrencyId> {
	fn weight_of_transfer_multiasset(asset: &VersionedAsset, dest: &VersionedLocation) -> Weight;
	fn weight_of_transfer(currency_id: CurrencyId, amount: Balance, dest: &VersionedLocation) -> Weight;
	fn weight_of_transfer_multicurrencies(
		currencies: &[(CurrencyId, Balance)],
		fee_item: &u32,
		dest: &VersionedLocation,
	) -> Weight;
	fn weight_of_transfer_multiassets(assets: &VersionedAssets, fee_item: &u32, dest: &VersionedLocation) -> Weight;
}
