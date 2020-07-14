#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed,
                   offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer}, };
use sp_std::prelude::*;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{transaction_validity::{TransactionPriority}, SaturatedConversion};
use core::convert::TryInto;
use sp_runtime::traits::Saturating;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//This is the application key to be used as the prefix for this pallet in underlying storage.
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

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
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
    for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
    /// The type to sign and send transactions.
    type UnsignedPriority: Get<TransactionPriority>;
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
		Something get(fn something): Option<u32>;

		Number get(fn number):  Option<u64>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),

		NumberStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,

		SubmitNumberSignedError,
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
        pub fn submit_number_signed(origin, number: u64) -> dispatch::DispatchResult {
            debug::info!("submit_number_signed: {:?}", number);
            let who = ensure_signed(origin.clone())?;
            return Self::save_number(origin, number);
        }

		#[weight = 10_000]
		pub fn save_number(origin, number: u64) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/

			Number::put(number);

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			 Self::signed_submit_number(block_number);
		}

	}
}


impl<T: Trait> Module<T> {
    fn signed_submit_number(block_number: T::BlockNumber) -> Result<(), Error<T>> {
        // 2.1 取得 Signer
        let signer = Signer::<T, T::AuthorityId>::all_accounts();

        //计算值
        let original_value = Number::get();
        let latest_value;
        match original_value {
            Some(x) => latest_value = x,
            None => latest_value = 0,
        }
        let index: u64 = block_number.try_into().ok().unwrap() as u64;
        let final_number = latest_value.saturating_add((index+1).saturating_pow(2));

        // 2.2 用 Signer 调用 send_signed_transaction
        let results = signer.send_signed_transaction(|_acct| {
            // We are just submitting the current block number back on-chain
            Call::submit_number_signed(final_number)
        });

        // 2.3 查看提交交易结果
        for (acc, res) in &results {
            return match res {
                Ok(()) => {
                    debug::native::info!("success");
                    Ok(())
                }
                Err(e) => {
                    debug::error!("error");
                    Err(<Error<T>>::SubmitNumberSignedError)
                }
            };
        }
        Ok(())
    }
}