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

use setheum_client::{AccountId, Connection, KeyPair, Pair, SignedConnection};

use crate::{accounts::get_validators_raw_keys, config::Config};

async fn setup(config: &Config) -> (Connection, KeyPair, AccountId) {
    let accounts = get_validators_raw_keys(config);
    let (from, to) = (
        KeyPair::new(accounts[0].clone()),
        KeyPair::new(accounts[1].clone()),
    );
    let to = AccountId::from(to.signer().public());
    (Connection::new(&config.node).await, from, to)
}

pub async fn setup_for_transfer(config: &Config) -> (SignedConnection, AccountId) {
    let (connection, from, to) = setup(config).await;
    (SignedConnection::from_connection(connection, from), to)
}
