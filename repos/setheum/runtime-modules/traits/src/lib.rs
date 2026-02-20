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

#![cfg_attr(not(feature = "std"), no_std)]

use impl_trait_for_tuples::impl_for_tuples;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::Vec,
};

pub use asset_registry::{FixedConversionRateProvider, WeightToFeeConverter};
pub use auction::{Auction, AuctionHandler, AuctionInfo, OnNewBidResult};
pub use currency::{
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
	LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency,
	NamedBasicReservableCurrency, NamedMultiReservableCurrency,
};
pub use data_provider::{DataFeeder, DataProvider, DataProviderExtended};
pub use get_by_key::GetByKey;
pub use multi_asset::ConcreteFungibleAsset;
pub use nft::InspectExtended;
pub use price::{DefaultPriceProvider, PriceProvider};
pub use rate_limit::{RateLimiter, RateLimiterError};
pub use rewards::RewardHandler;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
pub use xcm_transfer::{XcmTransfer, XtokensWeightInfo};

pub mod arithmetic;
pub mod asset_registry;
pub mod auction;
pub mod currency;
pub mod data_provider;
pub mod get_by_key;
pub mod location;
pub mod multi_asset;
pub mod nft;
pub mod parameters;
pub mod price;
pub mod rate_limit;
pub mod rewards;
pub mod xcm_transfer;

/// New data handler
#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait OnNewData<AccountId, Key, Value> {
	/// New data is available
	fn on_new_data(who: &AccountId, key: &Key, value: &Value);
}

/// Combine data provided by operators
pub trait CombineData<Key, TimestampedValue> {
	/// Combine data provided by operators
	fn combine_data(
		key: &Key,
		values: Vec<TimestampedValue>,
		prev_value: Option<TimestampedValue>,
	) -> Option<TimestampedValue>;
}

/// Indicate if should change a value
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Change<Value> {
	/// No change.
	NoChange,
	/// Changed to new value.
	NewValue(Value),
}

#[derive(Encode, Decode, DecodeWithMemTracking, RuntimeDebug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TimestampedValue<Value, Moment>
where
	Value: Ord + PartialOrd,
{
	pub value: Value,
	pub timestamp: Moment,
}

#[impl_for_tuples(30)]
pub trait Happened<T> {
	fn happened(t: &T);
}

pub trait Handler<T> {
	fn handle(t: &T) -> DispatchResult;
}

#[impl_for_tuples(30)]
impl<T> Handler<T> for Tuple {
	fn handle(t: &T) -> DispatchResult {
		for_tuples!( #( Tuple::handle(t); )* );
		Ok(())
	}
}
