use crate::{mock::*, mock_utils as utils, no_type_args, script_transaction, Error};

use frame_support::{assert_err, assert_ok};
use move_core_types::{identifier::Identifier, language_storage::StructTag};

fn get_vm_resource(
    module_owner: &AccountAddress,
    module_id: &str,
    key_id: &str,
    address: &AccountId32,
) -> Result<Option<Vec<u8>>, anyhow::Error> {
    let tag = StructTag {
        address: *module_owner,
        module: Identifier::new(module_id)?,
        name: Identifier::new(key_id)?,
        type_params: vec![],
    };
    let bytes = bcs::to_bytes(&tag)?;
    let o_vec = MoveModule::get_resource(address, &bytes)
        .map_err(|e| anyhow::anyhow!("MoveModule::get_resource {e:?}"))?;
    Ok(o_vec)
}

/// Test execution of a script.
#[test]
fn execute_script_empty() {
    let addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let script = utils::read_script_from_project("move-basics", "empty_scr");

        let transaction_bc = script_transaction!(script, no_type_args!());

        let res = MoveModule::execute(
            RuntimeOrigin::signed(addr_native.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        );

        assert_ok!(res);

        let script = utils::read_script_from_project("move-basics", "empty_loop");

        let transaction_bc = script_transaction!(script, no_type_args!());

        let res = MoveModule::execute(
            RuntimeOrigin::signed(addr_native),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        );

        assert_ok!(res);
    });
}

/// Test execution of a script with parametrized function.
#[test]
fn execute_script_params() {
    let addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let script = utils::read_script_from_project("move-basics", "empty_loop_param");

        let transaction_bc = script_transaction!(script, no_type_args!(), &10u64);

        let res = MoveModule::execute(
            RuntimeOrigin::signed(addr_native),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        );

        assert_ok!(res);
    });
}

/// Test execution of a script with generic function which should fail since generic parameters
/// are not allowed.
#[test]
fn execute_script_generic_fails() {
    let addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let script = utils::read_script_from_project("move-basics", "generic_1");

        let transaction_bc = script_transaction!(script, no_type_args!(), &100u64);

        let res = MoveModule::execute(
            RuntimeOrigin::signed(addr_native),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        );

        assert_err!(res, Error::<Test>::InvalidMainFunctionSignature);
    });
}

/// Test execution of a script with correct parameters which stores something inside the storage.
#[test]
fn execute_script_storage_correct() {
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);
    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        // Bob publishes the move-module 'Counter', test preparation.
        let module = utils::read_module_from_project("get-resource", "Counter");
        assert_ok!(MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            module,
            MAX_GAS_AMOUNT,
        ));

        // Check that there are no counters available.
        assert!(
            get_vm_resource(&bob_addr_mv, "Counter", "Counter", &alice_addr_32)
                .unwrap()
                .is_none()
        );
        assert!(
            get_vm_resource(&bob_addr_mv, "Counter", "Counter", &bob_addr_32)
                .unwrap()
                .is_none()
        );

        // Alice and Bob execute a script to create a counter using the move-module 'Counter'.
        let script = utils::read_script_from_project("get-resource", "create_counter");
        let transaction_bc = script_transaction!(script.clone(), no_type_args!(), &alice_addr_mv);
        assert_ok!(MoveModule::execute(
            RuntimeOrigin::signed(alice_addr_32.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        ));

        let transaction_bc = script_transaction!(script, no_type_args!(), &bob_addr_mv);
        assert_ok!(MoveModule::execute(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        ));

        // Verify counter has been created.
        let counter = get_vm_resource(&bob_addr_mv, "Counter", "Counter", &alice_addr_32)
            .unwrap()
            .expect("couldn't find Alice's counter");
        assert_eq!(counter, vec![0, 0, 0, 0, 0, 0, 0, 0]);

        let counter = get_vm_resource(&bob_addr_mv, "Counter", "Counter", &bob_addr_32)
            .unwrap()
            .expect("couldn't find Bob's counter");
        assert_eq!(counter, vec![0, 0, 0, 0, 0, 0, 0, 0]);

        // Execute script that counts that created counter, but only for Alice.
        let script = utils::read_script_from_project("get-resource", "count");
        let transaction_bc = script_transaction!(script.clone(), no_type_args!(), &alice_addr_mv);
        assert_ok!(MoveModule::execute(
            RuntimeOrigin::signed(alice_addr_32.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        ));

        // Verify counter has been increased by 1.
        let counter = get_vm_resource(&bob_addr_mv, "Counter", "Counter", &alice_addr_32)
            .unwrap()
            .expect("couldn't find Alice's counter");
        assert_eq!(counter, vec![1, 0, 0, 0, 0, 0, 0, 0]);
        // Verify counter has still the same value.
        let counter = get_vm_resource(&bob_addr_mv, "Counter", "Counter", &bob_addr_32)
            .unwrap()
            .expect("couldn't find Bob's counter");
        assert_eq!(counter, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    });
}

/// Test execution of a script with correct parameters but with insufficient gas.
#[test]
fn execute_script_insufficient_gas() {
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        // Bob publishes the move-module 'Counter', test preparation.
        let module = utils::read_module_from_project("get-resource", "Counter");
        assert_ok!(MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            module,
            MAX_GAS_AMOUNT,
        ));

        // Bob wants to execute a script which shall trigger that module but with too little gas.
        let script = utils::read_script_from_project("get-resource", "create_counter");
        let transaction_bc = script_transaction!(script.clone(), no_type_args!(), &bob_addr_mv);
        assert!(MoveModule::execute(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            transaction_bc,
            10,
            0,
        )
        .is_err());
    });
}

/// Test execution of a script with corrupted bytecode.
#[test]
fn execute_script_corrupted_bytecode() {
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        // Bob publishes the move-module 'Counter', test preparation.
        let module = utils::read_module_from_project("get-resource", "Counter");
        assert_ok!(MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            module,
            MAX_GAS_AMOUNT,
        ));

        // Bob executes a corrupted script.
        let script = utils::read_script_from_project("get-resource", "create_counter");
        let mut transaction_bc = script_transaction!(script.clone(), no_type_args!(), &bob_addr_mv);
        transaction_bc[10] += 1;
        assert!(MoveModule::execute(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            0,
        )
        .is_err());
    });
}

// TODO(eiger): Test for a script that receives an invalid number of arguments and generates NumberOfArgumentsMismatch error.
