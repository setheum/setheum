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

use super::cross_contract_calls::*;
use ink_e2e::ContractsBackend;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn flip_and_get<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    // given
    let other_contract_code = client
        .upload("other-contract", &ink_e2e::alice())
        .submit()
        .await
        .expect("other_contract upload failed");

    let mut constructor = CrossContractCallsRef::new_v1(other_contract_code.code_hash);
    let contract = client
        .instantiate("cross-contract-calls", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("basic-contract-caller instantiate failed");
    let mut call_builder = contract.call_builder::<CrossContractCalls>();
    let call = call_builder.flip_and_get_v1();

    // when
    let result = client
        .call(&ink_e2e::alice(), &call)
        .submit()
        .await
        .expect("Calling `flip_and_get` failed")
        .return_value();

    assert!(!result);

    Ok(())
}

#[ink_e2e::test]
async fn instantiate_v2_with_insufficient_storage_deposit_limit<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let other_contract_code = client
        .upload("other-contract", &ink_e2e::alice())
        .submit()
        .await
        .expect("other_contract upload failed");

    const REF_TIME_LIMIT: u64 = 500_000_000;
    const PROOF_SIZE_LIMIT: u64 = 100_000;
    const STORAGE_DEPOSIT_LIMIT: u128 = 100_000_000_000;

    let mut constructor = CrossContractCallsRef::new_v2_with_limits(
        other_contract_code.code_hash,
        REF_TIME_LIMIT,
        PROOF_SIZE_LIMIT,
        STORAGE_DEPOSIT_LIMIT,
    );
    let contract = client
        .instantiate("cross-contract-calls", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await;

    let Err(ink_e2e::Error::InstantiateDryRun(err)) = contract else {
        panic!("instantiate should have failed at the dry run");
    };

    // insufficient storage deposit limit
    assert!(
        err.error
            .to_string()
            .contains("StorageDepositLimitExhausted"),
        "should have failed with StorageDepositLimitExhausted"
    );

    Ok(())
}

#[ink_e2e::test]
async fn instantiate_v2_with_sufficient_limits<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let other_contract_code = client
        .upload("other-contract", &ink_e2e::alice())
        .submit()
        .await
        .expect("other_contract upload failed");

    const REF_TIME_LIMIT: u64 = 500_000_000;
    const PROOF_SIZE_LIMIT: u64 = 100_000;
    const STORAGE_DEPOSIT_LIMIT: u128 = 500_000_000_000;

    let mut constructor = CrossContractCallsRef::new_v2_with_limits(
        other_contract_code.code_hash,
        REF_TIME_LIMIT,
        PROOF_SIZE_LIMIT,
        STORAGE_DEPOSIT_LIMIT,
    );
    let contract = client
        .instantiate("cross-contract-calls", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await;

    assert!(contract.is_ok(), "{}", contract.err().unwrap());

    Ok(())
}

#[ink_e2e::test]
async fn instantiate_v2_no_limits<Client: E2EBackend>(
    mut client: Client,
) -> E2EResult<()> {
    // given
    let other_contract_code = client
        .upload("other-contract", &ink_e2e::alice())
        .submit()
        .await
        .expect("other_contract upload failed");

    let mut constructor =
        CrossContractCallsRef::new_v2_no_limits(other_contract_code.code_hash);
    let contract = client
        .instantiate("cross-contract-calls", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await;

    assert!(contract.is_ok(), "{}", contract.err().unwrap());

    Ok(())
}

#[ink_e2e::test]
async fn flip_and_get_v2<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    // given
    let other_contract_code = client
        .upload("other-contract", &ink_e2e::alice())
        .submit()
        .await
        .expect("other_contract upload failed");

    let mut constructor = CrossContractCallsRef::new_v1(other_contract_code.code_hash);
    let contract = client
        .instantiate("cross-contract-calls", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("cross-contract-calls instantiate failed");
    let mut call_builder = contract.call_builder::<CrossContractCalls>();

    const REF_TIME_LIMIT: u64 = 500_000_000;
    const PROOF_SIZE_LIMIT: u64 = 100_000;
    const STORAGE_DEPOSIT_LIMIT: u128 = 1_000_000_000;

    // when
    let call = call_builder.flip_and_get_invoke_v2_with_limits(
        REF_TIME_LIMIT,
        PROOF_SIZE_LIMIT,
        STORAGE_DEPOSIT_LIMIT,
    );
    let result = client
        .call(&ink_e2e::alice(), &call)
        .submit()
        .await
        .expect("Calling `flip_and_get` failed")
        .return_value();

    assert!(!result);

    let call = call_builder.flip_and_get_invoke_v2_no_weight_limit();
    let result = client
        .call(&ink_e2e::alice(), &call)
        .submit()
        .await
        .expect("Calling `flip_and_get` failed")
        .return_value();

    assert!(result);

    Ok(())
}
