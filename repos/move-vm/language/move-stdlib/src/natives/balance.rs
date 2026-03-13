// Copyright (c) Eiger, Equilibrium Group
// SPDX-License-Identifier: Apache-2.0

use crate::natives::helpers::make_module_natives;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::VecDeque, sync::Arc};
use move_binary_format::errors::PartialVMResult;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::values::SignerRef;
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};

/***************************************************************************************************
 * native fun transfer
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct TransferGasParameters {
    pub base: InternalGas,
}

pub fn native_transfer(
    gas_params: &TransferGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    let amount = pop_arg!(args, u128);
    let dst = pop_arg!(args, AccountAddress);
    let src = pop_arg!(args, SignerRef);

    let src = src.address()?;
    let ret = context.transfer(src, dst, amount)?;

    NativeResult::map_partial_vm_result_one(gas_params.base, Ok(Value::bool(ret)))
}

pub fn make_native_transfer(gas_params: TransferGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_transfer(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun cheque_amount
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct ChequeAmountGasParameters {
    pub base: InternalGas,
}

pub fn native_cheque_amount(
    gas_params: &ChequeAmountGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let account_addr = pop_arg!(args, AccountAddress);

    let ret = context.cheque_amount(account_addr)?;

    NativeResult::map_partial_vm_result_one(gas_params.base, Ok(Value::u128(ret)))
}

pub fn make_native_cheque_amount(gas_params: ChequeAmountGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_cheque_amount(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * native fun total_amount
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct TotalAmountGasParameters {
    pub base: InternalGas,
}

pub fn native_total_amount(
    gas_params: &TotalAmountGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let account_addr = pop_arg!(args, AccountAddress);

    let ret = context.total_amount(account_addr)?;

    NativeResult::map_partial_vm_result_one(gas_params.base, Ok(Value::u128(ret)))
}

pub fn make_native_total_amount(gas_params: TotalAmountGasParameters) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_total_amount(&gas_params, context, ty_args, args)
        },
    )
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
#[derive(Debug, Clone)]
pub struct GasParameters {
    pub transfer: TransferGasParameters,
    pub cheque_amount: ChequeAmountGasParameters,
    pub total_amount: TotalAmountGasParameters,
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        ("transfer", make_native_transfer(gas_params.transfer)),
        (
            "cheque_amount",
            make_native_cheque_amount(gas_params.cheque_amount),
        ),
        (
            "total_amount",
            make_native_total_amount(gas_params.total_amount),
        ),
    ];

    make_module_natives(natives)
}
