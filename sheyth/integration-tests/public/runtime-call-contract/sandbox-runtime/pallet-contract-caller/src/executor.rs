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
    AccountIdOf,
    BalanceOf,
};
use frame_support::pallet_prelude::Weight;
use ink::env::{
    call::{
        ExecutionInput,
        Executor,
    },
    Environment,
};

pub struct PalletContractsExecutor<E: Environment, Runtime: pallet_contracts::Config> {
    pub origin: AccountIdOf<Runtime>,
    pub contract: AccountIdOf<Runtime>,
    pub value: BalanceOf<Runtime>,
    pub gas_limit: Weight,
    pub storage_deposit_limit: Option<BalanceOf<Runtime>>,
    pub marker: core::marker::PhantomData<E>,
}

impl<E, R> Executor<E> for PalletContractsExecutor<E, R>
where
    E: Environment,
    R: pallet_contracts::Config,
{
    type Error = sp_runtime::DispatchError;

    fn exec<Args, Output>(
        &self,
        input: &ExecutionInput<Args>,
    ) -> Result<ink::MessageResult<Output>, Self::Error>
    where
        Args: codec::Encode,
        Output: codec::Decode,
    {
        let data = codec::Encode::encode(&input);

        let result = pallet_contracts::Pallet::<R>::bare_call(
            self.origin.clone(),
            self.contract.clone(),
            self.value,
            self.gas_limit,
            self.storage_deposit_limit,
            data,
            pallet_contracts::DebugInfo::UnsafeDebug,
            pallet_contracts::CollectEvents::Skip,
            pallet_contracts::Determinism::Enforced,
        );

        let output = result.result?.data;
        let result = codec::Decode::decode(&mut &output[..]).map_err(|_| {
            sp_runtime::DispatchError::Other("Failed to decode contract output")
        })?;

        Ok(result)
    }
}
