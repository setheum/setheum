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

use std::iter::repeat;

use setheum_client::{pallets::balances::BalanceUserBatchExtApi, TxStatus};

use crate::{config::setup_test, transfer::setup_for_transfer};

#[tokio::test]
pub async fn batch_transactions() -> anyhow::Result<()> {
	let config = setup_test();
	const NUMBER_OF_TRANSACTIONS: usize = 100;

	let (connection, to) = setup_for_transfer(config).await;

	let accounts: Vec<_> = repeat(to.clone()).take(NUMBER_OF_TRANSACTIONS).collect();
	connection.batch_transfer_keep_alive(&accounts, 1000, TxStatus::Finalized).await?;

	Ok(())
}
