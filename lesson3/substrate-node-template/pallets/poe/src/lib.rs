#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure, decl_module, decl_storage, decl_event, decl_error,
    dispatch, traits::{Get, Currency, ExistenceRequirement}};

use frame_support::weights::{Weight, DispatchClass, FunctionOf, Pays};

use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;

    /// The minimum length a claim may be.
    type MinLength: Get<usize>;
    /// The maximum length a claim may be.
    type MaxLength: Get<usize>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Poe {
		Proofs get(fn get_proof): map hasher(blake2_128_concat) Vec<u8> => Option<(T::AccountId, T::BlockNumber)>;
		Prices get(fn get_price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
		BlockNumber = <T as system::Trait>::BlockNumber,
		Price = BalanceOf<T>,
	{
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>, BlockNumber),
		/// ClaimTransferred: from, to, claim, block_number
		ClaimTransferred(AccountId, AccountId, Vec<u8>, BlockNumber),
		ClaimAsked(AccountId, Vec<u8>, Price),
		/// ClaimSold: claim was transferred from, to, claim
		ClaimSold(AccountId, AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ProofNotExist,
		NotClaimOwner,
		/// A claim is too short.
		TooShort,
		/// A claim is too long.
		TooLong,
		PriceTooLow,
		ClaimNotForSale,
		CannotBuyYourOwnClaim,
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

		/// The minimum length a claim may be.
		const MinLength: u32 = T::MinLength::get() as u32;

		/// The maximum length a claim may be.
		const MaxLength: u32 = T::MaxLength::get() as u32;

		#[weight = 10_000]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(claim.len() >= T::MinLength::get(), Error::<T>::TooShort);
			ensure!(claim.len() <= T::MaxLength::get(), Error::<T>::TooLong);

			let o = Self::get_proof(&claim);
			ensure!(None == o, Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}

		#[weight = 10_000]
		pub fn revoke_claim(origin, claim:Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_acc, bn) = Self::must_get_with_owner(&sender, &claim)?;
			Proofs::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim, bn));
			Ok(())
		}

		#[weight = 10_000]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_acc, _bn) = Self::must_get_with_owner(&sender, &claim)?;
			let dest = T::Lookup::lookup(dest)?;
			let bn = system::Module::<T>::block_number();
			Proofs::<T>::insert(&claim, (dest.clone(), bn));
			Self::deposit_event(RawEvent::ClaimTransferred(sender, dest, claim, bn));
			Ok(())
		}

		#[weight = FunctionOf(
			|(claim, _): (&Vec<u8>, &BalanceOf<T>)| (claim.len() + 10_000) as Weight,
			DispatchClass::Normal,
			Pays::Yes,
		)]
		pub fn ask_claim(origin, claim: Vec<u8>, price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_acc, _bn) = Self::must_get_with_owner(&sender, &claim)?;
			let old_price = Self::get_price(&claim);
			if old_price != price {
				Prices::<T>::insert(&claim, &price);
				Self::deposit_event(RawEvent::ClaimAsked(sender, claim, price));
			}
			Ok(())
		}

		#[weight = FunctionOf(
			|(claim, _): (&Vec<u8>, &BalanceOf<T>)| (claim.len() * 2 + 10_000) as Weight,
			DispatchClass::Normal,
			Pays::Yes,
		)]
		pub fn buy_claim(origin, claim: Vec<u8>, buy_price: BalanceOf<T>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let o = Self::get_proof(&claim);
			ensure!(None != o, Error::<T>::ProofNotExist);

			let ask_price = Self::get_price(&claim);
			ensure!(ask_price > 0.into(), Error::<T>::ClaimNotForSale);
			ensure!(buy_price >= ask_price, Error::<T>::PriceTooLow);

			let (owner, _bn) = o.expect("Not None ;qed");
			ensure!(owner != sender, Error::<T>::CannotBuyYourOwnClaim);
			T::Currency::transfer(&sender, &owner, ask_price, ExistenceRequirement::KeepAlive)?;

			let bn = system::Module::<T>::block_number();
			Proofs::<T>::insert(&claim, (sender.clone(), bn));
			Prices::<T>::remove(&claim);
			Self::deposit_event(RawEvent::ClaimSold(owner, sender, claim));
			Ok(())
		}
	}
}

impl<T> Module<T> where T: Trait {
    pub(crate) fn must_get_with_owner(sender: &T::AccountId, claim: &Vec<u8>) -> Result<(T::AccountId, T::BlockNumber), dispatch::DispatchError> {
        let o = Self::get_proof(&claim);
        ensure!(None != o, Error::<T>::ProofNotExist);
        let (acc, bn) = o.expect("must be a Some ;qed");
        ensure!(&acc == sender, Error::<T>::NotClaimOwner);
        Ok((acc, bn))
    }
}

