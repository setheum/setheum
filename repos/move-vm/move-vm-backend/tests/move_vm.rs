//! Integration tests for our MoveVM backend.
//!
//! Note:
//! These test heavily depend on Move projects within tests/assets/move-projects.
//! Some of these tests use addresses that need to match the address in Move project files -
//! otherwise executing scripts or publishing won't work as expected.
//!
use crate::mock::BalanceMock;
use crate::mock::StorageMock;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use move_core_types::language_storage::CORE_CODE_ADDRESS as ADDR_STD;
use move_vm_backend::balance::BalanceHandler;
use move_vm_backend::genesis::VmGenesisConfig;
use move_vm_backend::types::GasAmount;
use move_vm_backend::Mvm;
use move_vm_backend_common::types::ModuleBundle;

use move_core_types::language_storage::TypeTag;
use move_vm_backend::types::GasStrategy;

pub mod mock;

/// Reads bytes from a file for the given path.
/// Can panic if the file doesn't exist.
fn read_bytes(file_path: &str) -> Vec<u8> {
    std::fs::read(file_path)
        .unwrap_or_else(|e| panic!("Can't read {file_path}: {e} - make sure you run move-vm-backend/tests/assets/move-projects/smove-build-all.sh"))
}

/// Reads a precompiled Move module from our assets directory.
fn read_module_bytes_from_project(project: &str, module_name: &str) -> Vec<u8> {
    const MOVE_PROJECTS: &str = "tests/assets/move-projects";

    let path =
        format!("{MOVE_PROJECTS}/{project}/build/{project}/bytecode_modules/{module_name}.mv");

    read_bytes(&path)
}

/// Reads a precompiled Move bundle from our assets directory.
fn read_bundle_from_project(project: &str, bundle_name: &str) -> Vec<u8> {
    const MOVE_PROJECTS: &str = "tests/assets/move-projects";

    let path = format!("{MOVE_PROJECTS}/{project}/build/{project}/bundles/{bundle_name}.mvb");

    read_bytes(&path)
}

/// Reads a precompiled Move scripts from our assets directory.
fn read_script_bytes_from_project(project: &str, script_name: &str) -> Vec<u8> {
    const MOVE_PROJECTS: &str = "tests/assets/move-projects";

    let path =
        format!("{MOVE_PROJECTS}/{project}/build/{project}/bytecode_scripts/{script_name}.mv");

    read_bytes(&path)
}

/// Estimate gas for published module / bundle.
#[inline]
fn estimate_gas_for_published_bytecode(bytecode: &[u8]) -> u64 {
    let raw_gas_cost = bytecode.len() as u64
        * move_vm_backend_common::gas_schedule::MILLIGAS_COST_PER_PUBLISHED_BYTE;
    num_integer::div_ceil(raw_gas_cost, 1000)
}

fn store_preloaded_with_genesis_cfg() -> StorageMock {
    let genesis_cfg = VmGenesisConfig::default();
    let store = StorageMock::new();

    // Publish the stdlib.
    assert!(
        genesis_cfg.apply(store.clone()).is_ok(),
        "genesis configuration failure"
    );
    store
}

#[test]
fn publish_module_test() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let address = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let module = read_module_bytes_from_project("empty", "Empty");

    // For the first case, use the maximum amount of gas.
    let provided_gas_amount = GasAmount::max();
    let gas = GasStrategy::Metered(provided_gas_amount);

    let result = vm.publish_module(&module, address, gas);

    let estimated_gas = estimate_gas_for_published_bytecode(&module);
    assert!(result.is_ok(), "failed to publish the module");
    assert_eq!(result.gas_used, estimated_gas, "invalid gas estimate");
    assert!(
        result.gas_used < provided_gas_amount.inner(),
        "invalid gas calulation"
    );

    // Prove that publishing will fail with insufficient gas.
    {
        let store = StorageMock::new();
        let vm = Mvm::new(store, BalanceMock::new()).unwrap();
        let gas = GasStrategy::Metered(GasAmount::new(estimated_gas).unwrap());
        let result = vm.publish_module(&module, address, gas);
        assert!(result.is_ok(), "failed to publish the module");
    }

    // Prove that publishing will succeeded with the exact amount of gas.
    {
        let store = StorageMock::new();
        let vm = Mvm::new(store, BalanceMock::new()).unwrap();
        let gas = GasStrategy::Metered(GasAmount::new(estimated_gas - 1).unwrap());
        let result = vm.publish_module(&module, address, gas);
        assert!(result.is_err(), "failed to publish the module");
    }
}

