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
