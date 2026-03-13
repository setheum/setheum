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

use log::info;
use primitives::HEAP_PAGES;
use sc_cli::{clap::Parser, SubstrateCli};
use sc_network::config::Role;
use sc_service::Configuration;
use setheum_node::{new_authority, new_partial, Cli, ConfigValidator, ServiceComponents, Subcommand};

fn enforce_heap_pages(config: &mut Configuration) {
	config.default_heap_pages = Some(HEAP_PAGES);
}

fn main() -> sc_cli::Result<()> {
	let mut cli = Cli::parse();

	let config_validation_result = ConfigValidator::process(&mut cli);

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let ServiceComponents { client, task_manager, import_queue, .. } = new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let ServiceComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let ServiceComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let ServiceComponents { client, task_manager, import_queue, .. } = new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let ServiceComponents { client, task_manager, backend, .. } = new_partial(&config)?;
				Ok((cmd.run(client, backend, None), task_manager))
			})
		},
		#[cfg(feature = "runtime-benchmarks")]
		Some(Subcommand::Benchmark(cmd)) => {
			use primitives::Block;
			use sc_executor::NativeExecutionDispatch;
			use setheum_node::ExecutorDispatch;

			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				if let frame_benchmarking_cli::BenchmarkCmd::Pallet(cmd) = cmd {
					cmd.run::<Block, <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions>(config)
				} else {
					Err(sc_cli::Error::Input("Wrong subcommand".to_string()))
				}
			})
		},
		#[cfg(not(feature = "runtime-benchmarks"))]
		Some(Subcommand::Benchmark) => Err("Benchmarking wasn't enabled when building the node. You can enable it with \
                    `--features runtime-benchmarks`."
			.into()),
		None => {
			let runner = cli.create_runner(&cli.run)?;

			config_validation_result.report();

			let mut setheum_cli_config = cli.setheum;
			runner.run_node_until_exit(|mut config| async move {
				if matches!(config.role, Role::Full) {
					if !setheum_cli_config.external_addresses().is_empty() {
						panic!("A non-validator node cannot be run with external addresses specified.");
					}
					// We ensure that external addresses for non-validator nodes are set, but to a
					// value that is not routable. This will no longer be neccessary once we have
					// proper support for non-validator nodes, but this requires a major
					// refactor.
					info!("Running as a non-validator node, setting dummy addressing configuration.");
					setheum_cli_config.set_dummy_external_addresses();
				}
				enforce_heap_pages(&mut config);
				new_authority(config, setheum_cli_config).map_err(sc_cli::Error::Service)
			})
		},
	}
}
