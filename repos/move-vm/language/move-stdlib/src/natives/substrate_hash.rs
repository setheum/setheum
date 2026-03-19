// Copyright (c) Eiger, Equilibrium Group
// SPDX-License-Identifier: Apache-2.0

use crate::natives::helpers::make_module_natives;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::VecDeque, sync::Arc};
use core::hash::Hasher;
use move_binary_format::errors::PartialVMResult;
use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte, NumBytes};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::values::Vector;
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use sha3::Digest;
use smallvec::smallvec;
use tiny_keccak::{Hasher as KeccakHasher, Keccak};

/***************************************************************************************************
 * native fun sip_hash
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct SipHashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_sip_hash(
    gas_params: &SipHashGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    // SipHash of the serialized bytes
    let mut hasher = siphasher::sip::SipHasher::new();
    hasher.write(&bytes);
    let hash = hasher.finish();

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::u64(hash)]))
}

pub fn make_native_sip_hash(gas_params: SipHashGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_sip_hash(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun blake2b_256
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct Blake2b256GasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_blake2b_256(
    gas_params: &Blake2b256GasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    let output = blake2_rfc::blake2b::blake2b(32, &[], &bytes)
        .as_bytes()
        .to_vec();

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

pub fn make_native_blake2b_256(gas_params: Blake2b256GasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_blake2b_256(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun ripemd160
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct Ripemd160HashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_ripemd160(
    gas_params: &Ripemd160HashGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(&bytes);
    let output = hasher.finalize().to_vec();

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

pub fn make_native_ripemd160(gas_params: Ripemd160HashGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_ripemd160(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun keccak256
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct Keccak256HashGasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_keccak256(
    gas_params: &Keccak256HashGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    let mut hasher = Keccak::v256();
    hasher.update(&bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

pub fn make_native_keccak256(gas_params: Keccak256HashGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_keccak256(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun sha2_512
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct Sha2_512GasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_sha2_512(
    gas_params: &Sha2_512GasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    let mut hasher = sha2::Sha512::new();
    hasher.update(&bytes);
    let output = hasher.finalize().to_vec();

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

pub fn make_native_sha2_512(gas_params: Sha2_512GasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_sha2_512(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun sha3_512
 *
 *   gas cost: base_cost + per_byte
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct Sha3_512GasParameters {
    pub base: InternalGas,
    pub per_byte: InternalGasPerByte,
}

pub fn native_sha3_512(
    gas_params: &Sha3_512GasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let bytes = pop_arg!(args, Vector);
    let bytes = bytes.to_vec_u8()?;

    let mut hasher = sha3::Sha3_512::new();
    hasher.update(&bytes);
    let output = hasher.finalize().to_vec();

    let cost = gas_params.base + gas_params.per_byte * NumBytes::new(bytes.len() as u64);

    Ok(NativeResult::ok(cost, smallvec![Value::vector_u8(output)]))
}

pub fn make_native_sha3_512(gas_params: Sha3_512GasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_sha3_512(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct GasParameters {
    pub sip_hash: SipHashGasParameters,
    pub blake2b_256: Blake2b256GasParameters,
    pub ripemd160: Ripemd160HashGasParameters,
    pub keccak256: Keccak256HashGasParameters,
    pub sha2_512: Sha2_512GasParameters,
    pub sha3_512: Sha3_512GasParameters,
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        ("sip_hash", make_native_sip_hash(gas_params.sip_hash)),
        (
            "blake2b_256",
            make_native_blake2b_256(gas_params.blake2b_256),
        ),
        ("ripemd160", make_native_ripemd160(gas_params.ripemd160)),
        ("keccak256", make_native_keccak256(gas_params.keccak256)),
        ("sha2_512", make_native_sha2_512(gas_params.sha2_512)),
        ("sha3_512", make_native_sha3_512(gas_params.sha3_512)),
    ];

    make_module_natives(natives)
}
