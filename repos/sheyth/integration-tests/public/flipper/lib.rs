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

#[ink::contract]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::new_default();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new(false);
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            // when
            let flip = call_builder.flip();
            let _flip_res = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), true));

            Ok(())
        }

        #[ink_e2e::test]
        async fn default_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new_default();

            // when
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            Ok(())
        }

        /// This test illustrates how to test an existing on-chain contract.
        ///
        /// You can utilize this to e.g. create a snapshot of a production chain
        /// and run the E2E tests against a deployed contract there.
        /// This process is explained [here](https://use.ink/5.x/basics/contract-testing/chain-snapshot).
        ///
        /// Before executing the test:
        ///   * Make sure you have a node running in the background,
        ///   * Supply the environment variable `CONTRACT_HEX` that points to a deployed
        ///     flipper contract. You can take the SS58 address which `cargo contract
        ///     instantiate` gives you and convert it to hex using `subkey inspect
        ///     <SS58>`.
        ///
        /// The test is then run like this:
        ///
        /// ```
        /// # The env variable needs to be set, otherwise `ink_e2e` will spawn a new
        /// # node process for each test.
        /// $ export CONTRACTS_NODE_URL=ws://127.0.0.1:9944
        ///
        /// $ export CONTRACT_ADDR_HEX=0x2c75f0aa09dbfbfd49e6286a0f2edd3b4913f04a58b13391c79e96782f5713e3
        /// $ cargo test --features e2e-tests e2e_test_deployed_contract -- --ignored
        /// ```
        ///
        /// # Developer Note
        ///
        /// The test is marked as ignored, as it has the above pre-conditions to succeed.
        #[ink_e2e::test]
        #[ignore]
        async fn e2e_test_deployed_contract<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let addr = std::env::var("CONTRACT_ADDR_HEX")
                .unwrap()
                .replace("0x", "");
            let acc_id = hex::decode(addr).unwrap();
            let acc_id = AccountId::try_from(&acc_id[..]).unwrap();

            use std::str::FromStr;
            let suri = ink_e2e::subxt_signer::SecretUri::from_str("//Alice").unwrap();
            let caller = ink_e2e::Keypair::from_uri(&suri).unwrap();

            // when
            // Invoke `Flipper::get()` from `caller`'s account
            let call_builder = ink_e2e::create_call_builder::<Flipper>(acc_id);
            let get = call_builder.get();
            let get_res = client.call(&caller, &get).dry_run().await?;

            // then
            assert_eq!(get_res.return_value(), true);

            Ok(())
        }
    }
}
