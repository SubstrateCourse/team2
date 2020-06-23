#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_error, ensure, StorageValue, StorageMap, 
	traits::{Randomness, Currency, ExistenceRequirement}};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError, DispatchResult, traits::StaticLookup};

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
	type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;
		/// Get the owner of kitties by kitty_id
		pub KittiesOwner get(fn kitties_owner): map hasher(blake2_128_concat) u32 => T::AccountId;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) (T::AccountId, u32) => u32;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(fn owned_kitties_count): map hasher(blake2_128_concat) T::AccountId => u32;
		/// Get price of kitties by kitty_id
		pub Prices get(fn prices): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		KittyNotExist,
		NotKittyOwner,
		CannotBuySelfOwnedKitty,
		KittyPriceNotSet,
		OfferPriceTooLow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		/// Create a new kitty
		#[weight = 0]
		pub fn create_kitty(origin) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;

			// Generate a random 128bit value
			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);

			// 作业：补完剩下的部分
			Self::insert_kitty(sender.clone(), &kitty_id, &kitty);
			KittiesOwner::<T>::insert(&kitty_id, sender.clone());

			Ok(())
		}

		/// Breed kitties
		#[weight = 0]
		pub fn breed_kitty(origin, kitty_id_1: u32, kitty_id_2: u32) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		// Transfer kitties
		#[weight = 0]
		pub fn transfer_kitty(origin, kitty_id: u32, dest: <T::Lookup as StaticLookup>::Source) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::KittyNotExist);

			let owner = KittiesOwner::<T>::get(&kitty_id);

			ensure!(owner == sender, Error::<T>::NotKittyOwner);

			let dest = T::Lookup::lookup(dest)?;

			Self::exchange_kitty(sender.clone(), kitty_id, dest.clone());

			Ok(())
		}

		// Owner can set the kitty's price
		#[weight = 0]
		pub fn set_kitty_price(origin, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::KittyNotExist);

			let owner = KittiesOwner::<T>::get(&kitty_id);

			ensure!(owner == sender, Error::<T>::NotKittyOwner);

			Prices::<T>::insert(&kitty_id, price);

			Ok(())
		}

		// User can purchase the kitty if offer_price is not lower than the set price
		#[weight = 0]
		pub fn purchase_kitty(origin, kitty_id: u32, offer_price: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::KittyNotExist);

			let owner = KittiesOwner::<T>::get(&kitty_id);

			ensure!(owner != sender, Error::<T>::CannotBuySelfOwnedKitty);

			ensure!(Prices::<T>::contains_key(&kitty_id), Error::<T>::KittyPriceNotSet);

			let kitty_price = Prices::<T>::get(&kitty_id);

			ensure!(offer_price >= kitty_price, Error::<T>::OfferPriceTooLow);

			T::Currency::transfer(&sender, &owner, kitty_price, ExistenceRequirement::KeepAlive)?;

			Self::exchange_kitty(owner.clone(), kitty_id, sender.clone());
			
			Ok(())
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		// 作业：完成方法
		let payload = (
			<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> sp_std::result::Result<u32, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == u32::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: &u32, kitty: &Kitty) {
		// 作业：完成方法
		Kitties::insert(kitty_id, kitty);
		KittiesCount::put(kitty_id + 1);

		let owned_kitties_count = Self::owned_kitties_count(&owner);
		OwnedKitties::<T>::insert((owner.clone(), owned_kitties_count), kitty_id);
		OwnedKittiesCount::<T>::insert(owner.clone(), owned_kitties_count + 1);
	}

	fn exchange_kitty(owner_old: T::AccountId, kitty_id: u32, owner_new: T::AccountId) {
		let owned_kitties_count_owner_old = Self::owned_kitties_count(&owner_old);
		let owned_kitties_count_owner_new = Self::owned_kitties_count(&owner_new);
		OwnedKitties::<T>::remove((&owner_old, owned_kitties_count_owner_old));
		OwnedKittiesCount::<T>::insert(&owner_old, owned_kitties_count_owner_old - 1);
		OwnedKitties::<T>::insert((&owner_new, owned_kitties_count_owner_new), &kitty_id);
		OwnedKittiesCount::<T>::insert(&owner_new, owned_kitties_count_owner_new + 1);
		KittiesOwner::<T>::insert(&kitty_id, &owner_new);
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: u32, kitty_id_2: u32) -> DispatchResult {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender.clone(), &kitty_id, &Kitty(new_dna));

		Ok(())
	}
}
