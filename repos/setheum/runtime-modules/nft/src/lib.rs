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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use frame_support::{
	pallet_prelude::*,
	require_transactional,
	traits::{
		tokens::nonfungibles::{Inspect, Mutate, Transfer},
		Currency,
		ExistenceRequirement::{AllowDeath, KeepAlive},
		NamedReservableCurrency,
		Get,
	},
	PalletId,
	BoundedVec,
	Parameter,
};
use frame_system::pallet_prelude::*;
use module_traits::InspectExtended;
use primitives::{
	nft::{Attributes, ClassProperty, NFTBalance, Properties, CID},
	ReserveIdentifier,
};
use scale_info::TypeInfo;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};

use serde::{Deserialize, Serialize};
use sp_runtime::{
	traits::{AccountIdConversion, Hash, Saturating, StaticLookup, Zero, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, MaybeSerializeDeserialize, Member, One},
	DispatchResult, RuntimeDebug, ArithmeticError,
};
use sp_std::vec::Vec;

pub mod benchmarking;
mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

/// Class info
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct ClassInfo<TokenId, AccountId, Data, ClassMetadataOf> {
	/// Class metadata
	pub metadata: ClassMetadataOf,
	/// Total issuance for the class
	pub total_issuance: TokenId,
	/// Class owner
	pub owner: AccountId,
	/// Class Properties
	pub data: Data,
}

/// Token info
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct TokenInfo<AccountId, Data, TokenMetadataOf> {
	/// Token metadata
	pub metadata: TokenMetadataOf,
	/// Token owner
	pub owner: AccountId,
	/// Token Properties
	pub data: Data,
}

#[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Serialize, Deserialize)]
pub struct ClassData<Balance> {
	/// Deposit reserved to create token class
	pub deposit: Balance,
	/// Class properties
	pub properties: Properties,
	/// Class attributes
	pub attributes: Attributes,
}

#[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo, Serialize, Deserialize)]
pub struct TokenData<Balance> {
	/// Deposit reserved to create token
	pub deposit: Balance,
	/// Token attributes
	pub attributes: Attributes,
}

pub type TokenIdOf<T> = <T as Config>::TokenId;
pub type ClassIdOf<T> = <T as Config>::ClassId;
pub type BalanceOf<T> =
	<<T as pallet_proxy::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type ClassMetadataOf<T> = BoundedVec<u8, <T as Config>::MaxClassMetadata>;
pub type TokenMetadataOf<T> = BoundedVec<u8, <T as Config>::MaxTokenMetadata>;
pub type ClassInfoOf<T> = ClassInfo<
	<T as Config>::TokenId,
	<T as frame_system::Config>::AccountId,
	ClassData<BalanceOf<T>>,
	ClassMetadataOf<T>,
>;
pub type TokenInfoOf<T> =
	TokenInfo<<T as frame_system::Config>::AccountId, TokenData<BalanceOf<T>>, TokenMetadataOf<T>>;

pub type GenesisTokenData<T> = (
	<T as frame_system::Config>::AccountId, // Token owner
	Vec<u8>,                                // Token metadata
	TokenData<BalanceOf<T>>,
);
pub type GenesisTokens<T> = (
	<T as frame_system::Config>::AccountId, // Token class owner
	Vec<u8>,                                // Token class metadata
	ClassData<BalanceOf<T>>,
	Vec<GenesisTokenData<T>>, // Vector of tokens belonging to this class
);

#[frame_support::pallet]
pub mod module {
	use super::*;

	pub const RESERVE_ID: ReserveIdentifier = ReserveIdentifier::Nft;

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_proxy::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency type for reserve balance.
		type Currency: NamedReservableCurrency<
			Self::AccountId,
			Balance = BalanceOf<Self>,
			ReserveIdentifier = ReserveIdentifier,
		>;

		/// The minimum balance to create class
		#[pallet::constant]
		type CreateClassDeposit: Get<BalanceOf<Self>>;

		/// The minimum balance to create token
		#[pallet::constant]
		type CreateTokenDeposit: Get<BalanceOf<Self>>;

		/// Deposit required for per byte.
		#[pallet::constant]
		type DataDepositPerByte: Get<BalanceOf<Self>>;

