// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std, no_main)]

//! This is a simple example contract for use with e2e tests of the setheum-client contract interaction.

#[ink::contract]
mod adder {
    #[ink(storage)]
    pub struct Adder {
        name: Option<[u8; 20]>,
        value: u32,
    }

    #[ink(event)]
    pub struct ValueChanged {
        new_value: u32,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Overflow,
    }

    impl Adder {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: 0,
                name: None,
            }
        }

        #[ink(message)]
        pub fn add(&mut self, value: u32) -> Result<(), Error> {
            self.value = self.value.checked_add(value).ok_or(Error::Overflow)?;

            Self::env().emit_event(ValueChanged {
                new_value: self.value,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn set_name(&mut self, name: Option<[u8; 20]>) {
            self.name = name;
        }

        #[ink(message)]
        pub fn get_name(&self) -> Option<[u8; 20]> {
            self.name
        }
    }

    impl Default for Adder {
        fn default() -> Self {
            Self::new()
        }
    }
}
