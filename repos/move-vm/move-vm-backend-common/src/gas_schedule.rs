// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! This module lays out the basic abstract costing schedule for bytecode instructions and for the
//! native functions.

use alloc::vec;
use lazy_static::lazy_static;
use move_binary_format::{
    file_format::{
        Bytecode::*, ConstantPoolIndex, FieldHandleIndex, FieldInstantiationIndex,
        FunctionHandleIndex, FunctionInstantiationIndex, SignatureIndex,
        StructDefInstantiationIndex, StructDefinitionIndex,
    },
    file_format_common::instruction_key,
};
use move_core_types::u256;
use move_stdlib::natives::GasParameters;
use move_vm_test_utils::gas_schedule::{new_from_instructions, CostTable, GasCost};

/// A predefined gas cost to published byte ratio.
pub const MILLIGAS_COST_PER_PUBLISHED_BYTE: u64 = 1000;

/// Generic gas cost table scale factor, that will be applied linearly.
pub const TABLE_GAS_COST_SCALE_FACTOR: f64 = 1.0;

macro_rules! gas_cost {
    ($op:expr, $gas:literal) => {
        (
            $op,
            GasCost::new((($gas as f64) * TABLE_GAS_COST_SCALE_FACTOR) as u64, 0),
        )
    };
}

lazy_static! {
    /// A predefined gas strategy for instruction table cost.
    pub static ref INSTRUCTION_COST_TABLE: CostTable = {
        let mut instrs = vec![
        gas_cost!(MoveTo(StructDefinitionIndex::new(0)), 1838),
        gas_cost!(
            MoveToGeneric(StructDefInstantiationIndex::new(0)),
            1838
        ),
        gas_cost!(
            MoveFrom(StructDefinitionIndex::new(0)),
            1286
        ),
        gas_cost!(
            MoveFromGeneric(StructDefInstantiationIndex::new(0)),
            1286
        ),
        gas_cost!(BrTrue(0), 441),
        gas_cost!(WriteRef, 735),
        gas_cost!(Mul, 588),
        gas_cost!(MoveLoc(0), 441),
        gas_cost!(And, 588),
        gas_cost!(Pop, 147),
        gas_cost!(BitAnd, 588),
        gas_cost!(ReadRef, 735),
        gas_cost!(Sub, 588),
        gas_cost!(MutBorrowField(FieldHandleIndex::new(0)), 735),
        gas_cost!(
            MutBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            735
        ),
        gas_cost!(ImmBorrowField(FieldHandleIndex::new(0)), 735),
        gas_cost!(
            ImmBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            735
        ),
        gas_cost!(Add, 588),
        gas_cost!(CopyLoc(0), 294),
        gas_cost!(StLoc(0), 441),
        gas_cost!(Ret, 220),
        gas_cost!(Lt, 588),
        gas_cost!(LdU8(0), 220),
        gas_cost!(LdU64(0), 220),
        gas_cost!(LdU128(0), 220),
        gas_cost!(CastU8, 441),
        gas_cost!(CastU64, 441),
        gas_cost!(CastU128, 441),
        gas_cost!(Abort, 220),
        gas_cost!(MutBorrowLoc(0), 220),
        gas_cost!(ImmBorrowLoc(0), 220),
        gas_cost!(LdConst(ConstantPoolIndex::new(0)), 2389),
        gas_cost!(Ge, 588),
        gas_cost!(Xor, 588),
        gas_cost!(Shl, 588),
        gas_cost!(Shr, 588),
        gas_cost!(Neq, 367),
        gas_cost!(Not, 588),
        gas_cost!(Call(FunctionHandleIndex::new(0)), 3676),
        gas_cost!(
            CallGeneric(FunctionInstantiationIndex::new(0)),
            808
        ),
        gas_cost!(Le, 588),
        gas_cost!(Branch(0), 294),
        gas_cost!(Unpack(StructDefinitionIndex::new(0)), 808),
        gas_cost!(
            UnpackGeneric(StructDefInstantiationIndex::new(0)),
            808
        ),
        gas_cost!(Or, 588),
        gas_cost!(LdFalse, 220),
        gas_cost!(LdTrue, 220),
        gas_cost!(Mod, 588),
        gas_cost!(BrFalse(0), 441),
        gas_cost!(Exists(StructDefinitionIndex::new(0)), 919),
        gas_cost!(
            ExistsGeneric(StructDefInstantiationIndex::new(0)),
            919
        ),
        gas_cost!(BitOr, 588),
        gas_cost!(FreezeRef, 36),
        gas_cost!(
            MutBorrowGlobal(StructDefinitionIndex::new(0)),
            1838
        ),
        gas_cost!(
            MutBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            1838
        ),
        gas_cost!(
            ImmBorrowGlobal(StructDefinitionIndex::new(0)),
            1838
        ),
        gas_cost!(
            ImmBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            1838
        ),
        gas_cost!(Div, 588),
        gas_cost!(Eq, 367),
        gas_cost!(Gt, 588),
        gas_cost!(Pack(StructDefinitionIndex::new(0)), 808),
        gas_cost!(
            PackGeneric(StructDefInstantiationIndex::new(0)),
            808
        ),
        gas_cost!(Nop, 36),
        gas_cost!(VecPack(SignatureIndex::new(0), 0), 2205),
        gas_cost!(VecLen(SignatureIndex::new(0)), 808),
        gas_cost!(VecImmBorrow(SignatureIndex::new(0)), 1213),
        gas_cost!(VecMutBorrow(SignatureIndex::new(0)), 1213),
        gas_cost!(VecPushBack(SignatureIndex::new(0)), 1396),
        gas_cost!(VecPopBack(SignatureIndex::new(0)), 955),
        gas_cost!(VecUnpack(SignatureIndex::new(0), 0), 1838),
        gas_cost!(VecSwap(SignatureIndex::new(0)), 1102),
        gas_cost!(LdU16(0), 220),
        gas_cost!(LdU32(0), 220),
        gas_cost!(LdU256(u256::U256::zero()), 294),
        gas_cost!(CastU16, 441),
        gas_cost!(CastU32, 441),
        gas_cost!(CastU256, 441),
    ];

        // Note that the DiemVM is expecting the table sorted by instruction order.
        instrs.sort_by_key(|cost| instruction_key(&cost.0));

        new_from_instructions(instrs)
    };
}

