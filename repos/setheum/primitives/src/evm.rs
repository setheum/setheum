// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

//! CurrencyId <-> EVM address (H160) bit encoding.
//!
//! This module intentionally contains only the address encoding rules that
//! survived the removal of Frontier. It no longer depends on `fp_evm` or
//! `ethereum`.

use crate::currency::{CurrencyId, CurrencyIdType, DexShare, DexShareType, ForeignAssetId};
use core::ops::Range;
use sp_core::H160;
use sp_runtime::{
	traits::Zero,
	SaturatedConversion,
};

/// EVM Address.
pub type EvmAddress = sp_core::H160;

// GAS MASK
const GAS_MASK: u64 = 100_000u64;
// STORAGE MASK
const STORAGE_MASK: u64 = 100u64;
// GAS LIMIT CHUNK
const GAS_LIMIT_CHUNK: u64 = 30_000u64;
// MAX GAS_LIMIT CC, log2(BLOCK_STORAGE_LIMIT)
pub const MAX_GAS_LIMIT_CC: u32 = 21u32;

/// Ethereum precompiles
/// 0 - 0x0000000000000000000000000000000000000400
/// Setheum precompiles
/// 0x0000000000000000000000000000000000000400 - 0x0000000000000000000000000000000000000800
pub const PRECOMPILE_ADDRESS_START: EvmAddress =
	H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0]);
/// Predeployed system contracts (except Mirrored ERC20)
/// 0x0000000000000000000000000000000000000800 - 0x0000000000000000000000000000000000001000
pub const PREDEPLOY_ADDRESS_START: EvmAddress =
	H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0]);
pub const MIRRORED_TOKENS_ADDRESS_START: EvmAddress =
	H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
pub const MIRRORED_NFT_ADDRESS_START: u64 = 0x2000000;
/// ERC20 Holding Account used for transfer ERC20 token
pub const ERC20_HOLDING_ACCOUNT: EvmAddress =
	H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// System contract address prefix
pub const SYSTEM_CONTRACT_ADDRESS_PREFIX: [u8; 9] = [0u8; 9];

/// Check if the given `address` is a system contract.
///
/// It's a system contract if the address starts with `SYSTEM_CONTRACT_ADDRESS_PREFIX`.
pub fn is_system_contract(address: &EvmAddress) -> bool {
	address.as_bytes().starts_with(&SYSTEM_CONTRACT_ADDRESS_PREFIX)
}

pub const H160_POSITION_CURRENCY_ID_TYPE: usize = 9;
pub const H160_POSITION_TOKEN: usize = 19;
pub const H160_POSITION_TOKEN_NFT: Range<usize> = 16..20;
pub const H160_POSITION_DEXSHARE_LEFT_TYPE: usize = 10;
pub const H160_POSITION_DEXSHARE_LEFT_FIELD: Range<usize> = 11..15;
pub const H160_POSITION_DEXSHARE_RIGHT_TYPE: usize = 15;
pub const H160_POSITION_DEXSHARE_RIGHT_FIELD: Range<usize> = 16..20;
pub const H160_POSITION_FOREIGN_ASSET: Range<usize> = 18..20;

/// Generate the EvmAddress from CurrencyId so that EVM contracts can call the
/// mirrored ERC20 contract.
///
/// NOTE: Can not be used directly, need to check the ERC20 is mapped.
impl TryFrom<CurrencyId> for EvmAddress {
	type Error = ();

	fn try_from(val: CurrencyId) -> Result<Self, Self::Error> {
		let mut address = [0u8; 20];
		match val {
			CurrencyId::Token(token) => {
				address[H160_POSITION_CURRENCY_ID_TYPE] = CurrencyIdType::Token.into();
				address[H160_POSITION_TOKEN] = token.into();
			}
			CurrencyId::DexShare(left, right) => {
				let left_field: u32 = left.into();
				let right_field: u32 = right.into();
				address[H160_POSITION_CURRENCY_ID_TYPE] = CurrencyIdType::DexShare.into();
				address[H160_POSITION_DEXSHARE_LEFT_TYPE] = Into::<DexShareType>::into(left).into();
				address[H160_POSITION_DEXSHARE_LEFT_FIELD].copy_from_slice(&left_field.to_be_bytes());
				address[H160_POSITION_DEXSHARE_RIGHT_TYPE] = Into::<DexShareType>::into(right).into();
				address[H160_POSITION_DEXSHARE_RIGHT_FIELD].copy_from_slice(&right_field.to_be_bytes());
			}
			CurrencyId::Erc20(erc20) => {
				address[..].copy_from_slice(erc20.as_bytes());
			}
			CurrencyId::ForeignAsset(foreign_asset_id) => {
				address[H160_POSITION_CURRENCY_ID_TYPE] = CurrencyIdType::ForeignAsset.into();
				address[H160_POSITION_FOREIGN_ASSET].copy_from_slice(&foreign_asset_id.to_be_bytes());
			}
		};

		Ok(EvmAddress::from_slice(&address))
	}
}

