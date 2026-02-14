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

//! TODO: Replace with precompile example since chain extensions are deprecated in pallet-revive.

/*
/// Fixed value returned by the example chain extension.
#[cfg(test)]
const CHAIN_EXTENSION_RETURN_VALUE: u32 = 100;

/// Here we put ink-side part of the example chain extension.
mod chain_extension_ink_side;

/// Here we put runtime-side part of the example chain extension.
#[cfg(test)]
mod chain_extension_runtime_side;

/// Simple ink! smart contract that calls a chain extension.
#[ink::contract(env = StakingEnvironment)]
mod contract_calling_chain_extension {
    use crate::chain_extension_ink_side::StakingEnvironment;

    #[ink(storage)]
    pub struct ContractCallingChainExtension {}

    impl ContractCallingChainExtension {
        #[allow(clippy::new_without_default)]
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn call_ce(&self) -> u32 {
            self.env().extension().get_num_of_validators()
        }
    }
}

#[cfg(test)]
mod tests {
    use drink::{
        create_sandbox,
        session::{Session, NO_ARGS, NO_ENDOWMENT, NO_SALT},
    };

    use crate::CHAIN_EXTENSION_RETURN_VALUE;

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    // We can inject arbitrary chain extension into the minimal runtime as follows:
    create_sandbox!(
        SandboxWithCE,
        crate::chain_extension_runtime_side::StakingExtension,
        ()
    );

    /// Test that we can call chain extension from ink! contract and get a correct result.
    #[drink::test(sandbox = SandboxWithCE)]
    fn we_can_test_chain_extension(mut session: Session) -> Result<(), Box<dyn std::error::Error>> {
        let result: u32 = session
            .deploy_bundle_and(
                BundleProvider::local()?,
                "new",
                NO_ARGS,
                NO_SALT,
                NO_ENDOWMENT,
            )?
            .call("call_ce", NO_ARGS, NO_ENDOWMENT)??;

        assert_eq!(result, CHAIN_EXTENSION_RETURN_VALUE);

        Ok(())
    }
}
*/