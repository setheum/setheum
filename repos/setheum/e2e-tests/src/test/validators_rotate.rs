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

use std::collections::HashMap;

use setheum_client::{
    pallets::{elections::ElectionsSudoApi, session::SessionApi},
    primitives::CommitteeSeats,
    utility::BlocksApi,
    waiting::{SetheumWaiting, BlockStatus, WaitingExt},
    TxStatus,
};
use anyhow::anyhow;

use crate::{
    accounts::account_ids_from_keys, config::setup_test, elections::compute_session_committee,
    validators::get_test_validators,
};

const TEST_LENGTH: u32 = 5;

#[tokio::test]
pub async fn validators_rotate() -> anyhow::Result<()> {
    let config = setup_test();
    let connection = config.get_first_signed_connection().await;
    let root_connection = config.create_root_connection().await;

    let era_validators = get_test_validators(config);
    let reserved_validators = account_ids_from_keys(&era_validators.reserved);

    let non_reserved_validators = account_ids_from_keys(&era_validators.non_reserved);

    let seats = CommitteeSeats {
        reserved_seats: 2,
        non_reserved_seats: 2,
        non_reserved_finality_seats: 2,
    };

    root_connection
        .change_validators(
            Some(reserved_validators.clone()),
            Some(non_reserved_validators.clone()),
            Some(seats.clone()),
            TxStatus::InBlock,
        )
        .await?;
    root_connection
        .wait_for_n_eras(2, BlockStatus::Finalized)
        .await;
    let current_session = root_connection.get_session(None).await;
    root_connection
        .wait_for_n_sessions(TEST_LENGTH, BlockStatus::Finalized)
        .await;

    let mut non_reserved_count = HashMap::new();

    for session in current_session..current_session + TEST_LENGTH {
        let elected = connection
            .get_validators(connection.first_block_of_session(session).await?)
            .await;

        let (_, non_reserved) = compute_session_committee(&root_connection, session).await?;

        for nr in non_reserved.clone() {
            *non_reserved_count.entry(nr).or_insert(0) += 1;
        }

        let reserved_included = reserved_validators
            .clone()
            .iter()
            .all(|reserved| elected.contains(reserved));

        let non_reserved_include = non_reserved
            .iter()
            .all(|non_reserved| elected.contains(non_reserved));

        let only_expected_validators = elected
            .iter()
            .all(|elected| reserved_validators.contains(elected) || non_reserved.contains(elected));

        assert!(
            reserved_included,
            "Reserved nodes should always be present, session #{session}"
        );
        assert!(
            non_reserved_include,
            "Missing non reserved node, session #{session}"
        );
        assert!(
            only_expected_validators,
            "Only expected validators should be present, session #{session}"
        );
    }

    let max_elected = non_reserved_count.values().max().unwrap();
    let min_elected = non_reserved_count.values().min().unwrap();
    assert!(max_elected - min_elected <= 1);

    let block_number = connection
        .get_best_block()
        .await?
        .ok_or(anyhow!("Failed to retrieve best block number!"))?;
    connection
        .wait_for_block(|n| n >= block_number, BlockStatus::Finalized)
        .await;

    Ok(())
}