#[test]
fn publish_module_bundle_from_multiple_module_files() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let module_1 = read_module_bytes_from_project("using_stdlib_natives", "Vector");
    let module_2 = read_module_bytes_from_project("using_stdlib_natives", "DependsOnVector");
    let addr = AccountAddress::from_hex_literal("0x2").unwrap();

    // Order matters - module_2 depends on module_1!
    let modules = ModuleBundle::new(vec![module_1.clone(), module_2.clone()])
        .encode()
        .unwrap();
    let result = vm.publish_module_bundle(&modules, addr, gas);
    assert!(result.is_ok(), "failed to publish the bundle");

    // Recreate the storage and the MoveVM
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    // Order matters - we cannot publish module_2 before module_1!
    let modules = ModuleBundle::new(vec![module_2, module_1])
        .encode()
        .unwrap();
    let result = vm.publish_module_bundle(&modules, addr, gas);
    assert!(
        result.is_err(),
        "publishing a bundle with a wrong order succeeded"
    );
}

#[test]
fn publish_module_bundle_from_bundle_file() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let provided_gas_amount = GasAmount::max();
    let gas = GasStrategy::Metered(provided_gas_amount);

    let bundle = read_bundle_from_project("using_stdlib_natives", "using_stdlib_natives");
    let addr = AccountAddress::from_hex_literal("0x2").unwrap();

    let result = vm.publish_module_bundle(&bundle, addr, gas);
    assert!(result.is_ok(), "failed to publish the bundle");

    let estimated_gas = estimate_gas_for_published_bytecode(&bundle);
    assert_eq!(result.gas_used, estimated_gas, "invalid gas estimate");
    assert!(
        result.gas_used < provided_gas_amount.inner(),
        "invalid gas calulation"
    );
}

#[test]
fn publish_module_dependent_on_stdlib_natives() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let mod_using_stdlib_natives = read_module_bytes_from_project("using_stdlib_natives", "Vector");
    let addr_std_natives_user = AccountAddress::from_hex_literal("0x2").unwrap();

    // Natives are part of the MoveVM so no need to publish compiled stdlib bytecode modules.
    let result = vm.publish_module(&mod_using_stdlib_natives, addr_std_natives_user, gas);
    assert!(result.is_ok(), "the first module cannot be published");

    let mod_depends_on_using_stdlib_natives =
        read_module_bytes_from_project("depends_on__using_stdlib_natives", "VectorUser");
    let addr_testing_natives = AccountAddress::from_hex_literal("0x4").unwrap();

    let result = vm.publish_module(
        &mod_depends_on_using_stdlib_natives,
        addr_testing_natives,
        gas,
    );
    assert!(result.is_ok(), "the second module cannot be published");
}

#[test]
fn publish_module_using_stdlib_full_fails() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let mod_using_stdlib_natives =
        read_module_bytes_from_project("using_stdlib_full", "StringAndVector");
    let address = AccountAddress::from_hex_literal("0x3").unwrap();

    // In order to publish a module which is using the full stdlib bundle, we first must publish
    // the stdlib bundle itself to the MoveVM.
    let result = vm.publish_module(&mod_using_stdlib_natives, address, gas);
    assert!(result.is_err(), "the module shouldn't be published");
}

#[test]
fn genesis_config_inits_stdlib_so_stdlib_full_can_be_published() {
    let store = store_preloaded_with_genesis_cfg();

    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let module = read_module_bytes_from_project("using_stdlib_full", "StringAndVector");
    let address = AccountAddress::from_hex_literal("0x3").unwrap();

    let gas = GasStrategy::DryRun;
    let result = vm.publish_module(&module, address, gas);
    assert!(result.is_ok(), "failed to publish the module");
}

#[test]
fn get_module_and_module_abi() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let module = read_module_bytes_from_project("using_stdlib_natives", "Vector");
    let address = AccountAddress::from_hex_literal("0x2").unwrap();

    let gas = GasStrategy::Unmetered;
    let result = vm.publish_module(&module, address, gas);
    assert!(result.is_ok(), "failed to publish the module");

    let result = vm.get_module(address, "Vector");
    assert_eq!(
        result.expect("failed to get the module"),
        Some(module),
        "invalid module received"
    );

    let result = vm.get_module_abi(address, "Vector");
    assert!(result.unwrap().is_some(), "failed to get the module abi");
}

