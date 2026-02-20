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

/*
use drink::pallet_revive::{
    chain_extension::{ChainExtension, Config as ContractsConfig, Environment, Ext, RetVal},
    wasm::Memory,
};
use scale::Encode;

use crate::CHAIN_EXTENSION_RETURN_VALUE;

/// Simple chain extension that provides some mocked data.
#[derive(Default)]
pub struct StakingExtension;

impl<Runtime: ContractsConfig> ChainExtension<Runtime> for StakingExtension {
    fn call<E: Ext<T = Runtime>, M: ?Sized + Memory<E::T>>(
        &mut self,
        env: Environment<E, M>,
    ) -> drink::pallet_revive::chain_extension::Result<RetVal> {
        // Ensure that the contract called extension method with id `41`.
        assert_eq!(env.func_id(), 41);

        // Write fixed result of the extension call into the return buffer.
        env.buf_in_buf_out()
            .write(&CHAIN_EXTENSION_RETURN_VALUE.encode(), false, None)
            .expect("Failed to write result");

        // Return `Converging(0)` to indicate that the extension call has finished successfully.
        Ok(RetVal::Converging(0))
    }
}
*/