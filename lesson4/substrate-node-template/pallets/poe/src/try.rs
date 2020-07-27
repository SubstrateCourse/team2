#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
/// A FRAME pallet template with necessary imports
/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references
/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
    traits::{Get},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;
/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type MaxClaimLength: Get<u32>;
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type Proofs;
}
impl<T: Trait + 'static> Store for Module<T> {
    type Proofs = Proofs<T>;
}
impl<T: Trait + 'static> Module<T> {
    pub fn proofs<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<Vec<u8>>,
    >(
        key: K,
    ) -> (T::AccountId, T::BlockNumber, u32) {
        < Proofs < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < Vec < u8 > , ( T :: AccountId , T :: BlockNumber , u32 ) > > :: get ( key )
    }
}
#[doc(hidden)]
pub struct __GetByteStructProofs<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Proofs:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructProofs<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Proofs
            .get_or_init(|| {
                let def_val: (T::AccountId, T::BlockNumber, u32) = Default::default();
                <(T::AccountId, T::BlockNumber, u32) as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructProofs<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructProofs<T> {}
impl<T: Trait + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TemplateModule" ) , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Proofs" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Vec<u8>" ) , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::BlockNumber, u32)" ) , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructProofs :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] [ .. ] ) , }
    }
}
/// Tag a type as an instance of a module.
///
/// Defines storage prefixes, they must be unique.
#[doc(hidden)]
pub trait __GeneratedInstantiable: 'static {
    /// The prefix used by any storage entry of an instance.
    const PREFIX: &'static str;
}
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {}
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl __GeneratedInstantiable for __InherentHiddenInstance {
    const PREFIX: &'static str = "TemplateModule";
}
struct Proofs<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
        T::AccountId,
        T::BlockNumber,
        u32,
    )> for Proofs<T>
{
    fn module_prefix() -> &'static [u8] {
        __InherentHiddenInstance::PREFIX.as_bytes()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Proofs"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        Vec<u8>,
        (T::AccountId, T::BlockNumber, u32),
    > for Proofs<T>
{
    type Query = (T::AccountId, T::BlockNumber, u32);
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        __InherentHiddenInstance::PREFIX.as_bytes()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Proofs"
    }
    fn from_optional_value_to_query(v: Option<(T::AccountId, T::BlockNumber, u32)>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<(T::AccountId, T::BlockNumber, u32)> {
        Some(v)
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as system::Trait>::AccountId>;
/// Events for this module.
///
pub enum RawEvent<AccountId> {
    ClaimCreated(AccountId, Vec<u8>),
    ClaimRevoked(AccountId, Vec<u8>),
    ClaimTransfered(AccountId, Vec<u8>, AccountId),
    ClaimSelled(Vec<u8>, AccountId, AccountId, u32),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId> {
        match (&*self,) {
            (&RawEvent::ClaimCreated(ref __self_0, ref __self_1),) => RawEvent::ClaimCreated(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::ClaimRevoked(ref __self_0, ref __self_1),) => RawEvent::ClaimRevoked(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::ClaimTransfered(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::ClaimTransfered(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::ClaimSelled(ref __self_0, ref __self_1, ref __self_2, ref __self_3),) => {
                RawEvent::ClaimSelled(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                    ::core::clone::Clone::clone(&(*__self_3)),
                )
            }
        }
    }
}
impl<AccountId> ::core::marker::StructuralPartialEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::ClaimCreated(ref __self_0, ref __self_1),
                        &RawEvent::ClaimCreated(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::ClaimRevoked(ref __self_0, ref __self_1),
                        &RawEvent::ClaimRevoked(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::ClaimTransfered(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::ClaimTransfered(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::ClaimSelled(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                        ),
                        &RawEvent::ClaimSelled(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                        ),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::ClaimCreated(ref __self_0, ref __self_1),
                        &RawEvent::ClaimCreated(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::ClaimRevoked(ref __self_0, ref __self_1),
                        &RawEvent::ClaimRevoked(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::ClaimTransfered(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::ClaimTransfered(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::ClaimSelled(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                        ),
                        &RawEvent::ClaimSelled(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                        ),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId> ::core::marker::StructuralEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Encode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                RawEvent::ClaimCreated(ref aa, ref ba) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                RawEvent::ClaimRevoked(ref aa, ref ba) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                RawEvent::ClaimTransfered(ref aa, ref ba, ref ca) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                }
                RawEvent::ClaimSelled(ref aa, ref ba, ref ca, ref da) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                    dest.push(da);
                }
                _ => (),
            }
        }
    }
    impl<AccountId> _parity_scale_codec::EncodeLike for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Decode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(RawEvent::ClaimCreated(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimCreated.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimCreated.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 1usize as u8 => Ok(RawEvent::ClaimRevoked(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimRevoked.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimRevoked.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 2usize as u8 => Ok(RawEvent::ClaimTransfered(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: ClaimTransfered.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: ClaimTransfered.1".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: ClaimTransfered.2".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 3usize as u8 => Ok(RawEvent::ClaimSelled(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimSelled.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimSelled.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimSelled.2".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ClaimSelled.3".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x => Err("No such variant in enum RawEvent".into()),
            }
        }
    }
};
impl<AccountId> core::fmt::Debug for RawEvent<AccountId>
where
    AccountId: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ClaimCreated(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::ClaimCreated")
                .field(a0)
                .field(a1)
                .finish(),
            Self::ClaimRevoked(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::ClaimRevoked")
                .field(a0)
                .field(a1)
                .finish(),
            Self::ClaimTransfered(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::ClaimTransfered")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::ClaimSelled(ref a0, ref a1, ref a2, ref a3) => fmt
                .debug_tuple("RawEvent::ClaimSelled")
                .field(a0)
                .field(a1)
                .field(a2)
                .field(a3)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId> From<RawEvent<AccountId>> for () {
    fn from(_: RawEvent<AccountId>) -> () {
        ()
    }
}
impl<AccountId> RawEvent<AccountId> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ClaimCreated"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ClaimRevoked"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ClaimTransfered"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                    "AccountId",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ClaimSelled"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "Vec<u8>",
                    "AccountId",
                    "AccountId",
                    "u32",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
pub enum Error<T: Trait> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    ProofAlreadyExist,
    ClaimNotExist,
    NotClaimOwner,
    ClaimLenghtLimit,
    NotEnoughPrice,
}
impl<T: Trait> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Trait> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::ProofAlreadyExist => 0,
            Error::ClaimNotExist => 0 + 1,
            Error::NotClaimOwner => 0 + 1 + 1,
            Error::ClaimLenghtLimit => 0 + 1 + 1 + 1,
            Error::NotEnoughPrice => 0 + 1 + 1 + 1 + 1,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::ProofAlreadyExist => "ProofAlreadyExist",
            Error::ClaimNotExist => "ClaimNotExist",
            Error::NotClaimOwner => "NotClaimOwner",
            Error::ClaimLenghtLimit => "ClaimLenghtLimit",
            Error::NotEnoughPrice => "NotEnoughPrice",
        }
    }
}
impl<T: Trait> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Trait> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::ModuleToIndex as ::frame_support::traits::ModuleToIndex>::module_to_index::<
            Module<T>,
        >()
        .expect("Every active module has an index in the runtime; qed") as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Trait> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("ProofAlreadyExist"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("ClaimNotExist"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NotClaimOwner"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("ClaimLenghtLimit"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NotEnoughPrice"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
/// The module declaration.
pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Trait> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: Trait> ::frame_support::traits::OnInitialize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnFinalize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OffchainWorker<T::BlockNumber> for Module<T> {}
impl<T: Trait> Module<T> {
    fn deposit_event(event: impl Into<<T as Trait>::Event>) {
        <system::Module<T>>::deposit_event(event.into())
    }
}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Trait> Module<T> {
    /// Just a dummy entry point.
    /// function that can be called by the external world as an extrinsics call
    /// takes a parameter of the type `AccountId`, stores it, and emits an event
    pub fn create_claim(origin: T::Origin, claim: Vec<u8>, price: u32) -> dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::tracing::dispatcher::has_been_set()
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                {
                    use ::tracing::callsite;
                    use ::tracing::callsite::Callsite;
                    let callsite = {
                        use ::tracing::{callsite, subscriber::Interest, Metadata, __macro_support::*};
                        struct MyCallsite;
                        static META: Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "create_claim",
                                "pallet_poe",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/poe/src/lib.rs"),
                                Some(67u32),
                                Some("pallet_poe"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&MyCallsite),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static INTEREST: AtomicUsize = AtomicUsize::new(0);
                        static REGISTRATION: Once = Once::new();
                        impl MyCallsite {
                            #[inline]
                            fn interest(&self) -> Interest {
                                match INTEREST.load(Ordering::Relaxed) {
                                    0 => Interest::never(),
                                    2 => Interest::always(),
                                    _ => Interest::sometimes(),
                                }
                            }
                        }
                        impl callsite::Callsite for MyCallsite {
                            fn set_interest(&self, interest: Interest) {
                                let interest = match () {
                                    _ if interest.is_never() => 0,
                                    _ if interest.is_always() => 2,
                                    _ => 1,
                                };
                                INTEREST.store(interest, Ordering::SeqCst);
                            }
                            fn metadata(&self) -> &Metadata {
                                &META
                            }
                        }
                        REGISTRATION.call_once(|| {
                            callsite::register(&MyCallsite);
                        });
                        &MyCallsite
                    };
                    let meta = callsite.metadata();
                    if {
                        let interest = callsite.interest();
                        if interest.is_never() {
                            false
                        } else if interest.is_always() {
                            true
                        } else {
                            let meta = callsite.metadata();
                            ::tracing::dispatcher::get_default(|current| current.enabled(meta))
                        }
                    } {
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !!Proofs::<T>::contains_key(&claim) {
                {
                    return Err(Error::<T>::ProofAlreadyExist.into());
                };
            }
        };
        {
            if !(claim.len() as u32 <= T::MaxClaimLength::get()) {
                {
                    return Err(Error::<T>::ClaimLenghtLimit.into());
                };
            }
        };
        Proofs::<T>::insert(
            &claim,
            (sender.clone(), system::Module::<T>::block_number(), price),
        );
        Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
        Ok(())
    }
    pub fn revoke_claim(origin: T::Origin, claim: Vec<u8>) -> dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::tracing::dispatcher::has_been_set()
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                {
                    use ::tracing::callsite;
                    use ::tracing::callsite::Callsite;
                    let callsite = {
                        use ::tracing::{callsite, subscriber::Interest, Metadata, __macro_support::*};
                        struct MyCallsite;
                        static META: Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "revoke_claim",
                                "pallet_poe",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/poe/src/lib.rs"),
                                Some(67u32),
                                Some("pallet_poe"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&MyCallsite),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static INTEREST: AtomicUsize = AtomicUsize::new(0);
                        static REGISTRATION: Once = Once::new();
                        impl MyCallsite {
                            #[inline]
                            fn interest(&self) -> Interest {
                                match INTEREST.load(Ordering::Relaxed) {
                                    0 => Interest::never(),
                                    2 => Interest::always(),
                                    _ => Interest::sometimes(),
                                }
                            }
                        }
                        impl callsite::Callsite for MyCallsite {
                            fn set_interest(&self, interest: Interest) {
                                let interest = match () {
                                    _ if interest.is_never() => 0,
                                    _ if interest.is_always() => 2,
                                    _ => 1,
                                };
                                INTEREST.store(interest, Ordering::SeqCst);
                            }
                            fn metadata(&self) -> &Metadata {
                                &META
                            }
                        }
                        REGISTRATION.call_once(|| {
                            callsite::register(&MyCallsite);
                        });
                        &MyCallsite
                    };
                    let meta = callsite.metadata();
                    if {
                        let interest = callsite.interest();
                        if interest.is_never() {
                            false
                        } else if interest.is_always() {
                            true
                        } else {
                            let meta = callsite.metadata();
                            ::tracing::dispatcher::get_default(|current| current.enabled(meta))
                        }
                    } {
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !Proofs::<T>::contains_key(&claim) {
                {
                    return Err(Error::<T>::ClaimNotExist.into());
                };
            }
        };
        let (owner, _block_number, _price) = Proofs::<T>::get(&claim);
        {
            if !(owner == sender) {
                {
                    return Err(Error::<T>::NotClaimOwner.into());
                };
            }
        };
        Proofs::<T>::remove(&claim);
        Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
        Ok(())
    }
    pub fn transfer_claim(
        origin: T::Origin,
        claim: Vec<u8>,
        dest: <T::Lookup as StaticLookup>::Source,
    ) -> dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::tracing::dispatcher::has_been_set()
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                {
                    use ::tracing::callsite;
                    use ::tracing::callsite::Callsite;
                    let callsite = {
                        use ::tracing::{callsite, subscriber::Interest, Metadata, __macro_support::*};
                        struct MyCallsite;
                        static META: Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "transfer_claim",
                                "pallet_poe",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/poe/src/lib.rs"),
                                Some(67u32),
                                Some("pallet_poe"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&MyCallsite),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static INTEREST: AtomicUsize = AtomicUsize::new(0);
                        static REGISTRATION: Once = Once::new();
                        impl MyCallsite {
                            #[inline]
                            fn interest(&self) -> Interest {
                                match INTEREST.load(Ordering::Relaxed) {
                                    0 => Interest::never(),
                                    2 => Interest::always(),
                                    _ => Interest::sometimes(),
                                }
                            }
                        }
                        impl callsite::Callsite for MyCallsite {
                            fn set_interest(&self, interest: Interest) {
                                let interest = match () {
                                    _ if interest.is_never() => 0,
                                    _ if interest.is_always() => 2,
                                    _ => 1,
                                };
                                INTEREST.store(interest, Ordering::SeqCst);
                            }
                            fn metadata(&self) -> &Metadata {
                                &META
                            }
                        }
                        REGISTRATION.call_once(|| {
                            callsite::register(&MyCallsite);
                        });
                        &MyCallsite
                    };
                    let meta = callsite.metadata();
                    if {
                        let interest = callsite.interest();
                        if interest.is_never() {
                            false
                        } else if interest.is_always() {
                            true
                        } else {
                            let meta = callsite.metadata();
                            ::tracing::dispatcher::get_default(|current| current.enabled(meta))
                        }
                    } {
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !Proofs::<T>::contains_key(&claim) {
                {
                    return Err(Error::<T>::ClaimNotExist.into());
                };
            }
        };
        let (owner, _block_number, price) = Proofs::<T>::get(&claim);
        {
            if !(owner == sender) {
                {
                    return Err(Error::<T>::NotClaimOwner.into());
                };
            }
        };
        let dest = T::Lookup::lookup(dest)?;
        Proofs::<T>::insert(
            &claim,
            (dest.clone(), system::Module::<T>::block_number(), price),
        );
        Self::deposit_event(RawEvent::ClaimTransfered(sender, claim, dest));
        Ok(())
    }
    pub fn buy_claim(
        origin: T::Origin,
        claim: Vec<u8>,
        offer_price: u32,
    ) -> dispatch::DispatchResult {
        let __tracing_span__ = {
            {
                if ::tracing::dispatcher::has_been_set()
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                {
                    use ::tracing::callsite;
                    use ::tracing::callsite::Callsite;
                    let callsite = {
                        use ::tracing::{callsite, subscriber::Interest, Metadata, __macro_support::*};
                        struct MyCallsite;
                        static META: Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "buy_claim",
                                "pallet_poe",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/poe/src/lib.rs"),
                                Some(67u32),
                                Some("pallet_poe"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&MyCallsite),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static INTEREST: AtomicUsize = AtomicUsize::new(0);
                        static REGISTRATION: Once = Once::new();
                        impl MyCallsite {
                            #[inline]
                            fn interest(&self) -> Interest {
                                match INTEREST.load(Ordering::Relaxed) {
                                    0 => Interest::never(),
                                    2 => Interest::always(),
                                    _ => Interest::sometimes(),
                                }
                            }
                        }
                        impl callsite::Callsite for MyCallsite {
                            fn set_interest(&self, interest: Interest) {
                                let interest = match () {
                                    _ if interest.is_never() => 0,
                                    _ if interest.is_always() => 2,
                                    _ => 1,
                                };
                                INTEREST.store(interest, Ordering::SeqCst);
                            }
                            fn metadata(&self) -> &Metadata {
                                &META
                            }
                        }
                        REGISTRATION.call_once(|| {
                            callsite::register(&MyCallsite);
                        });
                        &MyCallsite
                    };
                    let meta = callsite.metadata();
                    if {
                        let interest = callsite.interest();
                        if interest.is_never() {
                            false
                        } else if interest.is_always() {
                            true
                        } else {
                            let meta = callsite.metadata();
                            ::tracing::dispatcher::get_default(|current| current.enabled(meta))
                        }
                    } {
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !Proofs::<T>::contains_key(&claim) {
                {
                    return Err(Error::<T>::ClaimNotExist.into());
                };
            }
        };
        let (owner, _block_number, price) = Proofs::<T>::get(&claim);
        {
            if !(offer_price >= price) {
                {
                    return Err(Error::<T>::NotEnoughPrice.into());
                };
            }
        };
        Self::deposit_event(RawEvent::ClaimSelled(claim, owner, sender, offer_price));
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Trait> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    /// Just a dummy entry point.
    /// function that can be called by the external world as an extrinsics call
    /// takes a parameter of the type `AccountId`, stores it, and emits an event
    create_claim(Vec<u8>, u32),
    #[allow(non_camel_case_types)]
    revoke_claim(Vec<u8>),
    #[allow(non_camel_case_types)]
    transfer_claim(Vec<u8>, <T::Lookup as StaticLookup>::Source),
    #[allow(non_camel_case_types)]
    buy_claim(Vec<u8>, u32),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Encode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Call::create_claim(ref aa, ref ba) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                Call::revoke_claim(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                Call::transfer_claim(ref aa, ref ba) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                Call::buy_claim(ref aa, ref ba) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                _ => (),
            }
        }
    }
    impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Decode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(Call::create_claim(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: create_claim.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: create_claim.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 1usize as u8 => Ok(Call::revoke_claim({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Call :: revoke_claim.0".into()),
                        Ok(a) => a,
                    }
                })),
                x if x == 2usize as u8 => Ok(Call::transfer_claim(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: transfer_claim.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field Call :: transfer_claim.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 3usize as u8 => Ok(Call::buy_claim(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field Call :: buy_claim.0".into()),
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field Call :: buy_claim.1".into()),
                            Ok(a) => a,
                        }
                    },
                )),
                x => Err("No such variant in enum Call".into()),
            }
        }
    }
};
impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::create_claim(ref claim, ref price) => {
                let base_weight = 0;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>, &u32)>>::weigh_data(
                        &base_weight,
                        (claim, price),
                    );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , & u32 ) > > :: classify_dispatch ( & base_weight , ( claim , price ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>, &u32)>>::pays_fee(
                    &base_weight,
                    (claim, price),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::revoke_claim(ref claim) => {
                let base_weight = 0;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                    &base_weight,
                    (claim,),
                );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , ) > > :: classify_dispatch ( & base_weight , ( claim , ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                    &base_weight,
                    (claim,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::transfer_claim(ref claim, ref dest) => {
                let base_weight = 0;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &Vec<u8>,
                    &<T::Lookup as StaticLookup>::Source,
                )>>::weigh_data(&base_weight, (claim, dest));
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &Vec<u8>,
                    &<T::Lookup as StaticLookup>::Source,
                )>>::classify_dispatch(&base_weight, (claim, dest));
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &Vec<u8>,
                    &<T::Lookup as StaticLookup>::Source,
                )>>::pays_fee(&base_weight, (claim, dest));
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::buy_claim(ref claim, ref offer_price) => {
                let base_weight = 0;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>, &u32)>>::weigh_data(
                        &base_weight,
                        (claim, offer_price),
                    );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , & u32 ) > > :: classify_dispatch ( & base_weight , ( claim , offer_price ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>, &u32)>>::pays_fee(
                    &base_weight,
                    (claim, offer_price),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::create_claim(ref claim, ref price) => {
                let _ = (claim, price);
                "create_claim"
            }
            Call::revoke_claim(ref claim) => {
                let _ = (claim);
                "revoke_claim"
            }
            Call::transfer_claim(ref claim, ref dest) => {
                let _ = (claim, dest);
                "transfer_claim"
            }
            Call::buy_claim(ref claim, ref offer_price) => {
                let _ = (claim, offer_price);
                "buy_claim"
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &[
            "create_claim",
            "revoke_claim",
            "transfer_claim",
            "buy_claim",
        ]
    }
}
impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::create_claim(ref claim, ref price) => {
                Call::create_claim((*claim).clone(), (*price).clone())
            }
            Call::revoke_claim(ref claim) => Call::revoke_claim((*claim).clone()),
            Call::transfer_claim(ref claim, ref dest) => {
                Call::transfer_claim((*claim).clone(), (*dest).clone())
            }
            Call::buy_claim(ref claim, ref offer_price) => {
                Call::buy_claim((*claim).clone(), (*offer_price).clone())
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::create_claim(ref claim, ref price) => {
                let self_params = (claim, price);
                if let Call::create_claim(ref claim, ref price) = *_other {
                    self_params == (claim, price)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::revoke_claim(ref claim) => {
                let self_params = (claim,);
                if let Call::revoke_claim(ref claim) = *_other {
                    self_params == (claim,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::transfer_claim(ref claim, ref dest) => {
                let self_params = (claim, dest);
                if let Call::transfer_claim(ref claim, ref dest) = *_other {
                    self_params == (claim, dest)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::buy_claim(ref claim, ref offer_price) => {
                let self_params = (claim, offer_price);
                if let Call::buy_claim(ref claim, ref offer_price) = *_other {
                    self_params == (claim, offer_price)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::create_claim(ref claim, ref price) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"create_claim", &(claim.clone(), price.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::revoke_claim(ref claim) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"revoke_claim", &(claim.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::transfer_claim(ref claim, ref dest) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"transfer_claim", &(claim.clone(), dest.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::buy_claim(ref claim, ref offer_price) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"buy_claim", &(claim.clone(), offer_price.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Dispatchable for Call<T> {
    type Trait = T;
    type Origin = T::Origin;
    type Info = ::frame_support::weights::DispatchInfo;
    type PostInfo = ::frame_support::weights::PostDispatchInfo;
    fn dispatch(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::create_claim(claim, price) => <Module<T>>::create_claim(_origin, claim, price)
                .map(Into::into)
                .map_err(Into::into),
            Call::revoke_claim(claim) => <Module<T>>::revoke_claim(_origin, claim)
                .map(Into::into)
                .map_err(Into::into),
            Call::transfer_claim(claim, dest) => <Module<T>>::transfer_claim(_origin, claim, dest)
                .map(Into::into)
                .map_err(Into::into),
            Call::buy_claim(claim, offer_price) => {
                <Module<T>>::buy_claim(_origin, claim, offer_price)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn dispatch<
        D: ::frame_support::dispatch::Dispatchable<
            Trait = T,
            PostInfo = ::frame_support::weights::PostDispatchInfo,
        >,
    >(
        d: D,
        origin: D::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        d.dispatch(origin)
    }
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("create_claim"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("claim"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("price"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Just a dummy entry point.",
                    r" function that can be called by the external world as an extrinsics call",
                    r" takes a parameter of the type `AccountId`, stores it, and emits an event",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("revoke_claim"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("claim"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("transfer_claim"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("claim"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("dest"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode(
                            "<T::Lookup as StaticLookup>::Source",
                        ),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("buy_claim"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("claim"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("offer_price"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        &[]
    }
}
impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
