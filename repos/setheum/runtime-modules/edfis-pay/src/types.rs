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

#![allow(unused_qualifications)]
use crate::{pallet, AssetIdOf, BalanceOf};
use frame_system::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode, HasCompact, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{DispatchResult, Percent};

/// The PaymentDetail struct stores information about the payment/escrow
/// A "payment" in Setheum Pay is similar to an escrow, it is used to
/// guarantee proof of funds and can be released once an agreed upon condition
/// has reached between the payment creator and recipient. The payment lifecycle
/// is tracked using the state field.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PaymentDetail<T: pallet::Config> {
/// type of asset used for payment
	pub asset: AssetIdOf<T>,
/// amount of asset used for payment
	#[codec(compact)]
	pub amount: BalanceOf<T>,
/// incentive amount that is credited to creator for resolving
	#[codec(compact)]
	pub incentive_amount: BalanceOf<T>,
/// enum to track payment lifecycle [Created, NeedsReview, RefundRequested,
/// Requested]
	pub state: PaymentState<T>,
/// account that can settle any disputes created in the payment
	pub resolver_account: T::AccountId,
/// fee charged and recipient account details
	pub fee_detail: Option<(T::AccountId, BalanceOf<T>)>,
}

/// The `PaymentState` enum tracks the possible states that a payment can be in.
/// When a payment is 'completed' or 'cancelled' it is removed from storage and
/// hence not tracked by a state.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaymentState<T: pallet::Config> {
/// Amounts have been reserved and waiting for release/cancel
	Created,
/// A judge needs to review and release manually
	NeedsReview,
/// The user has requested refund and will be processed by `BlockNumber`
	RefundRequested { cancel_block: BlockNumberFor<T> },
/// The recipient of this transaction has created a request
	PaymentRequested,
}

/// trait that defines how to create/release payments for users
pub trait PaymentHandler<T: pallet::Config> {
/// Create a PaymentDetail from the given payment details
/// Calculate the fee amount and store PaymentDetail in storage
/// Possible reasons for failure include:
/// - Payment already exists and cannot be overwritten
	fn create_payment(
		from: &T::AccountId,
		to: &T::AccountId,
		asset: AssetIdOf<T>,
		amount: BalanceOf<T>,
		payment_state: PaymentState<T>,
		incentive_percentage: Percent,
		remark: Option<&[u8]>,
	) -> Result<PaymentDetail<T>, sp_runtime::DispatchError>;

/// Attempt to reserve an amount of the given asset from the caller
/// If not possible then return Error. Possible reasons for failure include:
/// - User does not have enough balance.
	fn reserve_payment_amount(from: &T::AccountId, to: &T::AccountId, payment: PaymentDetail<T>) -> DispatchResult;

// Settle a payment of `from` to `to`. To release a payment, the
// recipient_share=100, to cancel a payment recipient_share=0
// Possible reasonse for failure include
///
/// - The payment does not exist
/// - The unreserve operation fails
/// - The transfer operation fails
	fn settle_payment(from: &T::AccountId, to: &T::AccountId, recipient_share: Percent) -> DispatchResult;

/// Attempt to fetch the details of a payment from the given payment_id
/// Possible reasons for failure include:
/// - The payment does not exist
	fn get_payment_details(from: &T::AccountId, to: &T::AccountId) -> Option<PaymentDetail<T>>;
}

/// DisputeResolver trait defines how to create/assign judges for solving
/// payment disputes
pub trait DisputeResolver<Account> {
/// Returns an `Account`
	fn get_resolver_account() -> Account;
}

/// Fee Handler trait that defines how to handle marketplace fees to every
/// payment/swap
pub trait FeeHandler<T: pallet::Config> {
/// Get the distribution of fees to marketplace participants
	fn apply_fees(
		from: &T::AccountId,
		to: &T::AccountId,
		detail: &PaymentDetail<T>,
		remark: Option<&[u8]>,
	) -> (T::AccountId, Percent);
}

/// Types of Tasks that can be scheduled in the pallet
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub enum Task {
// payment `from` to `to` has to be cancelled
	Cancel,
}

/// The details of a scheduled task
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub struct ScheduledTask<Time: HasCompact> {
/// the type of scheduled task
	pub task: Task,
/// the 'time' at which the task should be executed
	pub when: Time,
}
