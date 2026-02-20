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

// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Module exposing errors and result types for the session API.

use frame_support::sp_runtime::DispatchError;
use parity_scale_codec::Decode;
use thiserror::Error;

use crate::errors::MessageResult;

/// Session specific errors.
#[derive(Clone, Error, Debug)]
pub enum SessionError {
    /// Encoding data failed.
    #[error("Encoding call data failed: {0}")]
    Encoding(String),
    /// Decoding data failed.
    #[error("Decoding call data failed: {0}")]
    Decoding(String),
    /// Crate-specific error.
    #[error("{0:?}")]
    Drink(#[from] crate::Error),
    /// Deployment has been reverted by the contract.
    #[error("Contract deployment has been reverted")]
    DeploymentReverted,
    /// Deployment failed (aborted by the pallet).
    #[error("Contract deployment failed before execution: {0:?}")]
    DeploymentFailed(DispatchError),
    /// Code upload failed (aborted by the pallet).
    #[error("Code upload failed: {0:?}")]
    UploadFailed(DispatchError),
    /// Call has been reverted by the contract.
    #[error("Contract call has been reverted. Encoded error: {0:?}")]
    CallReverted(Vec<u8>),
    /// Contract call failed (aborted by the pallet).
    #[error("Contract call failed before execution: {0:?}")]
    CallFailed(DispatchError),
    /// There is no deployed contract to call.
    #[error("No deployed contract")]
    NoContract,
    /// There is no registered transcoder to encode/decode messages for the called contract.
    #[error("Missing transcoder")]
    NoTranscoder,
}

impl SessionError {
    /// Check if the error is a revert error and if so, decode the error message.
    pub fn decode_revert<T: Decode>(&self) -> Result<MessageResult<T>, Self> {
        match self {
            SessionError::CallReverted(error) => {
                Ok(MessageResult::decode(&mut &error[..]).expect("Failed to decode error"))
            }
            _ => Err(self.clone()),
        }
    }
}
