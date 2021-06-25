#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Posts<T: Config> = StorageMap<_, Twox64Concat, u32, Post<T::AccountId>>;

	#[pallet::storage]
	pub type NextPost<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		// 2. Change `post()` and add `post_encrypted()`

		#[pallet::weight(0)]
		pub fn post(origin: OriginFor<T>, title: Vec<u8>, content: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let post_id = NextPost::<T>::get().unwrap_or(0);
			Posts::<T>::insert(post_id, Post::<T::AccountId> {
				title,
				content: Content::Plain(content),
				author: who,
			});

			NextPost::<T>::put(post_id + 1);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn post_encrypted(origin: OriginFor<T>, title: Vec<u8>, content: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let post_id = NextPost::<T>::get().unwrap_or(0);
			Posts::<T>::insert(post_id, Post::<T::AccountId> {
				title,
				content: Content::Encrypted(content),
				author: who,
			});

			NextPost::<T>::put(post_id + 1);
			Ok(())
		}
	}

	// 1. Define v2 structs

	use v2::{Content, Post};

	pub mod v1 {
		use codec::{Encode, Decode};
		use sp_std::vec::Vec;

		#[derive(Encode, Decode, Default, Clone)]
		pub struct Post<AccountId> {
			pub title: Vec<u8>,
			pub content: Vec<u8>,
			pub author: AccountId,
		}
	}

	// 1. Define v2 structs

	pub mod v2 {
		use codec::{Encode, Decode};
		use sp_std::vec::Vec;

		#[derive(Encode, Decode, Clone)]
		pub struct Post<AccountId> {
			pub title: Vec<u8>,
			pub content: Content,
			pub author: AccountId,
		}

		#[derive(Encode, Decode, Clone)]
		pub enum Content {
			Encrypted(Vec<u8>),
			Plain(Vec<u8>)
		}
	}
}
