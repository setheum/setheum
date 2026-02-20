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

use crate::Change;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, FullCodec, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{AtLeast32Bit, Bounded, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
	result,
};

/// Auction info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, DecodeWithMemTracking, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AuctionInfo<AccountId, Balance, BlockNumber> {
	/// Current bidder and bid price.
	pub bid: Option<(AccountId, Balance)>,
	/// Define which block this auction will be started.
	pub start: BlockNumber,
	/// Define which block this auction will be ended.
	pub end: Option<BlockNumber>,
}

/// Abstraction over a simple auction system.
pub trait Auction<AccountId, BlockNumber> {
	/// The id of an AuctionInfo
	type AuctionId: FullCodec + Default + Copy + Eq + PartialEq + MaybeSerializeDeserialize + Bounded + Debug;
	/// The price to bid.
	type Balance: AtLeast32Bit + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

	/// The auction info of `id`
	fn auction_info(id: Self::AuctionId) -> Option<AuctionInfo<AccountId, Self::Balance, BlockNumber>>;
	/// Update the auction info of `id` with `info`
	fn update_auction(id: Self::AuctionId, info: AuctionInfo<AccountId, Self::Balance, BlockNumber>) -> DispatchResult;
	/// Create new auction with specific startblock and endblock, return the id
	/// of the auction
	fn new_auction(start: BlockNumber, end: Option<BlockNumber>) -> result::Result<Self::AuctionId, DispatchError>;
	/// Remove auction by `id`
	fn remove_auction(id: Self::AuctionId);
}

/// The result of bid handling.
pub struct OnNewBidResult<BlockNumber> {
	/// Indicates if the bid was accepted
	pub accept_bid: bool,
	/// The auction end change.
	pub auction_end_change: Change<Option<BlockNumber>>,
}

/// Hooks for auction to handle bids.
pub trait AuctionHandler<AccountId, Balance, BlockNumber, AuctionId> {
	/// Called when new bid is received.
	/// The return value determines if the bid should be accepted and update
	/// auction end time. Implementation should reserve money from current
	/// winner and refund previous winner.
	fn on_new_bid(
		now: BlockNumber,
		id: AuctionId,
		new_bid: (AccountId, Balance),
		last_bid: Option<(AccountId, Balance)>,
	) -> OnNewBidResult<BlockNumber>;
	/// End an auction with `winner`
	fn on_auction_ended(id: AuctionId, winner: Option<(AccountId, Balance)>);
}
