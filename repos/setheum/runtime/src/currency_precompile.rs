#![cfg_attr(not(feature = "std"), no_std)]
use core::marker::PhantomData;
use module_evm_utility::evm::{ExitError, ExitRevert, ExitSucceed};
use pallet_evm::{Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult};
use primitives::{CurrencyId, TokenSymbol, Balance};
use sp_core::{H160, U256};
use sp_std::{vec, vec::Vec};
use orml_traits::MultiCurrency;

pub struct CurrencyPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for CurrencyPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + module_currencies::Config + frame_system::Config,
	<Runtime as frame_system::Config>::AccountId: From<H160>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let address = handle.code_address();
		let currency_id = match get_currency_id(address) {
			Some(id) => id,
			None => return None,
		};

		let input = handle.input();
		if input.len() < 4 {
			return Some(Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Input too short".into()),
			}));
		}

		let action = &input[0..4];

		match action {
			// name() -> string
			&[0x06, 0xfd, 0xde, 0x03] => {
				let name = match currency_id {
					CurrencyId::Token(TokenSymbol::SEU) => "Setheum",
					CurrencyId::Token(TokenSymbol::SEUSD) => "Setheum USD",
					_ => "Unknown",
				};
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: encode_string(name),
				}))
			}
			// symbol() -> string
			&[0x95, 0xd8, 0x9b, 0x41] => {
				let symbol = match currency_id {
					CurrencyId::Token(TokenSymbol::SEU) => "SEU",
					CurrencyId::Token(TokenSymbol::SEUSD) => "SEUSD",
					_ => "UNK",
				};
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: encode_string(symbol),
				}))
			}
			// decimals() -> uint8
			&[0x31, 0x3c, 0xe5, 0x67] => {
				let mut output = [0u8; 32];
				output[31] = 18; // 18 decimals
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: output.to_vec(),
				}))
			}
			// totalSupply() -> uint256
			&[0x18, 0x16, 0x0d, 0xdd] => {
				let total = <module_currencies::Pallet<Runtime> as MultiCurrency<<Runtime as frame_system::Config>::AccountId>>::total_issuance(currency_id);
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: encode_balance(total.into()),
				}))
			}
			// balanceOf(address) -> uint256
			&[0x70, 0xa0, 0x82, 0x31] => {
				if input.len() < 36 {
					return Some(Err(PrecompileFailure::Error {
						exit_status: ExitError::Other("Input too short".into()),
					}));
				}
				let who: H160 = H160::from_slice(&input[16..36]);
				let who_account = <Runtime as frame_system::Config>::AccountId::from(who);
				let balance = <module_currencies::Pallet<Runtime> as MultiCurrency<<Runtime as frame_system::Config>::AccountId>>::free_balance(currency_id, &who_account);
				Some(Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: encode_balance(balance.into()),
				}))
			}
			// transfer(address,uint256) -> bool
			&[0xa9, 0x05, 0x9c, 0xbb] => {
				if input.len() < 68 {
					return Some(Err(PrecompileFailure::Error {
						exit_status: ExitError::Other("Input too short".into()),
					}));
				}
				let to: H160 = H160::from_slice(&input[16..36]);
				let amount = U256::from_big_endian(&input[36..68]);

				let caller = handle.context().caller;
				let caller_account = <Runtime as frame_system::Config>::AccountId::from(caller);
				let to_account = <Runtime as frame_system::Config>::AccountId::from(to);

				let amount_balance: Balance = match amount.try_into() {
					Ok(val) => val,
					Err(_) => {
						return Some(Err(PrecompileFailure::Revert {
							exit_status: ExitRevert::Reverted,
							output: "Amount overflow".into(),
						}))
					}
				};

				match <module_currencies::Pallet<Runtime> as MultiCurrency<<Runtime as frame_system::Config>::AccountId>>::transfer(
					currency_id,
					&caller_account,
					&to_account,
					amount_balance.into(),
				) {
					Ok(_) => {
						let mut output = [0u8; 32];
						output[31] = 1; // true
						Some(Ok(PrecompileOutput {
							exit_status: ExitSucceed::Returned,
							output: output.to_vec(),
						}))
					}
					Err(_) => Some(Err(PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: "Transfer failed".into(),
					})),
				}
			}
			_ => None,
		}
	}
}

pub fn get_currency_id(address: H160) -> Option<CurrencyId> {
	let bytes = address.as_bytes();
	if bytes[9] == 0 {
		// CurrencyIdType::Token is 0
		match bytes[19] {
			0 => Some(CurrencyId::Token(TokenSymbol::SEU)),
			1 => Some(CurrencyId::Token(TokenSymbol::SEUSD)),
			_ => None,
		}
	} else {
		None
	}
}

fn encode_string(s: &str) -> Vec<u8> {
	let mut output = vec![0u8; 64];
	output[31] = 32; // Offset
	let len = s.len();
	let mut len_bytes = [0u8; 32];
	len_bytes[31] = len as u8;
	output.extend_from_slice(&len_bytes);
	let mut str_bytes = s.as_bytes().to_vec();
	str_bytes.resize((str_bytes.len() + 31) / 32 * 32, 0);
	output.extend_from_slice(&str_bytes);
	output
}

fn encode_balance(val: Balance) -> Vec<u8> {
	let mut output = [0u8; 32];
	let bytes = val.to_be_bytes();
	output[16..32].copy_from_slice(&bytes);
	output.to_vec()
}
