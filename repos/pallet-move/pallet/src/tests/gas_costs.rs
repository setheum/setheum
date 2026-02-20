//! This module is not a unit-test, it is rather a tool that wouldn't have fit in somehwere else.
//! It's purpose is to get some gas cost values for certain border cases of using Move-scripts.

use crate::{mock::*, mock_utils as utils, weight_info::WeightInfo, weights::SubstrateWeight, *};
use frame_support::weights::Weight;

#[test]
fn pseudo_benchmark_gas_costs() {
    let (bob_addr_32, bob_addr_move) = utils::account_n_address::<Test>(utils::BOB_ADDR);
    let (alice_addr_32, alice_addr_move) = utils::account_n_address::<Test>(utils::ALICE_ADDR);

    ExtBuilder::default().build().execute_with(|| {
        // Setup: load modules that are needed in our pseudo-benchmarks.
        let module = utils::read_module_from_project("car-wash-example", "CarWash");
        MoveModule::publish_module(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            module,
            MAX_GAS_AMOUNT,
        )
        .unwrap();
        let script = utils::read_script_from_project("car-wash-example", "initial_coin_minting");
        let script_bc = script_transaction!(script, no_type_args!(), &bob_addr_move);
        MoveModule::execute(
            RuntimeOrigin::signed(bob_addr_32.clone()),
            script_bc,
            MAX_GAS_AMOUNT,
            0,
        )
        .unwrap();
        let script = utils::read_script_from_project("car-wash-example", "register_new_user");
        let script_bc = script_transaction!(script, no_type_args!(), &alice_addr_move);
        MoveModule::execute(
            RuntimeOrigin::signed(alice_addr_32.clone()),
            script_bc,
            MAX_GAS_AMOUNT,
            0,
        )
        .unwrap();
        let module = utils::read_module_from_project("gas-costs", "TheModule");
        MoveModule::publish_module(RuntimeOrigin::signed(bob_addr_32), module, MAX_GAS_AMOUNT)
            .unwrap();

        // Short and cheap script.
        let script = utils::read_script_from_project("gas-costs", "short_cheap_script");
        let script_bc = script_transaction!(script, no_type_args!());
        let gas1: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight1: Weight = SubstrateWeight::<Test>::execute(gas1 as u32);

        // Short and expensive script.
        Balances::force_set_balance(
            RuntimeOrigin::root(),
            alice_addr_32.clone(),
            u64::MAX.into(),
        )
        .unwrap();
        let script = utils::read_script_from_project("gas-costs", "short_expensive_script");
        let script_bc = script_transaction!(script, no_type_args!(), &alice_addr_move);
        let gas2: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight2: Weight = SubstrateWeight::<Test>::execute(gas2 as u32);

        // Long and cheap script.
        let script = utils::read_script_from_project("gas-costs", "long_script");
        let script_bc = script_transaction!(script, no_type_args!(), &alice_addr_move, &true);
        let gas3: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight3: Weight = SubstrateWeight::<Test>::execute(gas3 as u32);

        // Long and expensive script.
        Balances::force_set_balance(
            RuntimeOrigin::root(),
            alice_addr_32.clone(),
            u64::MAX.into(),
        )
        .unwrap();
        let script = utils::read_script_from_project("gas-costs", "long_script");
        let script_bc = script_transaction!(script, no_type_args!(), &alice_addr_move, &false);
        let gas4: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight4: Weight = SubstrateWeight::<Test>::execute(gas4 as u32);

        // Simple math script that doesn't use a module.
        let script = utils::read_script_from_project("gas-costs", "uses_no_module");
        let script_bc = script_transaction!(script, no_type_args!());
        let gas5: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight5: Weight = SubstrateWeight::<Test>::execute(gas5 as u32);

        // Simple math script that uses a module to do the same.
        let script = utils::read_script_from_project("gas-costs", "uses_module");
        let script_bc = script_transaction!(script, no_type_args!());
        let gas6: u64 = MoveModule::rpc_estimate_gas_execute_script(script_bc)
            .unwrap()
            .gas_used;
        let weight6: Weight = SubstrateWeight::<Test>::execute(gas6 as u32);

        // Write all results to file "gas_costs.txt".
        let output = format!(
            "Short and cheap script:
    Gas:    {gas1:?}
    {weight1:?}

Small and expensive script:
    Gas:    {gas2:?}
    {weight2:?}

Long and cheap script:
    Gas:    {gas3:?}
    {weight3:?}

Long and expensive script:
    Gas:    {gas4:?}
    {weight4:?}

Simple math script, no module used:
    Gas:    {gas5:?}
    {weight5:?}

Simple math script, module used:
    Gas:    {gas6:?}
    {weight6:?}
",
        );
        std::fs::write("./../gas_costs.txt", output).unwrap();
    })
}
