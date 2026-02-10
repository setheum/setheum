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

use pallet_contracts_uapi::ReturnFlags;
use scale::{
    Decode,
    Encode,
    MaxEncodedLen,
};
use scale_info::TypeInfo;
use sp_runtime::{
    DispatchError,
    RuntimeDebug,
};
use sp_weights::Weight;

// A copy of primitive types defined within `pallet_contracts`, required for RPC calls.

/// Result type of a `bare_call` or `bare_instantiate` call as well as
/// `ContractsApi::call` and `ContractsApi::instantiate`.
///
/// It contains the execution result together with some auxiliary information.
///
/// #Note
///
/// It has been extended to include `events` at the end of the struct while not bumping
/// the `ContractsApi` version. Therefore when SCALE decoding a `ContractResult` its
/// trailing data should be ignored to avoid any potential compatibility issues.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ContractResult<R, Balance> {
    /// How much weight was consumed during execution.
    pub gas_consumed: Weight,
    /// How much weight is required as gas limit in order to execute this call.
    ///
    /// This value should be used to determine the weight limit for on-chain execution.
    ///
    /// # Note
    ///
    /// This can only different from [`Self::gas_consumed`] when weight pre charging
    /// is used. Currently, only `seal_call_runtime` makes use of pre charging.
    /// Additionally, any `seal_call` or `seal_instantiate` makes use of pre-charging
    /// when a non-zero `gas_limit` argument is supplied.
    pub gas_required: Weight,
    /// How much balance was paid by the origin into the contract's deposit account in
    /// order to pay for storage.
    ///
    /// The storage deposit is never actually charged from the origin in case of
    /// [`Self::result`] is `Err`. This is because on error all storage changes are
    /// rolled back including the payment of the deposit.
    pub storage_deposit: StorageDeposit<Balance>,
    /// An optional debug message. This message is only filled when explicitly requested
    /// by the code that calls into the contract. Otherwise it is empty.
    ///
    /// The contained bytes are valid UTF-8. This is not declared as `String` because
    /// this type is not allowed within the runtime.
    ///
    /// Clients should not make any assumptions about the format of the buffer.
    /// They should just display it as-is. It is **not** only a collection of log lines
    /// provided by a contract but a formatted buffer with different sections.
    ///
    /// # Note
    ///
    /// The debug message is never generated during on-chain execution. It is reserved
    /// for RPC calls.
    pub debug_message: Vec<u8>,
    /// The execution result of the wasm code.
    pub result: R,
}

/// Result type of a `bare_call` call as well as `ContractsApi::call`.
pub type ContractExecResult<Balance> =
    ContractResult<Result<ExecReturnValue, DispatchError>, Balance>;

/// Result type of a `bare_instantiate` call as well as `ContractsApi::instantiate`.
pub type ContractInstantiateResult<AccountId, Balance> =
    ContractResult<Result<InstantiateReturnValue<AccountId>, DispatchError>, Balance>;

/// Result type of a `bare_code_upload` call.
pub type CodeUploadResult<CodeHash, Balance> =
    Result<CodeUploadReturnValue<CodeHash, Balance>, DispatchError>;

/// Result type of a `get_storage` call.
pub type GetStorageResult = Result<Option<Vec<u8>>, ContractAccessError>;

/// The possible errors that can happen querying the storage of a contract.
#[derive(
    Copy, Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo,
)]
pub enum ContractAccessError {
    /// The given address doesn't point to a contract.
    DoesntExist,
    /// Storage key cannot be decoded from the provided input data.
    KeyDecodingFailed,
    /// Storage is migrating. Try again later.
    MigrationInProgress,
}

/// Output of a contract call or instantiation which ran to completion.
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ExecReturnValue {
    /// Flags passed along by `seal_return`. Empty when `seal_return` was never called.
    pub flags: ReturnFlags,
    /// Buffer passed along by `seal_return`. Empty when `seal_return` was never called.
    pub data: Vec<u8>,
}

impl ExecReturnValue {
    /// The contract did revert all storage changes.
    pub fn did_revert(&self) -> bool {
        self.flags.contains(ReturnFlags::REVERT)
    }
}

/// The result of a successful contract instantiation.
#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct InstantiateReturnValue<AccountId> {
    /// The output of the called constructor.
    pub result: ExecReturnValue,
    /// The account id of the new contract.
    pub account_id: AccountId,
}

/// The result of successfully uploading a contract.
#[derive(Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct CodeUploadReturnValue<CodeHash, Balance> {
    /// The key under which the new code is stored.
    pub code_hash: CodeHash,
    /// The deposit that was reserved at the caller. Is zero when the code already
    /// existed.
    pub deposit: Balance,
}

/// Reference to an existing code hash or a new wasm module.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Code<Hash> {
    /// A wasm module as raw bytes.
    Upload(Vec<u8>),
    /// The code hash of an on-chain wasm blob.
    Existing(Hash),
}

/// The amount of balance that was either charged or refunded in order to pay for storage.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Encode,
    Decode,
    MaxEncodedLen,
    RuntimeDebug,
    TypeInfo,
    serde::Serialize,
)]
pub enum StorageDeposit<Balance> {
    /// The transaction reduced storage consumption.
    ///
    /// This means that the specified amount of balance was transferred from the involved
    /// deposit accounts to the origin.
    Refund(Balance),
    /// The transaction increased storage consumption.
    ///
    /// This means that the specified amount of balance was transferred from the origin
    /// to the involved deposit accounts.
    Charge(Balance),
}
