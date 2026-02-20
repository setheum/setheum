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

use super::{
    Flipper,
    FlipperRef,
};
use ink_e2e::{
    ChainBackend,
    ContractsBackend,
};

type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Just instantiate a contract using non-default runtime.
#[ink_e2e::test(backend(runtime_only(sandbox = sandbox_runtime::ContractCallerSandbox)))]
async fn instantiate_and_get<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
    use flipper_traits::Flip;

    let initial_value = false;
    let mut constructor = FlipperRef::new(initial_value);

    let contract = client
        .instantiate("runtime-call-contract", &ink_e2e::alice(), &mut constructor)
        .submit()
        .await
        .expect("instantiate failed");

    let mut call_builder = contract.call_builder::<Flipper>();
    let flip_dry_run = client
        .call(&ink_e2e::bob(), &call_builder.flip())
        .dry_run()
        .await?;
    let gas_required = flip_dry_run.exec_result.gas_required;

    // call pallet dispatchable
    client
        .runtime_call(
            &ink_e2e::alice(),
            "ContractCaller",
            "contract_call_flip",
            vec![
                scale_value::Value::from_bytes(contract.account_id),
                scale_value::serde::to_value(frame_support::weights::Weight::from_parts(
                    gas_required.ref_time(),
                    gas_required.proof_size(),
                ))
                .unwrap(),
                scale_value::serde::to_value(None::<u128>).unwrap(),
            ],
        )
        .await
        .expect("runtime call failed");

    // now check that the flip was executed via the pallet
    let get_result = client
        .call(&ink_e2e::alice(), &call_builder.get())
        .dry_run()
        .await?;

    assert_eq!(get_result.return_value(), !initial_value);

    Ok(())
}
