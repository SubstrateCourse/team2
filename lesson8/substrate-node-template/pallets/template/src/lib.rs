#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

use core::{convert::TryInto};
use frame_support::{
	debug,
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	dispatch,
};
use frame_system::{
	self as system,
	ensure_signed,
	offchain::{
		Signer,
		CreateSignedTransaction,
		SendSignedTransaction,
		AppCrypto,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_std::vec::Vec;
use sp_runtime::{
		offchain::{
			http,
			Duration,
		},
};
use sp_std::prelude::*;
use sp_std;
// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct SumInfo {
    sum: u64,
}

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_application_crypto::{app_crypto, sr25519};

	app_crypto!(sr25519, KEY_TYPE);

	pub type AuthorityId = Public;
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;

}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Numbers get(fn numbers): map hasher(blake2_128_concat) u64 => u64;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
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
		pub fn save_number(origin, index: u64, number: u64) -> dispatch::DispatchResult {
			debug::info!("Entering save_number");
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			Self::add_number(who, index, number);

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			let res = Self::fetch_number_and_signed(block_number);

			if let Err(e) = res {
				debug::error!("Submit signed: Error happends: {}", e);
			}
		}
	}
}

impl<T: Trait> Module<T> {
	fn add_number(who: T::AccountId, index: u64, number: u64) {
		debug::info!("Submit signed: Adding to the number: {} to block: {}", number, index);
		Numbers::insert(index, number);

		Self::deposit_event(RawEvent::NumberAppended(who, index, number));
	}

	fn fetch_number_and_signed(block_number: T::BlockNumber) -> Result<(), &'static str> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			return Err(
				"No local accounts available. Consider adding one via `author_insertKey` RPC."
			)?
		}

		let index: u64 = block_number.try_into().ok().unwrap() as u64;

		let latest = if index > 0 {
			Self::numbers((index - 1) as u64)
		} else {
			0
		};

		let number: u64 = latest.saturating_add((index + 1).saturating_pow(2));

		// Make an external HTTP request to fetch the current price.
		// Note this call will block until response is received.
		// let number = Self::fetch_number(index).map_err(|_| "Submit signed: Failed to fetch price")?;
		// debug::info!("fetch number: {}", number);

		// Using `send_signed_transaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		// Submit signed will return a vector of results for all accounts that were found in the
		// local keystore with expected `KEY_TYPE`.
		let results = signer.send_signed_transaction(
			|_account| {
				// Received price is wrapped into a call to `submit_price` public function of this pallet.
				// This means that the transaction, when executed, will simply call that function passing
				// `price` as an argument.
				Call::save_number(index, number)
			}
		);

		for (acc, res) in &results {
			match res {
				Ok(()) => debug::info!("Submit signed: [{:?}] Submitted price of {} cents", acc.id, number),
				Err(e) => debug::error!("Submit signed: [{:?}] Failed to submit transcation, {:?}", acc.id, e),
			}
		}

		Ok(())
	}

	// fn fetch_number(index: u64) -> Result<u64, http::Error> {
	// 	let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(5000));
	// 	// Initiate an external HTTP GET request.
	// 	// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
	// 	// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
	// 	// since we are running in a custom WASM execution environment we can't simply
	// 	// import the library here.

	// 	debug::info!("index: {}", index);

	// 	let url = b"http://127.0.0.1:7000/api/v1/sum?n=";
	// 	let mut remote_url = url.to_vec();
	// 	debug::info!("remote url: {:?}", remote_url);

	// 	let n = index.to_be_bytes();
	// 	debug::info!("n: {:?}", n);

	// 	remote_url.extend(&n);
	// 	debug::info!("remote url2: {:?}", remote_url);

	// 	let remote_url_str = core::str::from_utf8(&remote_url).unwrap();
	// 	debug::info!("remote url: {}", remote_url_str);

	// 	let request = http::Request::get(
	// 		// "http://127.0.0.1:7000/api/v1/sum?n=2"
	// 		remote_url_str
	// 	);
	// 	// We set the deadline for sending of the request, note that awaiting response can
	// 	// have a separate deadline. Next we send the request, before that it's also possible
	// 	// to alter request headers or stream body content in case of non-GET requests.
	// 	let pending = request
	// 		.deadline(deadline)
	// 		.send()
	// 		.map_err(|_| http::Error::IoError)?;
	// 	// The request is already being processed by the host, we are free to do anything
	// 	// else in the worker (we can send multiple concurrent requests too).
	// 	// At some point however we probably want to check the response though,
	// 	// so we can block current thread and wait for it to finish.
	// 	// Note that since the request is being driven by the host, we don't have to wait
	// 	// for the request to have it complete, we will just not read the response.
	// 	let response = pending.try_wait(deadline)
	// 		.map_err(|_| http::Error::DeadlineReached)??;

	// 	if response.code != 200 {
	// 		debug::warn!("Submit signed: Unexpected status code: {}", response.code);
	// 		return Err(http::Error::Unknown);
	// 	}

	// 	let body = response.body().collect::<Vec<u8>>();

	// 	let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
	// 		debug::warn!("Not UTF8 body");
	// 		http::Error::Unknown
	// 	})?;

	// 	let sum_info: SumInfo = serde_json::from_str(&body_str).unwrap();
	// 	debug::warn!("Submit Signed: Got sum: {} ", sum_info.sum);

	// 	Ok(sum_info.sum)
	// }
}
