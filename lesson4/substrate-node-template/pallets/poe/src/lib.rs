#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
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

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;
	type MaxClaimCommentsLength: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,
		T::BlockNumber, Vec<u8>);
		AccountToProofHashList get(fn a2phs): map hasher(identity) T::AccountId => Vec<Vec<u8>>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		ClaimCreated(AccountId, Vec<u8>, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		CommentsTooLong,
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
		pub fn create_claim(origin, claim: Vec<u8>, comments: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
			ensure!(T::MaxClaimCommentsLength::get() >= comments.len() as u32,
			Error::<T>::CommentsTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number(),
			&comments));

			if AccountToProofHashList::<T>::contains_key(&sender) {
				let mut vec = AccountToProofHashList::<T>::get(&sender);
				match vec.binary_search(&claim) {
					Ok(_) => (),
					Err(index) => vec.insert(index, claim.clone())
				};
				AccountToProofHashList::<T>::insert(&sender, vec);
			} else {
				let mut vec = Vec::<Vec<u8>>::new();
				vec.push(claim.clone());
				AccountToProofHashList::<T>::insert(&sender, vec);
			}

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim, comments));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			if AccountToProofHashList::<T>::contains_key(&sender) {
				let mut vec = AccountToProofHashList::<T>::get(&sender);

				match vec.binary_search(&claim) {
					Ok(index) => vec.remove(index),
					Err(_) => [0].to_vec()
				};
				AccountToProofHashList::<T>::insert(&sender, vec);
			}

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		// 第二题答案
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number, comments) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			let dest = T::Lookup::lookup(dest)?;

			Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number(), comments));
			if AccountToProofHashList::<T>::contains_key(&owner) {
				let mut vec = AccountToProofHashList::<T>::get(&owner);

				match vec.binary_search(&claim) {
					Ok(index) => vec.remove(index),
					Err(_) => [0].to_vec()
				};
				AccountToProofHashList::<T>::insert(&owner, vec);
			}

			if AccountToProofHashList::<T>::contains_key(&sender) {
				let mut vec = AccountToProofHashList::<T>::get(&sender);
				match vec.binary_search(&claim) {
					Ok(_) => (),
					Err(index) => vec.insert(index, claim.clone())
				};
				AccountToProofHashList::<T>::insert(&sender, vec);
			} else {
				let mut vec = Vec::<Vec<u8>>::new();
				vec.push(claim.clone());
				AccountToProofHashList::<T>::insert(&sender, vec);
			}
			Ok(())
		}
	}
}
