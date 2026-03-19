//! Tools for bytecode verification.

use move_binary_format::file_format::{CompiledScript, Signature, SignatureToken};
use move_core_types::vm_status::StatusCode;

/// Check the script bytecode and ensure the signer rule is followed.
/// Upon success, return the number of signers in the script argument list.
///
/// The general signer rule:
/// A transaction script can have an arbitrary number of signers as long as the signers are a prefix to
/// any other arguments. In other words, all of the signer arguments must come first:
/// ```move
/// script {
///     fun main(s1: signer, s2: signer, x: u64, y: u8) {
///         // ...
///     }
/// }
/// ```
///
/// Additional function signature rules:
///
/// The allowed types for paramters are:
/// - signers and signer references: `signer`, `&signer` - only when in the front of the list.
///
/// The rest of the allowed types for parameters are:
/// - integers: `u8`, `u16, `u32`, `u64`, `u128`, `u256`
/// - booleans and addresses: `bool`, `address`
/// - vectors but only with booleans, addresses and integers.
///
/// Disallowed script parameteters are:
/// - any types of structs
/// - generic types
/// - references and mutable references
pub fn verify_script_integrity_and_check_signers(script_bc: &[u8]) -> Result<usize, StatusCode> {
    let compiled_script = CompiledScript::deserialize(script_bc).map_err(|e| e.major_status())?;

    let Signature(script_params) = &compiled_script
        .signatures
        .first()
        .ok_or(StatusCode::INVALID_SIGNATURE)?;

    let mut signer_param_cnt = 0;
    // Find all signer params at the beginning of the parameter list.
    for ty in script_params.iter() {
        match ty {
            // Do not allow `&mut signer`.
            SignatureToken::Signer => (),
            SignatureToken::Reference(inner) if inner.is_signer() => (),
            _ => break,
        }

        signer_param_cnt += 1;
    }

    // Check that the rest of the parameter list contains no hidden signers or unallowed types.
    for ty in script_params[signer_param_cnt..].iter() {
        let is_script_param_type_allowed = is_valid_txn_arg(ty);

        if !is_script_param_type_allowed {
            return Err(StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE);
        }
    }

    Ok(signer_param_cnt)
}

/// Check whether the argument is allowed.
fn is_valid_txn_arg(typ: &SignatureToken) -> bool {
    use SignatureToken::*;

    match typ {
        // The most basic types are allowed.
        Bool | U8 | U16 | U32 | U64 | U128 | U256 | Address => true,

        // Vector with basic types are allowed.
        Vector(inner) => is_valid_txn_arg(inner),

        // More complex types are not allowed (which do not offer more benefit anyway for script arguments)
        Signer
        | Reference(_)
        // TODO(eiger): Consider allowing `std::String` here.
        | Struct { .. }
        | StructInstantiation { .. }
        | MutableReference(_)
        | TypeParameter(_) => false,
    }
}
