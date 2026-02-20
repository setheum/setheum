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

use std::{collections::HashMap, error::Error};

use anyhow::{anyhow, bail, Result};
use contract_transcode::Value;
use futures::{channel::mpsc::UnboundedSender, StreamExt};
use subxt::{events::EventDetails, ext::sp_core::H256};

use crate::{
    api::contracts::events::ContractEmitted, connections::TxInfo, contract::ContractInstance,
    utility::BlocksApi, AccountId, AlephConfig, Connection,
};

/// Represents details about the block contianing the event.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlockDetails {
/// the block number
    pub block_number: u32,
/// the block hash
    pub block_hash: H256,
}

/// Represents a single event emitted by a contract.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContractEvent {
/// The address of the contract that emitted the event.
    pub contract: AccountId,
/// The name of the event.
    pub name: Option<String>,
/// Data contained in the event.
    pub data: HashMap<String, Value>,
/// details about the block containing the event
    pub block_details: Option<BlockDetails>,
}

/// Fetch and decode all events that correspond to the call identified by `tx_info` made to
/// `contract`.
///
/// ```no_run
/// # use setheum_client::{AccountId, Connection, SignedConnection};
/// # use setheum_client::contract::ContractInstance;
/// # use setheum_client::contract::event::{get_contract_events, listen_contract_events};
/// # use anyhow::Result;
/// use futures::{channel::mpsc::unbounded, StreamExt};
///
/// # async fn example(conn: Connection, signed_conn: SignedConnection, address: AccountId, path: &str) -> Result<()> {
/// let contract = ContractInstance::new(address, path)?;
///
/// let tx_info = contract.contract_exec0(&signed_conn, "some_method").await?;
///
/// println!("Received events {:?}", get_contract_events(&conn, &contract, tx_info).await);
///
/// #   Ok(())
/// # }
/// ```
pub async fn get_contract_events(
    conn: &Connection,
    contract: &ContractInstance,
    tx_info: TxInfo,
) -> Result<Vec<ContractEvent>> {
    let events = conn.get_tx_events(tx_info).await?;
    translate_events(events.iter(), &[contract], None)
        .into_iter()
        .collect()
}

/// Starts an event listening loop. Will send contract event and every error encountered while
/// fetching through the provided [UnboundedSender].
///
/// Only events coming from the address of one of the `contracts` will be decoded.
///
/// The loop will terminate once `sender` is closed. The loop may also terminate in case of errors while fetching blocks
/// or decoding events (pallet events, contract event decoding errors are sent over the channel).
///
/// You most likely want to `tokio::spawn` the resulting future, so that it runs concurrently.
///
/// ```no_run
/// # use std::sync::Arc;
/// # use std::sync::mpsc::channel;
/// # use std::time::Duration;
/// # use setheum_client::{AccountId, Connection, SignedConnection};
/// # use setheum_client::contract::ContractInstance;
/// # use setheum_client::contract::event::{listen_contract_events};
/// # use anyhow::Result;
/// use futures::{channel::mpsc::unbounded, StreamExt};
///
/// # async fn example(conn: Connection, signed_conn: SignedConnection, address1: AccountId, address2: AccountId, path1: &str, path2: &str) -> Result<()> {
/// // The `Arc` makes it possible to pass a reference to the contract to another thread
/// let contract1 = Arc::new(ContractInstance::new(address1, path1)?);
/// let contract2 = Arc::new(ContractInstance::new(address2, path2)?);
///
/// let conn_copy = conn.clone();
/// let contract1_copy = contract1.clone();
/// let contract2_copy = contract2.clone();
///
/// let (tx, mut rx) = unbounded();
/// let listen = || async move {
///     listen_contract_events(&conn, &[contract1_copy.as_ref(), contract2_copy.as_ref()], tx).await?;
///     <Result<(), anyhow::Error>>::Ok(())
/// };
/// let join = tokio::spawn(listen());
///
/// contract1.contract_exec0(&signed_conn, "some_method").await?;
/// contract2.contract_exec0(&signed_conn, "some_other_method").await?;
///
/// println!("Received event {:?}", rx.next().await);
///
/// rx.close();
/// join.await??;
///
/// #   Ok(())
/// # }
/// ```
pub async fn listen_contract_events(
    conn: &Connection,
    contracts: &[&ContractInstance],
    sender: UnboundedSender<Result<ContractEvent>>,
) -> Result<()> {
    let mut block_subscription = conn.as_client().blocks().subscribe_finalized().await?;

    while let Some(block) = block_subscription.next().await {
        if sender.is_closed() {
            break;
        }

        let block = block?;

        let events = block.events().await?;
        for event in translate_events(
            events.iter(),
            contracts,
            Some(BlockDetails {
                block_number: block.number(),
                block_hash: block.hash(),
            }),
        ) {
            sender.unbounded_send(event)?;
        }
    }

    Ok(())
}

/// Try to convert `events` to `ContractEvent` using matching contract from `contracts`.
pub fn translate_events<
    Err: Error + Into<anyhow::Error> + Send + Sync + 'static,
    E: Iterator<Item = Result<EventDetails<AlephConfig>, Err>>,
>(
    events: E,
    contracts: &[&ContractInstance],
    block_details: Option<BlockDetails>,
) -> Vec<Result<ContractEvent>> {
    events
        .filter_map(|maybe_event| {
            maybe_event
                .map(|e| e.as_event::<ContractEmitted>().ok().flatten())
                .transpose()
        })
        .map(|maybe_event| match maybe_event {
            Ok(e) => translate_event(&e, contracts, block_details.clone()),
            Err(e) => Err(anyhow::Error::from(e)),
        })
        .collect()
}

/// Try to convert `event` to `ContractEvent` using matching contract from `contracts`.
fn translate_event(
    event: &ContractEmitted,
    contracts: &[&ContractInstance],
    block_details: Option<BlockDetails>,
) -> Result<ContractEvent> {
    let matching_contract = contracts
        .iter()
        .find(|contract| contract.address() == &event.contract.0)
        .ok_or_else(|| anyhow!("The event wasn't emitted by any of the provided contracts"))?;

    let data = zero_prefixed(&event.data);
    let data = matching_contract
        .transcoder
        .decode_contract_event(&mut data.as_slice())?;

    build_event(matching_contract.address.clone(), data, block_details)
}

/// The contract transcoder assumes there is an extra byte (that it discards) indicating the size of the data. However,
/// data arriving through the subscription as used in this file don't have this extra byte. This function adds it.
fn zero_prefixed(data: &[u8]) -> Vec<u8> {
    let mut result = vec![0];
    result.extend_from_slice(data);
    result
}

fn build_event(
    address: AccountId,
    event_data: Value,
    block_details: Option<BlockDetails>,
) -> Result<ContractEvent> {
    match event_data {
        Value::Map(map) => Ok(ContractEvent {
            contract: address,
            name: map.ident(),
            data: map
                .iter()
                .map(|(key, value)| (key.to_string(), value.clone()))
                .collect(),
            block_details,
        }),
        _ => bail!("Contract event data is not a map"),
    }
}
