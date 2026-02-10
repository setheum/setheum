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

use crate::{
    io::{ReceiveError, SendError},
    metrics::Metrics,
    Data, PublicKey, SecretKey, Splittable,
};

mod handshake;
mod negotiation;
mod v1;

use handshake::HandshakeError;
pub use negotiation::{protocol, ProtocolNegotiationError};

pub type Version = u32;

/// What connections send back to the service after they become established. Starts with a public
/// key of the remote node, followed by a channel for sending data to that node, with None if the
/// connection was unsuccessful and should be reestablished.
pub type ResultForService<PK, D> = (PK, Option<mpsc::UnboundedSender<D>>);

/// Defines the protocol for communication. Currently single variant, but left in case of protocol change.
#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
/// The current version of the protocol, with pseudorandom connection direction and
/// multiplexing.
    V1,
}

/// Protocol error.
#[derive(Debug)]
pub enum ProtocolError<PK: PublicKey> {
/// Error during performing a handshake.
    HandshakeError(HandshakeError<PK>),
/// Sending failed.
    SendError(SendError),
/// Receiving failed.
    ReceiveError(ReceiveError),
/// Heartbeat stopped.
    CardiacArrest,
/// Channel to the parent service closed.
    NoParentConnection,
/// Data channel closed.
    NoUserConnection,
/// Authorization error.
    NotAuthorized,
/// Send operation took too long
    SendTimeout,
}

impl<PK: PublicKey> Display for ProtocolError<PK> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        use ProtocolError::*;
        match self {
            HandshakeError(e) => write!(f, "handshake error: {e}"),
            SendError(e) => write!(f, "send error: {e}"),
            ReceiveError(e) => write!(f, "receive error: {e}"),
            CardiacArrest => write!(f, "heartbeat stopped"),
            NoParentConnection => write!(f, "cannot send result to service"),
            NoUserConnection => write!(f, "cannot send data to user"),
            NotAuthorized => write!(f, "peer not authorized"),
            SendTimeout => write!(f, "send timed out"),
        }
    }
}

impl<PK: PublicKey> From<HandshakeError<PK>> for ProtocolError<PK> {
    fn from(e: HandshakeError<PK>) -> Self {
        ProtocolError::HandshakeError(e)
    }
}

impl<PK: PublicKey> From<SendError> for ProtocolError<PK> {
    fn from(e: SendError) -> Self {
        ProtocolError::SendError(e)
    }
}

impl<PK: PublicKey> From<ReceiveError> for ProtocolError<PK> {
    fn from(e: ReceiveError) -> Self {
        ProtocolError::ReceiveError(e)
    }
}

impl Protocol {
/// Minimal supported protocol version.
    const MIN_VERSION: Version = 1;

/// Maximal supported protocol version.
    const MAX_VERSION: Version = 1;

/// Launches the proper variant of the protocol (receiver half).
    pub async fn manage_incoming<SK: SecretKey, D: Data, S: Splittable>(
        &self,
        stream: S,
        secret_key: SK,
        result_for_parent: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
        data_for_user: mpsc::UnboundedSender<D>,
        authorization_requests_sender: mpsc::UnboundedSender<(
            SK::PublicKey,
            oneshot::Sender<bool>,
        )>,
        metrics: Metrics,
    ) -> Result<(), ProtocolError<SK::PublicKey>> {
        use Protocol::*;
        match self {
            V1 => {
                v1::incoming(
                    stream,
                    secret_key,
                    authorization_requests_sender,
                    result_for_parent,
                    data_for_user,
                    metrics,
                )
                .await
            }
        }
    }

/// Launches the proper variant of the protocol (sender half).
    pub async fn manage_outgoing<SK: SecretKey, D: Data, S: Splittable>(
        &self,
        stream: S,
        secret_key: SK,
        public_key: SK::PublicKey,
        result_for_service: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
        data_for_user: mpsc::UnboundedSender<D>,
        metrics: Metrics,
    ) -> Result<(), ProtocolError<SK::PublicKey>> {
        use Protocol::*;
        match self {
            V1 => {
                v1::outgoing(
                    stream,
                    secret_key,
                    public_key,
                    result_for_service,
                    data_for_user,
                    metrics,
                )
                .await
            }
        }
    }
}

impl TryFrom<Version> for Protocol {
    type Error = Version;

    fn try_from(version: Version) -> Result<Self, Self::Error> {
        match version {
            1 => Ok(Protocol::V1),
            unknown_version => Err(unknown_version),
        }
    }
}
