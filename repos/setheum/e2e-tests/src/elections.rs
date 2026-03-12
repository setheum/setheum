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

use std::{collections::HashSet, iter::empty};

use setheum_client::{
    pallets::{
        committee_management::CommitteeManagementApi, elections::ElectionsApi, session::SessionApi,
        staking::StakingApi,
    },
    primitives::{CommitteeSeats, EraValidators},
    utility::{BlocksApi, SessionEraApi},
    AccountId, AsConnection,
};
use log::debug;
use primitives::SessionIndex;

pub async fn compute_session_committee<C: AsConnection + Sync>(
    connection: &C,
    session: SessionIndex,
) -> anyhow::Result<(Vec<AccountId>, Vec<AccountId>)> {
    let sessions_per_era = connection.get_session_per_era().await?;
    let era = connection.get_active_era_for_session(session).await?;
    let first_session = era * sessions_per_era;
    let first_block_in_era = connection.first_block_of_session(first_session).await?;

    let validators = connection
        .get_current_era_validators(first_block_in_era)
        .await;

    let committee = connection
        .get_session_committee(session, first_block_in_era)
        .await?
        .expect("Committee should be known at this point")
        .producers;

    Ok(committee
        .into_iter()
        .partition(|id| validators.reserved.contains(id)))
}

pub async fn get_and_test_members_for_session<C: AsConnection + Sync>(
    connection: &C,
    seats: CommitteeSeats,
    era_validators: &EraValidators<AccountId>,
    session: SessionIndex,
) -> anyhow::Result<(Vec<AccountId>, Vec<AccountId>)> {
    let (reserved_members_for_session, non_reserved_members_for_session) =
        compute_session_committee(connection, session).await?;
    let reserved_members_bench =
        get_bench_members(&era_validators.reserved, &reserved_members_for_session);
    let non_reserved_members_bench = get_bench_members(
        &era_validators.non_reserved,
        &non_reserved_members_for_session,
    );
    let members_bench = empty()
        .chain(reserved_members_bench)
        .chain(non_reserved_members_bench)
        .collect();

    let members_active: Vec<_> = empty()
        .chain(reserved_members_for_session)
        .chain(non_reserved_members_for_session)
        .collect();

    let members_active_set: HashSet<_> = members_active.iter().cloned().collect();
    let block = connection.first_block_of_session(session).await?;
    let network_members: HashSet<_> = connection.get_validators(block).await.into_iter().collect();

    debug!(
        "expected era validators for session {}: reserved - {:?}, non-reserved - {:?}",
        session, era_validators.reserved, era_validators.non_reserved
    );
    debug!("Seats for session {}: {:?}", session, seats);
    debug!(
        "members for session - computed {:?} ; retrieved {:?}",
        members_active, network_members
    );

    assert_eq!(members_active_set, network_members);

    Ok((members_active, members_bench))
}

fn get_bench_members(all_members: &[AccountId], members_active: &[AccountId]) -> Vec<AccountId> {
    all_members
        .iter()
        .filter(|account_id| !members_active.contains(account_id))
        .cloned()
        .collect()
}
