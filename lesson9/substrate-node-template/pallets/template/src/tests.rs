// Tests to be written here

use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn test_offchain() {
	let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();

	t.execute_with(|| {
		// 4 submit_number being called
		assert_ok!(TemplateModule::store_eth_price(Ok(245)));
	});	
}
