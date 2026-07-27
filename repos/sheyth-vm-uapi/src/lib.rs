// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![cfg_attr(not(feature = "std"), no_std)]

bitflags::bitflags! {
	pub struct ReturnFlags: u32 {
		const REVERT = 0x01;
	}
}

impl parity_scale_codec::Encode for ReturnFlags {
	fn encode_to<W: parity_scale_codec::Output + ?Sized>(&self, dest: &mut W) {
		self.bits().encode_to(dest)
	}
}

impl parity_scale_codec::Decode for ReturnFlags {
	fn decode<I: parity_scale_codec::Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
		Ok(Self::from_bits_retain(u32::decode(input)?))
	}
}

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReturnErrorCode {
	CalleeTrap = 0x01,
	CalleeReverted = 0x02,
}

impl From<ReturnErrorCode> for u32 {
	fn from(code: ReturnErrorCode) -> u32 { code as u32 }
}

impl From<u32> for ReturnErrorCode {
	fn from(val: u32) -> ReturnErrorCode {
		match val {
			0x02 => ReturnErrorCode::CalleeReverted,
			_ => ReturnErrorCode::CalleeTrap,
		}
	}
}

bitflags::bitflags! {
	pub struct CallFlags: u32 {
		const CLONE_INPUT = 0x01;
		const TAIL_CALL = 0x02;
		const ALLOW_REENTRY = 0x04;
	}
}

impl parity_scale_codec::Encode for CallFlags {
	fn encode_to<W: parity_scale_codec::Output + ?Sized>(&self, dest: &mut W) {
		self.bits().encode_to(dest)
	}
}

impl parity_scale_codec::Decode for CallFlags {
	fn decode<I: parity_scale_codec::Input>(input: &mut I) -> Result<Self, parity_scale_codec::Error> {
		Ok(Self::from_bits_retain(u32::decode(input)?))
	}
}

/// Host function API for SheythVM smart contracts.
///
/// On riscv32 (SheythVM target), these emit `ecalli` instructions.
/// On native (off-chain testing), these delegate to the engine.
pub mod ext {
	use super::ReturnFlags;

	macro_rules! ecalli_fn {
		($name:ident, $index:expr, ($($arg:ident: $ty:ty),*)) => {
			#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
			#[inline]
			pub fn $name($($arg: $ty),*) {
				unsafe {
					core::arch::asm!(
						".insn r 0xb, 0, 0, zero, zero, zero",
						$(
											in(format!("a{}", $index_of_arg!($arg, $($arg),*))) $arg,
						)*
					);
				}
			}
		};
	}

	pub fn hash_blake2_128(input: &[u8], output: &mut [u8; 16]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") input.as_ptr(),
				in("a1") input.len(),
				in("a2") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (input, output);
		}
	}

	pub fn hash_blake2_256(input: &[u8], output: &mut [u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") input.as_ptr(),
				in("a1") input.len(),
				in("a2") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (input, output);
		}
	}

	pub fn hash_sha2_256(input: &[u8], output: &mut [u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") input.as_ptr(),
				in("a1") input.len(),
				in("a2") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (input, output);
		}
	}

	pub fn hash_keccak_256(input: &[u8], output: &mut [u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") input.as_ptr(),
				in("a1") input.len(),
				in("a2") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (input, output);
		}
	}

	pub fn set_storage(key: &[u8], value: &[u8]) -> u32 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u32;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
				in("a0") key.as_ptr(),
				in("a1") key.len(),
				in("a2") value.as_ptr(),
				in("a3") value.len(),
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (key, value);
			0
		}
	}

	pub fn get_storage(key: &[u8], output: &mut [u8], output_len_ptr: &mut u32) -> u32 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u32;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
				in("a0") key.as_ptr(),
				in("a1") key.len(),
				in("a2") output.as_mut_ptr(),
				in("a3") output.len(),
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (key, output, output_len_ptr);
			0
		}
	}

	pub fn clear_storage(key: &[u8]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") key.as_ptr(),
				in("a1") key.len(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = key;
		}
	}

	pub fn caller(output: &mut [u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn address(output: &mut [u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn balance(output: &mut [u8; 16]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn gas_left() -> u64 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u64;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			0
		}
	}

	pub fn value_transferred(output: &mut [u8; 16]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn now(output: &mut [u8; 8]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn block_number(output: &mut [u8; 4]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn minimum_balance(output: &mut [u8; 16]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") output.as_mut_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = output;
		}
	}

	pub fn transfer(destination: &[u8; 32], value: &[u8; 16]) -> u32 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u32;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
				in("a0") destination.as_ptr(),
				in("a1") value.as_ptr(),
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (destination, value);
			0
		}
	}

	pub fn deposit_event(topics: &[u8], data: &[u8]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") topics.as_ptr(),
				in("a1") topics.len(),
				in("a2") data.as_ptr(),
				in("a3") data.len(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (topics, data);
		}
	}

	pub fn set_code_hash(code_hash: &[u8; 32]) -> u32 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u32;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
				in("a0") code_hash.as_ptr(),
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = code_hash;
			0
		}
	}

	pub fn terminate(beneficiary: &[u8; 32]) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") beneficiary.as_ptr(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = beneficiary;
		}
	}

	pub fn ecdsa_recover(signature: &[u8; 65], message_hash: &[u8; 32], output: &mut [u8; 33]) -> u32 {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			let result: u32;
			core::arch::asm!(
				".insn r 0xb, 0, 0, {dst}, zero, zero",
				dst = lateout(reg) result,
				in("a0") signature.as_ptr(),
				in("a1") message_hash.as_ptr(),
				in("a2") output.as_mut_ptr(),
			);
			result
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = (signature, message_hash, output);
			0
		}
	}

	pub fn debug_message(message: &str) {
		#[cfg(all(target_arch = "riscv32", target_feature = "e"))]
		unsafe {
			core::arch::asm!(
				".insn r 0xb, 0, 0, zero, zero, zero",
				in("a0") message.as_ptr(),
				in("a1") message.len(),
			);
		}
		#[cfg(not(all(target_arch = "riscv32", target_feature = "e")))]
		{
			let _ = message;
		}
	}
}
