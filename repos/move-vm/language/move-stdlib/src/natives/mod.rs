// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod balance;
pub mod bcs;
pub mod debug;
pub mod event;
pub mod hash;
pub mod signer;
pub mod string;
pub mod substrate_hash;
pub mod type_name;
#[cfg(feature = "testing")]
pub mod unit_test;
pub mod vector;

mod helpers;

use alloc::string::ToString;
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub bcs: bcs::GasParameters,
    pub hash: hash::GasParameters,
    pub signer: signer::GasParameters,
    pub string: string::GasParameters,
    pub type_name: type_name::GasParameters,
    pub vector: vector::GasParameters,
    pub balance: balance::GasParameters,
    pub substrate_hash: substrate_hash::GasParameters,

    #[cfg(feature = "testing")]
    pub unit_test: unit_test::GasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            bcs: bcs::GasParameters {
                to_bytes: bcs::ToBytesGasParameters {
                    per_byte_serialized: 0.into(),
                    legacy_min_output_size: 0.into(),
                    failure: 0.into(),
                },
            },

            hash: hash::GasParameters {
                sha2_256: hash::Sha2_256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                    legacy_min_input_len: 0.into(),
                },
                sha3_256: hash::Sha3_256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                    legacy_min_input_len: 0.into(),
                },
            },
            type_name: type_name::GasParameters {
                get: type_name::GetGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            signer: signer::GasParameters {
                borrow_address: signer::BorrowAddressGasParameters { base: 0.into() },
            },
            string: string::GasParameters {
                check_utf8: string::CheckUtf8GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                is_char_boundary: string::IsCharBoundaryGasParameters { base: 0.into() },
                sub_string: string::SubStringGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                index_of: string::IndexOfGasParameters {
                    base: 0.into(),
                    per_byte_pattern: 0.into(),
                    per_byte_searched: 0.into(),
                },
            },
            vector: vector::GasParameters {
                empty: vector::EmptyGasParameters { base: 0.into() },
                length: vector::LengthGasParameters { base: 0.into() },
                push_back: vector::PushBackGasParameters {
                    base: 0.into(),
                    legacy_per_abstract_memory_unit: 0.into(),
                },
                borrow: vector::BorrowGasParameters { base: 0.into() },
                pop_back: vector::PopBackGasParameters { base: 0.into() },
                destroy_empty: vector::DestroyEmptyGasParameters { base: 0.into() },
                swap: vector::SwapGasParameters { base: 0.into() },
            },
            balance: balance::GasParameters {
                transfer: balance::TransferGasParameters { base: 0.into() },
                cheque_amount: balance::ChequeAmountGasParameters { base: 0.into() },
                total_amount: balance::TotalAmountGasParameters { base: 0.into() },
            },
            substrate_hash: substrate_hash::GasParameters {
                sip_hash: substrate_hash::SipHashGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                blake2b_256: substrate_hash::Blake2b256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                ripemd160: substrate_hash::Ripemd160HashGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                keccak256: substrate_hash::Keccak256HashGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                sha2_512: substrate_hash::Sha2_512GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                sha3_512: substrate_hash::Sha3_512GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            #[cfg(feature = "testing")]
            unit_test: unit_test::GasParameters {
                create_signers_for_testing: unit_test::CreateSignersForTestingGasParameters {
                    base_cost: 0.into(),
                    unit_cost: 0.into(),
                },
            },
        }
    }
}

pub fn all_natives(
    move_std_addr: AccountAddress,
    gas_params: GasParameters,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives!("bcs", bcs::make_all(gas_params.bcs));
    add_natives!("hash", hash::make_all(gas_params.hash));
    add_natives!("signer", signer::make_all(gas_params.signer));
    add_natives!("string", string::make_all(gas_params.string));
    add_natives!("type_name", type_name::make_all(gas_params.type_name));
    add_natives!("vector", vector::make_all(gas_params.vector));
    add_natives!("balance", balance::make_all(gas_params.balance));
    add_natives!(
        "substrate_hash",
        substrate_hash::make_all(gas_params.substrate_hash)
    );
    #[cfg(feature = "testing")]
    {
        add_natives!("unit_test", unit_test::make_all(gas_params.unit_test));
    }

    make_table_from_iter(move_std_addr, natives)
}

#[derive(Debug, Clone)]
pub struct NurseryGasParameters {
    event: event::GasParameters,
    debug: debug::GasParameters,
}

impl NurseryGasParameters {
    pub fn zeros() -> Self {
        Self {
            event: event::GasParameters {
                write_to_event_store: event::WriteToEventStoreGasParameters {
                    unit_cost: 0.into(),
                },
            },
            debug: debug::GasParameters {
                print: debug::PrintGasParameters {
                    base_cost: 0.into(),
                },
                print_stack_trace: debug::PrintStackTraceGasParameters {
                    base_cost: 0.into(),
                },
            },
        }
    }
}

pub fn nursery_natives(
    move_std_addr: AccountAddress,
    gas_params: NurseryGasParameters,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives!("event", event::make_all(gas_params.event));
    add_natives!("debug", debug::make_all(gas_params.debug, move_std_addr));

    make_table_from_iter(move_std_addr, natives)
}
