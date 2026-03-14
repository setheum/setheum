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

use std::fs;

use codec::Encode;
use log::info;
use move_vm_backend_common::types::ScriptTransaction;
use setheum_move::{run_with_args, SetheumMoveArgs, SetheumMoveCommand};
use move_cli::Move;

use crate::{
	commands::{MoveCommand, MoveExecute, MovePackageCommand, MovePublish},
	ConnectionConfig,
};

pub async fn move_command(cfg: ConnectionConfig, command: MoveCommand) -> anyhow::Result<()> {
	match command {
		MoveCommand::Execute(exec) => execute(cfg.get_signed_connection().await, exec).await,
		MoveCommand::PublishModule(pub_mod) => publish_module(cfg.get_signed_connection().await, pub_mod).await,
		MoveCommand::PublishBundle(pub_bundle) => publish_bundle(cfg.get_signed_connection().await, pub_bundle).await,
		MoveCommand::GetResource { account, tag } => {
			get_resource(cfg.get_connection().await, account, tag).await
		},
		MoveCommand::GetModuleABI { address, name } => {
			get_module_abi(cfg.get_connection().await, address, name).await
		},
		MoveCommand::GetModule { address, name } => {
			get_module(cfg.get_connection().await, address, name).await
		},
		MoveCommand::Package { cmd } => move_package_command(cmd).await,
	}
}

async fn move_package_command(command: MovePackageCommand) -> anyhow::Result<()> {
	let cwd = std::env::current_dir()?;
	
	let move_args = Move {
		package_path: None,
		dev: false,
		test: false,
		verbose: false,
		force: false,
		skip_fetch_latest_git_deps: false,
		fetch_deps_only: false,
		install_dir: None,
		bytecode_version: None,
	};

	let setheum_move_cmd = match command {
		MovePackageCommand::Build => SetheumMoveCommand::MoveCommand(move_cli::Command::Build),
		MovePackageCommand::Bundle => SetheumMoveCommand::Bundle {
			cmd: setheum_move::cmd::bundle::Bundle {
				output: None,
			},
		},
		MovePackageCommand::Test => SetheumMoveCommand::MoveCommand(move_cli::Command::Test {
			test_filter: None,
			debug: false,
			coverage: false,
			test_args: vec![],
		}),
		MovePackageCommand::New { name } => SetheumMoveCommand::MoveCommand(move_cli::Command::New {
			name,
		}),
	};

	let args = SetheumMoveArgs {
		move_args,
		cmd: setheum_move_cmd,
	};

	run_with_args(cwd, args).map_err(|e| anyhow::anyhow!("Move package command failed: {:?}", e))
}

async fn execute(signed_connection: SignedConnection, command: MoveExecute) -> anyhow::Result<()> {
	let MoveExecute { script_path, gas_limit, cheque_limit, args } = command;

	let bytecode = fs::read(script_path).expect("Move script not found");
	
	let script_args = args
		.unwrap_or_default()
		.into_iter()
		.map(|arg| hex::decode(arg).expect("Invalid hex argument"))
		.collect::<Vec<_>>();

	let script_tx = ScriptTransaction {
		bytecode,
		args: script_args,
		type_args: vec![], // TODO: Support type arguments in CLI
	};

	let transaction_bc = script_tx.encode();
	
	info!("Executing Move script...");
	let tx_info = signed_connection
		.execute(transaction_bc, gas_limit, cheque_limit, TxStatus::InBlock)
		.await?;
	
	info!("Move script execution submitted: {:?}", tx_info);
	Ok(())
}

async fn publish_module(signed_connection: SignedConnection, command: MovePublish) -> anyhow::Result<()> {
	let MovePublish { bytecode_path, gas_limit } = command;
	let bytecode = fs::read(bytecode_path).expect("Move module not found");
	
	info!("Publishing Move module...");
	let tx_info = signed_connection
		.publish_module(bytecode, gas_limit, TxStatus::InBlock)
		.await?;
	
	info!("Move module publication submitted: {:?}", tx_info);
	Ok(())
}

async fn publish_bundle(signed_connection: SignedConnection, command: MovePublish) -> anyhow::Result<()> {
	let MovePublish { bytecode_path, gas_limit } = command;
	let bundle = fs::read(bytecode_path).expect("Move bundle not found");
	
	info!("Publishing Move bundle...");
	let tx_info = signed_connection
		.publish_module_bundle(bundle, gas_limit, TxStatus::InBlock)
		.await?;
	
	info!("Move bundle publication submitted: {:?}", tx_info);
	Ok(())
}

async fn get_resource(connection: Connection, account: AccountId, tag: String) -> anyhow::Result<()> {
	let tag_bytes = hex::decode(tag).expect("Invalid hex tag");
	info!("Getting Move resource for account {} with tag {}...", account, hex::encode(&tag_bytes));
	
	let resource = connection.get_resource(account, tag_bytes).await?;
	if let Some(res) = resource {
		info!("Resource: {}", hex::encode(res));
	} else {
		info!("Resource not found.");
	}
	Ok(())
}

async fn get_module_abi(connection: Connection, address: AccountId, name: String) -> anyhow::Result<()> {
	info!("Getting Move module ABI for address {} and name {}...", address, name);
	// Note: MoveModuleApi doesn't have get_module_abi yet, but we can add it or use get_module
	info!("ABI retrieval via CLI pending implementation on MoveModuleApi");
	Ok(())
}

async fn get_module(connection: Connection, address: AccountId, name: String) -> anyhow::Result<()> {
	info!("Getting Move module bytecode for address {} and name {}...", address, name);
	let module = connection.get_module(address, name).await?;
	if let Some(m) = module {
		info!("Module bytecode length: {}", m.len());
		info!("Bytecode: {}", hex::encode(m));
	} else {
		info!("Module not found.");
	}
	Ok(())
}