		/// The NFT's module id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Maximum number of bytes in attributes
		#[pallet::constant]
		type MaxAttributesBytes: Get<u32>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The class ID type
		type ClassId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
		/// The token ID type
		type TokenId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
		/// The maximum size of a class's metadata
		type MaxClassMetadata: Get<u32>;
		/// The maximum size of a token's metadata
		type MaxTokenMetadata: Get<u32>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// ClassId not found
		ClassIdNotFound,
		/// TokenId not found
		TokenIdNotFound,
		/// The operator is not the owner of the token and has no permission
		NoPermission,
		/// Quantity is invalid. need >= 1
		InvalidQuantity,
		/// Property of class don't support transfer
		NonTransferable,
		/// Property of class don't support burn
		NonBurnable,
		/// Property of class don't support mint
		NonMintable,
		/// Can not destroy class
		/// Total issuance is not 0
		CannotDestroyClass,
		/// Cannot perform mutable action
		Immutable,
		/// Attributes too large
		AttributesTooLarge,
		/// The given token ID is not correct
		IncorrectTokenId,
		/// No available class ID
		NoAvailableClassId,
		/// No available token ID
		NoAvailableTokenId,
		/// Failed because the Maximum amount of metadata was exceeded
		MaxMetadataExceeded,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Created NFT class.
		CreatedClass {
			owner: T::AccountId,
			class_id: ClassIdOf<T>,
		},
		/// Minted NFT token.
		MintedToken {
			from: T::AccountId,
			to: T::AccountId,
			class_id: ClassIdOf<T>,
			quantity: u32,
		},
		/// Transferred NFT token.
		TransferredToken {
			from: T::AccountId,
			to: T::AccountId,
			class_id: ClassIdOf<T>,
			token_id: TokenIdOf<T>,
		},
		/// Burned NFT token.
		BurnedToken {
			owner: T::AccountId,
			class_id: ClassIdOf<T>,
			token_id: TokenIdOf<T>,
		},
		/// Burned NFT token with remark.
		BurnedTokenWithRemark {
			owner: T::AccountId,
			class_id: ClassIdOf<T>,
			token_id: TokenIdOf<T>,
			remark_hash: T::Hash,
		},
		/// Destroyed NFT class.
		DestroyedClass {
			owner: T::AccountId,
			class_id: ClassIdOf<T>,
		},
	}

	/// Next available class ID.
	#[pallet::storage]
	#[pallet::getter(fn next_class_id)]
	pub type NextClassId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;

	/// Next available token ID.
	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub type NextTokenId<T: Config> = StorageMap<_, Twox64Concat, T::ClassId, T::TokenId, ValueQuery>;

	/// Store class info.
	///
	/// Returns `None` if class info not set or removed.
	#[pallet::storage]
	#[pallet::getter(fn classes)]
	pub type Classes<T: Config> = StorageMap<_, Twox64Concat, T::ClassId, ClassInfoOf<T>>;

	/// Store token info.
	///
	/// Returns `None` if token info not set or removed.
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::ClassId, Twox64Concat, T::TokenId, TokenInfoOf<T>>;

	/// Token existence check by owner and class ID.
	#[pallet::storage]
	#[pallet::getter(fn tokens_by_owner)]
	pub type TokensByOwner<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // owner
			NMapKey<Blake2_128Concat, T::ClassId>,
			NMapKey<Blake2_128Concat, T::TokenId>,
		),
		(),
		ValueQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub tokens: Vec<GenesisTokens<T>>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				tokens: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			self.tokens.iter().for_each(|token_class| {
				let class_id = Pallet::<T>::do_create_class(&token_class.0, token_class.1.to_vec(), token_class.2.clone())
					.expect("Create class cannot fail while building genesis");
				for (account_id, token_metadata, token_data) in &token_class.3 {
					Pallet::<T>::do_mint(account_id, class_id, token_metadata.to_vec(), token_data.clone())
						.expect("Token mint cannot fail during genesis");
				}
			})
		}
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create NFT class, tokens belong to the class.
		///
		/// - `metadata`: external metadata
		/// - `properties`: class property, include `Transferable` `Burnable`
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::create_class())]
		pub fn create_class(
			origin: OriginFor<T>,
			metadata: CID,
			properties: Properties,
			attributes: Attributes,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let next_id = Self::next_class_id();
			let owner: T::AccountId = T::PalletId::get().into_sub_account_truncating(next_id);
			let class_deposit = T::CreateClassDeposit::get();

			let data_deposit = Self::data_deposit(&metadata, &attributes)?;
			let proxy_deposit = <pallet_proxy::Pallet<T>>::deposit(1u32);
			let deposit = class_deposit.saturating_add(data_deposit);
			let total_deposit = proxy_deposit.saturating_add(deposit);

			// https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/balances/src/lib.rs#L965
			// Now the pallet-balances judges whether provider is based on the `free balance` instead of
			// `total balance`. When there's no other providers, error will throw in following reserve
			// operation, which want to make `free balance` zero and `reserved balance` not zero.
			// If receiver account doesn't have enough ED, transfer an additional ED to make sure of the subsequent
			// reserve operation.
			let total_transfer_amount =
				total_deposit.saturating_add(<T as module::Config>::Currency::minimum_balance());

			// ensure enough token for proxy deposit + class deposit + data deposit + ed
			<T as module::Config>::Currency::transfer(&who, &owner, total_transfer_amount, KeepAlive)?;

			<T as module::Config>::Currency::reserve_named(&RESERVE_ID, &owner, deposit)?;

			// owner add proxy delegate to origin
			<pallet_proxy::Pallet<T>>::add_proxy_delegate(&owner, who, Default::default(), Zero::zero())?;

			let data = ClassData {
				deposit,
				properties,
				attributes,
			};
			Self::do_create_class(&owner, metadata, data)?;

			Self::deposit_event(Event::CreatedClass {
				owner,
				class_id: next_id,
			});
			Ok(().into())
		}

		/// Mint NFT token
		///
		/// - `to`: the token owner's account
		/// - `class_id`: token belong to the class id
		/// - `metadata`: external metadata
		/// - `quantity`: token quantity
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::mint(*quantity))]
		pub fn mint(
			origin: OriginFor<T>,
			to: <T::Lookup as StaticLookup>::Source,
			class_id: ClassIdOf<T>,
			metadata: CID,
			attributes: Attributes,
			#[pallet::compact] quantity: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let to = T::Lookup::lookup(to)?;
			Self::mint_token(&who, &to, class_id, metadata, attributes, quantity)?;
			Ok(())
		}

		/// Transfer NFT token to another account
		///
		/// - `to`: the token owner's account
		/// - `token`: (class_id, token_id)
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: <T::Lookup as StaticLookup>::Source,
			token: (ClassIdOf<T>, TokenIdOf<T>),
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let to = T::Lookup::lookup(to)?;
			Self::transfer_token(&who, &to, token)
		}

		/// Burn NFT token
		///
		/// - `token`: (class_id, token_id)
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::burn())]
		pub fn burn(origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::burn_token(who, token, None)
		}

		/// Burn NFT token
		///
		/// - `token`: (class_id, token_id)
		/// - `remark`: Vec<u8>
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::burn_with_remark(remark.len() as u32))]
		pub fn burn_with_remark(
			origin: OriginFor<T>,
			token: (ClassIdOf<T>, TokenIdOf<T>),
			remark: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::burn_token(who, token, Some(remark))
		}

		/// Destroy NFT class, remove dest from proxy, and send all the free
		/// balance to dest
		///
		/// - `class_id`: The class ID to destroy
		/// - `dest`: The proxy account that will receive free balance
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::destroy_class())]
		pub fn destroy_class(
			origin: OriginFor<T>,
			class_id: ClassIdOf<T>,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;
			let class_info = Self::classes(class_id).ok_or(Error::<T>::ClassIdNotFound)?;
			ensure!(who == class_info.owner, Error::<T>::NoPermission);
			ensure!(
				class_info.total_issuance == Zero::zero(),
				Error::<T>::CannotDestroyClass
			);

			let data = class_info.data;

			<T as module::Config>::Currency::unreserve_named(&RESERVE_ID, &who, data.deposit);

			Self::do_destroy_class(&who, class_id)?;

			// this should unresere proxy deposit
			pallet_proxy::Pallet::<T>::remove_proxy_delegate(&who, dest.clone(), Default::default(), Zero::zero())?;

			<T as module::Config>::Currency::transfer(
				&who,
				&dest,
				<T as module::Config>::Currency::free_balance(&who),
				AllowDeath,
			)?;

			Self::deposit_event(Event::DestroyedClass { owner: who, class_id });
			Ok(().into())
		}

		/// Update NFT class properties. The current class properties must contains
		/// ClassPropertiesMutable.
		///
		/// - `class_id`: The class ID to update
		/// - `properties`: The new properties
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::update_class_properties())]
		pub fn update_class_properties(
			origin: OriginFor<T>,
			class_id: ClassIdOf<T>,
			properties: Properties,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Classes::<T>::try_mutate(class_id, |class_info| {
				let class_info = class_info.as_mut().ok_or(Error::<T>::ClassIdNotFound)?;
				ensure!(who == class_info.owner, Error::<T>::NoPermission);

				let data = &mut class_info.data;
				ensure!(
					data.properties.0.contains(ClassProperty::ClassPropertiesMutable),
					Error::<T>::Immutable
				);

				data.properties = properties;

				Ok(())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	// Internal functions ported from module-nft

	pub fn do_create_class(
		owner: &T::AccountId,
		metadata: Vec<u8>,
		data: ClassData<BalanceOf<T>>,
	) -> Result<T::ClassId, DispatchError> {
		let bounded_metadata: BoundedVec<u8, T::MaxClassMetadata> =
			metadata.try_into().map_err(|_| Error::<T>::MaxMetadataExceeded)?;

		let class_id = NextClassId::<T>::try_mutate(|id| -> Result<T::ClassId, DispatchError> {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableClassId)?;
			Ok(current_id)
		})?;

		let info = ClassInfo {
			metadata: bounded_metadata,
			total_issuance: Default::default(),
			owner: owner.clone(),
			data,
		};
		Classes::<T>::insert(class_id, info);

		Ok(class_id)
	}

	pub fn do_mint(
		owner: &T::AccountId,
		class_id: T::ClassId,
		metadata: Vec<u8>,
		data: TokenData<BalanceOf<T>>,
	) -> Result<T::TokenId, DispatchError> {
		NextTokenId::<T>::try_mutate(class_id, |id| -> Result<T::TokenId, DispatchError> {
			let bounded_metadata: BoundedVec<u8, T::MaxTokenMetadata> =
				metadata.try_into().map_err(|_| Error::<T>::MaxMetadataExceeded)?;

			let token_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableTokenId)?;

			Classes::<T>::try_mutate(class_id, |class_info| -> DispatchResult {
				let info = class_info.as_mut().ok_or(Error::<T>::ClassIdNotFound)?;
				info.total_issuance = info
					.total_issuance
					.checked_add(&One::one())
					.ok_or(ArithmeticError::Overflow)?;
				Ok(())
			})?;

			let token_info = TokenInfo {
				metadata: bounded_metadata,
				owner: owner.clone(),
				data,
			};
			Tokens::<T>::insert(class_id, token_id, token_info);
			TokensByOwner::<T>::insert((owner, class_id, token_id), ());

			Ok(token_id)
		})
	}

	pub fn do_transfer_logic(from: &T::AccountId, to: &T::AccountId, token: (T::ClassId, T::TokenId)) -> DispatchResult {
		Tokens::<T>::try_mutate(token.0, token.1, |token_info| -> DispatchResult {
			let info = token_info.as_mut().ok_or(Error::<T>::TokenIdNotFound)?;
			ensure!(info.owner == *from, Error::<T>::NoPermission);
			if from == to {
				// no change needed
				return Ok(());
			}

			info.owner = to.clone();

			TokensByOwner::<T>::remove((from, token.0, token.1));
			TokensByOwner::<T>::insert((to, token.0, token.1), ());

			Ok(())
		})
	}

	pub fn do_burn_logic(owner: &T::AccountId, token: (T::ClassId, T::TokenId)) -> DispatchResult {
		Tokens::<T>::try_mutate_exists(token.0, token.1, |token_info| -> DispatchResult {
			let t = token_info.take().ok_or(Error::<T>::TokenIdNotFound)?;
			ensure!(t.owner == *owner, Error::<T>::NoPermission);

			Classes::<T>::try_mutate(token.0, |class_info| -> DispatchResult {
				let info = class_info.as_mut().ok_or(Error::<T>::ClassIdNotFound)?;
				info.total_issuance = info
					.total_issuance
					.checked_sub(&One::one())
					.ok_or(ArithmeticError::Overflow)?;
				Ok(())
			})?;

			TokensByOwner::<T>::remove((owner, token.0, token.1));

			Ok(())
		})
	}

	pub fn do_destroy_class(owner: &T::AccountId, class_id: T::ClassId) -> DispatchResult {
		Classes::<T>::try_mutate_exists(class_id, |class_info| -> DispatchResult {
			let info = class_info.take().ok_or(Error::<T>::ClassIdNotFound)?;
			ensure!(info.owner == *owner, Error::<T>::NoPermission);
			ensure!(info.total_issuance == Zero::zero(), Error::<T>::CannotDestroyClass);

			NextTokenId::<T>::remove(class_id);

			Ok(())
		})
	}

	pub fn is_owner(account: &T::AccountId, token: (T::ClassId, T::TokenId)) -> bool {
		TokensByOwner::<T>::contains_key((account, token.0, token.1))
	}

	// Wrapper functions for high-level logic (renamed from old impl to avoid conflicts)

	#[require_transactional]
	pub fn transfer_token(from: &T::AccountId, to: &T::AccountId, token: (ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult {
		let class_info = Self::classes(token.0).ok_or(Error::<T>::ClassIdNotFound)?;
		let data = class_info.data;
		ensure!(
			data.properties.0.contains(ClassProperty::Transferable),
			Error::<T>::NonTransferable
		);

		let token_info = Self::tokens(token.0, token.1).ok_or(Error::<T>::TokenIdNotFound)?;

		Self::do_transfer_logic(from, to, token)?;

		let reserve_balance = token_info.data.deposit;

		// https://github.com/paritytech/substrate/blob/569aae5341ea0c1d10426fa1ec13a36c0b64393b/frame/balances/src/lib.rs#L965
		// Now the pallet-balances judges whether provider is based on the `free balance` instead of
		// `total balance`. When there's no other providers, error will throw in following reserve
		// operation, which want to make `free balance` zero and `reserved balance` not zero.
		// If receiver account doesn't have enough ED, transfer an additional ED to make sure of the subsequent
		// reserve operation.
		let transfer_amount =
			if <T as module::Config>::Currency::free_balance(to) < <T as module::Config>::Currency::minimum_balance() {
				reserve_balance.saturating_add(<T as module::Config>::Currency::minimum_balance())
			} else {
				reserve_balance
			};

		<T as module::Config>::Currency::unreserve_named(&RESERVE_ID, from, reserve_balance);
		<T as module::Config>::Currency::transfer(from, to, transfer_amount, AllowDeath)?;
		<T as module::Config>::Currency::reserve_named(&RESERVE_ID, to, reserve_balance)?;

		Self::deposit_event(Event::TransferredToken {
			from: from.clone(),
			to: to.clone(),
			class_id: token.0,
			token_id: token.1,
		});
		Ok(())
	}

	#[require_transactional]
	pub fn mint_token(
		who: &T::AccountId,
		to: &T::AccountId,
		class_id: ClassIdOf<T>,
		metadata: CID,
		attributes: Attributes,
		quantity: u32,
	) -> DispatchResult {
		ensure!(quantity >= 1, Error::<T>::InvalidQuantity);
		let class_info = Self::classes(class_id).ok_or(Error::<T>::ClassIdNotFound)?;
		ensure!(who == &class_info.owner, Error::<T>::NoPermission);
		let data = class_info.data;
		ensure!(
			data.properties.0.contains(ClassProperty::Mintable),
			Error::<T>::NonMintable
		);

		let deposit = T::CreateTokenDeposit::get();
		let data_deposit = Self::data_deposit(&metadata, &attributes)?;
		let total_deposit = deposit.saturating_add(data_deposit);

		let token_data = TokenData {
			deposit: total_deposit,
			attributes,
		};

		for _ in 0..quantity {
			<T as module::Config>::Currency::reserve_named(&RESERVE_ID, to, total_deposit)?;
			let token_id = Self::do_mint(to, class_id, metadata.clone(), token_data.clone())?;
			Self::deposit_event(Event::MintedToken {
				from: who.clone(),
				to: to.clone(),
				class_id,
				quantity,
			});
			Self::deposit_event(Event::TransferredToken {
				from: who.clone(),
				to: to.clone(),
				class_id,
				token_id,
			});
		}
		Ok(())
	}

	#[require_transactional]
	pub fn burn_token(who: T::AccountId, token: (ClassIdOf<T>, TokenIdOf<T>), remark: Option<Vec<u8>>) -> DispatchResult {
		let class_info = Self::classes(token.0).ok_or(Error::<T>::ClassIdNotFound)?;
		let data = class_info.data;
		ensure!(
			data.properties.0.contains(ClassProperty::Burnable),
			Error::<T>::NonBurnable
		);

		let token_info = Self::tokens(token.0, token.1).ok_or(Error::<T>::TokenIdNotFound)?;
		ensure!(who == token_info.owner, Error::<T>::NoPermission);

		Self::do_burn_logic(&who, token)?;

		<T as module::Config>::Currency::unreserve_named(&RESERVE_ID, &who, token_info.data.deposit);

		if let Some(remark) = remark {
			let hash = T::Hashing::hash(&remark);
			Self::deposit_event(Event::BurnedTokenWithRemark {
				owner: who,
				class_id: token.0,
				token_id: token.1,
				remark_hash: hash,
			});
		} else {
			Self::deposit_event(Event::BurnedToken {
				owner: who,
				class_id: token.0,
				token_id: token.1,
			});
		}
		Ok(())
	}

	pub fn data_deposit(metadata: &[u8], attributes: &Attributes) -> Result<BalanceOf<T>, DispatchError> {
		let metadata_len = metadata.len() as u32;
		let attributes_len = attributes.iter().fold(0, |acc, (k, v)| acc + k.len() + v.len()) as u32;

		ensure!(attributes_len <= T::MaxAttributesBytes::get(), Error::<T>::AttributesTooLarge);

		let total_len = metadata_len.saturating_add(attributes_len);
		Ok(T::DataDepositPerByte::get().saturating_mul(total_len.into()))
	}
}

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type ItemId = T::TokenId;
	type CollectionId = T::ClassId;

	fn owner(collection: &Self::CollectionId, item: &Self::ItemId) -> Option<T::AccountId> {
		Self::tokens(collection, item).map(|t| t.owner)
	}

	fn collection_owner(collection: &Self::CollectionId) -> Option<T::AccountId> {
		Self::classes(collection).map(|c| c.owner)
	}

	fn attribute(collection: &Self::CollectionId, item: &Self::ItemId, key: &[u8]) -> Option<Vec<u8>> {
		Self::tokens(collection, item).and_then(|t| {
			t.data.attributes.iter().find(|(k, _)| k.as_slice() == key).map(|(_, v)| v.clone())
		})
	}

	fn collection_attribute(collection: &Self::CollectionId, key: &[u8]) -> Option<Vec<u8>> {
		Self::classes(collection).and_then(|c| {
			c.data.attributes.iter().find(|(k, _)| k.as_slice() == key).map(|(_, v)| v.clone())
		})
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	fn transfer(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		destination: &T::AccountId,
	) -> DispatchResult {
		let token_info = Self::tokens(collection, item).ok_or(Error::<T>::TokenIdNotFound)?;
		let from = token_info.owner;
		Self::transfer_token(&from, destination, (*collection, *item))
	}
}

impl<T: Config> InspectExtended<T::AccountId> for Pallet<T> {
	type Balance = u64;

	fn balance(who: &T::AccountId) -> Self::Balance {
		TokensByOwner::<T>::iter_key_prefix((who,)).count() as u64
	}

	fn next_token_id(class: Self::CollectionId) -> Self::ItemId {
		Self::next_token_id(class)
	}
}
