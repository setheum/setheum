// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod storage_types {
    use ink::prelude::{
        string::String,
        vec,
        vec::Vec,
    };

    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[allow(clippy::enum_variant_names)]
    pub enum CustomError {
        EmptyError,
        StringError(String),
        StringStringError(String, String),
        StringUnsignedError(String, u32),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EnumWithoutValues {
        #[default]
        A,
        B,
        C,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EnumWithValues {
        OneValue(u32),
        TwoValues(u32, u32),
        ThreeValues(u32, u32, u32),
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct PrimitiveTypes {
        bool_value: bool,
        enum_without_values: EnumWithoutValues,
        enum_with_values: EnumWithValues,
        array_value: [u32; 3],
        tuple_value: (u32, u32),
        tuple_triplet_value: (i32, i32, i32),
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct SignedIntegers {
        i128_value_max: i128,
        i128_value_min: i128,
        i16_value_max: i16,
        i16_value_min: i16,
        i32_value_max: i32,
        i32_value_min: i32,
        i64_value_max: i64,
        i64_value_min: i64,
        i8_value_max: i8,
        i8_value_min: i8,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct SubstrateTypes {
        account_id_value: AccountId,
        balance_value_max: Balance,
        balance_value_min: Balance,
        hash_value: Hash,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct InkPreludeTypes {
        string_value: String,
        vec_string_value: Vec<String>,
        vec_vec_string_value: Vec<Vec<String>>,
    }

    #[derive(Debug, Clone)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct UnsignedIntegers {
        u128_value_max: u128,
        u128_value_min: u128,
        u16_value_max: u16,
        u16_value_min: u16,
        u32_value_max: u32,
        u32_value_min: u32,
        u64_value_max: u64,
        u64_value_min: u64,
        u8_value_max: u8,
        u8_value_min: u8,
    }

    #[ink(storage)]
    pub struct StorageTypes {
        ink_prelude_types: InkPreludeTypes,
        primitive_types: PrimitiveTypes,
        signed_integers: SignedIntegers,
        substrate_types: SubstrateTypes,
        unsigned_integers: UnsignedIntegers,
    }

    impl Default for StorageTypes {
        fn default() -> Self {
            Self::new()
        }
    }

    impl StorageTypes {
        #[ink(constructor)]
        pub fn new() -> Self {
            let vec_string_value = vec![
                String::from("This is a String"),
                String::from("This is another String"),
            ];
            let vec_vec_string_value = vec![vec_string_value.clone()];

            Self {
                unsigned_integers: UnsignedIntegers {
                    u128_value_max: u128::MAX,
                    u128_value_min: u128::MIN,
                    u16_value_max: u16::MAX,
                    u16_value_min: u16::MIN,
                    u32_value_max: u32::MAX,
                    u32_value_min: u32::MIN,
                    u64_value_max: u64::MAX,
                    u64_value_min: u64::MIN,
                    u8_value_max: u8::MAX,
                    u8_value_min: u8::MIN,
                },
                signed_integers: SignedIntegers {
                    i128_value_max: i128::MAX,
                    i128_value_min: i128::MIN,
                    i16_value_max: i16::MAX,
                    i16_value_min: i16::MIN,
                    i32_value_max: i32::MAX,
                    i32_value_min: i32::MIN,
                    i64_value_max: i64::MAX,
                    i64_value_min: i64::MIN,
                    i8_value_max: i8::MAX,
                    i8_value_min: i8::MIN,
                },
                ink_prelude_types: InkPreludeTypes {
                    string_value: String::from("This is a string"),
                    vec_string_value,
                    vec_vec_string_value,
                },
                primitive_types: PrimitiveTypes {
                    bool_value: true,
                    enum_with_values: EnumWithValues::ThreeValues(1, 2, 3),
                    enum_without_values: EnumWithoutValues::A,
                    array_value: [3, 2, 1],
                    tuple_value: (7, 8),
                    tuple_triplet_value: (-123, 0, 123),
                },
                substrate_types: SubstrateTypes {
                    account_id_value: AccountId::from([0x00; 32]),
                    balance_value_max: Balance::MAX,
                    balance_value_min: Balance::MIN,
                    hash_value: Hash::from([0x00; 32]),
                },
            }
        }

        #[ink(message)]
        pub fn get_unsigned_integers(&self) -> UnsignedIntegers {
            self.unsigned_integers.clone()
        }

        #[ink(message)]
        pub fn get_signed_integers(&self) -> SignedIntegers {
            self.signed_integers.clone()
        }

        #[ink(message)]
        pub fn get_ink_prelude_types(&self) -> InkPreludeTypes {
            self.ink_prelude_types.clone()
        }

        #[ink(message)]
        pub fn get_substrate_types(&self) -> SubstrateTypes {
            self.substrate_types.clone()
        }

        #[ink(message)]
        pub fn get_primitive_types(&self) -> PrimitiveTypes {
            self.primitive_types.clone()
        }

        #[ink(message)]
        pub fn get_option_some(&self) -> Option<bool> {
            Some(true)
        }

        #[ink(message)]
        pub fn get_option_none(&self) -> Option<bool> {
            None
        }

        #[ink(message)]
        pub fn get_result_ok(&self) -> Result<bool, CustomError> {
            Ok(true)
        }

        #[ink(message)]
        pub fn get_result_error(&self) -> Result<bool, CustomError> {
            Err(CustomError::EmptyError)
        }

        #[ink(message)]
        pub fn get_result_error_with_string(&self) -> Result<bool, CustomError> {
            Err(CustomError::StringError(String::from(
                "This is the Error Message.",
            )))
        }

        #[ink(message)]
        pub fn get_result_error_with_string_string(&self) -> Result<bool, CustomError> {
            Err(CustomError::StringStringError(
                String::from("This is the Error Message."),
                String::from("This is the second string of this Error Message."),
            ))
        }

        #[ink(message)]
        pub fn get_result_error_with_string_unsigned(&self) -> Result<bool, CustomError> {
            Err(CustomError::StringUnsignedError(
                String::from("This is the Error Message."),
                42,
            ))
        }

        #[ink(message)]
        pub fn get_panic(&self) -> Result<(), ()> {
            panic!("This is the Panic message.")
        }

        #[ink(message, payable)]
        pub fn payable(&self) -> Result<Balance, ()> {
            Ok(self.env().transferred_value())
        }
    }

    #[cfg(test)]
    mod tests {}
}
