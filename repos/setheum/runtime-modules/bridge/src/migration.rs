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

#[allow(unused_imports)]
use super::*;

#[cfg(feature = "try-runtime")]
use frame_support::ensure;
use frame_support::traits::{Get, OnRuntimeUpgrade, StorageVersion};
use log;
use module_bridge_traits::MpcAddress;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

const EXPECTED_STORAGE_VERSION: StorageVersion = StorageVersion::new(0);
#[cfg(feature = "try-runtime")]
const FINAL_STORAGE_VERSION: StorageVersion = StorageVersion::new(1);
const MPC_ADDR: &str = "B01137123EF02fAeF251a39108c6ef513AAaC485";

pub struct FixMpcAddress<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for FixMpcAddress<T> {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		if StorageVersion::get::<Pallet<T>>() == EXPECTED_STORAGE_VERSION {
			log::info!("Start sygma bridge migration");

			let mut slice: [u8; 20] = [0; 20];
			slice.copy_from_slice(&hex::decode(MPC_ADDR).unwrap()[..20]);
			MpcAddr::<T>::kill();
			MpcAddr::<T>::set(MpcAddress(slice));

			// Set new storage version to 1
			StorageVersion::new(1).put::<Pallet<T>>();

			log::info!("Sygma bridge migration done👏");

			// kill + set + put
			T::DbWeight::get().writes(3)
		} else {
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		ensure!(
			StorageVersion::get::<Pallet<T>>() == EXPECTED_STORAGE_VERSION,
			"Incorrect Sygma bridge storage version in pre migrate"
		);

		log::info!("Sygma bridge pre migration check passed👏");

		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		ensure!(
			StorageVersion::get::<Pallet<T>>() == FINAL_STORAGE_VERSION,
			"Incorrect Sygma bridge storage version in post migrate"
		);

		let mut slice: [u8; 20] = [0; 20];
		slice.copy_from_slice(&hex::decode(MPC_ADDR).unwrap()[..20]);
		ensure!(MpcAddr::<T>::get() == MpcAddress(slice), "Unexpected MPC address in post migrate");

		log::info!("Sygma bridge post migration check passed👏");

		Ok(())
	}
}
