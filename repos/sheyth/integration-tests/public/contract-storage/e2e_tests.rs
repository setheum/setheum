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

use super::contract_storage::*;
use ink_e2e::ContractsBackend;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn get_contract_storage_consumes_entire_buffer<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let mut constructor = ContractStorageRef::new();
    let contract = client
        .instantiate("contract-storage", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let call_builder = contract.call_builder::<ContractStorage>();

    // when
    let result = client
        .call(
            &ink_e2e::alice(),
            &call_builder.set_and_get_storage_all_data_consumed(),
        )
        .submit()
        .await
        .expect("Calling `insert_balance` failed")
        .return_value();

    assert!(result.is_ok());

    Ok(())
}

#[ink_e2e::test]
async fn get_contract_storage_fails_when_extra_data<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let mut constructor = ContractStorageRef::new();
    let contract = client
        .instantiate("contract-storage", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let call_builder = contract.call_builder::<ContractStorage>();

    // when
    let result = client
        .call(
            &ink_e2e::alice(),
            &call_builder.set_and_get_storage_partial_data_consumed(),
        )
        .submit()
        .await;

    assert!(
        result.is_err(),
        "Expected the contract to revert when only partially consuming the buffer"
    );

    Ok(())
}

#[ink_e2e::test]
async fn take_contract_storage_consumes_entire_buffer<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let mut constructor = ContractStorageRef::new();
    let contract = client
        .instantiate("contract-storage", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let call_builder = contract.call_builder::<ContractStorage>();

    // when
    let result = client
        .call(
            &ink_e2e::alice(),
            &call_builder.set_and_take_storage_all_data_consumed(),
        )
        .submit()
        .await
        .expect("Calling `insert_balance` failed")
        .return_value();

    assert!(result.is_ok());

    Ok(())
}

#[ink_e2e::test]
async fn take_contract_storage_fails_when_extra_data<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let mut constructor = ContractStorageRef::new();
    let contract = client
        .instantiate("contract-storage", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");
    let call_builder = contract.call_builder::<ContractStorage>();

    // when
    let result = client
        .call(
            &ink_e2e::alice(),
            &call_builder.set_and_take_storage_partial_data_consumed(),
        )
        .submit()
        .await;

    assert!(
        result.is_err(),
        "Expected the contract to revert when only partially consuming the buffer"
    );

    Ok(())
}
