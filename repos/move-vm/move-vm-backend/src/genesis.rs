//! Provides a configuration to prepare the initial MoveVM storage state.

use crate::balance::DummyBalanceHandler;
use crate::Mvm;
use crate::VmResult;
use crate::{storage::Storage, types::GasStrategy};
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt;
use hashbrown::HashMap;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_stdlib::{move_stdlib_bundle, substrate_stdlib_bundle};

/// Error codes for [`GenesisConfig`].
#[derive(Debug)]
pub enum GenesisConfigError {
    /// MoveVM initialization failure.
    MoveVmInitFailure,
    /// Publish bundle failure.
    PublishBundle(VmResult),
}

impl fmt::Display for GenesisConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MoveVmInitFailure => write!(f, "MoveVM failed to initalize"),
            Self::PublishBundle(vm_result) => write!(f, "Publish bundle failed: {:?}", vm_result),
        }
    }
}

/// Configuration to prepare the initial MoveVM storage state.
///
/// By default - a precompiled standard library is used from the `substrate-move` repository.
pub struct VmGenesisConfig {
    /// Move standard library.
    stdlib_bundle: Vec<u8>,
    /// An extended standard library for Substrate framework.
    substrate_stdlib_bundle: Vec<u8>,
    // - additional_bundles: Vec<Vec<u8>>,
    // - initial_script
}

impl Default for VmGenesisConfig {
    fn default() -> Self {
        Self {
            stdlib_bundle: move_stdlib_bundle().to_vec(),
            substrate_stdlib_bundle: substrate_stdlib_bundle().to_vec(),
        }
    }
}

impl VmGenesisConfig {
    /// Configure the Move standard library.
    pub fn configure_stdlib(&mut self, bundle: Vec<u8>) {
        self.stdlib_bundle = bundle;
    }

    /// Configure the extendend standard library made for Substrate.
    pub fn configure_substrate_stdlib(&mut self, bundle: Vec<u8>) {
        self.substrate_stdlib_bundle = bundle;
    }

    /// Apply the configuration to the storage.
    pub fn apply<S: Storage>(self, storage: S) -> Result<(), GenesisConfigError> {
        let storage_safe = StorageSafe::new(storage);
        let vm = Mvm::new(&storage_safe, DummyBalanceHandler {})
            .map_err(|_| GenesisConfigError::MoveVmInitFailure)?;

        let publish_under_stdaddr = |bundle: &[u8]| {
            let result =
                vm.publish_module_bundle(bundle, CORE_CODE_ADDRESS, GasStrategy::Unmetered);

            if !result.is_ok() {
                return Err(GenesisConfigError::PublishBundle(result));
            }
            Ok(())
        };

        publish_under_stdaddr(&self.stdlib_bundle)?;
        publish_under_stdaddr(&self.substrate_stdlib_bundle)?;

        // In case of the successful initialization, apply changes to the storage.
        storage_safe.apply_changes();

        Ok(())
    }
}

/// Storage safe keeps internal storage immutable until the changes are specificially applied.
struct StorageSafe<S: Storage> {
    /// A safe place for our storage.
    inner: S,
    /// Separate list of storage changesets.
    diff: RefCell<HashMap<Cow<'static, [u8]>, Option<Vec<u8>>>>,
}

impl<S: Storage> StorageSafe<S> {
    /// Creates a [`StorageSafe`].
    fn new(storage: S) -> StorageSafe<S> {
        StorageSafe {
            inner: storage,
            diff: RefCell::new(Default::default()),
        }
    }

    /// Finally applies internal changesets to the internal storage.
    fn apply_changes(self) {
        for (key, val) in self.diff.take() {
            match val {
                None => {
                    self.inner.remove(key.as_ref());
                }
                Some(val) => {
                    self.inner.set(key.as_ref(), val.as_ref());
                }
            }
        }
    }
}

impl<S: Storage> Storage for &StorageSafe<S> {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let diff = self.diff.borrow();

        if let Some(val) = diff.get(key) {
            val.to_owned()
        } else {
            self.inner.get(key)
        }
    }

    fn set(&self, key: &[u8], value: &[u8]) {
        let mut diff = self.diff.borrow_mut();

        diff.insert(Cow::Owned(key.to_vec()), Some(value.to_vec()));
    }

    fn remove(&self, key: &[u8]) {
        let mut diff = self.diff.borrow_mut();
        diff.insert(Cow::Owned(key.to_vec()), None);
    }
}
