use crate::{Error, RawEvent, mock::*};
use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};
use frame_system::RawOrigin;

#[test]
fn create_and_revoke() {
    ExtBuilder::build().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Poe::get_proof(claim.clone()), Some((1, 1)));
        assert_noop!(
			Poe::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
        assert_noop!(
			Poe::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
        assert_noop!(
			Poe::revoke_claim(Origin::signed(2), vec![2, 2]),
			Error::<Test>::ProofNotExist
		);
        assert_ok!(Poe::revoke_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
			Poe::revoke_claim(Origin::signed(1), claim),
			Error::<Test>::ProofNotExist
		);
        assert_noop!(
            Poe::create_claim(Origin::signed(1), vec![0u8; 1024]),
            Error::<Test>::TooLong
        );
        assert_noop!(
            Poe::create_claim(Origin::signed(1), vec![0u8; 0]),
            Error::<Test>::TooShort
        );
        // println!("{:?}", System::events());
        let expect_value = vec![
            TestEvent::poe_event(RawEvent::ClaimCreated(1, vec![1, 1])),
            TestEvent::poe_event(RawEvent::ClaimRevoked(1, vec![1, 1], 1)),
        ];
        assert_eq!(System::events().len(), expect_value.len());
        assert!(!System::events().iter().zip(expect_value).any(|(a, b)| a.event != b));
    });
}

#[test]
fn root_and_unsigned() {
    ExtBuilder::build().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_noop!(
            Poe::create_claim(RawOrigin::Root.into(), claim.clone()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            Poe::create_claim(RawOrigin::None.into(), claim.clone()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn transfer() {
    ExtBuilder::build().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(Poe::transfer_claim(Origin::signed(1), claim.clone(), 2));
        let expected_event = TestEvent::poe_event(RawEvent::ClaimTransferred(1, 2, claim.clone(), 1));
        assert!(System::events().iter().any(|a| a.event == expected_event));
        assert_eq!(Poe::get_proof(claim.clone()), Some((2, 1)));
        assert_noop!(
            Poe::transfer_claim(Origin::signed(3), claim.clone(), 4),
            Error::<Test>::NotClaimOwner
        );
        assert_noop!(
            Poe::transfer_claim(Origin::signed(3), vec![8,8,8], 4),
            Error::<Test>::ProofNotExist
        );
    });
}

#[test]
fn ask_and_buy() {
    ExtBuilder::build().execute_with(|| {
        let claim = vec![1u8, 1];
        assert_ok!(Poe::create_claim(Origin::signed(555), vec![1,1,1]));
        assert_ok!(Poe::create_claim(Origin::signed(555), claim.clone()));
        assert_noop!(
            Poe::ask_claim(Origin::signed(3), claim.clone(), 4),
            Error::<Test>::NotClaimOwner
        );
        assert_noop!(
            Poe::ask_claim(Origin::signed(3), vec![8,8,8], 4),
            Error::<Test>::ProofNotExist
        );
        assert_ok!(Poe::ask_claim(Origin::signed(555), claim.clone(), 2244));
        assert_noop!(
            Poe::buy_claim(Origin::signed(3), vec![8,8,8], 2241),
            Error::<Test>::ProofNotExist
        );
        assert_noop!(
            Poe::buy_claim(Origin::signed(555), claim.clone(), 2244),
            Error::<Test>::CannotBuyYourOwnClaim
        );
        assert_noop!(
            Poe::buy_claim(Origin::signed(3), claim.clone(), 2241),
            Error::<Test>::PriceTooLow
        );
        assert_noop!(
            Poe::buy_claim(Origin::signed(3), vec![1,1,1], 2241),
            Error::<Test>::ClaimNotForSale
        );
        assert_eq!(Poe::get_proof(claim.clone()), Some((555, 1)));
        assert_ok!(Poe::buy_claim(Origin::signed(3), claim.clone(), 22411));
        assert_eq!(Poe::get_proof(claim.clone()), Some((3, 1)));
        // println!("{:?}", System::events());
        let expect_value = vec![
            TestEvent::poe_event(RawEvent::ClaimCreated(555, vec![1, 1, 1])),
            TestEvent::poe_event(RawEvent::ClaimCreated(555, vec![1, 1])),
            TestEvent::poe_event(RawEvent::ClaimAsked(555, vec![1, 1], 2244)),
            TestEvent::system(frame_system::RawEvent::NewAccount(555)),
            TestEvent::balances(balances::RawEvent::Endowed(555, 2244)),
            TestEvent::balances(balances::RawEvent::Transfer(3, 555, 2244)),
            TestEvent::poe_event(RawEvent::ClaimSold(555, 3, vec![1, 1])),
        ];
        assert_eq!(System::events().len(), expect_value.len());
        assert!(!System::events().iter().zip(expect_value).any(|(a, b)| a.event != b));
    });
}

