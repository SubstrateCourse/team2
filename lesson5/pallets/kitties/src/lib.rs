#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_error, decl_storage, ensure, StorageValue, StorageMap, traits::Randomness};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError, DispatchResult};

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Option<Kitty>;

        pub KittiesCount get(fn kitties_count): u32;

        pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) (T::AccountId, u32) => u32;

        pub OwnedKittiesCount get(fn owned_kitties_count): map hasher(blake2_128_concat) T::AccountId => u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        
        #[weight = 0] 
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;

            let kitty_id = Self::next_kitty_id()?;

            let dna = Self::random_value(&sender);
            
            let kitty = Kitty(dna);
        
            Self::insert_kitty(sender, kitty_id, kitty);
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: u32, kitty_id_2: u32) {
            let sender = ensure_signed(origin)?;
            Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
        }

    }
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {

    fn next_kitty_id() -> sp_std::result::Result<u32, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == u32::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }

        Ok(kitty_id)
    }
   
    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
                <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
                sender,
                <frame_system::Module<T>>::extrinsic_index(),
            );

        payload.using_encoded(blake2_128)
    }

    fn insert_kitty(owner: T::AccountId, kitty_id: u32, kitty: Kitty) {
        Kitties::insert(kitty_id.clone(), kitty);
        KittiesCount::put(kitty_id.clone() + 1);

        let user_kitties_id = Self::owned_kitties_count(&owner);
        <OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
        <OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1); 
    }

    fn do_breed(sender: T::AccountId, kitty_id_1: u32, kitty_id_2: u32) -> DispatchResult {
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        let kitty_id = Self::next_kitty_id()?;
        let selector = Self::random_value(&sender);
        
        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;
        
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        let kitty = Kitty(new_dna);

        Self::insert_kitty(sender, kitty_id, kitty);

        Ok(())
    }
}
