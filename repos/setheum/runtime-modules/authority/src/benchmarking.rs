// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
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
