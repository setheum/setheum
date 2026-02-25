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

use std::time::Duration;

use current_set_bft::{create_config, default_delay_config, Config, LocalIO, Terminator};
use log::debug;
use network_clique::SpawnHandleExt;

mod network;
mod performance;
mod traits;

pub use network::NetworkData;
pub use performance::{Service as PerformanceService, ServiceIO as PerformanceServiceIO};

pub use crate::setbft_primitives::CURRENT_FINALITY_VERSION as VERSION;
use crate::{
    abft::{
        common::{unit_creation_delay_fn, MAX_ROUNDS, SESSION_LEN_LOWER_BOUND_MS},
        NetworkWrapper,
    },
    block::UnverifiedHeader,
    crypto::Signature,
    data_io::SetBFTData,
    network::data::Network,
    oneshot,
    party::{
        backup::ABFTBackup,
        manager::{Task, TaskCommon},
    },
    CurrentNetworkData, Hasher, Keychain, NodeIndex, SessionId, SignatureSet, UnitCreationDelay,
};

type WrappedNetwork<H, ADN> = NetworkWrapper<
    current_set_bft::NetworkData<Hasher, SetBFTData<H>, Signature, SignatureSet<Signature>>,
    ADN,
>;

pub fn run_member<UH, ADN>(
    subtask_common: TaskCommon,
    multikeychain: Keychain,
    config: Config,
    network: WrappedNetwork<UH, ADN>,
    data_provider: impl current_set_bft::DataProvider<Output = SetBFTData<UH>> + 'static,
    ordered_data_interpreter: impl current_set_bft::UnitFinalizationHandler<
        Data = SetBFTData<UH>,
        Hasher = Hasher,
    >,
    backup: ABFTBackup,
) -> Task
where
    UH: UnverifiedHeader,
    ADN: Network<CurrentNetworkData<UH>> + 'static,
{
    let TaskCommon {
        spawn_handle,
        session_id,
    } = subtask_common;
    let (stop, exit) = oneshot::channel();
    let member_terminator = Terminator::create_root(exit, "member");
    let local_io = LocalIO::new_with_unit_finalization_handler(
        data_provider,
        ordered_data_interpreter,
        backup.0,
        backup.1,
    );

    let task = {
        let spawn_handle = spawn_handle.clone();
        async move {
            debug!(target: "setbft-party", "Running the member task for {:?}", session_id);
            current_set_bft::run_session(
                config,
                local_io,
                network,
                multikeychain,
                spawn_handle,
                member_terminator,
            )
            .await;
            debug!(target: "setbft-party", "Member task stopped for {:?}", session_id);
        }
    };

    let handle = spawn_handle.spawn_essential("setbft/consensus_session_member", task);
    Task::new(handle, stop)
}

pub fn create_setbft_config(
    n_members: usize,
    node_id: NodeIndex,
    session_id: SessionId,
    unit_creation_delay: UnitCreationDelay,
) -> Config {
    let mut delay_config = default_delay_config();
    delay_config.unit_creation_delay = unit_creation_delay_fn(unit_creation_delay);
    match create_config(n_members.into(), node_id.into(), session_id.0 as u64, MAX_ROUNDS, delay_config, Duration::from_millis(SESSION_LEN_LOWER_BOUND_MS as u64)) {
        Ok(config) => config,
        Err(_) => panic!("Incorrect setting of delays. Make sure the total SetBFT session time is at least {} ms.", SESSION_LEN_LOWER_BOUND_MS),
    }
}