#[test]
fn get_resource() {
    let store = StorageMock::new();
    let gas = GasStrategy::Unmetered;

    // Publish the stdlib.
    let genesis_cfg = VmGenesisConfig::default();
    assert!(
        genesis_cfg.apply(store.clone()).is_ok(),
        "failed to apply the genesis configuration"
    );

    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    // Publish a module that can create resources.
    let cafe = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let module = read_module_bytes_from_project("basic_coin", "BasicCoin");
    let result = vm.publish_module(&module, cafe, gas);
    assert!(result.is_ok(), "failed to publish the module");

    let publish_basic_coin_for = |who| {
        let script = read_script_bytes_from_project("basic_coin", "publish_balance");
        let addr_param = bcs::to_bytes(&who).unwrap();
        let type_args: Vec<TypeTag> = vec![];
        let params: Vec<&[u8]> = vec![&addr_param];
        let result = vm.execute_script(&script, type_args, params, gas);
        assert!(result.is_ok(), "script execution failed for {who}");
    };

    let bob = AccountAddress::from_hex_literal("0xB0B").unwrap();
    publish_basic_coin_for(cafe);
    publish_basic_coin_for(bob);

    let get_basic_coin_resource_for = |who, module| {
        let tag = StructTag {
            address: module,
            module: Identifier::new("BasicCoin").unwrap(),
            name: Identifier::new("Balance").unwrap(),
            type_params: vec![],
        };
        // Check if the resource exists and is published on our address.
        vm.get_resource(&who, &bcs::to_bytes(&tag).unwrap())
            .unwrap()
    };

    // Make sure the resource can be published for different addresses.
    let cafe_resource = get_basic_coin_resource_for(cafe, cafe).expect("resource not found");
    let bob_resource = get_basic_coin_resource_for(bob, cafe).expect("resource not found");
    assert_eq!(
        bob_resource, cafe_resource,
        "failure: the amount of coins should be the same"
    );

    let mint_coins_to = |who, amount: u64| {
        let script = read_script_bytes_from_project("basic_coin", "mint_some");

        // We don't need to encapsulate it in Move::Signer token, since once serialized it shall
        // have the same 32-byte format.
        let module_owner_signer = bcs::to_bytes(&cafe).unwrap();
        let addr_param = bcs::to_bytes(&who).unwrap();
        let amount = bcs::to_bytes(&amount).unwrap();

        let type_args: Vec<TypeTag> = vec![];
        let params: Vec<&[u8]> = vec![&module_owner_signer, &addr_param, &amount];
        let result = vm.execute_script(&script, type_args, params, gas);
        assert!(result.is_ok(), "script execution failed for {who}");
    };

    mint_coins_to(cafe, 99999); // Cafe: I'm the main guy here!
    mint_coins_to(bob, 5); // Cafe: I dunno why you want my tokens, but here's some.

    let cafe_resource = get_basic_coin_resource_for(cafe, cafe).expect("resource not found");
    let bob_resource = get_basic_coin_resource_for(bob, cafe).expect("resource not found");
    assert_ne!(
        bob_resource, cafe_resource,
        "failure: the amount of coins shouldn't be the same"
    );

    // ---- an extra test case here ----
    // Make sure the non-existing resource actually doesn't exist.
    let tag = StructTag {
        address: cafe,
        module: Identifier::new("BasicCoin").unwrap(),
        name: Identifier::new("NonExisting").unwrap(),
        type_params: vec![],
    };
    let result = vm.get_resource(&cafe, &bcs::to_bytes(&tag).unwrap());

    // Check if the resource does not exist
    assert!(result.unwrap().is_none(), "resource found in the module");
}

#[test]
fn execute_script_with_no_params_test() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("simple_scripts", "empty_loop");

    let gas = GasStrategy::Unmetered;

    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_ok(), "failed to execute the script");
}

#[test]
fn execute_script_params_test() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("simple_scripts", "empty_loop_param");

    let gas = GasStrategy::Unmetered;

    let iter_count = bcs::to_bytes(&10u64).unwrap();
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![&iter_count];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_ok(), "failed to execute the script");
}

#[test]
fn execute_script_generics_test() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("simple_scripts", "generic_1");

    let gas = GasStrategy::Unmetered;

    let param = bcs::to_bytes(&100u64).unwrap();
    let type_args: Vec<TypeTag> = vec![TypeTag::U64];
    let params: Vec<&[u8]> = vec![&param];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_ok(), "failed to execute the script");

    // Execute once more but change param type
    let param = bcs::to_bytes(&true).unwrap();
    let type_args: Vec<TypeTag> = vec![TypeTag::Bool];
    let params: Vec<&[u8]> = vec![&param];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_ok(), "failed to execute the script");
}

