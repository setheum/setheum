//! Helper methods to load move-project test assets. Constants and methods are sorted into
//! different modules for visibility reasons in depdency of enabled/disabled features.

pub(crate) use always_used::*;
#[cfg(test)]
pub(crate) use tests_only::*;

mod always_used {
    extern crate alloc;

    use frame_system::Config as SysConfig;
    use sp_core::crypto::Ss58Codec;

    use crate::Config;

    // Reusable constants for test accounts.
    pub const BOB_ADDR: &str = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
    pub const ALICE_ADDR: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

    /// Creates a native 32-byte address from a given ss58 string.
    pub fn account<T: SysConfig + Config>(ss58addr: &str) -> T::AccountId
    where
        T::AccountId: Ss58Codec,
    {
        T::AccountId::from_ss58check_with_version(ss58addr)
            .unwrap()
            .0
    }

    #[macro_export]
    macro_rules! script_transaction {
        ($bytecode:expr, $type_args:expr $(, $args:expr)*) => {
            {
                let transaction = ScriptTransaction {
                    bytecode: $bytecode,
                    type_args: $type_args,
                    args: vec![$(bcs::to_bytes($args).unwrap()),*],
                };
                bcs::to_bytes(&transaction).unwrap()
            }
        }
    }

    #[macro_export]
    macro_rules! no_type_args {
        () => {
            vec![]
        };
    }
}

#[cfg(test)]
mod tests_only {
    extern crate alloc;
    extern crate std;

    use alloc::format;
    use frame_system::Config as SysConfig;
    use sp_core::crypto::Ss58Codec;
    use sp_std::vec::Vec;

    pub use move_core_types::account_address::AccountAddress;

    use super::account;
    use crate::{Config, Pallet};

    // Reusable constants for test accounts.
    pub const DAVE_ADDR: &str = "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy";
    pub const EVE_ADDR: &str = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw";
    // equivalent to 0xCAFE
    pub const CAFE_ADDR: &str = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSv4fmh4G";
    // equivalent to 0x1
    pub const PROHIBITED_ADDR: &str = "gkKH52LJ2UumhVBim1n3mCsSj3ctj3GkV8JLVLdhJakxmEDcq";

    // Location of our assets folder.
    pub const MOVE_PROJECTS: &str = "src/assets/move-projects";

    /// Reads bytes from a file for the given path.
    /// Can panic if the file doesn't exist.
    pub fn read_bytes(file_path: &str) -> Vec<u8> {
        std::fs::read(file_path)
            .unwrap_or_else(|e| panic!("Can't read {file_path}: {e} - make sure you run pallet-move/pallet/src/assets/move-projects/smove-build-all.sh"))
    }

    /// Reads a precompiled Move scripts from our assets directory.
    pub fn read_script_from_project(project: &str, script_name: &str) -> Vec<u8> {
        let path =
            format!("{MOVE_PROJECTS}/{project}/build/{project}/bytecode_scripts/{script_name}.mv");
        read_bytes(&path)
    }

    /// Reads a precompiled Move module from our assets directory.
    pub fn read_module_from_project(project: &str, module_name: &str) -> Vec<u8> {
        let path =
            format!("{MOVE_PROJECTS}/{project}/build/{project}/bytecode_modules/{module_name}.mv");
        read_bytes(&path)
    }

    /// Reads a precompiled Move bundle from our assets directory.
    pub fn read_bundle_from_project(project: &str, bundle_name: &str) -> Vec<u8> {
        let path = format!("{MOVE_PROJECTS}/{project}/build/{project}/bundles/{bundle_name}.mvb");
        read_bytes(&path)
    }

    /// Creates a native 32-byte address and it's Move memory address by given ss58 string.
    pub fn account_n_address<T: SysConfig + Config>(ss58: &str) -> (T::AccountId, AccountAddress)
    where
        T::AccountId: Ss58Codec,
    {
        let addr_32 = account::<T>(ss58);
        let addr_mv = Pallet::<T>::to_move_address(&addr_32).unwrap();
        (addr_32, addr_mv)
    }
}
