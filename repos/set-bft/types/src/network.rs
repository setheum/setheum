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

use crate::NodeIndex;

use codec::{Decode, Encode};

/// A recipient of a message, either a specific node or everyone.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Decode, Encode)]
pub enum Recipient {
    Everyone,
    Node(NodeIndex),
}

/// Network represents an interface for sending and receiving NetworkData.
///
/// Note on Rate Control: it is assumed that Network implements a rate control mechanism guaranteeing
/// that no node is allowed to spam messages without limits. We do not specify details yet, but in
/// future releases we plan to publish recommended upper bounds for the amounts of bandwidth and
/// number of messages allowed per node per a unit of time. These bounds must be carefully crafted
/// based upon the number of nodes N and the configured delays between subsequent Dag rounds, so
/// that at the same time spammers are cut off but honest nodes are able function correctly within
/// these bounds.
///
/// Note on Network Reliability: it is not assumed that each message that AlephBFT orders to send
/// reaches its intended recipient, there are some built-in reliability mechanisms within AlephBFT
/// that will automatically detect certain failures and resend messages as needed. Clearly, the less
/// reliable the network is, the worse the performance of AlephBFT will be (generally slower to
/// produce output). Also, not surprisingly if the percentage of dropped messages is too high
/// AlephBFT might stop making progress, but from what we observe in tests, this happens only when
/// the reliability is extremely bad, i.e., drops below 50% (which means there is some significant
/// issue with the network).
///
/// We refer to the documentation https://cardinal-cryptography.github.io/AlephBFT/aleph_bft_api.html
/// Section 3.1.2 for a discussion of the required guarantees of this trait's implementation.
#[async_trait::async_trait]
pub trait Network<D>: Send + 'static {
    /// Send a message to a single node or everyone, depending on the value of the recipient
    /// argument.
    ///
    /// Note on the implementation: this function should be implemented in a non-blocking manner.
    /// Otherwise, the performance might be affected negatively or the execution may end up in a deadlock.
    fn send(&self, data: D, recipient: Recipient);
    /// Receive a message from the network.
    async fn next_event(&mut self) -> Option<D>;
}