#[test]
fn execute_script_generics_incorrect_params_test() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("simple_scripts", "generic_1");

    let gas = GasStrategy::Unmetered;

    // Execute with mismatched params
    let param = bcs::to_bytes(&true).unwrap();
    let type_args: Vec<TypeTag> = vec![TypeTag::U64];
    let params: Vec<&[u8]> = vec![&param];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_err(), "script execution should fail");

    // Execute with wrong params count
    let param = bcs::to_bytes(&true).unwrap();
    let type_args: Vec<TypeTag> = vec![TypeTag::U64, TypeTag::Bool];
    let params: Vec<&[u8]> = vec![&param];

    let result = vm.execute_script(&script, type_args, params, gas);

    assert!(result.is_err(), "script execution should fail");
}

#[test]
fn execute_function_test() {
    let store = StorageMock::new();
    let gas = GasStrategy::Unmetered;

    // Publish the stdlib.
    let genesis_cfg = VmGenesisConfig::default();
    assert!(
        genesis_cfg.apply(store.clone()).is_ok(),
        "failed to apply the genesis configuration"
    );

    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let address = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let module = read_module_bytes_from_project("basic_coin", "BasicCoin");
    let result = vm.publish_module(&module, address, gas);

    assert!(result.is_ok(), "failed to publish the module");

    let addr_param = bcs::to_bytes(&address).unwrap();
    let mod_name = Identifier::new("BasicCoin").unwrap();
    let func_name = Identifier::new("publish_balance").unwrap();

    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![&addr_param];
    let result = vm.execute_function(address, mod_name, func_name, type_args, params, gas);

    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn publishing_fails_with_insufficient_gas() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    // Try to publish a bundle
    {
        let stdlib = move_stdlib::move_stdlib_bundle();

        let gas = GasStrategy::Metered(GasAmount::new(1).unwrap());
        let result = vm.publish_module_bundle(stdlib, ADDR_STD, gas);

        assert!(
            result.is_err(),
            "publishing succeeded with insufficient amount of gas"
        );
    }

    // Try to publish a module
    {
        let address = AccountAddress::from_hex_literal("0xCAFE").unwrap();
        let module = read_module_bytes_from_project("empty", "Empty");

        let gas = GasStrategy::Metered(GasAmount::new(1).unwrap());
        let result = vm.publish_module(&module, address, gas);

        assert!(
            result.is_err(),
            "publishing succeeded with insufficient amount of gas"
        );
    }
}

#[test]
fn script_execution_fails_with_insufficient_gas() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let stdlib = move_stdlib::move_stdlib_bundle();
    let result = vm.publish_module_bundle(stdlib, ADDR_STD, gas);

    assert!(result.is_ok(), "failed to publish the stdlib bundle");

    let address = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let module = read_module_bytes_from_project("basic_coin", "BasicCoin");
    let result = vm.publish_module(&module, address, gas);

    assert!(result.is_ok(), "failed to publish the module");

    let addr_param = bcs::to_bytes(&address).unwrap();
    let mod_name = Identifier::new("BasicCoin").unwrap();
    let func_name = Identifier::new("publish_balance").unwrap();

    let gas = GasStrategy::Metered(GasAmount::new(1).unwrap());
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![&addr_param];
    let result = vm.execute_function(address, mod_name, func_name, type_args, params, gas);

    assert!(
        result.is_err(),
        "script execution succeeded with small amount of gas"
    );
}

#[test]
fn dry_run_gas_strategy_doesnt_update_storage() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let module = read_module_bytes_from_project("using_stdlib_natives", "Vector");
    let address = AccountAddress::from_hex_literal("0x2").unwrap();

    let gas = GasStrategy::DryRun;
    let result = vm.publish_module(&module, address, gas);
    assert!(result.is_ok(), "failed to publish the module");

    let estimated_gas = estimate_gas_for_published_bytecode(&module);
    assert_eq!(result.gas_used, estimated_gas, "invalid gas estimate");

    let result = vm.get_module(address, "Vector");
    assert_eq!(
        result.expect("failed to get the module"),
        None,
        "received module although the dry run strategy was enabled"
    );
}

#[test]
fn manually_publish_substrate_stdlib_bundle() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let stdlib = move_stdlib::substrate_stdlib_bundle();
    let result = vm.publish_module_bundle(stdlib, ADDR_STD, gas);
    assert!(result.is_ok(), "failed to publish the substrate stdlib");
}

