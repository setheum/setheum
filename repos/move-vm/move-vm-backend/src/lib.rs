#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod balance;
pub mod genesis;
pub mod storage;
pub mod types;
mod warehouse;

use crate::storage::Storage;
use crate::types::{Call, Transaction, VmResult};
use crate::warehouse::Warehouse;
use alloc::{format, string::ToString, vec::Vec};
use anyhow::{anyhow, Error};
use balance::BalanceHandler;
use move_binary_format::{errors::VMResult, file_format::CompiledModule};
use move_core_types::{
    account_address::AccountAddress,
    effects::{ChangeSet, Event},
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag, CORE_CODE_ADDRESS},
    resolver::{ModuleResolver, ResourceResolver},
    vm_status::StatusCode,
};
use move_stdlib::natives::all_natives;
use move_vm_backend_common::{
    abi::ModuleAbi, gas_schedule::NATIVE_COST_PARAMS, types::ModuleBundle,
};
use move_vm_runtime::move_vm::MoveVM;
use types::{GasHandler, GasStrategy};

/// Main MoveVM structure, which is used to represent the virutal machine itself.
pub struct Mvm<S, B>
where
    S: Storage,
    B: BalanceHandler,
{
    // MoveVM instance - from move_vm_runtime crate
    vm: MoveVM,
    // Storage instance
    warehouse: Warehouse<S, B>,
}

