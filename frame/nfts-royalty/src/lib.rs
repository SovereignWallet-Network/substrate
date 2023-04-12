// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

//! # NFTs Royalty Pallet
//!
//! A pallet for dealing with NFT royalties.
//!
//! ## Related Modules
//!
//! * [`System`](../frame_system/index.html)
//! * [`Support`](../frame_support/index.html)

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use scale_info::Type;
pub use types::*;
use frame_system::Config as SystemConfig;
use sp_runtime::traits::StaticLookup;

/// The log target of this pallet.
pub const LOG_TARGET: &'static str = "runtime::nfts-royalty";

type AccountIdLookupOf<T> = <<T as SystemConfig>::Lookup as StaticLookup>::Source;

// type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use sp_std::fmt::Display;
	use frame_system::pallet_prelude::*;

	use frame_support::{
		pallet_prelude::*,
		sp_runtime::Permill,
		traits::{
			tokens::{
				nonfungibles_v2::{Inspect as NonFungiblesInspect, Mutate as NonFungiblesMutate}
			},
			ReservableCurrency
		}
	};

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency mechanism, used for paying for deposits.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Identifier for the collection of NFT.
		type NftCollectionId: Member + Parameter + MaxEncodedLen + Copy + Display;

		/// The type used to identify an NFT within a collection.
		type NftItemId: Member + Parameter + MaxEncodedLen + Copy + Display;

		/// Registry for minted NFTs.
		type Nfts: NonFungiblesInspect<
				Self::AccountId,
				ItemId = Self::NftItemId,
				CollectionId = Self::NftCollectionId,
			>;
	}

	/// Keeps track of the corresponding NFT ID, royalty percentage, and royalty recipient.
	#[pallet::storage]
	#[pallet::getter(fn nft_with_royalty)]
	pub type NftWithRoyalty<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::NftCollectionId, T::NftItemId),
		RoyaltyDetails<T::AccountId>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An NFT roaylty was successfully created.
		NftRoyaltyCreated {
			nft_collection: T::NftCollectionId,
			nft: T::NftItemId,
			royalty_percentage: Permill,
			royalty_recipient: T::AccountId,
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		// errors
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {	
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn mint_item_with_royalty(
			origin: OriginFor<T>,
			collection_id: T::NftCollectionId,
			item_id: T::NftItemId,
			mint_to: AccountIdLookupOf<T>,
			royalty_percentage: Permill,
			royalty_recipient: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let mint_to = T::Lookup::lookup(mint_to)?;
			// TODO: Get this to work:
			// let item_config = ItemConfig { settings: Self::get_default_item_settings(&collection)? };
			// T::Nfts::mint_into(&collection_id, &item_id, &mint_to, item_config, false)?;

			NftWithRoyalty::<T>::insert(
				(collection_id, item_id),
				RoyaltyDetails::<T::AccountId> {
					royalty_percentage,
					royalty_recipient: royalty_recipient.clone(),
				},
			);

			Self::deposit_event(Event::NftRoyaltyCreated {
				nft_collection: collection_id,
				nft: item_id,
				royalty_percentage,
				royalty_recipient: royalty_recipient.clone(),
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// private functions
	}
}