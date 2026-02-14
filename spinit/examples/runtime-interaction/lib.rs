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

#[cfg(test)]
mod tests {
    use drink::{
        minimal::{MinimalSandbox, RuntimeCall},
        pallet_balances, pallet_revive,
        sandbox_api::prelude::*,
        session::mocking_api::read_contract_binary,
        AccountId32, Sandbox,
    };

    #[test]
    fn we_can_make_a_token_transfer_call() {
        // We create a sandbox object, which represents a blockchain runtime.
        let mut sandbox = MinimalSandbox::default();

        // Bob will be the recipient of the transfer.
        const BOB: AccountId32 = AccountId32::new([2u8; 32]);

        // Firstly, let us check that the recipient account (`BOB`) is not the default actor, that
        // will be used as the caller.
        assert_ne!(MinimalSandbox::default_actor(), BOB);

        // Recipient's balance before the transfer.
        let initial_balance = sandbox.free_balance(&BOB);

        // Prepare a call object, a counterpart of a blockchain transaction.
        let call_object = RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
            dest: BOB.into(),
            value: 100,
        });

        // Submit the call to the runtime.
        sandbox
            .runtime_call(call_object, Some(MinimalSandbox::default_actor()))
            .expect("Failed to execute a call");

        // In the end, the recipient's balance should be increased by 100.
        assert_eq!(sandbox.free_balance(&BOB), initial_balance + 100);
    }

    #[test]
    fn we_can_work_with_the_contracts_pallet_in_low_level() {
        let mut sandbox = MinimalSandbox::default();

        // Construct the path to the contract file.
        let contract_path = std::path::Path::new(file!())
            .parent()
            .expect("Failed to determine the base path")
            .join("test-resources")
            .join("dummy.polkavm");

        // A few runtime calls are also available directly from the sandbox. This includes a part of
        // the contracts API.
        let actor = MinimalSandbox::default_actor();
        let origin = MinimalSandbox::convert_account_to_origin(actor);
        let upload_result = sandbox
            .upload_contract(read_contract_binary(&contract_path), origin, 1_000_000)
            .expect("Failed to upload a contract");

        // If a particular call is not available directly in the sandbox, it can always be executed
        // via the `runtime_call` method.
        let call_object = RuntimeCall::Revive(pallet_revive::Call::remove_code {
            code_hash: upload_result.code_hash,
        });

        sandbox
            .runtime_call(call_object, Some(MinimalSandbox::default_actor()))
            .expect("Failed to remove a contract");
    }
}
