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

#![cfg(test)]

use super::*;
use frame_support::{assert_err, assert_noop, assert_ok};
use mock::*;

#[test]
fn should_read_name() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_eq!(
				EVMBridge::<Runtime>::name(InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: Default::default(),
				}),
				Ok(
					b"long string name, long string name, long string name, long string name, long string name"
						.to_vec()
				)
			);
		});
}

#[test]
fn should_read_symbol() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_eq!(
				EVMBridge::<Runtime>::symbol(InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: Default::default(),
				}),
				Ok(b"TestToken".to_vec())
			);
		});
}

#[test]
fn should_read_decimals() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_eq!(
				EVMBridge::<Runtime>::decimals(InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: Default::default(),
				}),
				Ok(17)
			);
		});
}

#[test]
fn should_read_total_supply() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_eq!(
				EVMBridge::<Runtime>::total_supply(InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: Default::default(),
				}),
				Ok(ALICE_BALANCE)
			);
		});
}

#[test]
fn should_read_balance_of() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			let context = InvokeContext {
				contract: erc20_address(),
				sender: Default::default(),
				origin: Default::default(),
			};

			assert_eq!(EVMBridge::<Runtime>::balance_of(context, bob_evm_addr()), Ok(0));

			assert_eq!(
				EVMBridge::<Runtime>::balance_of(context, alice_evm_addr()),
				Ok(ALICE_BALANCE)
			);

			assert_eq!(EVMBridge::<Runtime>::balance_of(context, bob_evm_addr()), Ok(0));
		});
}

#[test]
fn should_transfer() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000), (bob(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_err!(
				EVMBridge::<Runtime>::transfer(
					InvokeContext {
						contract: erc20_address(),
						sender: bob_evm_addr(),
						origin: bob_evm_addr(),
					},
					alice_evm_addr(),
					10
				),
				Error::<Runtime>::ExecutionRevert
			);

			assert_ok!(EVMBridge::<Runtime>::transfer(
				InvokeContext {
					contract: erc20_address(),
					sender: alice_evm_addr(),
					origin: alice_evm_addr(),
				},
				bob_evm_addr(),
				100
			));
			assert_eq!(
				EVMBridge::<Runtime>::balance_of(
					InvokeContext {
						contract: erc20_address(),
						sender: alice_evm_addr(),
						origin: alice_evm_addr(),
					},
					bob_evm_addr()
				),
				Ok(100)
			);

			assert_ok!(EVMBridge::<Runtime>::transfer(
				InvokeContext {
					contract: erc20_address(),
					sender: bob_evm_addr(),
					origin: bob_evm_addr(),
				},
				alice_evm_addr(),
				10
			));

			assert_eq!(
				EVMBridge::<Runtime>::balance_of(
					InvokeContext {
						contract: erc20_address(),
						sender: alice_evm_addr(),
						origin: bob_evm_addr(),
					},
					bob_evm_addr()
				),
				Ok(90)
			);

			assert_err!(
				EVMBridge::<Runtime>::transfer(
					InvokeContext {
						contract: erc20_address(),
						sender: bob_evm_addr(),
						origin: bob_evm_addr(),
					},
					alice_evm_addr(),
					100
				),
				Error::<Runtime>::ExecutionRevert
			);
		});
}

#[test]
fn liquidation_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_liquidation_ok_contracts();
			let collateral = EvmAddress::from_str("1000000000000000000000000000000000000111").unwrap();
			let repay_dest = EvmAddress::from_str("1000000000000000000000000000000000000112").unwrap();

			assert_ok!(LiquidationEvmBridge::<Runtime>::liquidate(
				InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: alice_evm_addr(),
				},
				collateral,
				repay_dest,
				100,
				100,
			));
			System::assert_last_event(RuntimeEvent::EVM(module_evm::Event::Executed {
				from: Default::default(),
				contract: erc20_address(),
				logs: vec![module_evm::Log {
					address: erc20_address(),
					topics: vec![
						H256::from_str("0xf3fa0eaee8f258c23b013654df25d1527f98a5c7ccd5e951dd77caca400ef972").unwrap(),
					],
					data: {
						let mut buf = [0u8; 128];
						buf[12..32].copy_from_slice(collateral.as_bytes());
						buf[44..64].copy_from_slice(repay_dest.as_bytes());
						let mut amount_data = [0u8; 32];
						U256::from(100).to_big_endian(&mut amount_data);
						buf[64..96].copy_from_slice(&amount_data);
						buf[96..128].copy_from_slice(&amount_data);
						buf.to_vec()
					},
				}],
				used_gas: 25083,
				used_storage: 0,
			}));
		});
}

#[test]
fn on_collateral_transfer_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_liquidation_ok_contracts();
			let collateral = EvmAddress::from_str("1000000000000000000000000000000000000111").unwrap();
			LiquidationEvmBridge::<Runtime>::on_collateral_transfer(
				InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: alice_evm_addr(),
				},
				collateral,
				100,
			);
			System::assert_last_event(RuntimeEvent::EVM(module_evm::Event::Executed {
				from: Default::default(),
				contract: erc20_address(),
				logs: vec![module_evm::Log {
					address: erc20_address(),
					topics: vec![
						H256::from_str("0xa5625c5568ddba471a5e1190863744239495ca35883ce7f3e7d3beea2e89be74").unwrap(),
					],
					data: {
						let mut buf = [0u8; 64];
						buf[12..32].copy_from_slice(collateral.as_bytes());
						let mut amount_data = [0u8; 32];
						U256::from(100).to_big_endian(&mut amount_data);
						buf[32..64].copy_from_slice(&amount_data);
						buf.to_vec()
					},
				}],
				used_gas: 23573,
				used_storage: 0,
			}));
		});
}

#[test]
fn on_repayment_refund_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_liquidation_ok_contracts();
			let collateral = EvmAddress::from_str("1000000000000000000000000000000000000111").unwrap();
			LiquidationEvmBridge::<Runtime>::on_repayment_refund(
				InvokeContext {
					contract: erc20_address(),
					sender: Default::default(),
					origin: alice_evm_addr(),
				},
				collateral,
				100,
			);
			System::assert_last_event(RuntimeEvent::EVM(module_evm::Event::Executed {
				from: Default::default(),
				contract: erc20_address(),
				logs: vec![module_evm::Log {
					address: erc20_address(),
					topics: vec![
						H256::from_str("0x003d5a25faf4a774379f05de4f94d8967080f7e731902eb8f542b957a0712e18").unwrap(),
					],
					data: {
						let mut buf = [0u8; 64];
						buf[12..32].copy_from_slice(collateral.as_bytes());
						let mut amount_data = [0u8; 32];
						U256::from(100).to_big_endian(&mut amount_data);
						buf[32..64].copy_from_slice(&amount_data);
						buf.to_vec()
					},
				}],
				used_gas: 23595,
				used_storage: 0,
			}));
		});
}

#[test]
fn liquidation_err_fails_as_expected() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_liquidation_err_contracts();
			let collateral = EvmAddress::from_str("1000000000000000000000000000000000000111").unwrap();
			let repay_dest = EvmAddress::from_str("1000000000000000000000000000000000000112").unwrap();

			assert_noop!(
				LiquidationEvmBridge::<Runtime>::liquidate(
					InvokeContext {
						contract: erc20_address(),
						sender: Default::default(),
						origin: alice_evm_addr(),
					},
					collateral,
					repay_dest,
					100,
					100,
				),
				Error::<Runtime>::ExecutionRevert,
			);
		});
}
