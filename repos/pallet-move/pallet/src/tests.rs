//! Container module for all test modules.

mod address;
mod balance;
mod example;
mod execute;
#[cfg(feature = "gas-cost-measurement")]
mod gas_costs;
mod modules;
mod publish;
mod signer;
mod update_stdlib;
