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

use log::warn;
use sc_cli::arg_enums::SyncMode;

use crate::Cli;

/// Modifies the sync config to ensure only full sync is used.
pub struct SyncConfigValidator {
	overwritten: Option<SyncMode>,
}

impl SyncConfigValidator {
	/// Modifies the settings.
	pub fn process(cli: &mut Cli) -> Self {
		let overwritten = match cli.run.network_params.sync {
			SyncMode::Full => None,
			mode => Some(mode),
		};
		cli.run.network_params.sync = SyncMode::Full;
		SyncConfigValidator { overwritten }
	}

	/// Warns the user if they attempted to use a sync setting other than full.
	pub fn report(self) {
		if let Some(mode) = self.overwritten {
			warn!("Only full sync mode is supported, ignoring request for {:?} mode.", mode);
		}
	}
}
