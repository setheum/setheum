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

use crate::{mock::*, mock_utils as utils};

use frame_support::assert_ok;
use move_core_types::{identifier::Identifier, language_storage::StructTag};

/// Test getting a module.
#[test]
fn get_module_correct() {
    let addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module_name = "Empty";
        let module = utils::read_module_from_project("move-basics", module_name);

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(addr_native.clone()),
            module.clone(),
            MAX_GAS_AMOUNT,
        );

        assert_ok!(res);

        let res = MoveModule::get_module(&addr_native, module_name);

        assert_eq!(res, Ok(Some(module)));
    });
}

/// Test getting a module that does not exist.
#[test]
fn get_module_nonexistent() {
    let addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let res = MoveModule::get_module(&addr_native, "Empty");

        assert_eq!(res, Ok(None));
    });
}

/// Test getting resource from the module.
#[test]
fn get_resource_non_existent() {
    let (_, addr) = utils::account_n_address::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let addr_native = MoveModule::to_native_account(&addr).unwrap();

        let module = utils::read_module_from_project("move-basics", "Empty");

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(addr_native.clone()),
            module,
            MAX_GAS_AMOUNT,
        );

        assert_ok!(res);

        let tag = StructTag {
            address: addr,
            module: Identifier::new("Empty").unwrap(),
            name: Identifier::new("NonExistentStruct").unwrap(),
            type_params: vec![],
        };

        let res = MoveModule::get_resource(&addr_native, &bcs::to_bytes(&tag).unwrap());

        assert_eq!(res, Ok(None));
    });
}