lazy_static! {
    // TODO(rqnsom): tweak the cost for native parameter
    /// A predefined gas strategy for native functions.
    pub static ref NATIVE_COST_PARAMS: GasParameters = {
        GasParameters {
            bcs: move_stdlib::natives::bcs::GasParameters {
                to_bytes: move_stdlib::natives::bcs::ToBytesGasParameters {
                    per_byte_serialized: 36.into(),
                    failure: 3676.into(),
                    legacy_min_output_size: 1000.into(),
                },
            },

            hash: move_stdlib::natives::hash::GasParameters {
                sha2_256: move_stdlib::natives::hash::Sha2_256GasParameters {
                    base: 11028.into(),
                    per_byte: 183.into(),
                    legacy_min_input_len: 1000.into(),
                },
                sha3_256: move_stdlib::natives::hash::Sha3_256GasParameters {
                    base: 14704.into(),
                    per_byte: 165.into(),
                    legacy_min_input_len: 1000.into(),
                },
            },
            type_name: move_stdlib::natives::type_name::GasParameters {
                get: move_stdlib::natives::type_name::GetGasParameters {
                    base: 2805.into(),
                    per_byte: 98.into(),
                },
            },
            signer: move_stdlib::natives::signer::GasParameters {
                borrow_address: move_stdlib::natives::signer::BorrowAddressGasParameters { base: 735.into() },
            },
            string: move_stdlib::natives::string::GasParameters {
                check_utf8: move_stdlib::natives::string::CheckUtf8GasParameters {
                    base: 1102.into(),
                    per_byte: 29.into(),
                },
                is_char_boundary: move_stdlib::natives::string::IsCharBoundaryGasParameters { base: 1102.into() },
                sub_string: move_stdlib::natives::string::SubStringGasParameters {
                    base: 1470.into(),
                    per_byte: 11.into(),
                },
                index_of: move_stdlib::natives::string::IndexOfGasParameters {
                    base: 1470.into(),
                    per_byte_pattern: 73.into(),
                    per_byte_searched: 36.into(),
                },
            },
            // vectors will be charged by bytecode operations (see table above).
            vector: move_stdlib::natives::vector::GasParameters {
                empty: move_stdlib::natives::vector::EmptyGasParameters { base: 0.into() },
                length: move_stdlib::natives::vector::LengthGasParameters { base: 0.into() },
                push_back: move_stdlib::natives::vector::PushBackGasParameters {
                    base: 0.into(),
                    legacy_per_abstract_memory_unit: 0.into(),
                },
                borrow: move_stdlib::natives::vector::BorrowGasParameters { base: 0.into() },
                pop_back: move_stdlib::natives::vector::PopBackGasParameters { base: 0.into() },
                destroy_empty: move_stdlib::natives::vector::DestroyEmptyGasParameters { base: 0.into() },
                swap: move_stdlib::natives::vector::SwapGasParameters { base: 0.into() },
            },
            balance: move_stdlib::natives::balance::GasParameters {
                transfer: move_stdlib::natives::balance::TransferGasParameters { base: 12381.into() },
                cheque_amount: move_stdlib::natives::balance::ChequeAmountGasParameters { base: 3892.into() },
                total_amount: move_stdlib::natives::balance::TotalAmountGasParameters { base: 3769.into() },
            },
            substrate_hash: move_stdlib::natives::substrate_hash::GasParameters {
                sip_hash: move_stdlib::natives::substrate_hash::SipHashGasParameters {
                    base: 3676.into(),
                    per_byte: 183.into()
                },
                blake2b_256: move_stdlib::natives::substrate_hash::Blake2b256GasParameters {
                    base: 6433.into(),
                    per_byte: 55.into()
                },
                ripemd160: move_stdlib::natives::substrate_hash::Ripemd160HashGasParameters {
                    base: 11028.into(),
                    per_byte: 183.into()
                },
                keccak256: move_stdlib::natives::substrate_hash::Keccak256HashGasParameters {
                    base: 14704.into(),
                    per_byte: 165.into()
                },
                sha2_512: move_stdlib::natives::substrate_hash::Sha2_512GasParameters {
                    base: 11910.into(),
                    per_byte: 220.into()
                },
                sha3_512: move_stdlib::natives::substrate_hash::Sha3_512GasParameters {
                    base: 16542.into(),
                    per_byte: 183.into()
                },
            },
            #[cfg(feature = "testing")]
            unit_test: move_stdlib::natives::unit_test::GasParameters {
                create_signers_for_testing: move_stdlib::natives::unit_test::CreateSignersForTestingGasParameters {
                    base_cost: 1000.into(),
                    unit_cost: 1000.into(),
                },
            },
        }
    };
}
