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

//! Runtime API definition for pallet setbft.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(warnings)]
#![allow(deprecated)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use primitives::{
	crypto::SignatureSet, AccountId, ApiError, AuthorityId, AuthoritySignature, Balance, Perbill, Score,
	SessionAuthorityData, SessionCommittee, SessionIndex, SessionValidatorError, Version,
};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait SetBFTSessionApi {
		fn next_session_authorities() -> Result<Vec<AuthorityId>, ApiError>;
		fn authorities() -> Vec<AuthorityId>;
		fn next_session_authority_data() -> Result<SessionAuthorityData, ApiError>;
		fn authority_data() -> SessionAuthorityData;
		fn session_period() -> u32;
		fn millisecs_per_block() -> u64;
		fn finality_version() -> Version;
		fn next_session_finality_version() -> Version;
		fn score_submission_period() -> u32;
		/// Predict finality committee and block producers for the given session. `session` must be
		/// within the current era (current, in the staking context).
		///
		/// If the active era `E` starts in the session `a`, and ends in session `b` then from
		/// session `a` to session `b-1` this function can answer question who will be in the
		/// committee in the era `E`. In the last session of the era `E` (`b`) this can be used to
		/// determine all of the sessions in the era `E+1`.
		fn predict_session_committee(
			session: SessionIndex
		) -> Result<SessionCommittee<AccountId>, SessionValidatorError>;
		fn next_session_aura_authorities() -> Vec<(AccountId, AuraId)>;
		/// Returns owner (`AccountId`) corresponding to an AuthorityId (in some contexts referenced
		/// also as `setbft_key` - consensus engine's part of session keys) in the current session
		/// of SetBFT (finalisation committee).
		fn key_owner(key: AuthorityId) -> Option<AccountId>;
		/// Returns inflation from now to now + 1 year. Capped at 100%
		fn yearly_inflation() -> Perbill;
		/// Returns payout. First tuple item is a validators payout, 2nd is the rest.
		fn current_era_payout() -> (Balance, Balance);
		/// Submits score for a nonce in a session of performance of finality committee members.
		fn submit_sbft_score(score: Score, signature: SignatureSet<AuthoritySignature>) -> Option<()>;
	}
}
