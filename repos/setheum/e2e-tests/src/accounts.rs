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

use setheum_client::{keypair_from_string, raw_keypair_from_string, AccountId, KeyPair, Pair, RawKeyPair};

use crate::config::Config;

pub fn get_validator_seed(seed: u32) -> String {
	assert!(seed > 0, "//0 seed is reserved for RPC node!");
	format!("//{seed}")
}

// in default e2e setup, //0 is a RPC node and //1, //2, ... are validators
pub fn get_validators_seeds(config: &Config) -> Vec<String> {
	match config.validators_seeds {
		Some(ref seeds) => seeds.clone(),
		None => (1..config.validator_count + 1).map(get_validator_seed).collect(),
	}
}

pub fn get_validators_keys(config: &Config) -> Vec<KeyPair> {
	accounts_seeds_to_keys(&get_validators_seeds(config))
}
pub fn get_validators_raw_keys(config: &Config) -> Vec<RawKeyPair> {
	accounts_seeds_to_raw_keys(&get_validators_seeds(config))
}

pub fn accounts_seeds_to_keys(seeds: &[String]) -> Vec<KeyPair> {
	seeds.iter().map(String::as_str).map(keypair_from_string).collect()
}
pub fn accounts_seeds_to_raw_keys(seeds: &[String]) -> Vec<RawKeyPair> {
	seeds.iter().map(String::as_str).map(raw_keypair_from_string).collect()
}

pub fn get_sudo_key(config: &Config) -> KeyPair {
	keypair_from_string(&config.sudo_seed)
}

pub struct NodeKeys {
	pub validator: KeyPair,
}

impl From<String> for NodeKeys {
	fn from(seed: String) -> Self {
		Self { validator: keypair_from_string(&seed) }
	}
}

pub fn account_ids_from_keys(keys: &[KeyPair]) -> Vec<AccountId> {
	keys.iter().map(|pair| AccountId::from(pair.signer().public())).collect()
}
