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

pub use crate::*;

use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::vec;

/// Helper trait for benchmarking.
pub trait BenchmarkHelper<AsOriginId> {
	fn get_as_origin_id() -> Option<AsOriginId>;
}

impl<AsOriginId> BenchmarkHelper<AsOriginId> for () {
	fn get_as_origin_id() -> Option<AsOriginId> {
		None
	}
}

#[benchmarks(where
    <T as Config>::RuntimeCall: From<frame_system::Call<T>>
)]
mod benchmarks {
	use super::*;

	// dispatch a dispatchable as other origin
	#[benchmark]
	fn dispatch_as() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let call = frame_system::Call::remark { remark: vec![] }.into();

		#[extrinsic_call]
		_(RawOrigin::Root, as_origin, Box::new(call));
	}

	// schedule a dispatchable to be dispatched at later block.
	#[benchmark]
	fn schedule_dispatch_without_delay() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let sub_call = frame_system::Call::remark { remark: vec![] }.into();

		let call: Call<T> = Call::dispatch_as { as_origin, call: Box::new(sub_call) };

		let encoded_call = call.encode();
		let bounded_call = Box::new(Bounded::Inline(encoded_call.try_into().unwrap()));

		#[extrinsic_call]
		schedule_dispatch(RawOrigin::Root, DispatchTime::At(2u32.into()), 0, false, bounded_call);
	}

	// schedule a dispatchable to be dispatched at later block.
	// ensure that the delay is reached when scheduling
	#[benchmark]
	fn schedule_dispatch_with_delay() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let sub_call = frame_system::Call::remark { remark: vec![] }.into();

		let call: Call<T> = Call::dispatch_as { as_origin, call: Box::new(sub_call) };

		let encoded_call = call.encode();
		let bounded_call = Box::new(Bounded::Inline(encoded_call.try_into().unwrap()));

		#[extrinsic_call]
		schedule_dispatch(RawOrigin::Root, DispatchTime::At(2u32.into()), 0, true, bounded_call);
	}

	// fast track a scheduled dispatchable.
	#[benchmark]
	fn fast_track_scheduled_dispatch() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let sub_call = frame_system::Call::remark { remark: vec![] }.into();

		let call: Call<T> = Call::dispatch_as { as_origin, call: Box::new(sub_call) };

		let encoded_call = call.encode();
		let bounded_call = Box::new(Bounded::Inline(encoded_call.try_into().unwrap()));

		frame_system::Pallet::<T>::set_block_number(1u32.into());
		assert_ok!(Pallet::<T>::schedule_dispatch(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			DispatchTime::At(2u32.into()),
			0,
			true,
			bounded_call
		));

		let schedule_origin = {
			let origin: <T as Config>::RuntimeOrigin = From::from(<T as Config>::RuntimeOrigin::root());
			let origin: <T as Config>::RuntimeOrigin =
				From::from(DelayedOrigin::<BlockNumberFor<T>, <T as Config>::PalletsOrigin>::new(
					1u32.into(),
					Box::new(origin.caller().clone()),
				));
			origin
		};

		let pallets_origin = schedule_origin.caller().clone();

		#[extrinsic_call]
		fast_track_scheduled_dispatch(RawOrigin::Root, Box::new(pallets_origin), 0, DispatchTime::At(4u32.into()));
	}

	// delay a scheduled dispatchable.
	#[benchmark]
	fn delay_scheduled_dispatch() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let sub_call = frame_system::Call::remark { remark: vec![] }.into();

		let call: Call<T> = Call::dispatch_as { as_origin, call: Box::new(sub_call) };

		let encoded_call = call.encode();
		let bounded_call = Box::new(Bounded::Inline(encoded_call.try_into().unwrap()));

		frame_system::Pallet::<T>::set_block_number(1u32.into());
		assert_ok!(Pallet::<T>::schedule_dispatch(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			DispatchTime::At(2u32.into()),
			0,
			true,
			bounded_call
		));

		let schedule_origin = {
			let origin: <T as Config>::RuntimeOrigin = From::from(<T as Config>::RuntimeOrigin::root());
			let origin: <T as Config>::RuntimeOrigin =
				From::from(DelayedOrigin::<BlockNumberFor<T>, <T as Config>::PalletsOrigin>::new(
					1u32.into(),
					Box::new(origin.caller().clone()),
				));
			origin
		};

		let pallets_origin = schedule_origin.caller().clone();

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(pallets_origin), 0, 5u32.into());
	}

	// cancel a scheduled dispatchable
	#[benchmark]
	fn cancel_scheduled_dispatch() {
		let as_origin = T::BenchmarkHelper::get_as_origin_id().unwrap();

		let sub_call = frame_system::Call::remark { remark: vec![] }.into();

		let call: Call<T> = Call::dispatch_as { as_origin, call: Box::new(sub_call) };

		let encoded_call = call.encode();
		let bounded_call = Box::new(Bounded::Inline(encoded_call.try_into().unwrap()));

		frame_system::Pallet::<T>::set_block_number(1u32.into());
		assert_ok!(Pallet::<T>::schedule_dispatch(
			<T as frame_system::Config>::RuntimeOrigin::root(),
			DispatchTime::At(2u32.into()),
			0,
			true,
			bounded_call
		));

		let schedule_origin = {
			let origin: <T as Config>::RuntimeOrigin = From::from(<T as Config>::RuntimeOrigin::root());
			let origin: <T as Config>::RuntimeOrigin =
				From::from(DelayedOrigin::<BlockNumberFor<T>, <T as Config>::PalletsOrigin>::new(
					1u32.into(),
					Box::new(origin.caller().clone()),
				));
			origin
		};

		let pallets_origin = schedule_origin.caller().clone();

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(pallets_origin), 0u32.into());
	}

	// authorize a call that can be triggered later
	#[benchmark]
	fn authorize_call() {
		let caller: T::AccountId = whitelisted_caller();

		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let hash = <T as frame_system::Config>::Hashing::hash_of(&call);

		frame_system::Pallet::<T>::set_block_number(1u32.into());

		#[extrinsic_call]
		_(RawOrigin::Root, Box::new(call.clone()), Some(caller.clone()));

		assert_eq!(Pallet::<T>::saved_calls(&hash), Some((call, Some(caller))));
	}

	#[benchmark]
	fn remove_authorized_call() {
		let caller: T::AccountId = whitelisted_caller();

		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let hash = <T as frame_system::Config>::Hashing::hash_of(&call);

		frame_system::Pallet::<T>::set_block_number(1u32.into());
		assert_ok!(Pallet::<T>::authorize_call(RawOrigin::Root.into(), Box::new(call.clone()), Some(caller.clone())));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), hash);

		assert_eq!(Pallet::<T>::saved_calls(&hash), None);
	}

	#[benchmark]
	fn trigger_call() {
		let caller: T::AccountId = whitelisted_caller();

		let call: <T as Config>::RuntimeCall = frame_system::Call::remark { remark: vec![] }.into();
		let hash = <T as frame_system::Config>::Hashing::hash_of(&call);

		let call_weight_bound = call.get_dispatch_info().call_weight;

		frame_system::Pallet::<T>::set_block_number(1u32.into());
		assert_ok!(Pallet::<T>::authorize_call(RawOrigin::Root.into(), Box::new(call.clone()), Some(caller.clone())));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), hash, call_weight_bound);

		assert_eq!(Pallet::<T>::saved_calls(&hash), None);
	}

	impl_benchmark_test_suite! {
		Pallet,
		crate::mock::ExtBuilder::default().build(),
		crate::mock::Runtime,
	}
}
