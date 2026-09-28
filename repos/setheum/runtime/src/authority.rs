// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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

use crate::{
	AccountId, AccountIdConversion, AuthoritysOriginId, BadOrigin, BlockNumber, DispatchResult, EnsureRoot,
	EnsureRootOrHalfShuraCouncil, EnsureRootOrThreeFourthsShuraCouncil, EnsureRootOrHalfFinancialCouncil,
	EnsureRootOrOneThirdsTechnicalCommittee, EnsureRootOrTwoThirdsTechnicalCommittee, OneDay, RuntimeOrigin, SevenDays,
	TreasuryPalletId, OriginCaller, HOURS, 
};
pub use frame_support::traits::{schedule::Priority, EnsureOrigin, OriginTrait};
use frame_system::ensure_root;
use module_authority::EnsureDelayed;

pub struct AuthorityConfigImpl;
impl module_authority::AuthorityConfig<RuntimeOrigin, OriginCaller, BlockNumber> for AuthorityConfigImpl {
	fn check_schedule_dispatch(origin: RuntimeOrigin, _priority: Priority) -> DispatchResult {
		EnsureRoot::<AccountId>::try_origin(origin)
			.or_else(|o| EnsureRootOrHalfShuraCouncil::try_origin(o).map(|_| ()))
			.or_else(|o| EnsureRootOrHalfFinancialCouncil::try_origin(o).map(|_| ()))
			.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
	}

	fn check_fast_track_schedule(
		origin: RuntimeOrigin,
		_initial_origin: &OriginCaller,
		new_delay: BlockNumber,
	) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if new_delay < 12 * HOURS {
				EnsureRootOrTwoThirdsTechnicalCommittee::ensure_origin(origin)
					.map_or_else(|e| Err(e.into()), |_| Ok(()))
			} else {
				EnsureRootOrOneThirdsTechnicalCommittee::ensure_origin(origin)
					.map_or_else(|e| Err(e.into()), |_| Ok(()))
			}
		})
	}

	fn check_delay_schedule(origin: RuntimeOrigin, _initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			EnsureRootOrOneThirdsTechnicalCommittee::ensure_origin(origin).map_or_else(|e| Err(e.into()), |_| Ok(()))
		})
	}

	fn check_cancel_schedule(origin: RuntimeOrigin, initial_origin: &OriginCaller) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| {
			if origin.caller() == initial_origin
				|| EnsureRootOrThreeFourthsShuraCouncil::ensure_origin(origin).is_ok()
			{
				Ok(())
			} else {
				Err(BadOrigin.into())
			}
		})
	}
}

impl module_authority::AsOriginId<RuntimeOrigin, OriginCaller> for AuthoritysOriginId {
	fn into_origin(self) -> OriginCaller {
		match self {
			AuthoritysOriginId::Root => RuntimeOrigin::root().caller().clone(),
			AuthoritysOriginId::Treasury => RuntimeOrigin::signed(TreasuryPalletId::get().into_account_truncating()).caller().clone(),
			_ => unimplemented!("AuthoritysOriginId variant not supported"),
		}
	}

	fn check_dispatch_from(&self, origin: RuntimeOrigin) -> DispatchResult {
		ensure_root(origin.clone()).or_else(|_| match self {
			AuthoritysOriginId::Root => <EnsureDelayed<
				SevenDays,
				EnsureRootOrThreeFourthsShuraCouncil,
				BlockNumber,
				OriginCaller,
			> as EnsureOrigin<RuntimeOrigin>>::ensure_origin(origin)
			.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(())),
			AuthoritysOriginId::Treasury => {
				<EnsureDelayed<OneDay, EnsureRootOrHalfShuraCouncil, BlockNumber, OriginCaller> as EnsureOrigin<
					RuntimeOrigin,
				>>::ensure_origin(origin)
				.map_or_else(|_| Err(BadOrigin.into()), |_| Ok(()))
			}
			_ => Err(BadOrigin.into()),
		})
	}
}
