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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::DispatchResult;
use primitive_types::{H160, U256};
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use xcm::latest::prelude::*;

pub type DomainID = u8;
pub type DepositNonce = u64;
pub type ResourceId = [u8; 32];
pub type ChainID = U256;
pub type VerifyingContractAddress = H160;

#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub enum TransferType {
	FungibleTransfer,
	NonFungibleTransfer,
	GenericTransfer,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Encode, Decode, TypeInfo, Copy, Default, MaxEncodedLen)]
pub struct MpcAddress(pub [u8; 20]);

pub trait ExtractDestinationData {
	fn extract_dest(dest: &Location) -> Option<(Vec<u8>, DomainID)>;
}

pub trait FeeHandler {
	// Return fee represent by a specific asset
	fn get_fee(domain: DomainID, asset: Asset) -> Option<u128>;
}

impl FeeHandler for () {
	fn get_fee(_domain: DomainID, _asset: Asset) -> Option<u128> {
		None
	}
}

pub trait DecimalConverter {
	/// convert_to converts the Asset to u128 when bridging from sygma substrate pallet.
	/// Sygma relayer will always expect asset in 18 decimal
	fn convert_to(asset: &Asset) -> Option<u128>;
	/// convert_from converts a u128 to Asset when bridging to sygma substrate pallet.
	/// Sygma relayer will always send asset in 18 decimal
	fn convert_from(asset: &Asset) -> Option<Asset>;
}

// when integrating with parachain, parachain team can implement their own version
pub trait AssetTypeIdentifier {
	fn is_native_asset(asset: &Asset) -> bool;
}

pub trait TransactorForwarder {
	fn xcm_transactor_forwarder(sender: [u8; 32], what: Asset, dest: Location) -> DispatchResult;
	fn other_world_transactor_forwarder(sender: [u8; 32], what: Asset, dest: Location) -> DispatchResult;
}

pub trait Bridge {
	fn transfer(sender: [u8; 32], asset: Asset, dest: Location, max_weight: Option<Weight>) -> DispatchResult;
}

pub trait AssetReserveLocationParser {
	fn reserved_location(asset: &Asset) -> Option<Location>;
}
