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

use finality_setbft::UnitCreationDelay;
use log::warn;
use primitives::{DEFAULT_MAX_NON_FINALIZED_BLOCKS, DEFAULT_UNIT_CREATION_DELAY};
use sc_cli::{
	clap::{self, ArgGroup, Parser, Subcommand as ClapSubcommand},
	PurgeChainCmd, RunCmd, SubstrateCli,
};
use std::path::PathBuf;

use crate::chain_spec;

#[derive(Debug, Parser)]
#[clap(subcommand_negates_reqs(true), version(env!("SUBSTRATE_CLI_IMPL_VERSION")))]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[command(flatten)]
	pub setheum: SetheumCli,

	#[command(flatten)]
	pub run: RunCmd,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Setheum Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"docs.setheumzero.org".into()
	}

	fn copyright_start_year() -> i32 {
		2021
	}

	fn native_runtime_version(_: &Box<dyn sc_service::ChainSpec>) -> &'static sc_cli::RuntimeVersion {
		&setheum_runtime::VERSION
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		let default_chain = "testnet";
		let id = id.trim();
		let id = if id.is_empty() { default_chain } else { id };

		let chainspec = match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"local" => Box::new(chain_spec::local_testnet_config()?),
			"testnet-new" => Box::new(chain_spec::public_testnet_config()?),
			"mainnet-new" => Box::new(chain_spec::mainnet_config()?),
			"testnet" => Box::new(chain_spec::live_testnet_config()?),
			"mainnet" => Box::new(chain_spec::live_mainnet_config()?),
			path => Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};
		Ok(chainspec)
	}
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, ClapSubcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommand benchmarking runtime pallets.
	#[cfg(feature = "runtime-benchmarks")]
	#[clap(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// The custom benchmark subcommand benchmarking runtime pallets. Note: `runtime-benchmarks`
	/// feature must be enabled.
	#[cfg(not(feature = "runtime-benchmarks"))]
	Benchmark,
}

#[derive(Debug, Parser, Clone)]
#[clap(group(ArgGroup::new("backup")))]
pub struct SetheumCli {
	#[clap(long, default_value_t = DEFAULT_UNIT_CREATION_DELAY)]
	unit_creation_delay: u64,

	/// The addresses at which the node will be externally reachable for validator network
	/// purposes. Have to be provided for validators.
	#[clap(long)]
	public_validator_addresses: Option<Vec<String>>,

	/// The port on which to listen to validator network connections.
	#[clap(long, default_value_t = 30343)]
	validator_port: u16,

	/// Turn off backups, at the cost of limiting crash recoverability.
	///
	/// If backups are turned off and the node crashes, it most likely will not be able to continue
	/// the session during which it crashed. It will join SetBFT consensus in the next session.
	#[clap(long, group = "backup")]
	no_backup: bool,
	/// The path to save backups to.
	///
	/// Backups created by the node are saved under this path. When restarted after a crash,
	/// the backups will be used to recover the node's state, helping prevent auto-forks. The layout
	/// of the directory is unspecified. This flag must be specified unless backups are turned off
	/// with `--no-backup`, but note that that limits crash recoverability.
	#[clap(long, value_name = "PATH", group = "backup")]
	backup_path: Option<PathBuf>,

	/// The maximum number of nonfinalized blocks, after which block production should be locally
	/// stopped. DO NOT CHANGE THIS, PRODUCING MORE OR FEWER BLOCKS MIGHT BE CONSIDERED MALICIOUS
	/// BEHAVIOUR AND PUNISHED ACCORDINGLY!
	#[clap(long, default_value_t = DEFAULT_MAX_NON_FINALIZED_BLOCKS)]
	max_nonfinalized_blocks: u32,

	/// Enable database pruning. It removes older entries in the state-database. Pruning of blocks is not supported.
	/// Note that we only support pruning with ParityDB database backend.
	/// See also `--state-pruning` option for more details.
	#[clap(long, default_value_t = false)]
	enable_pruning: bool,

	/// Maximum bit-rate in bits per second of the setbft validator network.
	#[clap(long, default_value_t = 768 * 1024)]
	setbft_network_bit_rate: u64,

	/// Maximum bit-rate in bits per second of the substrate network.
	#[clap(long, default_value_t = 5*1024*1024)]
	substrate_network_bit_rate: u64,

	/// Don't spend some extra time to collect more debugging data (e.g. validator network details).
	/// By default collecting is enabled, as the impact on performance is negligible, if any.
	#[clap(long, default_value_t = true)]
	collect_validator_network_data: bool,
}

impl SetheumCli {
	pub fn unit_creation_delay(&self) -> UnitCreationDelay {
		UnitCreationDelay(self.unit_creation_delay)
	}

	pub fn external_addresses(&self) -> Vec<String> {
		self.public_validator_addresses.clone().unwrap_or_default()
	}

	pub fn set_dummy_external_addresses(&mut self) {
		self.public_validator_addresses = Some(vec!["192.0.2.43:30343".to_string()])
	}

	pub fn validator_port(&self) -> u16 {
		self.validator_port
	}

	pub fn backup_path(&self) -> Option<PathBuf> {
		self.backup_path.clone()
	}

	pub fn no_backup(&self) -> bool {
		self.no_backup
	}

	pub fn max_nonfinalized_blocks(&self) -> u32 {
		if self.max_nonfinalized_blocks != DEFAULT_MAX_NON_FINALIZED_BLOCKS {
			warn!("Running block production with a value of max-nonfinalized-blocks {}, which is not the default of 20. THIS MIGHT BE CONSIDERED MALICIOUS BEHAVIOUR AND RESULT IN PENALTIES!", self.max_nonfinalized_blocks);
		}
		self.max_nonfinalized_blocks
	}

	pub fn enable_pruning(&self) -> bool {
		self.enable_pruning
	}

	pub fn setbft_network_bit_rate(&self) -> u64 {
		self.setbft_network_bit_rate
	}

	pub fn substrate_network_bit_rate(&self) -> u64 {
		self.substrate_network_bit_rate
	}

	pub fn collect_validator_network_data(&self) -> bool {
		self.collect_validator_network_data
	}
}
