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

use setheum_client::{
    utility::BlocksApi,
    waiting::{SetheumWaiting, BlockStatus},
};
use anyhow::anyhow;
use log::info;

use crate::config::setup_test;

#[tokio::test]
pub async fn finalization() -> anyhow::Result<()> {
    let config = setup_test();
    let connection = config.create_root_connection().await;

    let finalized = connection.get_finalized_block_hash().await?;
    info!("Highest finalized block hash = {finalized}");
    let finalized_number = connection
        .get_block_number(finalized)
        .await?
        .ok_or(anyhow!(
            "Failed to retrieve block number for hash {finalized:?}"
        ))?;
    info!("Waiting for block {} to be finalized", finalized_number + 1);

    connection
        .wait_for_block(|n| n > finalized_number, BlockStatus::Finalized)
        .await;

    Ok(())
}
