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

#![allow(clippy::nonminimal_bool)]

use frame_support::dispatch::DispatchResult;
use parity_scale_codec::Encode;
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::DispatchError;

use crate::{
	pallet::{Config, Event, Pallet},
	traits::{
		AccountInfoProvider, BalancesProvider, BondedStashProvider, ContractInfoProvider, NextKeysSessionProvider,
	},
	LOG_TARGET,
};

impl<T: Config> Pallet<T> {
	/// Calculate expected consumers counter for a `who` account, and if actual
	/// counter is not as expected, increment or decrement current counter
	pub fn fix_consumer_counter(who: T::AccountId) -> DispatchResult {
		let current_consumers = T::AccountInfoProvider::get_consumers(&who);
		let mut expected_consumers: u32 = 0;

		if Self::reserved_or_frozen_non_zero(&who) {
			expected_consumers += 1;
		}
		if Self::is_contract_account(&who) {
			expected_consumers += 1;
		}
		if Self::is_bonded(&who) {
			expected_consumers += 1;
		}
		if Self::has_next_session_keys_and_account_is_controller(&who) {
			expected_consumers += 1;
		}

		#[allow(clippy::comparison_chain)]
		if current_consumers < expected_consumers {
			log::debug!(
                target: LOG_TARGET,
                "Account {:?} has consumers underflow: current({}) < expected ({}), incrementing ",
                HexDisplay::from(&who.encode()), current_consumers, expected_consumers);
			Self::increment_consumers(&who)?;
		} else if current_consumers > expected_consumers {
			log::debug!(
                target: LOG_TARGET,
                "Account {:?} has consumers overflow: current({}) > expected ({}), decrementing ",
                HexDisplay::from(&who.encode()), current_consumers, expected_consumers);
			Self::decrement_consumers(&who);
		} else {
			log::trace!(
				target: LOG_TARGET,
				"Account {:?} neither has underflow nor overflow of consumers counter.",
				HexDisplay::from(&who.encode())
			);
		}

		Ok(())
	}

	fn reserved_or_frozen_non_zero(who: &T::AccountId) -> bool {
		!T::BalancesProvider::is_reserved_zero(who) || !T::BalancesProvider::is_frozen_zero(who)
	}

	fn is_bonded(who: &T::AccountId) -> bool {
		T::BondedStashProvider::get_controller(who).is_some()
	}

	fn is_contract_account(who: &T::AccountId) -> bool {
		T::ContractInfoProvider::is_contract_account(who)
	}

	fn has_next_session_keys_and_account_is_controller(who: &T::AccountId) -> bool {
		let has_next_session_keys = T::NextKeysSessionProvider::has_next_session_keys(who);
		let stash_equal_to_controller = match T::BondedStashProvider::get_controller(who) {
			Some(controller) => *who == controller,
			None => false,
		};
		if has_next_session_keys && stash_equal_to_controller {
			return true;
		}
		match T::BondedStashProvider::get_stash(who) {
			Some(stash) => *who != stash && T::NextKeysSessionProvider::has_next_session_keys(&stash),
			None => false,
		}
	}

	fn increment_consumers(who: &T::AccountId) -> Result<(), DispatchError> {
		frame_system::Pallet::<T>::inc_consumers_without_limit(who)?;
		Self::deposit_event(Event::ConsumersCounterIncremented { who: who.clone() });
		Ok(())
	}

	fn decrement_consumers(who: &T::AccountId) {
		// dec_consumers does not return any error when current counter is 0, hence we need to
		// handle such case manually
		if T::AccountInfoProvider::get_consumers(who) > 0 {
			frame_system::Pallet::<T>::dec_consumers(who);
			Self::deposit_event(Event::ConsumersCounterDecremented { who: who.clone() });
		}
	}
}
