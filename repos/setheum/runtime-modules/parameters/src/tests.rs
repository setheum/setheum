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

//! Unit tests for the non-fungible-token module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use orml_traits::parameters::RuntimeParameterStore;

#[test]
fn set_parameters() {
	ExtBuilder::new().execute_with(|| {
		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key1),
			None
		);

		assert_noop!(
			ModuleParameters::set_parameter(
				RuntimeOrigin::signed(1),
				RuntimeParameters::Pallet1(pallet1::Parameters::Key1(pallet1::Key1, Some(123))),
			),
			DispatchError::BadOrigin
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::root(),
			RuntimeParameters::Pallet1(pallet1::Parameters::Key1(pallet1::Key1, Some(123))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key1),
			Some(123)
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::root(),
			RuntimeParameters::Pallet1(pallet1::Parameters::Key2(pallet1::Key2(234), Some(345))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key2(234)),
			Some(345)
		);

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key2(235)),
			None
		);

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet2::Parameters, _>(pallet2::Key3((1, 2))),
			None
		);

		assert_noop!(
			ModuleParameters::set_parameter(
				RuntimeOrigin::root(),
				RuntimeParameters::Pallet2(pallet2::Parameters::Key3(pallet2::Key3((1, 2)), Some(123))),
			),
			DispatchError::BadOrigin
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::signed(1),
			RuntimeParameters::Pallet2(pallet2::Parameters::Key3(pallet2::Key3((1, 2)), Some(456))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet2::Parameters, _>(pallet2::Key3((1, 2))),
			Some(456)
		);
	});
}
