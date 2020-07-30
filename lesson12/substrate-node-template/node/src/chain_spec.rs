use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

// public staging network
pub fn tea_staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Tea Staging Testnet",
		"tea_staging",
		ChainType::Live,
		|| testnet_genesis(
			// for i in 1 2; do for j in aura; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
			// and
			// for i in 1 2; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
			vec![(
				// 5DX46d2MbD41v4mQyrR3XoHgGNUV42yZzVGWjVGYaZcFdevS
				hex!["4054bff32d8ccb9e42bc7a87192c39f961796f02f332da8e00258c0e7916890f"].unchecked_into(),
				// 5G87DHR4n3Fjy6UWHzvCZ6aS31rKaLNfpAWFgvwmaxtdAhvv
				hex!["b3894a9a6e75e627a6ffd5a16cfb09f44e5096dea70e225e9dc1e0b40ab712ca"].unchecked_into(),
			),(
				// 5FvLpiCzgLuDhgSmosARApP7ecnihfWyQyzLWEpGYf4pG7iv
				hex!["aa902a2771fd91e2e9392f3a8b79ad466528a1bec190ffa11176994f2895d739"].unchecked_into(),
				// 5HQqA8fm9eGwfGkimK2weyPM7TkTXBnhv8bouKKeXwXWVrvm
				hex!["ec86a72fcfb6c6f6bc37aceae8c2c38e4b529186e093d79ab68949c9959aa7c7"].unchecked_into(),
			)],
			// subkey inspect "$SECRET//tea"
			hex![
				// 5Cf2tRMcSqxs1duUidgjFfo4bg9crB6Eq5u9CQ3jDdqPV6L2
				"1a2e67b6e2466ca3154f6a330ddc7507ba41ce6a7845159947f5f441cae0865c"
			].into(),
			vec![
				// 5Cf2tRMcSqxs1duUidgjFfo4bg9crB6Eq5u9CQ3jDdqPV6L2
				hex!["1a2e67b6e2466ca3154f6a330ddc7507ba41ce6a7845159947f5f441cae0865c"].into(),
				// 5EwDAxZA9zhF8pNztGfUPuAsf8ixBKiMoST2UteXjeH5Yzkr
				hex!["7efd6bdea03b704f5fcf1478aa89e074f605afcf869d39c6fdc3314967afc766"].into(),
			],
			true,
		),
		boot_nodes,
		None,
		None,
		None,
		None,
	)
}

fn testnet_genesis(initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
	}
}
