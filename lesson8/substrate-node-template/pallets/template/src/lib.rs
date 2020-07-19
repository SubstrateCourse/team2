#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get,dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed,
	 offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,},};
use sp_std::prelude::*;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	transaction_validity::{TransactionPriority,},
};
use core::convert::TryInto;



#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;



//创建一个App key，这个key用于标识这个pallet的app相关存储项
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

	//以下是做测试的时候用的实现
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

		//这个存储用于记录链下发过来的数据
		pub  Numbers get(fn numbers): Vec<u32>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),

		/// Event generated when a block related number has been saved.
		NumberSaved(Option<AccountId>, u64),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,

		/// Error returned when making signed transactions in off-chain worker
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


		//此函数作为链下ocw连接线上的entry point，接收传过来的数值，并存储到存储里
		//这里接收一个vector,然后一个一个用save_number做线上存储修改
		#[weight = 0]
		pub fn submit_number_signed(origin, numbers_vec: Vec<u32>) -> DispatchResult {
			debug::info!("submit_number_signed: {:?}", numbers_vec);
			let who = ensure_signed(origin.clone())?;
			Self::save_number(origin, numbers_vec);

			Ok(())
		}


		//这个函数修改存储Numbers这个线上vec
		#[weight = 10_000]
		pub fn save_number(origin, numbers_vec: Vec<u32>) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			let new_len = numbers_vec.len();	//传入数组长度

			Numbers::mutate(|numbers| {
			/*******
			 * 学员们在这里追加逻辑
			 *******/

			let original_len = numbers.len();		//原数组长度

			let gap = new_len - original_len;	//看新数组是不是比原数组长

			//如果新数组比原数组长的话，那就把新的内容push进去
			if gap > 0 {



				for i in original_len..new_len{

					numbers.push(numbers_vec[i]);

				}

			}
		});

		// Raise the NewNumber event
		Self::deposit_event(RawEvent::NumberSaved(Some(who), new_len as u64));

			Ok(())
		}


		// put all the off-chain logic here.所有的线下逻辑放在这里
		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/

			//获取本期的block_number,调用线下计算
			//线下计算提交签名交易的时候，要求连接线上的call是save_number这个函数
			//这个函数记录最新的计算值
 			Self::signed_submit_number(block_number);

		}

	}
}


impl<T: Trait> Module<T> {

	//实现一个计算函数,返回一个数组
	fn calculator(block_number: u32) -> Vec<u32>{

		let mut i: u32;
		let mut res = 0;
		let mut numbers_vec: Vec<u32> = Vec::new();
		 

		//每一个区块的编号，记录一个数值。一直记录到现在的区块的数字
		for i in 1..(block_number+1){
			res = res + i * i;

			numbers_vec.push(res);
		}

		return numbers_vec;

		}


	
	//这是线下函数，用来发送我们计算的数字给线上
	fn signed_submit_number(block_number: T::BlockNumber) -> Result<(), Error<T>> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			debug::error!("No local account available");
			return Err(<Error<T>>::SignedSubmitNumberError);
		}

		// Using `SubmitSignedTransaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		// Submit signed will return a vector of results for all accounts that were found in the
		// local keystore with expected `KEY_TYPE`.
		// let submission: u64 = block_number.try_into().ok().unwrap() as u64;

		//submission里边就是我们计算好的数值
		let submission: Vec<u32> = Self::calculator(block_number.try_into().ok().unwrap() as u32);

		let results = signer.send_signed_transaction(|_acct| {
			// We are just submitting the current block number back on-chain
			//下面这个函数是对应的线上函数，把传过来的数字做线上存储用
			Call::submit_number_signed(submission.clone())
		});

		for (acc, res) in &results {
			match res {
				Ok(()) => {
					debug::native::info!(
						"off-chain send_signed: acc: {:?}| number: {:?}",
						acc.id,
						submission
					);
				}
				Err(e) => {
					debug::error!("[{:?}] Failed in signed_submit_number: {:?}", acc.id, e);
					return Err(<Error<T>>::SignedSubmitNumberError);
				}
			};
		}
		Ok(())
	}




	}







