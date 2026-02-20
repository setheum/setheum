use core::marker::PhantomData;

use codec::{FullCodec, FullEncode};
use frame_support::storage::StorageMap;
use move_vm_backend::storage::Storage;
use sp_std::prelude::*;

/// Move Virtual Machine storage trait used to represent the native storage.
pub trait MoveVmStorage<T, K: FullEncode, V: FullCodec> {
    type VmStorage;

    /// Create a new instance of the VM storage.
    fn move_vm_storage() -> StorageAdapter<Self::VmStorage, K, V>
    where
        Self::VmStorage: StorageMap<K, V, Query = Option<V>>,
    {
        Default::default()
    }
}

/// Vm storage adapter for native storage.
pub struct StorageAdapter<T, K = Vec<u8>, V = Vec<u8>>(PhantomData<(T, K, V)>);

/// Default trait VM storage adapter implementation
impl<T, K, V> Default for StorageAdapter<T, K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Storage trait implementation for the StorageAdapter struct.
impl<T: StorageMap<Vec<u8>, Vec<u8>, Query = Option<Vec<u8>>>> Storage
    for StorageAdapter<T, Vec<u8>, Vec<u8>>
{
    /// Get a value specified by key.
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        T::get(key)
    }

    /// Set (insert) a value specified by key.
    fn set(&self, key: &[u8], value: &[u8]) {
        T::insert(key, value)
    }

    /// Remove a value specified by key and the key itself.
    fn remove(&self, key: &[u8]) {
        T::remove(key)
    }
}
