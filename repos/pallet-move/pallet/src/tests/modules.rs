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
