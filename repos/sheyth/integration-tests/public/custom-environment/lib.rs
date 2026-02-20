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

use ink::env::{
    DefaultEnvironment,
    Environment,
};

/// Our custom environment diverges from the `DefaultEnvironment` in the event topics
/// limit.
#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(TypeInfo)]
pub enum EnvironmentWithManyTopics {}

impl Environment for EnvironmentWithManyTopics {
    // We allow for 3 topics in the event, including the implicit topic for the event
    // signature. Therefore, the contract pallet's schedule must also allow for at
    // least 3 of them.
    const MAX_EVENT_TOPICS: usize =
        <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS - 1;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

#[ink::contract(env = crate::EnvironmentWithManyTopics)]
mod runtime_call {
    /// Trivial contract with a single message that emits an event with many topics.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Topics;

    /// An event that would be forbidden in the default environment, but is completely
    /// valid in our custom one.
    #[ink(event)]
    #[derive(Default)]
    pub struct EventWithTopics {
        #[ink(topic)]
        first_topic: Balance,
        #[ink(topic)]
        second_topic: Balance,
    }

    impl Topics {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Emit an event with many topics.
        #[ink(message)]
        pub fn trigger(&mut self) {
            self.env().emit_event(EventWithTopics::default());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn emits_event_with_many_topics() {
            let mut contract = Topics::new();
            contract.trigger();

            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);

            let emitted_event = <EventWithTopics as ink::scale::Decode>::decode(
                &mut &emitted_events[0].data[..],
            );

            assert!(emitted_event.is_ok());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[cfg(feature = "permissive-node")]
        #[ink_e2e::test(environment = crate::EnvironmentWithManyTopics)]
        async fn calling_custom_environment_works<Client: E2EBackend>(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = TopicsRef::new();
            let contract = client
                .instantiate("custom-environment", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Topics>();

            // when
            let message = call_builder.trigger();

            let call_res = client
                .call(&ink_e2e::alice(), &message)
                .submit()
                .await
                .expect("call failed");

            // then
            call_res.contains_event("Contracts", "ContractEmitted");

            Ok(())
        }

        #[cfg(not(feature = "permissive-node"))]
        #[ink_e2e::test(environment = crate::EnvironmentWithManyTopics)]
        async fn calling_custom_environment_fails_if_incompatible_with_node<
            Client: E2EBackend,
        >(
            mut client: Client,
        ) -> E2EResult<()> {
            // given
            let mut constructor = TopicsRef::new();
            let contract = client
                .instantiate("custom-environment", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Topics>();

            let message = call_builder.trigger();

            // when
            let call_res = client.call(&ink_e2e::alice(), &message).dry_run().await;

            // then
            assert!(call_res.is_err());

            Ok(())
        }
    }
}
