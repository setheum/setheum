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

#![cfg_attr(not(feature = "std"), no_std)]

mod executor;

use frame_support::{
    pallet_prelude::Weight,
    traits::fungible::Inspect,
};
pub use pallet::*;

type AccountIdOf<R> = <R as frame_system::Config>::AccountId;
type BalanceOf<R> = <<R as pallet_contracts::Config>::Currency as Inspect<
    <R as frame_system::Config>::AccountId,
>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use flipper_traits::Flip;
    use frame_support::{
        pallet_prelude::*,
        traits::fungible::Inspect,
    };
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_contracts::Config {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        [u8; 32]: From<<T as frame_system::Config>::AccountId>,
        <<T as pallet_contracts::Config>::Currency as Inspect<
            <T as frame_system::Config>::AccountId,
        >>::Balance: From<u128>,
    {
        /// Call the flip method on the contract at the given `contract` account.
        #[pallet::call_index(0)]
        #[pallet::weight(<T::WeightInfo as pallet_contracts::WeightInfo>::call().saturating_add(*gas_limit))]
        pub fn contract_call_flip(
            origin: OriginFor<T>,
            contract: AccountIdOf<T>,
            gas_limit: Weight,
            storage_deposit_limit: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let executor =
                executor::PalletContractsExecutor::<ink::env::DefaultEnvironment, T> {
                    origin: who.clone(),
                    contract: contract.clone(),
                    value: 0.into(),
                    gas_limit,
                    storage_deposit_limit,
                    marker: Default::default(),
                };

            let mut flipper = ink::message_builder!(Flip);
            let result = flipper.flip().exec(&executor)?;

            assert!(result.is_ok());

            Ok(())
        }
    }
}
