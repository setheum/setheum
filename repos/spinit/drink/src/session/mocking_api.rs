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

//! Mocking API for the sandbox.

use std::path::Path;

use frame_support::sp_runtime::traits::Bounded;
use ink_primitives::DepositLimit;
use ink_sandbox::{
    api::prelude::*,
    pallet_revive::{
        evm::{H160, U256},
        MomentOf,
    },
    Sandbox, H256,
};

use super::{BalanceOf, Session};
use crate::{
    pallet_revive::Config,
    session::mock::ContractMock, // DEFAULT_GAS_LIMIT,
};

/// Read the contract binary file.
pub fn read_contract_binary(path: &std::path::PathBuf) -> Vec<u8> {
    std::fs::read(path).expect("Failed to read contract file")
}

/// Interface for basic mocking operations.
pub trait MockingApi<R: Config> {
    /// Deploy `mock` as a standard contract. Returns the address of the deployed contract.
    fn deploy(&mut self, mock: ContractMock) -> H160;

    /// Mock part of an existing contract. In particular, allows to override real behavior of
    /// deployed contract's messages.
    fn mock_existing_contract(&mut self, _mock: ContractMock, _address: H160);
}

impl<T: Sandbox> MockingApi<T::Runtime> for Session<T>
where
    T::Runtime: Config,
    BalanceOf<T::Runtime>: Into<U256> + TryFrom<U256> + Bounded,
    MomentOf<T::Runtime>: Into<U256>,
    <<T as Sandbox>::Runtime as frame_system::Config>::Hash: frame_support::traits::IsType<H256>,
{
    fn deploy(&mut self, mock: ContractMock) -> H160 {
        let salt = self
            .mocks
            .lock()
            .expect("Should be able to acquire lock on registry")
            .salt();

        // Construct the path to the contract file.
        let contract_path = Path::new(file!())
            .parent()
            .and_then(Path::parent)
            .and_then(Path::parent)
            .expect("Failed to determine the base path")
            .join("test-resources")
            .join("dummy.polkavm");

        let origin = T::convert_account_to_origin(T::default_actor());
        let mock_address = self
            .sandbox()
            .deploy_contract(
                // Deploy a dummy contract to ensure that the pallet will treat the mock as a regular contract until it is
                // actually called.
                read_contract_binary(&contract_path),
                0u32.into(),
                vec![],
                Some(salt),
                origin,
                T::default_gas_limit(),
                DepositLimit::Unchecked,
            )
            .result
            .expect("Deployment of a dummy contract should succeed")
            .addr;

        self.mocks
            .lock()
            .expect("Should be able to acquire lock on registry")
            .register(mock_address, mock);

        mock_address
    }

    fn mock_existing_contract(&mut self, _mock: ContractMock, _address: H160) {
        todo!("soon")
    }
}
