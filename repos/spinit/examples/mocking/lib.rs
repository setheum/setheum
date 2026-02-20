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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This is a fixed selector of the `callee` message.
const CALLEE_SELECTOR: [u8; 4] = ink::selector_bytes!("callee");

#[ink::contract]
mod proxy {
    use ink::{
        env::{
            call::{build_call, ExecutionInput},
            DefaultEnvironment,
        },
        H160, U256,
    };

    use crate::CALLEE_SELECTOR;

    #[ink(storage)]
    pub struct Proxy {}

    impl Proxy {
        #[ink(constructor)]
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calls `callee` with the selector `CALLEE_SELECTOR` and forwards the result.
        #[ink(message)]
        pub fn forward_call(&self, callee: H160) -> (u8, u8) {
            build_call::<DefaultEnvironment>()
                .call(callee)
                .transferred_value(U256::zero())
                .exec_input(ExecutionInput::new(CALLEE_SELECTOR.into()))
                .returns::<(u8, u8)>()
                .invoke()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::{
        mock_message,
        session::{mocking_api::MockingApi, Session, NO_ARGS, NO_ENDOWMENT, NO_SALT},
        ContractMock,
    };

    use crate::CALLEE_SELECTOR;

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    #[drink::test]
    fn call_mocked_message(mut session: Session) -> Result<(), Box<dyn Error>> {
        // Firstly, we create the mocked contract.
        const RETURN_VALUE: (u8, u8) = (4, 1);
        let mocked_contract =
            ContractMock::new().with_message(CALLEE_SELECTOR, mock_message(|()| RETURN_VALUE));

        // Secondly, we deploy it, similarly to a standard deployment action.
        let mock_address = session.mocking_api().deploy(mocked_contract);

        // TODO: Deprecated due to `pallet-revive`. There is no `debug_message`.
        //
        // // Now, we can deploy our proper contract and verify its behavior.
        // let result: (u8, u8) = session
        //     .deploy_bundle_and(BundleProvider::local()?, "new", NO_ARGS, NO_SALT, None)?
        //     .call_and(
        //         "forward_call",
        //         &[format!("{:?}", mock_address)],
        //         NO_ENDOWMENT,
        //     )?
        //     .record()
        //     .last_call_return_decoded()?
        //     .expect("Call was successful");
        // assert_eq!(result, RETURN_VALUE);

        Ok(())
    }
}
