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

//! # Xcm Module

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::large_enum_variant)]

use frame_support::{pallet_prelude::*, traits::EnsureOrigin};
use frame_system::pallet_prelude::*;
use sp_std::boxed::Box;
use xcm::{v5::prelude::*, VersionedLocation, VersionedXcm};

pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		/// The required origin for sending XCM as parachain sovereign.
		///
		/// Typically root or the majority of collective.
		type SovereignOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// XCM message sent. \[to, message\]
		Sent { to: Location, message: Xcm<()> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The message and destination combination was not recognized as being
		/// reachable.
		Unreachable,
		/// The message and destination was recognized as being reachable but
		/// the operation could not be completed.
		SendFailure,
		/// The version of the `Versioned` value used is not able to be
		/// interpreted.
		BadVersion,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Send an XCM message as parachain sovereign.
		#[pallet::call_index(0)]
		// FIXME: Benchmark send
		#[pallet::weight(Weight::from_parts(100_000_000, 0))]
		pub fn send_as_sovereign(
			origin: OriginFor<T>,
			dest: Box<VersionedLocation>,
			message: Box<VersionedXcm<()>>,
		) -> DispatchResult {
			let _ = T::SovereignOrigin::ensure_origin(origin)?;
			let dest = Location::try_from(*dest).map_err(|()| Error::<T>::BadVersion)?;
			let message: Xcm<()> = (*message).try_into().map_err(|()| Error::<T>::BadVersion)?;

			pallet_xcm::Pallet::<T>::send_xcm(Here, dest.clone(), message.clone()).map_err(|e| match e {
				SendError::Unroutable => Error::<T>::Unreachable,
				_ => Error::<T>::SendFailure,
			})?;
			Self::deposit_event(Event::Sent { to: dest, message });
			Ok(())
		}
	}
}
