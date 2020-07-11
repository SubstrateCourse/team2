// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok};
use codec::{Decode};

#[test]
fn test_onchain() {
	let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
	t.execute_with(|| {
		let price = 9227;
		let value = [ 9227 ];
		let acct: <Test as system::Trait>::AccountId = Default::default();

		// when `save_number` is being called
		assert_ok!(TemplateModule::save_number(Origin::signed(acct), price));

		// added to storage
		assert_eq!(<Prices>::get(), value);

		// an event is emitted
		let expected_event = TestEvent::template(RawEvent::NewPrice(acct, price));
		assert!( System::events().iter().any(|er| er.event == expected_event) );

	});
}

#[test]
fn test_offchain() {

}