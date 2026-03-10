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

mod chain_spec;

use sc_chain_spec::ChainSpec;
use sc_cli::{
    clap::{self, Parser, Subcommand as ClapSubcommand},
    SubstrateCli,
};

use crate::chain_spec::{BootstrapChainCmd, ConvertChainspecToRawCmd};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Setheum Chain Bootstrapper".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Setheum chain-bootstrapper: generates initial chainspec and keystore for validators".into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/setheum-labs/setheum/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2019
    }

    fn load_spec(&self, _id: &str) -> Result<Box<dyn ChainSpec>, String> {
        panic!("This is not used.")
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, ClapSubcommand)]
pub enum Subcommand {
    /// Generates keystore (libp2p key and session keys), and generates chainspec to stdout
    BootstrapChain(BootstrapChainCmd),

    /// Takes a chainspec and generates a corresponding raw chainspec
    ConvertChainspecToRaw(ConvertChainspecToRawCmd),

    /// Key management cli utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),
}

fn main() -> sc_cli::Result<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Some(Subcommand::BootstrapChain(cmd)) => cmd.run(),
        Some(Subcommand::ConvertChainspecToRaw(cmd)) => cmd.run(),
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),

        None => Err("Command was required!".into()),
    }
}
