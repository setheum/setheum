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

use frame_support::pallet_prelude::*;
use sp_runtime::DispatchResult;
use sp_std::fmt::Debug;
use xcm::v5::prelude::*;
use xcm::VersionedLocation;

pub trait WeightToFeeConverter {
	fn convert_weight_to_fee(location: &Location, weight: Weight) -> Option<u128>;
}

pub trait FixedConversionRateProvider {
	fn get_fee_per_second(location: &Location) -> Option<u128>;
}

pub trait AssetProcessor<AssetId, Metadata> {
	fn pre_register(id: Option<AssetId>, asset_metadata: Metadata) -> Result<(AssetId, Metadata), DispatchError>;
	fn post_register(_id: AssetId, _asset_metadata: Metadata) -> Result<(), DispatchError> {
		Ok(())
	}
}

/// Data describing the asset properties.
#[derive(
	TypeInfo,
	Encode,
	Decode,
	CloneNoBound,
	EqNoBound,
	PartialEqNoBound,
	RuntimeDebugNoBound,
	MaxEncodedLen,
	DecodeWithMemTracking,
)]
#[codec(mel_bound(skip_type_params(StringLimit)))]
#[scale_info(skip_type_params(StringLimit))]
pub struct AssetMetadata<Balance, CustomMetadata, StringLimit: Get<u32>>
where
	Balance: Clone + Debug + Eq + PartialEq,
	CustomMetadata: Parameter + Member + TypeInfo,
{
	pub decimals: u32,
	pub name: BoundedVec<u8, StringLimit>,
	pub symbol: BoundedVec<u8, StringLimit>,
	pub existential_deposit: Balance,
	pub location: Option<VersionedLocation>,
	pub additional: CustomMetadata,
}

pub trait Inspect {
	/// AssetId type
	type AssetId;
	/// Balance type
	type Balance: Clone + Debug + Eq + PartialEq;
	/// Custom metadata type
	type CustomMetadata: Parameter + Member + TypeInfo;
	/// Name and symbol string limit
	type StringLimit: Get<u32>;

	fn asset_id(location: &Location) -> Option<Self::AssetId>;
	fn metadata(
		asset_id: &Self::AssetId,
	) -> Option<AssetMetadata<Self::Balance, Self::CustomMetadata, Self::StringLimit>>;
	fn metadata_by_location(
		location: &Location,
	) -> Option<AssetMetadata<Self::Balance, Self::CustomMetadata, Self::StringLimit>>;
	fn location(asset_id: &Self::AssetId) -> Result<Option<Location>, DispatchError>;
}

pub trait Mutate: Inspect {
	fn register_asset(
		asset_id: Option<Self::AssetId>,
		metadata: AssetMetadata<Self::Balance, Self::CustomMetadata, Self::StringLimit>,
	) -> DispatchResult;

	fn update_asset(
		asset_id: Self::AssetId,
		decimals: Option<u32>,
		name: Option<BoundedVec<u8, Self::StringLimit>>,
		symbol: Option<BoundedVec<u8, Self::StringLimit>>,
		existential_deposit: Option<Self::Balance>,
		location: Option<Option<VersionedLocation>>,
		additional: Option<Self::CustomMetadata>,
	) -> DispatchResult;
}
