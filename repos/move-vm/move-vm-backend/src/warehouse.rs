use crate::{balance::BalanceHandler, storage::Storage};
use alloc::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
    },
    vec::Vec,
};
use anyhow::{bail, Error, Result};
use core::ops::Deref;
use move_core_types::account_address::AccountAddress;
use move_core_types::effects::{
    ChangeSet,
    Op::{self, Delete, Modify, New},
};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::resolver::{BalanceResolver, ModuleResolver, ResourceResolver};
use move_core_types::vm_status::StatusCode;
use serde::{Deserialize, Serialize};

/// Structure holding account data which is held under one Move address
/// in Substrate storage).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct AccountData {
    /// Hashmap of the modules kept under this account.
    modules: BTreeMap<Identifier, Vec<u8>>,
    /// Hashmap of the resources kept under this account.
    resources: BTreeMap<StructTag, Vec<u8>>,
}

impl AccountData {
    fn apply_changes<K, V>(
        map: &mut BTreeMap<K, V>,
        changes: impl IntoIterator<Item = (K, Op<V>)>,
    ) -> Result<()>
    where
        K: Ord + core::fmt::Debug,
    {
        for (k, op) in changes.into_iter() {
            match (map.entry(k), op) {
                (Occupied(entry), New(_)) => {
                    bail!(
                        "Failed to apply changes -- key {:?} already exists",
                        entry.key()
                    )
                }
                (Occupied(entry), Delete) => {
                    entry.remove();
                }
                (Occupied(entry), Modify(val)) => {
                    *entry.into_mut() = val;
                }
                (Vacant(entry), New(val)) => {
                    entry.insert(val);
                }
                (Vacant(entry), Delete | Modify(_)) => bail!(
                    "Failed to apply changes -- key {:?} does not exist",
                    entry.key()
                ),
            }
        }
        Ok(())
    }
}

/// Move VM storage implementation for Substrate storage.
pub(crate) struct Warehouse<S: Storage, B: BalanceHandler> {
    /// Substrate storage implementing the Storage trait.
    storage: S,
    /// Balance handler which provides access to the external balance handling mechanism.
    balance_handler: B,
}

impl<S: Storage, B: BalanceHandler> Warehouse<S, B> {
    pub(crate) fn new(storage: S, balance_handler: B) -> Warehouse<S, B> {
        Self {
            storage,
            balance_handler,
        }
    }

    pub(crate) fn apply_changes(&self, changeset: ChangeSet) -> Result<()> {
        for (account, changeset) in changeset.into_inner() {
            let key = account.as_slice();
            let mut account = match self.storage.get(key) {
                Some(value) => bcs::from_bytes(&value).map_err(Error::msg)?,
                _ => AccountData::default(),
            };

            let (modules, resources) = changeset.into_inner();
            AccountData::apply_changes(&mut account.modules, modules)?;
            AccountData::apply_changes(&mut account.resources, resources)?;

            let account_bytes = bcs::to_bytes(&account).map_err(Error::msg)?;
            self.storage.set(key, &account_bytes);
        }

        Ok(())
    }
}

impl<S: Storage, B: BalanceHandler> Deref for Warehouse<S, B> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<S: Storage, B: BalanceHandler> ModuleResolver for Warehouse<S, B> {
    type Error = Error;

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        let raw_account = self.storage.get(module_id.address().as_slice());

        if let Some(raw_account) = raw_account {
            let mut account: AccountData = bcs::from_bytes(&raw_account).map_err(Error::msg)?;

            // Using remove to get the value since the account is already a copy of data from the storage.
            return Ok(account.modules.remove(module_id.name()));
        }

        // Even if the account is not found, we still return Ok(None) - it's not an error for MoveVM.
        Ok(None)
    }
}

impl<S: Storage, B: BalanceHandler> ResourceResolver for Warehouse<S, B> {
    type Error = Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let raw_account = self.storage.get(address.as_slice());

        if let Some(raw_account) = raw_account {
            let mut account: AccountData = bcs::from_bytes(&raw_account).map_err(Error::msg)?;

            // Using remove to get the value since the account is already a copy of data from the storage.
            return Ok(account.resources.remove(tag));
        }

        // Even if the account is not found, we still return Ok(None) - it's not an error for MoveVM.
        Ok(None)
    }
}

impl<S: Storage, B: BalanceHandler> BalanceResolver for Warehouse<S, B> {
    type Error = StatusCode;

    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error> {
        self.balance_handler
            .transfer(src, dst, cheque_amount)
            .map_err(Into::into)
    }

    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        self.balance_handler
            .cheque_amount(account)
            .map_err(Into::into)
    }

    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        self.balance_handler
            .total_amount(account)
            .map_err(Into::into)
    }
}
