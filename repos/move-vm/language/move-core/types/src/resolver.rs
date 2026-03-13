// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    language_storage::{ModuleId, StructTag},
    vm_status::StatusCode,
};
use alloc::vec::Vec;
use core::fmt::Debug;

/// Traits for resolving Move modules and resources from persistent storage

/// A persistent storage backend that can resolve modules by address + name.
/// Storage backends should return
///   - Ok(Some(..)) if the data exists
///   - Ok(None)     if the data does not exist
///   - Err(..)      only when something really wrong happens, for example
///                    - invariants are broken and observable from the storage side
///                      (this is not currently possible as ModuleId and StructTag
///                       are always structurally valid)
///                    - storage encounters internal error
pub trait ModuleResolver {
    type Error: Debug;

    fn get_module(&self, id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error>;
}

/// A persistent storage backend that can resolve resources by address + type
/// Storage backends should return
///   - Ok(Some(..)) if the data exists
///   - Ok(None)     if the data does not exist
///   - Err(..)      only when something really wrong happens, for example
///                    - invariants are broken and observable from the storage side
///                      (this is not currently possible as ModuleId and StructTag
///                       are always structurally valid)
///                    - storage encounters internal error
pub trait ResourceResolver {
    type Error: Debug;

    fn get_resource(
        &self,
        address: &AccountAddress,
        typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error>;
}

/// A balance backend that can resolve balance handling.
pub trait BalanceResolver {
    type Error: Into<StatusCode>;

    /// Resolver should update the inner state for the external balance handler.
    ///
    /// Once the MoveVM execution is complete, the data updated by this resolver should be used
    /// to execute the actual transfers.
    ///
    /// The cheque amount is the amount the account is allowed to transfer within the current execution
    /// context.
    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error>;

    /// Resolver should return the current cheque amount for a given address.
    ///
    /// The cheque amount is the amount the account is allowed to transfer within the current execution
    /// context.
    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error>;

    /// Resolver should return a total amount for a given address.
    ///
    /// The total amount is the amount the account owns.
    /// Account is allowed to transfer a bit of the total amount with the cheque.
    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error>;
}

/// A persistent storage implementation that can resolve both resources and modules
pub trait MoveResolver:
    ModuleResolver<Error = Self::Err>
    + ResourceResolver<Error = Self::Err>
    + BalanceResolver<Error = Self::StatusCodeErr>
{
    type Err: Debug;
    type StatusCodeErr: Into<StatusCode>;
}

impl<
        E: Debug,
        S: Into<StatusCode>,
        T: ModuleResolver<Error = E>
            + ResourceResolver<Error = E>
            + BalanceResolver<Error = S>
            + ?Sized,
    > MoveResolver for T
{
    type Err = E;
    type StatusCodeErr = S;
}

impl<T: ResourceResolver + ?Sized> ResourceResolver for &T {
    type Error = T::Error;

    fn get_resource(
        &self,
        address: &AccountAddress,
        tag: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        (**self).get_resource(address, tag)
    }
}

impl<T: ModuleResolver + ?Sized> ModuleResolver for &T {
    type Error = T::Error;
    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
        (**self).get_module(module_id)
    }
}

impl<T: BalanceResolver + ?Sized> BalanceResolver for &T {
    type Error = T::Error;
    fn transfer(
        &self,
        src: AccountAddress,
        dst: AccountAddress,
        cheque_amount: u128,
    ) -> Result<bool, Self::Error> {
        (**self).transfer(src, dst, cheque_amount)
    }
    fn cheque_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        (**self).cheque_amount(account)
    }
    fn total_amount(&self, account: AccountAddress) -> Result<u128, Self::Error> {
        (**self).total_amount(account)
    }
}

// Most existing tests won't need this Resolver so here's a quick solution for simple structs to make those test work.
#[macro_export]
macro_rules! quick_balance_resolver_impl {
    ($structname: ident, $error_type:tt) => {
        impl BalanceResolver for $structname {
            type Error = $error_type;

            fn transfer(
                &self,
                _src: AccountAddress,
                _dst: AccountAddress,
                _cheque_amount: u128,
            ) -> Result<bool, Self::Error> {
                unimplemented!("shouldn't be used");
            }

            fn cheque_amount(&self, _account: AccountAddress) -> Result<u128, Self::Error> {
                unimplemented!("shouldn't be used");
            }

            fn total_amount(&self, _account: AccountAddress) -> Result<u128, Self::Error> {
                unimplemented!("shouldn't be used");
            }
        }
    };
}
