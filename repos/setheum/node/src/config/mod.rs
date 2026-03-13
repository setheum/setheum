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

use crate::Cli;

mod pruning;
mod sync;

use pruning::PruningConfigValidator;
use sync::SyncConfigValidator;

/// Validate and modify the configuration to make it conform to our assumptions.
pub struct Validator {
	pruning: PruningConfigValidator,
	sync: SyncConfigValidator,
}

impl Validator {
	/// Modifies the settings.
	pub fn process(cli: &mut Cli) -> Self {
		Validator { pruning: PruningConfigValidator::process(cli), sync: SyncConfigValidator::process(cli) }
	}

	/// Warns the user about the modified settings.
	pub fn report(self) {
		let Validator { pruning, sync } = self;
		pruning.report();
		sync.report();
	}
}
