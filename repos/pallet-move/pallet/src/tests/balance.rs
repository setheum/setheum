use crate::{
    balance::{BalanceAdapter, BalanceOf},
    mock::*,
    mock_utils as utils, no_type_args, script_transaction,
};

use frame_support::assert_ok;
use move_vm_backend::balance::BalanceHandler;

#[test]
fn verify_get_balance() {
    const AMOUNT: u128 = EXISTENTIAL_DEPOSIT + 100;

    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(bob_addr_32.clone(), AMOUNT)])
        .build()
        .execute_with(|| {
            // Check the pallet side first.
            let balance: BalanceAdapter<Test> = BalanceAdapter::new();
            assert_eq!(balance.total_amount(bob_addr_mv).unwrap(), AMOUNT);

            // Now check that it works from within the MoveVM.
            let script = utils::read_script_from_project("balance", "verify_preconfigured_balance");

            let transaction_bc =
                script_transaction!(script, no_type_args!(), &bob_addr_mv, &AMOUNT);

            let res = MoveModule::execute(
                RuntimeOrigin::signed(bob_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                0,
            );

            assert_ok!(res);
        })
}

#[test]
fn verify_simple_transfer() {
    const AMOUNT: u128 = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), 10_000)])
        .build()
        .execute_with(|| {
            // Check initial state of balances of involved users.
            let ini_blnc_alice = Balances::free_balance(&alice_addr_32);
            let ini_blnc_bob = Balances::free_balance(&bob_addr_32);

            // Now check that it works from within the MoveVM.
            let script = utils::read_script_from_project("balance", "single_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &bob_addr_mv,
                &AMOUNT
            );

            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                1500,
            ));

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            let now_blnc_bob = Balances::free_balance(&bob_addr_32);
            assert_eq!(ini_blnc_alice - AMOUNT, now_blnc_alice);
            assert_eq!(ini_blnc_bob + AMOUNT, now_blnc_bob);
        })
}

#[test]
fn verify_multiple_transfers_different() {
    const AMOUNT: u128 = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);
    let (dave_addr_32, dave_addr_mv) = utils::account_n_address::<Test>(utils::DAVE_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), 10_000)])
        .build()
        .execute_with(|| {
            // Check initial state of balances of involved users.
            let ini_blnc_alice = Balances::free_balance(&alice_addr_32);
            let ini_blnc_bob = Balances::free_balance(&bob_addr_32);
            let ini_blnc_dave = Balances::free_balance(&dave_addr_32);

            // Now check that it works from within the MoveVM.
            let script = utils::read_script_from_project("balance", "double_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &bob_addr_mv,
                &AMOUNT,
                &dave_addr_mv,
                &AMOUNT
            );

            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                1500,
            ));

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            let now_blnc_bob = Balances::free_balance(&bob_addr_32);
            let now_blnc_dave = Balances::free_balance(&dave_addr_32);
            assert_eq!(ini_blnc_alice - AMOUNT * 2, now_blnc_alice);
            assert_eq!(ini_blnc_bob + AMOUNT, now_blnc_bob);
            assert_eq!(ini_blnc_dave + AMOUNT, now_blnc_dave);
        })
}

#[test]
fn verify_multiple_transfers_same() {
    const AMOUNT: u128 = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), 10_000)])
        .build()
        .execute_with(|| {
            // Check initial state of balances of involved users.
            let ini_blnc_alice = Balances::free_balance(&alice_addr_32);
            let ini_blnc_bob = Balances::free_balance(&bob_addr_32);

            // Now check that it works from within the MoveVM.
            let script = utils::read_script_from_project("balance", "double_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &bob_addr_mv,
                &AMOUNT,
                &bob_addr_mv,
                &AMOUNT
            );

            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                1500,
            ));

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            let now_blnc_bob = Balances::free_balance(&bob_addr_32);
            assert_eq!(ini_blnc_alice - AMOUNT * 2, now_blnc_alice);
            assert_eq!(ini_blnc_bob + AMOUNT * 2, now_blnc_bob);
        })
}

#[test]
fn verify_balance_limit_too_low() {
    const AMOUNT: BalanceOf<Test> = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (_, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), 10_000)])
        .build()
        .execute_with(|| {
            // Now check that it works from within the MoveVM.
            let script = utils::read_script_from_project("balance", "single_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &bob_addr_mv,
                &AMOUNT
            );

            assert!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                AMOUNT - 1,
            )
            .is_err());
        })
}

#[test]
fn verify_insufficient_balance() {
    const AMOUNT: BalanceOf<Test> = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (_, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        let script = utils::read_script_from_project("balance", "single_transfer");

        let transaction_bc = script_transaction!(
            script,
            no_type_args!(),
            &bob_addr_mv,
            &alice_addr_mv,
            &AMOUNT
        );

        assert!(MoveModule::execute(
            RuntimeOrigin::signed(alice_addr_32.clone()),
            transaction_bc,
            MAX_GAS_AMOUNT,
            AMOUNT,
        )
        .is_err());
    })
}

#[test]
fn verify_move_script_fails_after_successful_transfer() {
    const BALANCE: BalanceOf<Test> = 1000;
    const AMOUNT: BalanceOf<Test> = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);
    let (bob_addr_32, bob_addr_mv) = utils::account_n_address::<Test>(utils::BOB_ADDR);

    ExtBuilder::default()
        .with_balances(vec![
            (alice_addr_32.clone(), BALANCE),
            (bob_addr_32.clone(), BALANCE),
        ])
        .build()
        .execute_with(|| {
            // Execute script with a successful transfer but which fails after transfer.
            let script = utils::read_script_from_project("balance", "fail_at_the_end");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &bob_addr_mv,
                &AMOUNT
            );

            // Expect error because script will fail at the end.
            assert!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                AMOUNT,
            )
            .is_err());

            // Verify balances have not been modified and transfer was not applied.
            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            let now_blnc_bob = Balances::free_balance(&bob_addr_32);
            assert_eq!(now_blnc_alice, BALANCE);
            assert_eq!(now_blnc_bob, BALANCE);
        })
}

#[test]
fn verify_self_transfer() {
    const AMOUNT: BalanceOf<Test> = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), AMOUNT * 2)])
        .build()
        .execute_with(|| {
            let script = utils::read_script_from_project("balance", "single_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &alice_addr_mv,
                &AMOUNT
            );

            assert_ok!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                AMOUNT,
            ));

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            assert_eq!(now_blnc_alice, AMOUNT * 2);
        })
}

#[test]
fn verify_self_transfer_trying_to_cheat() {
    const AMOUNT: BalanceOf<Test> = 1000;
    const BALANCE: BalanceOf<Test> = 100;

    let (alice_addr_32, alice_addr_mv) = utils::account_n_address::<Test>(utils::ALICE_ADDR);

    ExtBuilder::default()
        .with_balances(vec![(alice_addr_32.clone(), BALANCE)])
        .build()
        .execute_with(|| {
            let script = utils::read_script_from_project("balance", "single_transfer");

            let transaction_bc = script_transaction!(
                script,
                no_type_args!(),
                &alice_addr_mv,
                &alice_addr_mv,
                &AMOUNT
            );

            assert!(MoveModule::execute(
                RuntimeOrigin::signed(alice_addr_32.clone()),
                transaction_bc,
                MAX_GAS_AMOUNT,
                AMOUNT,
            )
            .is_err());

            let now_blnc_alice = Balances::free_balance(&alice_addr_32);
            assert_eq!(now_blnc_alice, BALANCE);
        })
}
