#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use core::{convert::TryInto};
use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed,
    offchain:: {
        AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer
    },    
};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;

use sp_runtime::{
    offchain as rt_offchain,
    offchain::storage::StorageValueRef
};

use alt_serde::{Deserialize, Deserializer};
use sp_std::prelude::*;
use sp_std::str;
use parity_scale_codec::{Decode, Encode};


pub const HTTP_REMOTE_REQUEST_BYTES: &[u8] = b"https://www.mxc.ai/open/api/v1/data/depth?market=BTC_USDT&depth=1";
pub const HTTP_HEADER_USER_AGENT: &[u8] = b"lesson9";

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocw8");

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	// Add other types and constants required to configure this pallet.

    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    type Call: From<Call<Self>>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime:: {
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };

    app_crypto!(sr25519, KEY_TYPE);

    pub struct AuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for AuthId 
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
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
        Numbers get(fn number): map hasher(blake2_128_concat) u64 => u64;
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
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
        AlreadyFetched,
        HttpFetchingError,
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

            Numbers::insert(index, number);

            Self::deposit_event(RawEvent::NumberAppended(who, index, number));

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");
            Self::fetch_eth_price();
		}

	}
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct CacheInfo {
    code: u32
}

impl <T: Trait> Module<T> {
    fn fetch_eth_price() {

        let s_info = StorageValueRef::persistent(b"lesson9::eth-price");
        let s_lock = StorageValueRef::persistent(b"lesson9::lock");

        if let Some(Some(cache_info)) = s_info.get::<CacheInfo>() {
            debug::info!("cached cache_info: {:?}", cache_info.code);
        }

        let res: Result<Result<bool, bool>, Error<T>> = s_lock.mutate(|s: Option<Option<bool>>| {
            match s {
                None | Some(Some(false)) => Ok(true),
                _ => Err(<Error<T>>::AlreadyFetched),
            }
        });       

        if let Ok(Ok(true)) = res {
            match Self::fetch_eth_price_from_https() {
                Ok(info) => {
                    s_info.set(&info);
                    s_lock.set(&false);
                    debug::info!("fetched price: {:?}", info.code);
                }
                Err(_) => {
                    s_lock.set(&false);
                }
            }
        }

    }

    fn fetch_eth_price_from_https() -> Result<CacheInfo, Error<T>> {


        //just return a number from http
        

        let remote_url_bytes = HTTP_REMOTE_REQUEST_BYTES.to_vec();
		let user_agent = HTTP_HEADER_USER_AGENT.to_vec();
		let remote_url =
			str::from_utf8(&remote_url_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;

		debug::info!("sending request to: {}", remote_url);

		// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
		let request = rt_offchain::http::Request::get(remote_url);

		// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
		let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

		// For github API request, we also need to specify `user-agent` in http request header.
		//   See: https://developer.github.com/v3/#user-agent-required
		let pending = request
			.add_header(
				"User-Agent",
				str::from_utf8(&user_agent).map_err(|_| <Error<T>>::HttpFetchingError)?,
			)
			.deadline(timeout) // Setting the timeout time
			.send() // Sending the request out by the host
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		// By default, the http request is async from the runtime perspective. So we are asking the
		//   runtime to wait here.
		// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
		//   ref: https://substrate.dev/rustdocs/v2.0.0-rc2/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
		let response = pending
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpFetchingError)?
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		if response.code != 200 {
			debug::error!("Unexpected http request status code: {}", response.code);
			return Err(<Error<T>>::HttpFetchingError);
		}

		// Next we fully read the response body and collect it to a vector of bytes.
		let response_array = response.body().collect::<Vec<u8>>();

        let resp_str = str::from_utf8(&response_array).map_err(|_| <Error<T>>::HttpFetchingError)?;
		// Print out our fetched JSON string
		debug::info!("{}", resp_str);

		// Deserializing JSON to struct, thanks to `serde` and `serde_derive`
		let cache_info: CacheInfo =
			serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;


        Ok(cache_info)

    }
}
