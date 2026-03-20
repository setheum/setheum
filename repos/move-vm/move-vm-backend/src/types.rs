use alloc::string::String;
use alloc::vec::Vec;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_algebra::GasQuantity;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::StatusCode;
use move_vm_backend_common::gas_schedule::{
    INSTRUCTION_COST_TABLE, MILLIGAS_COST_PER_PUBLISHED_BYTE,
};
use move_vm_test_utils::gas_schedule::GasStatus;
use move_vm_types::gas::GasMeter;

/// Call type used to determine if we are calling script or function inside some module.
#[derive(Debug)]
pub enum Call {
    /// Script
    Script {
        /// Script bytecode.
        code: Vec<u8>,
    },
    /// Function in module with script viability.
    ScriptFunction {
        /// Module address.
        mod_address: AccountAddress,
        /// Module name.
        mod_name: Identifier,
        /// Function name - must be public and marked as `entry` in the module.
        func_name: Identifier,
    },
}

/// Transaction struct used in execute_script call.
#[derive(Debug)]
pub struct Transaction {
    /// Call type.
    pub call: Call,
    /// Type arguments.
    pub type_args: Vec<TypeTag>,
    /// Arguments of the call.
    pub args: Vec<Vec<u8>>,
}

/// Result of the execution.
#[derive(Debug)]
pub struct VmResult {
    /// Execution status code read from the MoveVM
    pub status_code: StatusCode,
    /// Optional error message.
    pub error_message: Option<String>,
    /// Gas used.
    pub gas_used: u64,
}

impl VmResult {
    /// Create a new VmResult.
    pub fn new(status_code: StatusCode, error_message: Option<String>, gas_used: u64) -> Self {
        Self {
            status_code,
            error_message,
            gas_used,
        }
    }

    /// Check if the execution was successful.
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.status_code == StatusCode::EXECUTED
    }

    /// Check if the execution failed.
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

/// Inner MoveVM gas handling multiplier.
///
/// Internally, MoveVM converts the input gas to gas units which are multiplied by this multiplier,
/// so in order to get used gas at the same input scale, we need to scale it back with this constant.
const INTERNAL_GAS_MULTIPLIER: u64 = 1000;

/// The maximum possible raw gas amount value.
pub const MAX_GAS_AMOUNT: u64 = u64::MAX / INTERNAL_GAS_MULTIPLIER;

/// Amount of gas.
#[derive(Debug, Clone, Copy)]
pub struct GasAmount(u64);

impl GasAmount {
    /// Creates a new [`GasAmount`] instance.
    ///
    /// Can return an error in case the provided value is too big.
    /// The maximum value is `2^64 / 1000`.
    pub fn new(value: u64) -> Result<Self, GasAmountError> {
        if value > MAX_GAS_AMOUNT {
            Err(GasAmountError::TooBig)
        } else {
            Ok(Self(value))
        }
    }

    /// Maximum possible [`GasAmount`] value.
    pub fn max() -> Self {
        Self(MAX_GAS_AMOUNT)
    }

    /// Inner gas amount value.
    pub fn inner(&self) -> u64 {
        self.0
    }
}

/// [`GasAmount`] errors.
#[derive(Debug, Clone, Copy)]
pub enum GasAmountError {
    /// Gas amount too big.
    TooBig,
}

/// Gas is a resource-fuel for executing Move scripts.
#[derive(Debug, Clone, Copy)]
pub enum GasStrategy {
    /// A metered gas with a provided limit.
    ///
    /// If the provided gas is not enough to execute the script or publish the script, then the
    /// MoveVM will return the out-of-gas error message.
    ///
    /// This should be the standard option for the MoveVM.
    Metered(GasAmount),
    /// It allows to run Move operations with an infinite amount of gas.
    ///
    /// This option should be used to estimate the required gas for the given MoveVM operation.
    DryRun,
    /// It allows to run the Move operations with the gas handling disabled.
    ///
    /// This option should be used only for testing and debugging purposes.
    Unmetered,
}

/// Internal gas handler.
pub(crate) struct GasHandler<'a> {
    /// Gas status is an input for the MoveVM which tracks spent gas.
    pub(crate) status: GasStatus<'a>,
    /// Dry run shouldn't make any changes to the MoveVM storage.
    pub(crate) dry_run: bool,
    /// An initial gas amount provided for metered gas strategy.
    starting_gas_amount: Option<u64>,
}

impl GasHandler<'_> {
    /// Constructs a new [`GasHandler`].
    pub(crate) fn new(strategy: GasStrategy) -> Self {
        let dry_run = matches!(strategy, GasStrategy::DryRun);
        let mut starting_gas_amount = None;

        let status = match strategy {
            GasStrategy::Metered(GasAmount(amount)) => {
                starting_gas_amount = Some(amount);
                GasStatus::new(&INSTRUCTION_COST_TABLE, amount.into())
            }
            GasStrategy::DryRun => {
                starting_gas_amount = Some(MAX_GAS_AMOUNT);
                GasStatus::new(&INSTRUCTION_COST_TABLE, MAX_GAS_AMOUNT.into())
            }
            GasStrategy::Unmetered => GasStatus::new_unmetered(),
        };

        Self {
            dry_run,
            status,
            starting_gas_amount,
        }
    }

    /// Charges write operations linearly according to the provided byte length.
    pub(crate) fn charge_publishing_to_storage(
        &mut self,
        num_bytes: usize,
    ) -> Result<(), VmResult> {
        let remaining_gas = self.status.remaining_gas();
        let amount = GasQuantity::new(num_bytes as u64 * MILLIGAS_COST_PER_PUBLISHED_BYTE);

        self.status.deduct_gas(amount).map_err(|e| VmResult {
            status_code: e.major_status(),
            error_message: None,
            gas_used: remaining_gas.into(),
        })
    }

    /// Calculates the used gas.
    pub(crate) fn gas_used(&self) -> u64 {
        let initial_gas = if let Some(amount) = self.starting_gas_amount {
            // Internally, the gas we provide to GasStatus gets multiplied by the multiplier.
            amount * INTERNAL_GAS_MULTIPLIER
        } else {
            return 0;
        };

        let remaining_gas: u64 = self.status.balance_internal().into();
        num_integer::div_ceil(initial_gas - remaining_gas, INTERNAL_GAS_MULTIPLIER)
    }
}
