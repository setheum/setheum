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

use setheum_client::{pallets::setheum::SetheumApi, utility::BlocksApi, Connection};
use log::info;
use primitives::{BlockNumber, Version};

pub async fn check_finality_version_at_block(
    connection: &Connection,
    block_number: BlockNumber,
    expected_version: Version,
) {
    info!(
        "Checking current session finality version for block {}",
        block_number
    );
    let block_hash = connection
        .get_block_hash(block_number)
        .await
        .expect("Should have been able to get a block hash!");
    let finality_version = connection.finality_version(block_hash).await;
    assert_eq!(finality_version, expected_version);
}

pub async fn check_next_session_finality_version_at_block(
    connection: &Connection,
    block_number: BlockNumber,
    expected_version: Version,
) {
    info!(
        "Checking next session finality version for block {}",
        block_number
    );
    let block_hash = connection
        .get_block_hash(block_number)
        .await
        .expect("Should have been able to get a block hash!");
    let next_finality_version = connection.next_session_finality_version(block_hash).await;
    assert_eq!(next_finality_version, expected_version);
}
