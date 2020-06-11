#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::Get,
	traits::{Currency, ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.
	type Currency: Currency<Self::AccountId>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		ProofsOfPrice get(fn proofsprice): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
		Price = BalanceOf<T>,
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimPriceCreated(AccountId, Vec<u8>, Price),
		ClaimBought(AccountId, AccountId, Vec<u8>, Price), //buyer owner
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		InsufficientBalance,
		BidTooLow,
		PriceAlreadySet,
		ClaimPriceNotSet
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		// 第二题答案
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			let dest = T::Lookup::lookup(dest)?;

			Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));

			Ok(())
		}

		#[weight = 0]
		pub fn create_claim_price(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			ensure!(!ProofsOfPrice::<T>::contains_key(&claim), Error::<T>::PriceAlreadySet);

			ProofsOfPrice::<T>::insert(&claim, price);


			Self::deposit_event(RawEvent::ClaimPriceCreated(sender, claim, price));

			Ok(())
		}

		#[weight = 0]
		pub fn buy_claim(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			ensure!(ProofsOfPrice::<T>::contains_key(&claim), Error::<T>::ClaimPriceNotSet);

			let std_price = ProofsOfPrice::<T>::get(&claim);
			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(price >= std_price, Error::<T>::BidTooLow);

			T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive)?;

			Proofs::<T>::insert(&claim, (&sender, system::Module::<T>::block_number()));
			ProofsOfPrice::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimBought(sender, owner, claim, price));


			Ok(())
		}
	}
}
