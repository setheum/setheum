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
