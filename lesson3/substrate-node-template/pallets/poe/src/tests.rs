// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// test cases for create claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), proof.clone()));
        assert_eq!(
            Proofs::<Test>::get(&proof),
            (1, system::Module::<Test>::block_number())
        );
    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2, 3, 4, 5, 6, 7];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

// test cases for revoke claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), proof.clone()),);
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), proof.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// test cases for transfer claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());

        assert_ok!(PoeModule::transfer_claim(
            Origin::signed(1),
            proof.clone(),
            2
        ),);
        assert_eq!(
            Proofs::<Test>::get(proof.clone()),
            (2, system::Module::<Test>::block_number())
        );
    })
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exsit() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), proof.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), proof.clone(), 1),
            Error::<Test>::NotClaimOwner
        );
    })
}
