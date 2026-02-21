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
