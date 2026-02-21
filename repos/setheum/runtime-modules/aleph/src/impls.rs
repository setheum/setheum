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

use primitives::{FinalityCommitteeManager, SessionIndex};
use sp_std::vec::Vec;

use crate::{
    Config, Event, FinalityScheduledVersionChange, FinalityVersion, NextFinalityCommittee, Pallet,
};

impl<T> pallet_session::SessionManager<T::AccountId> for Pallet<T>
where
    T: Config,
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        <T as Config>::SessionManager::new_session(new_index)
    }

    fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        <T as Config>::SessionManager::new_session_genesis(new_index)
    }

    fn end_session(end_index: SessionIndex) {
        <T as Config>::SessionManager::end_session(end_index);
    }

    fn start_session(start_index: SessionIndex) {
        <T as Config>::SessionManager::start_session(start_index);
        Self::update_version_change_history();
    }
}

impl<T> Pallet<T>
where
    T: Config,
{
// Check if a schedule version change has moved into the past. Update history, even if there is
// no change. Resets the scheduled version.
    fn update_version_change_history() {
        let current_session = Self::current_session();

        if let Some(scheduled_version_change) = <FinalityScheduledVersionChange<T>>::get() {
            let scheduled_session = scheduled_version_change.session;
            let scheduled_version = scheduled_version_change.version_incoming;

// Record the scheduled version as the current version as it moves into the past.
            if scheduled_session == current_session {
                <FinalityVersion<T>>::put(scheduled_version);

// Reset the scheduled version.
                <FinalityScheduledVersionChange<T>>::kill();

                Self::deposit_event(Event::FinalityVersionChange(scheduled_version_change));
            }
        }
    }
}

impl<T: Config> FinalityCommitteeManager<T::AccountId> for Pallet<T> {
    fn on_next_session_finality_committee(committee: Vec<T::AccountId>) {
        NextFinalityCommittee::<T>::put(committee);
    }
}
