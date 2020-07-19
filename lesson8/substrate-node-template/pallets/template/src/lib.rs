#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed, offchain::{
	AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,
}};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;
use core::convert::TryInto;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw1");

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>>  {
	// Add other types and constants required to configure this pallet.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Numbers get(fn numbers): map hasher(blake2_128_concat) u64 => u64;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		NumberAppended(AccountId, u64, u64),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		SignedSubmitNumberError,
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

		#[weight = 10_000]
		pub fn save_number(origin, index: u64, number: u64) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			Self::do_save_number(index, number);
            Self::deposit_event(RawEvent::NumberAppended(who, index, number));

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers: {:?}", block_number);

			Self::do_offchain_worker(block_number);
		}

	}
}


impl<T: Trait> Module<T> {
	fn do_save_number(index: u64, number: u64) {
		Numbers::insert(index, number);
	}

	fn do_offchain_worker(block_number: T::BlockNumber) {
		let index = block_number.try_into().ok().unwrap() as u64;
		let latest = if index > 0 {
			Self::numbers((index - 1) as u64)
		} else {
			0 as u64
		};

		let new: u64 = latest.saturating_add((index + 1).saturating_pow(2));

		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			debug::error!("No local account available");
			return;
		}

		let results = signer.send_signed_transaction(|_acct| {
			// We are just submitting the current block number back on-chain
			Call::save_number(index, new)
		});

		for (_acc, res) in &results {
			match res {
				Ok(()) => {
					debug::native::info!("off-chain tx succeeded: number: {}", new);
				}
				Err(_e) => {
					debug::error!("off-chain tx failed: number: {}", new);
				}
			};
		}
	}
}
