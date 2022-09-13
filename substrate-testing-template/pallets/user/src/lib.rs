#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

mod types;
use types::UserData;
use types::UserStorage;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::storage]
	pub type User<T: Config> = StorageMap <
		_,
		Blake2_128Concat,
		T::AccountId,
		crate::UserData,
		ValueQuery,
		GetDefault
	>;

	#[pallet::storage]
	pub type FriendList<T: Config> = StorageMap <
		_,
		Blake2_128Concat,
		T::AccountId,
		Vec<T::AccountId>,
		ValueQuery,
		GetDefault
	>; 

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Event generated when new user is registered
		/// parameters, [name, accountid]
		RegisterUser{ name: Vec<u8>, user_address: T::AccountId},

		/// Event generated when user signin
		/// parameters, [name, accountid]
		SignedIn{ name: Vec<u8>, user_address: T::AccountId},

		/// Event generated when user signedout
		/// parameters, [name, accountid]
		SignedOut{ name: Vec<u8>, user_address: T::AccountId},

		/// Event generated when user is removed from record
		/// parameters, [name, accountid]
		UserDeleted{ name: Vec<u8>, user_address: T::AccountId},

		/// Event generated when user is details is updated
		/// parameters, [accountid, email_id, mobile_number, address]
		UserUpdated {
			user_address: T::AccountId, 
			email_id: Option<Vec<u8>>, 
			phone_number: Option<Vec<u8>>, 
			address: Option<Vec<u8>>, 
		},
		/// Event generted when new friend is added via any user
		///  parameters, [accountId, accountid]
		AddedFriend {
			from: T::AccountId,
			friend: T::AccountId
		},
		/// Event generted when existing friend is deleted via any user
		///  parameters, [accountId, accountid]
		DeletedFriend {
			from: T::AccountId,
			friend: T::AccountId
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Errors thrown when same address is trying to register again
		AddressAlreadyRegistered,
		/// Errors throewn when address is not registered
		AddressNotRegistered,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_user(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let signer = ensure_signed(origin)?;

			ensure!(!<User<T>>::contains_key(&signer), Error::<T>::AddressAlreadyRegistered);

			crate::UserStorage::<T>::insert_new(&signer, name.clone())?;
	
			// Emit an event.
			Self::deposit_event(Event::RegisterUser {
				name,
				user_address: signer,
			});

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn signin(origin: OriginFor<T>) -> DispatchResult {

			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);

			// Update storage.
			User::<T>::mutate(&signer, |user_record| {
				user_record.active = true;	
			});
			let name = User::<T>::get(&signer).name;
			Self::deposit_event(Event::SignedIn {
				name,
				user_address: signer,
			});

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn signout(origin: OriginFor<T>) -> DispatchResult {

			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);

			// Update storage.
			User::<T>::mutate(&signer, |user_record| {
				user_record.active = false;	
			});
			let name = User::<T>::get(&signer).name;

			Self::deposit_event(Event::SignedOut {
				name,
				user_address: signer,
			});

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_user(
			origin: OriginFor<T>, 
			email_id: Option<Vec<u8>>, 
			phone_number: Option<Vec<u8>>, 
			address: Option<Vec<u8>>, 
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);

			// Update storage.
			crate::UserStorage::<T>::update_user_data(
				&signer,
				email_id.clone(),
				phone_number.clone(),
				address.clone()
			)?;

			Self::deposit_event(Event::UserUpdated {
				user_address: signer,
				email_id,
				phone_number,
				address
			});

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn delete_user(origin: OriginFor<T>) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);

			// Update storage.
			let name = User::<T>::get(&signer).name;
			User::<T>::remove(&signer);
			Self::deposit_event(Event::UserDeleted {
				name,
				user_address: signer,
			});

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_friend(origin: OriginFor<T>, friend_id: T::AccountId) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);
			ensure!(<User<T>>::contains_key(&friend_id), Error::<T>::AddressNotRegistered);

			// Update storage.
			FriendList::<T>::mutate(&signer, |list| {
				if !list.contains(&friend_id) {
					list.push(friend_id.clone());
				}
			});
			Self::deposit_event(Event::AddedFriend {
				from: signer,
				friend: friend_id,
			});


			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn delete_friend(origin: OriginFor<T>, friend_id: T::AccountId) -> DispatchResult {
			let signer = ensure_signed(origin)?;
			ensure!(<User<T>>::contains_key(&signer), Error::<T>::AddressNotRegistered);
			ensure!(<User<T>>::contains_key(&friend_id), Error::<T>::AddressNotRegistered);
			// Update storage.
			FriendList::<T>::mutate(&signer, |list| {
				if list.contains(&friend_id) {
					list.push(friend_id.clone());
				}
			});
			Self::deposit_event(Event::DeletedFriend {
				from: signer,
				friend: friend_id,
			});

			Ok(())
		}

	}
}
