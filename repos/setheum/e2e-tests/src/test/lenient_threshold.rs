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
	pallets::committee_management::{CommitteeManagementApi, CommitteeManagementSudoApi},
	sp_runtime::Perquintill,
	TxStatus,
};

use crate::config::setup_test;

#[tokio::test]
pub async fn set_lenient_threshold_test() -> anyhow::Result<()> {
	let config = setup_test();
	let root_connection = config.create_root_connection().await;

	root_connection.set_lenient_threshold(69, TxStatus::Finalized).await?;

	assert_eq!(Some(Perquintill::from_percent(69)), root_connection.get_lenient_threshold_percentage(None).await);

	Ok(())
}
