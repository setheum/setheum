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
mod mother {
    use ink::prelude::{
        format,
        string::{
            String,
            ToString,
        },
        vec::Vec,
    };

    use ink::storage::{
        Mapping,
        StorageVec,
    };

    /// Struct for storing winning bids per bidding sample (a block).
    /// Vector index corresponds to sample number.
    /// Wrapping vector, just added for testing UI components.
    #[derive(Default, PartialEq, Eq, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Bids(Vec<Vec<Option<(AccountId, Balance)>>>);

    /// Auction outline.
    #[derive(PartialEq, Eq, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Outline {
        NoWinner,
        WinnerDetected,
        PayoutCompleted,
    }

    /// Auction statuses.
    /// Logic inspired by
    /// [Parachain Auction](https://github.com/paritytech/polkadot/blob/master/runtime/common/src/traits.rs#L160)
    #[derive(PartialEq, Eq, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Status {
        /// An auction has not started yet.
        NotStarted,
        /// We are in the starting period of the auction, collecting initial bids.
        OpeningPeriod,
        /// We are in the ending period of the auction, where we are taking snapshots of
        /// the winning bids. Snapshots are taken currently on per-block basis,
        /// but this logic could be later evolve to take snapshots of on
        /// arbitrary length (in blocks).
        EndingPeriod(BlockNumber),
        /// Candle was blown.
        Ended(Outline),
        /// We have completed the bidding process and are waiting for the Random Function
        /// to return some acceptable randomness to select the winner. The number
        /// represents how many blocks we have been waiting.
        RfDelay(BlockNumber),
    }

    /// Struct for storing auction data.
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Auction {
        /// Branded name of the auction event.
        name: String,
        /// Some hash identifying the auction subject.
        subject: Hash,
        /// Structure storing the bids being made.
        bids: Bids,
        /// Auction terms encoded as:
        /// `[start_block, opening_period, closing_period]`
        terms: [BlockNumber; 3],
        /// Auction status.
        status: Status,
        /// Candle auction can have no winner.
        /// If auction is finalized, that means that the winner is determined.
        finalized: bool,
        /// Just a vector for the UI tests.
        vector: Vec<u8>,
    }

    impl Default for Auction {
        fn default() -> Auction {
            Auction {
                name: String::default(),
                subject: Hash::default(),
                bids: Bids::default(),
                terms: <[BlockNumber; 3]>::default(),
                status: Status::OpeningPeriod,
                finalized: false,
                vector: <Vec<u8>>::default(),
            }
        }
    }

    /// Way to fail a contract execution.
    #[derive(Debug, Eq, PartialEq, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Failure {
        Revert(String),
        Panic,
    }

    /// Event emitted when an auction being echoed.
    #[ink(event)]
    pub struct AuctionEchoed {
        auction: Auction,
    }

    /// Storage of the contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Mother {
        auction: Auction,
        balances: Mapping<AccountId, Balance>,
        log: StorageVec<String>,
    }

    impl Mother {
        #[ink(constructor)]
        pub fn new(auction: Auction) -> Self {
            Self {
                balances: Default::default(),
                log: Default::default(),
                auction,
            }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Default::default()
        }

        /// Demonstrates the ability to fail a constructor safely.
        #[ink(constructor)]
        pub fn failed_new(fail: bool) -> Result<Self, Failure> {
            if fail {
                Err(Failure::Revert("Reverting instantiation".to_string()))
            } else {
                Ok(Default::default())
            }
        }

        /// Takes an auction data struct as input and returns it back.
        #[ink(message)]
        pub fn echo_auction(&mut self, auction: Auction) -> Auction {
            self.env().emit_event(AuctionEchoed {
                auction: auction.clone(),
            });
            auction
        }

        /// Fails contract execution in the required way.
        #[ink(message)]
        pub fn revert_or_trap(&mut self, fail: Option<Failure>) -> Result<(), Failure> {
            match fail {
                Some(Failure::Revert(_)) => {
                    Err(Failure::Revert("Reverting on user demand!".to_string()))
                }
                Some(Failure::Panic) => {
                    panic!("Trapping on user demand!")
                }
                None => Ok(()),
            }
        }

        /// Prints the specified string into node's debug log.
        #[ink(message)]
        pub fn debug_log(&mut self, _message: String) {
            ink::env::debug_println!("debug_log: {}", _message);
        }

        /// Mutates the input string to return "Hello, { name }"
        #[ink(message)]
        pub fn mut_hello_world(&self, mut message: String) -> String {
            message = format!("Hello, {}", message);
            message
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn echo_auction_works() {
            let auction = Auction::default();
            let mut contract = Mother::new_default();
            assert_eq!(contract.echo_auction(auction.clone()), auction);
        }

        #[ink::test]
        fn revert_works() {
            let mut contract = Mother::default();
            assert_eq!(
                contract.revert_or_trap(Some(Failure::Revert(
                    "Testing reverting on demand!".to_string()
                ))),
                Err(Failure::Revert("Reverting on user demand!".to_string()))
            );
            contract
                .revert_or_trap(None)
                .expect("Contract unexpected failure!");
        }

        #[ink::test]
        fn constructor_works_or_fails() {
            let contract = Mother::failed_new(true);
            assert!(contract.is_err());
            assert_eq!(
                contract.err(),
                Some(Failure::Revert("Reverting instantiation".to_string()))
            );

            let contract = Mother::failed_new(false);
            assert!(contract.is_ok());
        }

        #[ink::test]
        #[should_panic]
        fn trap_works() {
            let mut contract = Mother::default();
            let _ = contract.revert_or_trap(Some(Failure::Panic));
        }

        #[ink::test]
        fn mut_works() {
            let contract = Mother::default();
            let res = contract.mut_hello_world("Alice".to_string());
            assert_eq!("Hello, Alice", res)
        }
    }
}
