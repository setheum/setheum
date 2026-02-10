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
    EventRecordOf,
    RuntimeCall,
    Sandbox,
};
use frame_support::sp_runtime::{
    traits::{
        Dispatchable,
        Saturating,
    },
    DispatchResultWithInfo,
};
use frame_system::pallet_prelude::BlockNumberFor;

/// System API for the sandbox.
pub trait SystemAPI {
    /// The runtime system config.
    type T: frame_system::Config;

    /// Build a new empty block and return the new height.
    fn build_block(&mut self) -> BlockNumberFor<Self::T>;

    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    fn build_blocks(&mut self, n: u32) -> BlockNumberFor<Self::T>;

    /// Return the current height of the chain.
    fn block_number(&mut self) -> BlockNumberFor<Self::T>;

    /// Return the events of the current block so far.
    fn events(&mut self) -> Vec<EventRecordOf<Self::T>>;

    /// Reset the events of the current block.
    fn reset_events(&mut self);

    /// Execute a runtime call (dispatchable).
    ///
    /// # Arguments
    ///
    /// * `call` - The runtime call to execute.
    /// * `origin` - The origin of the call.
    fn runtime_call<Origin: Into<<RuntimeCall<Self::T> as Dispatchable>::RuntimeOrigin>>(
        &mut self,
        call: RuntimeCall<Self::T>,
        origin: Origin,
    ) -> DispatchResultWithInfo<<RuntimeCall<Self::T> as Dispatchable>::PostInfo>;
}

impl<T> SystemAPI for T
where
    T: Sandbox,
    T::Runtime: frame_system::Config,
{
    type T = T::Runtime;

    fn build_block(&mut self) -> BlockNumberFor<Self::T> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<Self::T>::block_number();
            let block_hash = T::finalize_block(current_block);
            current_block.saturating_inc();
            T::initialize_block(current_block, block_hash);
            current_block
        })
    }

    fn build_blocks(&mut self, n: u32) -> BlockNumberFor<Self::T> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block());
        }
        last_block.unwrap_or_else(|| self.block_number())
    }

    fn block_number(&mut self) -> BlockNumberFor<Self::T> {
        self.execute_with(frame_system::Pallet::<Self::T>::block_number)
    }

    fn events(&mut self) -> Vec<EventRecordOf<Self::T>> {
        self.execute_with(frame_system::Pallet::<Self::T>::events)
    }

    fn reset_events(&mut self) {
        self.execute_with(frame_system::Pallet::<Self::T>::reset_events)
    }

    fn runtime_call<
        Origin: Into<<RuntimeCall<Self::T> as Dispatchable>::RuntimeOrigin>,
    >(
        &mut self,
        call: RuntimeCall<Self::T>,
        origin: Origin,
    ) -> DispatchResultWithInfo<<RuntimeCall<Self::T> as Dispatchable>::PostInfo> {
        self.execute_with(|| call.dispatch(origin.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::prelude::*,
        DefaultSandbox,
        RuntimeCall,
        RuntimeEventOf,
        RuntimeOf,
        Sandbox,
    };
    use frame_support::sp_runtime::{
        traits::Dispatchable,
        AccountId32,
        DispatchResultWithInfo,
    };

    fn make_transfer(
        sandbox: &mut DefaultSandbox,
        dest: AccountId32,
        value: u128,
    ) -> DispatchResultWithInfo<
        <RuntimeCall<<DefaultSandbox as Sandbox>::Runtime> as Dispatchable>::PostInfo,
    > {
        assert_ne!(
            DefaultSandbox::default_actor(),
            dest,
            "make_transfer should send to account different than default_actor"
        );
        sandbox.runtime_call(
            RuntimeCall::<RuntimeOf<DefaultSandbox>>::Balances(pallet_balances::Call::<
                RuntimeOf<DefaultSandbox>,
            >::transfer_allow_death {
                dest: dest.into(),
                value,
            }),
            Some(DefaultSandbox::default_actor()),
        )
    }

    #[test]
    fn dry_run_works() {
        let mut sandbox = DefaultSandbox::default();
        let actor = DefaultSandbox::default_actor();
        let initial_balance = sandbox.free_balance(&actor);

        sandbox.dry_run(|sandbox| {
            sandbox.mint_into(&actor, 100).unwrap();
            assert_eq!(sandbox.free_balance(&actor), initial_balance + 100);
        });

        assert_eq!(sandbox.free_balance(&actor), initial_balance);
    }

    #[test]
    fn runtime_call_works() {
        let mut sandbox = DefaultSandbox::default();

        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);
        let initial_balance = sandbox.free_balance(&RECIPIENT);

        let result = make_transfer(&mut sandbox, RECIPIENT, 100);
        assert!(result.is_ok());

        let expected_balance = initial_balance + 100;
        assert_eq!(sandbox.free_balance(&RECIPIENT), expected_balance);
    }

    #[test]
    fn current_events() {
        let mut sandbox = DefaultSandbox::default();
        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);

        let events_before = sandbox.events();
        assert!(events_before.is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");

        let events_after = sandbox.events();
        assert!(!events_after.is_empty());
        assert!(matches!(
            events_after.last().unwrap().event,
            RuntimeEventOf::<DefaultSandbox>::Balances(_)
        ));
    }

    #[test]
    fn resetting_events() {
        let mut sandbox = DefaultSandbox::default();
        const RECIPIENT: AccountId32 = AccountId32::new([3u8; 32]);

        make_transfer(&mut sandbox, RECIPIENT.clone(), 1)
            .expect("Failed to make transfer");

        assert!(!sandbox.events().is_empty());
        sandbox.reset_events();
        assert!(sandbox.events().is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");
        assert!(!sandbox.events().is_empty());
    }

    #[test]
    fn snapshot_works() {
        let mut sandbox = DefaultSandbox::default();

        // Check state before
        let block_before = sandbox.block_number();
        let snapshot_before = sandbox.take_snapshot();

        // Advance some blocks to have some state change
        let _ = sandbox.build_blocks(5);
        let block_after = sandbox.block_number();

        // Check block number and state after
        assert_eq!(block_before + 5, block_after);

        // Restore state
        sandbox.restore_snapshot(snapshot_before);

        // Check state after restore
        assert_eq!(block_before, sandbox.block_number());
    }
}