fn decode_evm_address_for_dex_share(address: &[u8], left: bool) -> Option<DexShare> {
	let (dex_share_type, dex_share_field) = if left {
		(H160_POSITION_DEXSHARE_LEFT_TYPE, H160_POSITION_DEXSHARE_LEFT_FIELD)
	} else {
		(H160_POSITION_DEXSHARE_RIGHT_TYPE, H160_POSITION_DEXSHARE_RIGHT_FIELD)
	};
	match DexShareType::try_from(address[dex_share_type]).ok()? {
		DexShareType::Token => address[dex_share_field][3].try_into().map(DexShare::Token).ok(),
		DexShareType::Erc20 => {
			// Dynamic Erc20 <-> u32 mapping requires asset-registry storage knowledge,
			// so pure bit-decoding intentionally returns None here.
			None
		}
		DexShareType::ForeignAsset => {
			let id = ForeignAssetId::from_be_bytes(address[dex_share_field][2..].try_into().ok()?);
			Some(DexShare::ForeignAsset(id))
		}
	}
}

impl TryFrom<EvmAddress> for CurrencyId {
	type Error = ();

	fn try_from(addr: EvmAddress) -> Result<Self, Self::Error> {
		if !is_system_contract(&addr) {
			return Ok(CurrencyId::Erc20(addr));
		}

		let address = addr.as_bytes();
		let currency_id = match CurrencyIdType::try_from(address[H160_POSITION_CURRENCY_ID_TYPE]).ok().ok_or(())? {
			CurrencyIdType::Token => address[H160_POSITION_TOKEN].try_into().map(CurrencyId::Token).ok().ok_or(())?,
			CurrencyIdType::DexShare => {
				let left = decode_evm_address_for_dex_share(address, true).ok_or(())?;
				let right = decode_evm_address_for_dex_share(address, false).ok_or(())?;
				CurrencyId::DexShare(left, right)
			}
			CurrencyIdType::ForeignAsset => {
				let id = ForeignAssetId::from_be_bytes(address[H160_POSITION_FOREIGN_ASSET].try_into().ok().ok_or(())?);
				CurrencyId::ForeignAsset(id)
			}
		};

		// Validation: encode it back and check equality
		if EvmAddress::try_from(currency_id).map_err(|_| ())? == addr {
			Ok(currency_id)
		} else {
			Err(())
		}
	}
}

pub fn decode_gas_price(gas_price: u64, gas_limit: u64, tx_fee_per_gas: u128) -> Option<(u128, u32)> {
	// ensure gas_price >= 100 Gwei
	if u128::from(gas_price) < tx_fee_per_gas {
		return None;
	}

	let mut tip: u128 = 0;
	let mut actual_gas_price = gas_price;
	const TEN_GWEI: u64 = 10_000_000_000u64;

	// tip = 10% * tip_number
	let tip_number = gas_price.checked_div(TEN_GWEI)?.checked_sub(10)?;
	if !tip_number.is_zero() {
		actual_gas_price = gas_price.checked_sub(tip_number.checked_mul(TEN_GWEI)?)?;
		tip = actual_gas_price
			.checked_mul(gas_limit)?
			.checked_mul(tip_number)?
			.checked_div(10)? // percentage
			.checked_div(1)? // SEU decimal is 18, ETH decimal is 18
			.into();
	}

	// valid_until max is u32::MAX.
	let valid_until: u32 = Into::<u128>::into(actual_gas_price)
		.checked_sub(tx_fee_per_gas)?
		.saturated_into();

	Some((tip, valid_until))
}

pub fn decode_gas_limit(gas_limit: u64) -> (u64, u32) {
	let gas_and_storage: u64 = gas_limit.checked_rem(GAS_MASK).expect("constant never failed; qed");
	let actual_gas_limit: u64 = gas_and_storage
		.checked_div(STORAGE_MASK)
		.expect("constant never failed; qed")
		.saturating_mul(GAS_LIMIT_CHUNK);
	let storage_limit_number: u32 = gas_and_storage
		.checked_rem(STORAGE_MASK)
		.expect("constant never failed; qed")
		.try_into()
		.expect("STORAGE_MASK is 100, the result maximum is 99; qed");

	let actual_storage_limit = if storage_limit_number.is_zero() {
		Default::default()
	} else if storage_limit_number > MAX_GAS_LIMIT_CC {
		2u32.saturating_pow(MAX_GAS_LIMIT_CC)
	} else {
		2u32.saturating_pow(storage_limit_number)
	};

	(actual_gas_limit, actual_storage_limit)
}
