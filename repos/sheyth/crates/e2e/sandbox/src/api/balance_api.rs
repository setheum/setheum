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

use crate::{
    AccountIdFor,
    Sandbox,
};
use frame_support::{
    sp_runtime::DispatchError,
    traits::fungible::Mutate,
};

type BalanceOf<R> = <R as pallet_balances::Config>::Balance;

/// Balance API for the sandbox.
pub trait BalanceAPI<T: Sandbox>
where
    T: Sandbox,
    T::Runtime: pallet_balances::Config,
{
    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    fn mint_into(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
        amount: BalanceOf<T::Runtime>,
    ) -> Result<BalanceOf<T::Runtime>, DispatchError>;

    /// Return the free balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    fn free_balance(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
    ) -> BalanceOf<T::Runtime>;
}

impl<T> BalanceAPI<T> for T
where
    T: Sandbox,
    T::Runtime: pallet_balances::Config,
{
    fn mint_into(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
        amount: BalanceOf<T::Runtime>,
    ) -> Result<BalanceOf<T::Runtime>, DispatchError> {
        self.execute_with(|| {
            pallet_balances::Pallet::<T::Runtime>::mint_into(address, amount)
        })
    }

    fn free_balance(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
    ) -> BalanceOf<T::Runtime> {
        self.execute_with(|| pallet_balances::Pallet::<T::Runtime>::free_balance(address))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DefaultSandbox;
    #[test]
    fn mint_works() {
        let mut sandbox = DefaultSandbox::default();
        let balance = sandbox.free_balance(&DefaultSandbox::default_actor());

        sandbox
            .mint_into(&DefaultSandbox::default_actor(), 100)
            .unwrap();

        assert_eq!(
            sandbox.free_balance(&DefaultSandbox::default_actor()),
            balance + 100
        );
    }
}
