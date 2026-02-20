use crate::{mock::*, mock_utils as utils, GasStrategy};

use frame_support::assert_ok;

/// Test that the module is published correctly.
#[test]
fn publish_module_as_user_correct() {
    let cafe_addr_native = utils::account::<Test>(utils::CAFE_ADDR);
    let bob_addr_native = utils::account::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("move-basics", "Empty");

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(cafe_addr_native),
            module,
            MAX_GAS_AMOUNT,
        );
        assert_ok!(res);

        let module = utils::read_module_from_project("move-basics", "EmptyBob");

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_native),
            module,
            MAX_GAS_AMOUNT,
        );
        assert_ok!(res);
    });
}

/// Test that the module is not published if the user is not the owner.
#[test]
fn publish_module_as_user_wrong_user() {
    let bob_addr_native = utils::account::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("move-basics", "Empty");

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_native),
            module,
            MAX_GAS_AMOUNT,
        );
        assert!(res.is_err());
    });
}

/// Test that the module is not published if the user does not have enough gas.
#[test]
fn publish_module_as_user_insufficient_gas() {
    let cafe_addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("move-basics", "Empty");

        let gas_limit = 1;
        let res =
            MoveModule::publish_module(RuntimeOrigin::signed(cafe_addr_native), module, gas_limit);
        assert!(res.is_err());
    });
}

/// Test that the module is not published if the bytecode is corrupted.
#[test]
fn publish_module_as_user_corrupted_bytecode() {
    let cafe_addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let mut module = utils::read_module_from_project("move-basics", "Empty");

        // This should be enough to corrupt the bytecode.
        for i in 0..(module.len() / 4) {
            module[i] += 1;
        }

        let res = MoveModule::publish_module(
            RuntimeOrigin::signed(cafe_addr_native),
            module,
            MAX_GAS_AMOUNT,
        );
        assert!(res.is_err());
    });
}

/// Test that the bundle is published correctly.
#[test]
fn publish_bundle_as_user_correct() {
    let bob_addr_native = utils::account::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let bundle =
            utils::read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");

        let res = MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(bob_addr_native),
            bundle,
            MAX_GAS_AMOUNT,
        );

        assert_ok!(res);
    });
}

/// Test that the bundle is not published if the user is not the owner.
#[test]
fn publish_bundle_as_user_wrong_user() {
    let cafe_addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let bundle =
            utils::read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");

        let res = MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(cafe_addr_native),
            bundle,
            MAX_GAS_AMOUNT,
        );

        assert!(res.is_err());
    });
}

/// Test that the bundle is not published if the user does not have enough gas.
#[test]
fn publish_bundle_as_user_insufficient_gas() {
    let bob_addr_native = utils::account::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let bundle =
            utils::read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");

        let gas_limit = 1;
        let res = MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(bob_addr_native),
            bundle,
            gas_limit,
        );

        assert!(res.is_err());
    });
}

/// Test that the bundle is not published if the bytecode is corrupted.
#[test]
fn publish_bundle_as_user_corrupted_bytecode() {
    let cafe_addr_native = utils::account::<Test>(utils::CAFE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let mut bundle =
            utils::read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");

        // This should be enough to corrupt the bytecode.
        for i in 0..(bundle.len() / 4) {
            bundle[i] += 1;
        }

        let res = MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(cafe_addr_native),
            bundle,
            MAX_GAS_AMOUNT,
        );
        assert!(res.is_err());
    });
}

/// Test that the module is published correctly when the gas is estimated.
#[test]
fn raw_publish_module_dry_run() {
    let (bob_addr_native, bob_addr_move) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("using_stdlib_natives", "Vector");

        let estimation =
            MoveModule::raw_publish_module(&bob_addr_move, module.clone(), GasStrategy::DryRun)
                .expect("failed to publish a module")
                .gas_used as u32;

        let insufficient_gas = estimation - 1;
        let invalid_publish = MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_native.clone()),
            module.clone(),
            insufficient_gas,
        );
        assert!(
            invalid_publish.is_err(),
            "managed to publish a module with insufficient gas"
        );

        // Use the exact amount of gas.
        MoveModule::publish_module(RuntimeOrigin::signed(bob_addr_native), module, estimation)
            .expect("failed to publish a module");
    });
}

/// Test that the bundle is published correctly when the gas is estimated.
#[test]
fn raw_publish_bundle_dry_run() {
    let (bob_addr_native, bob_addr_move) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let bundle =
            utils::read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");

        let estimation =
            MoveModule::raw_publish_bundle(&bob_addr_move, bundle.clone(), GasStrategy::DryRun)
                .expect("failed to publish a bundle")
                .gas_used as u32;

        let insufficient_gas = estimation - 1;
        let invalid_publish = MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_native.clone()),
            bundle.clone(),
            insufficient_gas,
        );
        assert!(
            invalid_publish.is_err(),
            "managed to publish a bundle with insufficient gas"
        );

        // Use the exact amount of gas.
        MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(bob_addr_native),
            bundle,
            estimation,
        )
        .expect("failed to publish a bundle");
    });
}

/// Test that a module cannot be published in the protected memory address area (0x1).
#[test]
fn publish_module_will_fail_in_protected_memory() {
    let bad_addr_native = utils::account::<Test>(utils::PROHIBITED_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("move-basics", "Prohibited");

        let result = MoveModule::publish_module(
            RuntimeOrigin::signed(bad_addr_native),
            module,
            MAX_GAS_AMOUNT,
        );
        assert!(
            result.is_err(),
            "managed to publish a module in the protected memory area"
        );
    })
}

#[test]
fn publish_bundle_will_fail_in_protected_memory() {
    let bad_addr_native = utils::account::<Test>(utils::PROHIBITED_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let bundle = utils::read_bundle_from_project("prohibited-bundle", "prohibited-bundle");

        let result = MoveModule::publish_module_bundle(
            RuntimeOrigin::signed(bad_addr_native),
            bundle,
            MAX_GAS_AMOUNT,
        );
        assert!(
            result.is_err(),
            "managed to publish a bundle in the protected memory area"
        );
    })
}

#[test]
fn publish_module_with_base58_address() {
    let bob_addr_32 = utils::account::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let module = utils::read_module_from_project("base58_smove_build", "BobBase58");

        MoveModule::publish_module(RuntimeOrigin::signed(bob_addr_32), module, MAX_GAS_AMOUNT)
            .unwrap();
    })
}
