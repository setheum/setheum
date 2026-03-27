//! List of RPC commands.

pub(super) mod estimate_gas_execute;
pub(super) mod estimate_gas_publish;
pub(super) mod get_module_abi;

use move_core_types::vm_status::StatusCode;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Default, Copy, Clone, Deserialize)]
#[allow(unused)]
struct Weight {
    /// The weight of computational time used based on some reference hardware.
    ref_time: u64,
    /// The weight of storage space used by proof of validity.
    proof_size: u64,
}

/// Gas estimation information.
#[derive(Deserialize)]
struct Estimation {
    /// Gas used.
    gas_used: u64,
    /// Status code for the MoveVM execution.
    vm_status_code: StatusCode,
    /// Substrate weight required for the complete extrinsic cost combined with the variable
    /// gas indicated in the `Estimation` struct.
    total_weight_including_gas_used: Weight,
}

impl fmt::Display for Estimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (gas_used, weight) = if matches!(self.vm_status_code, StatusCode::EXECUTED) {
            (self.gas_used, self.total_weight_including_gas_used)
        } else {
            (0, Weight::default())
        };

        write!(
            f,
            "Gas estimation:
    used gas: {gas_used}
    total extrinsic weight cost with the above gas:
    \t{weight:?}
    vm_status_code: {:?}",
            self.vm_status_code
        )
    }
}
