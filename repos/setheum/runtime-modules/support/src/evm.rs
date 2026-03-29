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

use parity_scale_codec::{Decode, Encode};
use primitives::currency::AssetIds;
use primitives::{
	evm::{CallInfo, EvmAddress},
	Balance, CurrencyId,
};
use sp_core::H160;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::*,
};

// AddressMapping and UnifiedAccountsManager are kept for unified-accounts

/// An abstraction of UnifiedAccountsManager
pub trait UnifiedAccountsManager<AccountId> {
	/// Returns the AccountId used to generate the given EvmAddress.
	fn get_account_id(address: &EvmAddress) -> AccountId;
	/// Returns the EvmAddress associated with a given AccountId or the underlying EvmAddress of the
	/// AccountId.
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	/// Claim account mapping between AccountId and a generated EvmAddress based off of the
	/// AccountId.
	fn claim_default_evm_address(account_id: &AccountId) -> Result<EvmAddress, DispatchError>;
}

/// A mapping between `AccountId` and `EvmAddress`.
pub trait AddressMapping<AccountId> {
	/// Returns the AccountId used go generate the given EvmAddress.
	fn get_account_id(evm: &EvmAddress) -> AccountId;
	/// Returns the EvmAddress associated with a given AccountId or the
	/// underlying EvmAddress of the AccountId.
	/// Returns None if there is no EvmAddress associated with the AccountId
	/// and there is no underlying EvmAddress in the AccountId.
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	/// Returns the EVM address associated with an account ID and generates an
	/// account mapping if no association exists.
	fn get_or_create_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns the default EVM address associated with an account ID.
	fn get_default_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns true if a given AccountId is associated with a given EvmAddress
	/// and false if is not.
	fn is_linked(account_id: &AccountId, evm: &EvmAddress) -> bool;
}

/// A mapping between `CurrencyId` and `EvmAddress`.
pub trait CurrencyIdMapping {
	/// Returns the name of the given `currency_id`.
	fn name(currency_id: CurrencyId) -> Option<Vec<u8>>;
	/// Returns the symbol of the given `currency_id`.
	fn symbol(currency_id: CurrencyId) -> Option<Vec<u8>>;
	/// Returns the decimals of the given `currency_id`.
	fn decimals(currency_id: CurrencyId) -> Option<u8>;
	/// Returns the `EvmAddress` associated with a given `CurrencyId`.
	fn encode_evm_address(currency_id: CurrencyId) -> Option<EvmAddress>;
	/// Returns the `CurrencyId` associated with a given `EvmAddress`.
	fn decode_evm_address(address: EvmAddress) -> Option<CurrencyId>;
}

/// A filter for precompile callers.
pub trait PrecompileCallerFilter {
	fn is_allowed(caller: H160) -> bool;
}

/// A trait for EVM state rent deposit management.
pub trait EVMStateRentTrait {
	fn reserve_rent_deposit(who: &H160, metadata: Vec<u8>) -> DispatchResult;
	fn claim_rent_deposit(who: &H160, metadata: Vec<u8>) -> DispatchResult;
}

// Erc20InfoMapping removed

// Limits removed
