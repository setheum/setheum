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

use std::fmt::{Display, Error as FmtError, Formatter};

use futures::channel::{mpsc, oneshot};
use log::{debug, info};

use crate::{
    metrics::Metrics,
    protocols::{protocol, ProtocolError, ProtocolNegotiationError, ResultForService},
    Data, PublicKey, SecretKey, Splittable, LOG_TARGET,
};

enum IncomingError<PK: PublicKey> {
    ProtocolNegotiationError(ProtocolNegotiationError),
    ProtocolError(ProtocolError<PK>),
}

impl<PK: PublicKey> Display for IncomingError<PK> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        use IncomingError::*;
        match self {
            ProtocolNegotiationError(e) => write!(f, "protocol negotiation error: {e}"),
            ProtocolError(e) => write!(f, "protocol error: {e}"),
        }
    }
}

impl<PK: PublicKey> From<ProtocolNegotiationError> for IncomingError<PK> {
    fn from(e: ProtocolNegotiationError) -> Self {
        IncomingError::ProtocolNegotiationError(e)
    }
}

impl<PK: PublicKey> From<ProtocolError<PK>> for IncomingError<PK> {
    fn from(e: ProtocolError<PK>) -> Self {
        IncomingError::ProtocolError(e)
    }
}

async fn manage_incoming<SK: SecretKey, D: Data, S: Splittable>(
    secret_key: SK,
    stream: S,
    result_for_parent: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
    data_for_user: mpsc::UnboundedSender<D>,
    authorization_requests_sender: mpsc::UnboundedSender<(SK::PublicKey, oneshot::Sender<bool>)>,
    metrics: Metrics,
) -> Result<(), IncomingError<SK::PublicKey>> {
    debug!(
        target: LOG_TARGET,
        "Performing incoming protocol negotiation."
    );
    let (stream, protocol) = protocol(stream).await?;
    debug!(target: LOG_TARGET, "Negotiated protocol, running.");
    Ok(protocol
        .manage_incoming(
            stream,
            secret_key,
            result_for_parent,
            data_for_user,
            authorization_requests_sender,
            metrics,
        )
        .await?)
}

/// Manage an incoming connection. After the handshake it will send the recognized PublicKey to
/// the parent, together with an exit channel for this process. When this channel is dropped the
/// process ends. Whenever data arrives on this connection it will be passed to the user. Any
/// failures in receiving data result in the process stopping, we assume the other side will
/// reestablish it if necessary.
pub async fn incoming<SK: SecretKey, D: Data, S: Splittable>(
    secret_key: SK,
    stream: S,
    result_for_parent: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
    data_for_user: mpsc::UnboundedSender<D>,
    authorization_requests_sender: mpsc::UnboundedSender<(SK::PublicKey, oneshot::Sender<bool>)>,
    metrics: Metrics,
) {
    let addr = stream.peer_address_info();
    if let Err(e) = manage_incoming(
        secret_key,
        stream,
        result_for_parent,
        data_for_user,
        authorization_requests_sender,
        metrics,
    )
    .await
    {
        info!(
            target: LOG_TARGET,
            "Incoming connection from {} failed: {}.", addr, e
        );
    }
}
