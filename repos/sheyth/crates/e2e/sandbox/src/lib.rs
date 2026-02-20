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

use core::any::Any;

pub mod api;
pub mod macros;

pub use frame_metadata::RuntimeMetadataPrefixed;
pub use frame_support::weights::Weight;
use frame_support::{
    sp_runtime::traits::Dispatchable,
    traits::fungible::Inspect,
};
use frame_system::{
    pallet_prelude::BlockNumberFor,
    EventRecord,
};
pub use macros::{
    BlockBuilder,
    DefaultSandbox,
};
use pallet_contracts::{
    ContractExecResult,
    ContractInstantiateResult,
};
/// Export pallets that are used in [`crate::create_sandbox`]
pub use {
    frame_support::sp_runtime::testing::H256,
    frame_support::{
        self,
        sp_runtime::{
            AccountId32,
            DispatchError,
        },
    },
    frame_system,
    pallet_balances,
    pallet_contracts,
    pallet_timestamp,
    paste,
    sp_core::crypto::Ss58Codec,
    sp_externalities::{
        self,
        Extension,
    },
    sp_io::TestExternalities,
};

/// A snapshot of the storage.
#[derive(Clone, Debug)]
pub struct Snapshot {
    /// The storage raw key-value pairs.
    pub storage: RawStorage,
    /// The storage root hash.
    pub storage_root: StorageRoot,
}

pub type RawStorage = Vec<(Vec<u8>, (Vec<u8>, i32))>;
pub type StorageRoot = H256;

/// Alias for the balance type.
type BalanceOf<R> =
    <<R as pallet_contracts::Config>::Currency as Inspect<AccountIdFor<R>>>::Balance;

/// Alias for the account ID type.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// Alias for the runtime call type.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

/// Alias for the event record type.
pub type EventRecordOf<Runtime> = EventRecord<
    <Runtime as frame_system::Config>::RuntimeEvent,
    <Runtime as frame_system::Config>::Hash,
>;

/// Alias for the contract instantiate result.
pub type ContractInstantiateResultFor<Runtime> = ContractInstantiateResult<
    AccountIdFor<Runtime>,
    BalanceOf<Runtime>,
    EventRecordOf<Runtime>,
>;

/// Alias for the contract exec result.
pub type ContractExecResultFor<Runtime> =
    ContractExecResult<BalanceOf<Runtime>, EventRecordOf<Runtime>>;

/// Alias for the runtime of a sandbox.
pub type RuntimeOf<S> = <S as Sandbox>::Runtime;

/// Alias for the runtime event of a sandbox.
pub type RuntimeEventOf<S> = <RuntimeOf<S> as frame_system::Config>::RuntimeEvent;

/// Sandbox defines the API of a sandboxed runtime.
pub trait Sandbox {
    /// The runtime associated with the sandbox.
    type Runtime: frame_system::Config;

    /// Execute the given externalities.
    fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T;

    /// Dry run an action without modifying the storage.
    fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T;

    /// Register an extension.
    fn register_extension<E: Any + Extension>(&mut self, ext: E);

    /// Initialize a new block at particular height.
    fn initialize_block(
        _height: BlockNumberFor<Self::Runtime>,
        _parent_hash: <Self::Runtime as frame_system::Config>::Hash,
    ) {
    }

    /// Finalize a block at particular height.
    fn finalize_block(
        _height: BlockNumberFor<Self::Runtime>,
    ) -> <Self::Runtime as frame_system::Config>::Hash {
        Default::default()
    }

    /// Default actor for the sandbox.
    fn default_actor() -> AccountIdFor<Self::Runtime>;

    fn default_gas_limit() -> Weight {
        Weight::from_parts(100_000_000_000, 3 * 1024 * 1024)
    }

    /// Metadata of the runtime.
    fn get_metadata() -> RuntimeMetadataPrefixed;

    /// Convert an account to an call origin.
    fn convert_account_to_origin(
        account: AccountIdFor<Self::Runtime>,
    ) -> <<Self::Runtime as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin;

    /// Take a snapshot of the storage.
    fn take_snapshot(&mut self) -> Snapshot;

    /// Restore the storage from the given snapshot.
    fn restore_snapshot(&mut self, snapshot: Snapshot);
}
