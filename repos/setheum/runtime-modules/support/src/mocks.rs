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

#![allow(clippy::type_complexity)]
use crate::{AddressMapping, CurrencyId, Erc20InfoMapping, TransactionPayment};
use frame_support::pallet_prelude::{DispatchClass, Pays, Weight};
use parity_scale_codec::Encode;
use primitives::{
	currency::TokenInfo,
	evm::{EvmAddress, H160_POSITION_TOKEN},
	Multiplier, ReserveIdentifier,
};
use sp_core::{crypto::AccountId32, H160};
use sp_io::hashing::blake2_256;
use sp_runtime::{transaction_validity::TransactionValidityError, DispatchError, DispatchResult};
use sp_std::{marker::PhantomData, vec::Vec};

#[cfg(feature = "std")]
use frame_support::traits::Imbalance;

pub struct MockAddressMapping;

impl AddressMapping<AccountId32> for MockAddressMapping {
	fn get_account_id(address: &H160) -> AccountId32 {
		let mut data = [0u8; 32];
		data[0..4].copy_from_slice(b"evm:");
		data[4..24].copy_from_slice(&address[..]);
		AccountId32::from(data)
	}

	fn get_evm_address(account_id: &AccountId32) -> Option<H160> {
		let data: [u8; 32] = account_id.clone().into();
		if data.starts_with(b"evm:") && data.ends_with(&[0u8; 8]) {
			Some(H160::from_slice(&data[4..24]))
		} else {
			None
		}
	}

	fn get_default_evm_address(account_id: &AccountId32) -> H160 {
		let slice: &[u8] = account_id.as_ref();
		H160::from_slice(&slice[0..20])
	}

	fn get_or_create_evm_address(account_id: &AccountId32) -> H160 {
		Self::get_evm_address(account_id).unwrap_or({
			let payload = (b"evm:", account_id);
			H160::from_slice(&payload.using_encoded(blake2_256)[0..20])
		})
	}

	fn is_linked(account_id: &AccountId32, evm: &H160) -> bool {
		Self::get_or_create_evm_address(account_id) == *evm
	}
}

pub struct MockErc20InfoMapping;

impl Erc20InfoMapping for MockErc20InfoMapping {
	fn name(currency_id: CurrencyId) -> Option<Vec<u8>> {
		currency_id.name().map(|v| v.as_bytes().to_vec())
	}

	fn symbol(currency_id: CurrencyId) -> Option<Vec<u8>> {
		currency_id.symbol().map(|v| v.as_bytes().to_vec())
	}

	fn decimals(currency_id: CurrencyId) -> Option<u8> {
		currency_id.decimals()
	}

	fn encode_evm_address(v: CurrencyId) -> Option<EvmAddress> {
		EvmAddress::try_from(v).ok()
	}

	fn decode_evm_address(v: EvmAddress) -> Option<CurrencyId> {
		let token = v.as_bytes()[H160_POSITION_TOKEN]
			.try_into()
			.map(CurrencyId::Token)
			.ok()?;
		EvmAddress::try_from(token)
			.map(|addr| if addr == v { Some(token) } else { None })
			.ok()?
	}
}

#[cfg(feature = "std")]
impl<AccountId, Balance: Default + Copy, NegativeImbalance: Imbalance<Balance>>
	TransactionPayment<AccountId, Balance, NegativeImbalance> for ()
{
	fn reserve_fee(
		_who: &AccountId,
		_fee: Balance,
		_named: Option<ReserveIdentifier>,
	) -> Result<Balance, DispatchError> {
		Ok(Default::default())
	}

	fn unreserve_fee(_who: &AccountId, _fee: Balance, _named: Option<ReserveIdentifier>) -> Balance {
		Default::default()
	}

	fn unreserve_and_charge_fee(
		_who: &AccountId,
		_weight: Weight,
	) -> Result<(Balance, NegativeImbalance), TransactionValidityError> {
		Ok((Default::default(), Imbalance::zero()))
	}

	fn refund_fee(
		_who: &AccountId,
		_weight: Weight,
		_payed: NegativeImbalance,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn charge_fee(
		_who: &AccountId,
		_len: u32,
		_weight: Weight,
		_tip: Balance,
		_pays_fee: Pays,
		_class: DispatchClass,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn weight_to_fee(_weight: Weight) -> Balance {
		Default::default()
	}

	fn apply_multiplier_to_fee(_fee: Balance, _multiplier: Option<Multiplier>) -> Balance {
		Default::default()
	}
}

/// Given provided `Currency`, implements default reserve behavior
pub struct MockReservedTransactionPayment<Currency>(PhantomData<Currency>);

#[cfg(feature = "std")]
impl<
		AccountId,
		Balance: Default + Copy,
		NegativeImbalance: Imbalance<Balance>,
		Currency: frame_support::traits::NamedReservableCurrency<
			AccountId,
			ReserveIdentifier = ReserveIdentifier,
			Balance = Balance,
		>,
	> TransactionPayment<AccountId, Balance, NegativeImbalance> for MockReservedTransactionPayment<Currency>
{
	fn reserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Result<Balance, DispatchError> {
		Currency::reserve_named(&named.unwrap(), who, fee)?;
		Ok(fee)
	}

	fn unreserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Balance {
		Currency::unreserve_named(&named.unwrap(), who, fee)
	}

	fn unreserve_and_charge_fee(
		_who: &AccountId,
		_weight: Weight,
	) -> Result<(Balance, NegativeImbalance), TransactionValidityError> {
		Ok((Default::default(), Imbalance::zero()))
	}

	fn refund_fee(
		_who: &AccountId,
		_weight: Weight,
		_payed: NegativeImbalance,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn charge_fee(
		_who: &AccountId,
		_len: u32,
		_weight: Weight,
		_tip: Balance,
		_pays_fee: Pays,
		_class: DispatchClass,
	) -> Result<(), TransactionValidityError> {
		Ok(())
	}

	fn weight_to_fee(_weight: Weight) -> Balance {
		Default::default()
	}

	fn apply_multiplier_to_fee(_fee: Balance, _multiplier: Option<Multiplier>) -> Balance {
		Default::default()
	}
}