#[test]
fn run_scipt_that_simply_tests_balance_api() {
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();
    let gas = GasStrategy::Unmetered;

    let script = read_script_bytes_from_project("substrate_balance", "balance_simple_api_test");

    let src = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let dst = AccountAddress::from_hex_literal("0x3EEE").unwrap();

    let amount = bcs::to_bytes(&0u128).unwrap();
    let src_addr = bcs::to_bytes(&src).unwrap();
    let dst_addr = bcs::to_bytes(&dst).unwrap();
    let params: Vec<&[u8]> = vec![&src_addr, &dst_addr, &amount];

    let type_args: Vec<TypeTag> = vec![];

    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "failed to execute the script");
}

#[test]
fn execute_transfer_script_and_check_balance_updates() {
    let store = store_preloaded_with_genesis_cfg();
    let mut balance = BalanceMock::new();
    let vm = Mvm::new(store, balance.clone()).unwrap();
    let gas = GasStrategy::Unmetered;

    let script = read_script_bytes_from_project("substrate_balance", "execute_transfer");

    let src = AccountAddress::from_hex_literal("0xCAFE").unwrap();
    let dst = AccountAddress::from_hex_literal("0x3EEE").unwrap();

    let amount = 10;
    balance.write_cheque(src, amount);

    // pre-transfer balance state
    assert_eq!(balance.cheque_amount(src).unwrap(), amount);
    assert_eq!(balance.cheque_amount(dst).unwrap(), 0);

    let amount_param = bcs::to_bytes(&amount).unwrap();
    let src_addr = bcs::to_bytes(&src).unwrap();
    let dst_addr = bcs::to_bytes(&dst).unwrap();
    let params: Vec<&[u8]> = vec![&src_addr, &dst_addr, &amount_param];

    // execute the transfer script
    let type_args: Vec<TypeTag> = vec![];
    let result = vm.execute_script(&script, type_args.clone(), params.clone(), gas);
    assert!(result.is_ok(), "failed to execute the script");

    // post-transfer balance state
    assert_eq!(balance.cheque_amount(src).unwrap(), 0);
    assert_eq!(balance.cheque_amount(dst).unwrap(), amount);

    // trying to re-run the script will fail since the cheque has been spent already
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(!result.is_ok(), "managed to execute the script");
}

#[test]
fn publish_module_with_base58_address() {
    let store = StorageMock::new();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let address = AccountAddress::from_hex_literal(
        "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
    )
    .unwrap();
    let module = read_module_bytes_from_project("base58_smove_build", "BobBase58");

    // For the first case, use the maximum amount of gas.
    let provided_gas_amount = GasAmount::max();
    let gas = GasStrategy::Metered(provided_gas_amount);

    let result = vm.publish_module(&module, address, gas);

    let estimated_gas = estimate_gas_for_published_bytecode(&module);
    assert!(result.is_ok(), "failed to publish the module");
    assert_eq!(result.gas_used, estimated_gas, "invalid gas estimate");
    assert!(
        result.gas_used < provided_gas_amount.inner(),
        "invalid gas calulation"
    );
}

#[test]
fn substrate_stdlib_sip_hash_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    // Here's the precalculation for the hash in the script.
    // use core::hash::Hasher;
    // let mut hasher = siphasher::sip::SipHasher::new();
    // let array: &[u8] = &[1, 2, 3];
    // hasher.write(&array);
    // println!("{}", hasher.finish());

    // tmp:
    let script = read_script_bytes_from_project("substrate_stdlib_hash", "sip_hash_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn substrate_stdlib_blake2b_256_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("substrate_stdlib_hash", "blake2b_256_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn substrate_stdlib_ripemd160_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("substrate_stdlib_hash", "ripemd160_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn substrate_stdlib_keccak256_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("substrate_stdlib_hash", "keccak256_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn substrate_stdlib_sha2_512_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("substrate_stdlib_hash", "sha2_512_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}

#[test]
fn substrate_stdlib_sha3_512_test() {
    let gas = GasStrategy::Unmetered;
    let store = store_preloaded_with_genesis_cfg();
    let vm = Mvm::new(store, BalanceMock::new()).unwrap();

    let script = read_script_bytes_from_project("substrate_stdlib_hash", "sha3_512_test");
    let type_args: Vec<TypeTag> = vec![];
    let params: Vec<&[u8]> = vec![];
    let result = vm.execute_script(&script, type_args, params, gas);
    assert!(result.is_ok(), "script execution failed");
}
