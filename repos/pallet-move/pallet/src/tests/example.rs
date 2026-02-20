use crate::{mock::*, mock_utils as utils, no_type_args, script_transaction};

use frame_support::assert_ok;

const PROJECT: &str = "car-wash-example";

/// Test the regular, ideal flow of our example project.
#[test]
fn verify_normal_use_case() {
    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), 10_000_000_000_000)])
        .build()
        .execute_with(|| {
            // Check initial state of balances of involved users.
            let ini_blnc_alice = Balances::free_balance(&alice_addr_32);
            let ini_blnc_bob = Balances::free_balance(&bob_addr_32);

            // Let's publish Bob's module CarWash.
            let module_bc = utils::read_module_from_project(PROJECT, "CarWash");
            assert_ok!(MoveModule::publish_module(
                RuntimeOrigin::signed(bob_addr_32.clone()),
                module_bc,
                MAX_GAS_AMOUNT,
            ));

            // Now Bob initialises his module.
            let script = utils::read_script_from_project(PROJECT, "initial_coin_minting");
            let transaction_bc = script_transaction!(script, no_type_args!(), &bob_addr_mv);
            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(bob_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                0,
            ));

            // Now Alice comes over to wash her car for the first time...
            let script = utils::read_script_from_project(PROJECT, "register_new_user");
            let transaction_bc = script_transaction!(script, no_type_args!(), &alice_addr_mv);
            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                0,
            ));

            let script = utils::read_script_from_project(PROJECT, "buy_coin");
            let transaction_bc = script_transaction!(script, no_type_args!(), &alice_addr_mv, &1u8);
            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                COIN_PRICE,
            ));

            // let script_bc = script_bytecode("wash_car", alice_addr_mv);
            let script = utils::read_script_from_project(PROJECT, "wash_car");
            let transaction_bc = script_transaction!(script, no_type_args!(), &alice_addr_mv);
            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                0,
            ));

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            let now_blnc_bob = Balances::free_balance(&bob_addr_32);
            assert_eq!(ini_blnc_alice - COIN_PRICE, now_blnc_alice);
            assert_eq!(ini_blnc_bob + COIN_PRICE, now_blnc_bob);
        })
}
