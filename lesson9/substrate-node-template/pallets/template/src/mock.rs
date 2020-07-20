
// Creating mock runtime here

use crate::{Module, Trait, Call, crypto, KEY_TYPE};
use sp_core::{
	H256,
	offchain::{
		testing::{self, OffchainState, PoolState},
		OffchainExt, TransactionPoolExt,
	},
	sr25519::{self, Signature},
	testing::KeyStore,
	traits::KeystoreExt,
};

use codec::{alloc::sync::Arc};
use parking_lot::RwLock;

use frame_support::{impl_outer_origin, impl_outer_event, parameter_types, weights::Weight};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentityLookup, Verify},
	Perbill,
};
use frame_system as system;
use crate as template;

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		system<T>,
		template<T>,
	}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

// --- mocking template trait
pub type TestExtrinsic = TestXt<Call<Test>, ()>;

impl Trait for Test {
	type AuthorityId = crypto::AuthId;
	type Call = Call<Test>;
	type Event = TestEvent;
}

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call<Test>: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call<Test>,
		_public: <Signature as Verify>::Signer,
		_account: <Test as system::Trait>::AccountId,
		index: <Test as system::Trait>::Index,
	) -> Option<(
		Call<Test>,
		<TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		Some((call, (index, ())))
	}
}

impl system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> system::offchain::SendTransactionTypes<C> for Test
where
	Call<Test>: From<C>,
{
	type OverarchingCall = Call<Test>;
	type Extrinsic = TestExtrinsic;
}

pub type System = system::Module<Test>;
pub type TemplateModule = Module<Test>;

pub struct ExtBuilder;

impl ExtBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (offchain, offchain_state) = testing::TestOffchainExt::new();
		let (pool, pool_state) = testing::TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.write()
			.sr25519_generate_new(KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();

		let storage = system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();

		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(keystore));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}

