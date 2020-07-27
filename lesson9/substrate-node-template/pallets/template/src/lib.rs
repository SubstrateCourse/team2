#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use core::{convert::TryInto, fmt};
use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get,};
use frame_system::{
	self as system, ensure_none, ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer, SubmitTransaction,
	},
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{
	offchain as rt_offchain,
	offchain::storage::StorageValueRef,
	transaction_validity::{
		InvalidTransaction, TransactionPriority, TransactionSource, TransactionValidity,
		ValidTransaction,
	},
};
use sp_std::prelude::*;
use sp_std::str;
// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// We are fetching information from github public API about organisation `substrate-developer-hub`.
//pub const HTTP_REMOTE_REQUEST_ETH_COINMARKETCAP: &[u8] = b"https://pro-api.coinmarketcap.com";
pub const HTTP_REMOTE_REQUEST_ETH_COINCAP: &[u8] = b"api.coincap.io/v2/assets/ethereum";

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct EthereumInfoCoincap {
	priceUsd: u32,
}

impl fmt::Debug for EthereumInfoCoincap {
	// `fmt` converts the vector of bytes inside the struct back to string for
	//   more friendly display.
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{{ priceUsd: {} }}",
			&self.priceUsd
		)
	}
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	// Add other types and constants required to configure this pallet.
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The type to sign and send transactions.
	type UnsignedPriority: Get<TransactionPriority>;
}

// Custom data type
#[derive(Debug)]
enum TransactionType {
	SignedSubmitNumber,
	UnsignedSubmitNumber,
	HttpFetching,
	None,
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
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
		/// Error returned when making remote http fetching
		HttpFetchingError,
		/// Error returned when gh-info has already been fetched
		AlreadyFetched,
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
		pub fn save_number(origin, number: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
		}

	}
}

impl<T: Trait> Module<T> {
	/// Check if we have fetched github info before. If yes, we use the cached version that is
	///   stored in off-chain worker storage `storage`. If no, we fetch the remote info and then
	///   write the info into the storage for future retrieval.
	fn fetch_if_needed() -> Result<(), Error<T>> {
		// Start off by creating a reference to Local Storage value.
		// Since the local storage is common for all offchain workers, it's a good practice
		// to prepend our entry with the pallet name.
		let s_info = StorageValueRef::persistent(b"template::gh-info");
		let s_lock = StorageValueRef::persistent(b"template::lock");

		// The local storage is persisted and shared between runs of the offchain workers,
		// and offchain workers may run concurrently. We can use the `mutate` function, to
		// write a storage entry in an atomic fashion.
		//
		// It has a similar API as `StorageValue` that offer `get`, `set`, `mutate`.
		// If we are using a get-check-set access pattern, we likely want to use `mutate` to access
		// the storage in one go.
		//
		// Ref: https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/offchain/storage/struct.StorageValueRef.html
		if let Some(Some(ether_info)) = s_info.get::<EthereumInfoCoincap>() {
			// gh-info has already been fetched. Return early.
			debug::info!("cached ether-info: {:?}", ether_info);
			return Ok(());
		}

		// We are implementing a mutex lock here with `s_lock`
		let res: Result<Result<bool, bool>, Error<T>> = s_lock.mutate(|s: Option<Option<bool>>| {
			match s {
				// `s` can be one of the following:
				//   `None`: the lock has never been set. Treated as the lock is free
				//   `Some(None)`: unexpected case, treated it as AlreadyFetch
				//   `Some(Some(false))`: the lock is free
				//   `Some(Some(true))`: the lock is held

				// If the lock has never been set or is free (false), return true to execute `fetch_n_parse`
				None | Some(Some(false)) => Ok(true),

				// Otherwise, someone already hold the lock (true), we want to skip `fetch_n_parse`.
				// Covering cases: `Some(None)` and `Some(Some(true))`
				_ => Err(<Error<T>>::AlreadyFetched),
			}
		});

		// Cases of `res` returned result:
		//   `Err(<Error<T>>)` - lock is held, so we want to skip `fetch_n_parse` function.
		//   `Ok(Err(true))` - Another ocw is writing to the storage while we set it,
		//                     we also skip `fetch_n_parse` in this case.
		//   `Ok(Ok(true))` - successfully acquire the lock, so we run `fetch_n_parse`
		if let Ok(Ok(true)) = res {
			match Self::fetch_n_parse() {
				Ok(ether_info) => {
					// set gh-info into the storage and release the lock
					s_info.set(&ether_info);
					s_lock.set(&false);

					debug::info!("fetched ether-info: {:?}", ether_info);
				}
				Err(err) => {
					// release the lock
					s_lock.set(&false);
					return Err(err);
				}
			}
		}
		Ok(())
	}

	/// This function uses the `offchain::http` API to query the ether price information,
	///   and returns the JSON response as vector of bytes.
	fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
		let remote_url_bytes = HTTP_REMOTE_REQUEST_ETH_COINCAP.to_vec();
		let remote_url =
			str::from_utf8(&remote_url_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;

		debug::info!("sending request to: {}", remote_url);

		// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
		let request = rt_offchain::http::Request::get(remote_url);

		// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
		let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

		// API request for Coincap Ether price
		let pending = request
			.deadline(timeout) // Setting the timeout time
			.send() // Sending the request out by the host
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		// By default, the http request is async from the runtime perspective. So we are asking the
		//   runtime to wait here.
		// The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
		//   ref: https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
		let response = pending
			.try_wait(timeout)
			.map_err(|_| <Error<T>>::HttpFetchingError)?
			.map_err(|_| <Error<T>>::HttpFetchingError)?;

		if response.code != 200 {
			debug::error!("Unexpected http request status code: {}", response.code);
			return Err(<Error<T>>::HttpFetchingError);
		}

		// Next we fully read the response body and collect it to a vector of bytes.
		Ok(response.body().collect::<Vec<u8>>())
	}

	/// Fetch from remote and deserialize the JSON to a struct
	fn fetch_n_parse() -> Result<EthereumInfoCoincap, Error<T>> {
		let resp_bytes = Self::fetch_from_remote().map_err(|e| {
			debug::error!("fetch_from_remote error: {:?}", e);
			<Error<T>>::HttpFetchingError
		})?;

		let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
		// Print out our fetched JSON string
		debug::info!("{}", resp_str);

		// Deserializing JSON to struct, thanks to `serde` and `serde_derive`
		let ether_info: EthereumInfoCoincap =
			serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
		Ok(ether_info)
	}
}
