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

mod builder;
mod cli;
pub mod commands;
mod keystore;

pub use commands::{BootstrapChainCmd, ConvertChainspecToRawCmd};

pub const CHAINTYPE_DEV: &str = "dev";
pub const CHAINTYPE_LOCAL: &str = "local";
pub const CHAINTYPE_LIVE: &str = "live";

pub const DEFAULT_CHAIN_ID: &str = "setheum-testnet";
pub const DEFAULT_SUDO_ACCOUNT_ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

pub type SetheumChainSpec = sc_service::GenericChainSpec<()>;

use primitives::AccountId;
use sc_chain_spec::ChainType;
use sc_cli::Error;
use sp_application_crypto::Ss58Codec;

fn parse_chaintype(s: &str) -> Result<ChainType, Error> {
    Ok(match s {
        CHAINTYPE_DEV => ChainType::Development,
        CHAINTYPE_LOCAL => ChainType::Local,
        CHAINTYPE_LIVE => ChainType::Live,
        s => panic!("Wrong chain type {s} Possible values: dev local live"),
    })
}

/// Generate AccountId based on string command line argument.
fn parse_account_id(s: &str) -> Result<AccountId, Error> {
    Ok(AccountId::from_string(s).expect("Passed string is not a hex encoding of a public key"))
}
