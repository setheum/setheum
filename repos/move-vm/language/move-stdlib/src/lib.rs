// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
pub mod utils;

pub mod natives;

#[cfg(feature = "std")]
pub mod doc;

/// Provides a precompiled bundle of move-stdlib bytecode modules.
#[cfg(feature = "stdlib-bytecode")]
pub fn move_stdlib_bundle() -> &'static [u8] {
    include_bytes!("../MoveStdlib/build/MoveStdlib/bundles/MoveStdlib.mvb")
}

/// Provides a precompiled bundle of substrate-stdlib bytecode modules.
#[cfg(feature = "stdlib-bytecode")]
pub fn substrate_stdlib_bundle() -> &'static [u8] {
    include_bytes!("../SubstrateStdlib/build/SubstrateStdlib/bundles/SubstrateStdlib.mvb")
}