impl<S, B> Mvm<S, B>
where
    S: Storage,
    B: BalanceHandler,
{
    /// Create a new Move VM with the given storage.
    pub fn new(storage: S, balance_handler: B) -> Result<Mvm<S, B>, Error> {
        Self::new_with_config(storage, balance_handler)
    }

    /// Create a new Move VM with the given storage and configuration.
    pub(crate) fn new_with_config(
        storage: S,
        balance_handler: B,
        // config: VMConfig,
    ) -> Result<Mvm<S, B>, Error> {
        Ok(Mvm {
            // TODO(rqnsom): see if we can avoid GAS_PARAMS cloning
            vm: MoveVM::new(all_natives(CORE_CODE_ADDRESS, NATIVE_COST_PARAMS.clone())).map_err(
                |err| {
                    let (code, _, msg, _, _, _, _) = err.all_data();
                    anyhow!("Error code:{:?}: msg: '{}'", code, msg.unwrap_or_default())
                },
            )?,
            warehouse: Warehouse::new(storage, balance_handler),
        })
    }

    /// Get module binary using the address and the name.
    pub fn get_module(
        &self,
        address: AccountAddress,
        name: &str,
    ) -> Result<Option<Vec<u8>>, Error> {
        let ident = Identifier::new(name)?;
        let module_id = ModuleId::new(address, ident);
        self.warehouse.get_module(&module_id)
    }

    /// Get module binary ABI using the address and the name.
    pub fn get_module_abi(
        &self,
        address: AccountAddress,
        name: &str,
    ) -> Result<Option<ModuleAbi>, Error> {
        if let Some(bytecode) = self.get_module(address, name)? {
            Ok(Some(ModuleAbi::from(
                CompiledModule::deserialize(&bytecode).map_err(Error::msg)?,
            )))
        } else {
            Ok(None)
        }
    }

    /// Get resource using an address and a tag.
    // TODO: could we use Identifier and AccountAddress here instead as arguments?
    pub fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &[u8],
    ) -> Result<Option<Vec<u8>>, Error> {
        let tag = bcs::from_bytes(tag).map_err(Error::msg)?;
        self.warehouse.get_resource(address, &tag)
    }

    /// Publish module into the storage. Module is published under the given address.
    pub fn publish_module(
        &self,
        module: &[u8],
        address: AccountAddress,
        gas: GasStrategy,
    ) -> VmResult {
        let mut gas_handler = GasHandler::new(gas);

        // MoveVM by default doesn't charge gas for publishing, so we need to do it manually here.
        if let Err(result) = gas_handler.charge_publishing_to_storage(module.len()) {
            return result;
        }

        let mut sess = self.vm.new_session(&self.warehouse);
        let result = sess.publish_module(module.to_vec(), address, &mut gas_handler.status);

        self.handle_result(result.and_then(|_| sess.finish()), gas_handler)
    }

    /// Publish a bundle of modules into the storage under the given address.
    pub fn publish_module_bundle(
        &self,
        bundle: &[u8],
        address: AccountAddress,
        gas: GasStrategy,
    ) -> VmResult {
        let mut gas_handler = GasHandler::new(gas);

        let modules = ModuleBundle::try_from(bundle)
            .map_err(|e| VmResult::new(StatusCode::UNKNOWN_MODULE, Some(e.to_string()), 0));

        let modules = match modules {
            Ok(modules) => modules.into_inner(),
            Err(e) => return e,
        };

        // MoveVM by default doesn't charge gas for publishing, so we need to do it manually here.
        if let Err(result) = gas_handler.charge_publishing_to_storage(bundle.len()) {
            return result;
        }

        let mut sess = self.vm.new_session(&self.warehouse);
        let result = sess.publish_module_bundle(modules, address, &mut gas_handler.status);

        self.handle_result(result.and_then(|_| sess.finish()), gas_handler)
    }

    /// Execute script using the given arguments (args).
    pub fn execute_script(
        &self,
        script: &[u8],
        type_args: Vec<TypeTag>,
        args: Vec<&[u8]>,
        gas: GasStrategy,
    ) -> VmResult {
        self.execute_script_worker(
            Transaction {
                call: Call::Script {
                    code: script.to_vec(),
                },
                type_args,
                args: args.iter().map(|x| x.to_vec()).collect(),
            },
            gas,
        )
    }

    /// Execute function from module using the given arguments (args).
    pub fn execute_function(
        &self,
        mod_address: AccountAddress,
        mod_name: Identifier,
        func_name: Identifier,
        type_args: Vec<TypeTag>,
        args: Vec<&[u8]>,
        gas: GasStrategy,
    ) -> VmResult {
        self.execute_script_worker(
            Transaction {
                call: Call::ScriptFunction {
                    mod_address,
                    mod_name,
                    func_name,
                },
                type_args,
                args: args.iter().map(|x| x.to_vec()).collect(),
            },
            gas,
        )
    }

    /// Execute script using the given arguments (args).
    fn execute_script_worker(&self, transaction: Transaction, gas: GasStrategy) -> VmResult {
        let mut gas_handler = GasHandler::new(gas);
        let mut sess = self.vm.new_session(&self.warehouse);

        let result = match transaction.call {
            Call::Script { code } => sess.execute_script(
                code,
                transaction.type_args,
                transaction.args,
                &mut gas_handler.status,
            ),
            Call::ScriptFunction {
                mod_address,
                mod_name,
                func_name,
            } => sess.execute_entry_function(
                &ModuleId::new(mod_address, mod_name),
                &func_name,
                transaction.type_args,
                transaction.args,
                &mut gas_handler.status,
            ),
        };

        self.handle_result(result.and_then(|_| sess.finish()), gas_handler)
    }

    fn handle_result(
        &self,
        result: VMResult<(ChangeSet, Vec<Event>)>,
        gas_handler: GasHandler,
    ) -> VmResult {
        match result {
            Ok((changeset, _)) => {
                let mut result = VmResult::new(StatusCode::EXECUTED, None, gas_handler.gas_used());

                // No storage update!
                if gas_handler.dry_run {
                    return result;
                }

                if let Err(e) = self.warehouse.apply_changes(changeset) {
                    result.status_code = StatusCode::STORAGE_ERROR;
                    result.error_message = Some(format!("Storage error: {}", e));
                }

                result
            }
            Err(err) => {
                let (status_code, _, msg, _, _, _, _) = err.all_data();
                VmResult::new(status_code, msg.clone(), 0)
            }
        }
    }
}
