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

use std::ops::Deref;

use anyhow::Result;
use primitives::Balance;
use rand::Rng;
use setheum_client::{
	pallets::balances::BalanceUserApi, AccountId, Connection, KeyPair, Pair, SignedConnection, SignedConnectionApi,
	TxStatus,
};

use crate::config::Config;

/// A wrapper around a KeyPair for purposes of converting to an account id in tests.
pub struct KeyPairWrapper(KeyPair);

impl KeyPairWrapper {
	/// Creates a copy of the `connection` signed by `signer`
	pub fn sign(&self, conn: &Connection) -> SignedConnection {
		SignedConnection::from_connection(conn.clone(), self.clone().0)
	}
}

impl Clone for KeyPairWrapper {
	fn clone(&self) -> Self {
		Self(KeyPair::new(self.0.signer().clone()))
	}
}

impl Deref for KeyPairWrapper {
	type Target = KeyPair;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<&KeyPairWrapper> for AccountId {
	fn from(keypair: &KeyPairWrapper) -> Self {
		keypair.signer().public().into()
	}
}

impl From<KeyPairWrapper> for AccountId {
	fn from(keypair: KeyPairWrapper) -> Self {
		(&keypair).into()
	}
}

/// Derives a test account based on a randomized string
pub fn random_account() -> KeyPairWrapper {
	KeyPairWrapper(setheum_client::keypair_from_string(&format!("//TestAccount/{}", rand::thread_rng().gen::<u128>())))
}

/// Transfer `amount` from `from` to `to`
pub async fn transfer<S: SignedConnectionApi>(conn: &S, to: &KeyPair, amount: Balance) -> Result<()> {
	conn.transfer_keep_alive(to.signer().public().into(), amount, TxStatus::Finalized).await.map(|_| ())
}

/// Returns a number representing the given amount of setheums (adding decimals)
pub fn setheums(basic_unit_amount: Balance) -> Balance {
	basic_unit_amount * 1_000_000_000_000
}

/// Prepares a `(conn, authority, account)` triple with some money in `account` for fees.
pub async fn basic_test_context(config: &Config) -> Result<(Connection, KeyPairWrapper, KeyPairWrapper)> {
	let conn = Connection::new(&config.node).await;
	let authority = KeyPairWrapper(setheum_client::keypair_from_string(&config.sudo_seed));
	let account = random_account();

	transfer(&authority.sign(&conn), &account, setheums(1)).await?;

	Ok((conn, authority, account))
}
