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

//! Unit tests for the non-fungible-token module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use orml_traits::parameters::RuntimeParameterStore;

#[test]
fn set_parameters() {
	ExtBuilder::new().execute_with(|| {
		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key1),
			None
		);

		assert_noop!(
			ModuleParameters::set_parameter(
				RuntimeOrigin::signed(1),
				RuntimeParameters::Pallet1(pallet1::Parameters::Key1(pallet1::Key1, Some(123))),
			),
			DispatchError::BadOrigin
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::root(),
			RuntimeParameters::Pallet1(pallet1::Parameters::Key1(pallet1::Key1, Some(123))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key1),
			Some(123)
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::root(),
			RuntimeParameters::Pallet1(pallet1::Parameters::Key2(pallet1::Key2(234), Some(345))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key2(234)),
			Some(345)
		);

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet1::Parameters, _>(pallet1::Key2(235)),
			None
		);

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet2::Parameters, _>(pallet2::Key3((1, 2))),
			None
		);

		assert_noop!(
			ModuleParameters::set_parameter(
				RuntimeOrigin::root(),
				RuntimeParameters::Pallet2(pallet2::Parameters::Key3(pallet2::Key3((1, 2)), Some(123))),
			),
			DispatchError::BadOrigin
		);

		assert_ok!(ModuleParameters::set_parameter(
			RuntimeOrigin::signed(1),
			RuntimeParameters::Pallet2(pallet2::Parameters::Key3(pallet2::Key3((1, 2)), Some(456))),
		));

		assert_eq!(
			<ModuleParameters as RuntimeParameterStore>::get::<pallet2::Parameters, _>(pallet2::Key3((1, 2))),
			Some(456)
		);
	});
}
