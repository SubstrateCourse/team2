// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

// test cases for create_claim
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number(), memo.clone()));
    })
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4, 5, 6];
        let memo = vec![0, 1];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

// test cases for revoke_claim
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone());

        assert_ok!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone())
        );
    })
}

#[test]
fn revoke_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// test cases for transfer_claim
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];
        let dest = 2;
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone());

        assert_ok!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), dest)
        );
        assert_eq!(Proofs::<Test>::get(&claim), (2, system::Module::<Test>::block_number(), memo.clone()));
    })
}

#[test]
fn transfer_claim_failed_when_claim_does_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let dest = 2;

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), dest),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let memo = vec![0, 1];
        let dest = 2;
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone(), memo.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), claim.clone(), dest),
            Error::<Test>::NotClaimOwner
        );
    })
}
