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

use std::fmt::{Debug, Display, Error as FmtError, Formatter};

use futures::channel::mpsc;
use log::{debug, info};
use tokio::time::{sleep, timeout, Duration};

use crate::{
    metrics::Metrics,
    protocols::{protocol, ProtocolError, ProtocolNegotiationError, ResultForService},
    ConnectionInfo, Data, Dialer, PeerAddressInfo, PublicKey, SecretKey, LOG_TARGET,
};

enum OutgoingError<PK: PublicKey, A: Data, ND: Dialer<A>> {
    Dial(ND::Error),
    ProtocolNegotiation(PeerAddressInfo, ProtocolNegotiationError),
    Protocol(PeerAddressInfo, ProtocolError<PK>),
    TimedOut,
}

impl<PK: PublicKey, A: Data, ND: Dialer<A>> Display for OutgoingError<PK, A, ND> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        use OutgoingError::*;
        match self {
            Dial(e) => write!(f, "dial error: {e}"),
            ProtocolNegotiation(addr, e) => write!(
                f,
                "communication with {addr} failed, protocol negotiation error: {e}"
            ),
            Protocol(addr, e) => write!(f, "communication with {addr} failed, protocol error: {e}"),
            TimedOut => write!(f, "dial timeout",),
        }
    }
}

/// Arbitrarily chosen timeout, should be more than enough.
const DIAL_TIMEOUT: Duration = Duration::from_secs(60);

async fn manage_outgoing<SK: SecretKey, D: Data, A: Data, ND: Dialer<A>>(
    secret_key: SK,
    public_key: SK::PublicKey,
    mut dialer: ND,
    address: A,
    result_for_parent: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
    data_for_user: mpsc::UnboundedSender<D>,
    metrics: Metrics,
) -> Result<(), OutgoingError<SK::PublicKey, A, ND>> {
    debug!(target: LOG_TARGET, "Trying to connect to {}.", public_key);
    let stream = timeout(DIAL_TIMEOUT, dialer.connect(address))
        .await
        .map_err(|_| OutgoingError::TimedOut)?
        .map_err(OutgoingError::Dial)?;
    let peer_address_info = stream.peer_address_info();
    debug!(
        target: LOG_TARGET,
        "Performing outgoing protocol negotiation."
    );
    let (stream, protocol) = protocol(stream)
        .await
        .map_err(|e| OutgoingError::ProtocolNegotiation(peer_address_info.clone(), e))?;
    debug!(target: LOG_TARGET, "Negotiated protocol, running.");
    protocol
        .manage_outgoing(
            stream,
            secret_key,
            public_key,
            result_for_parent,
            data_for_user,
            metrics,
        )
        .await
        .map_err(|e| OutgoingError::Protocol(peer_address_info.clone(), e))
}

const RETRY_DELAY: Duration = Duration::from_secs(10);

/// Establish an outgoing connection to the provided peer using the dialer and then manage it.
/// While this works it will send any data from the user to the peer. Any failures will be reported
/// to the parent, so that connections can be reestablished if necessary.
pub async fn outgoing<SK: SecretKey, D: Data, A: Data + Debug, ND: Dialer<A>>(
    secret_key: SK,
    public_key: SK::PublicKey,
    dialer: ND,
    address: A,
    result_for_parent: mpsc::UnboundedSender<ResultForService<SK::PublicKey, D>>,
    data_for_user: mpsc::UnboundedSender<D>,
    metrics: Metrics,
) {
    if let Err(e) = manage_outgoing(
        secret_key,
        public_key.clone(),
        dialer,
        address.clone(),
        result_for_parent.clone(),
        data_for_user,
        metrics,
    )
    .await
    {
        info!(
            target: LOG_TARGET,
            "Outgoing connection to {} {:?} failed: {}, will retry after {}s.",
            public_key,
            address,
            e,
            RETRY_DELAY.as_secs()
        );
        sleep(RETRY_DELAY).await;
        if result_for_parent
            .unbounded_send((public_key, None))
            .is_err()
        {
            debug!(target: LOG_TARGET, "Could not send the closing message, we've probably been terminated by the parent service.");
        }
    }
}
