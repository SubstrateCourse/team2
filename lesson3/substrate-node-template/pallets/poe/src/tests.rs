// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};

// test cases for create_claim

//create_claim works sucessfully
#[test]
fn create_claim_works(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    }
    )
}


//create_claim fails due to claim existence
#[test]
fn create_claim_fails_due_to_claim_existence(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);

    }
    )
}


//get claim that doesn't exist

#[test]
fn create_claim_fails_due_to_claim_too_long(){
    new_test_ext().execute_with(|| {

        let claim = vec![1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofTooLong);

    }
    )
}


//test cases for revoke_claim

// revoke_claim works successfully
#[test]
fn revoke_claim_works(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(Module::<Test>::revoke_claim(Origin::signed(1), claim.clone()));
    }
    )
}

// revoke_claim fails due to wrong owner
#[test]
fn revoke_claim_not_work_due_to_wrong_owner(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotClaimOwner);

    }
    )
}


//revoke_claim fails due to  claim not existing
#[test]
fn revoke_claim_not_work_due_to_claim_not_existing(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        
        let claim2 = vec![1,2];

        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim2.clone()), Error::<Test>::ClaimNotExist);

    }
    )
}


//test cases for transfer_claim

// transfer_claim works successfully
#[test]
fn transfer_claim_works(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(Module::<Test>::transfer_claim(Origin::signed(1), claim.clone(), 2 as <<mock::Test as system::Trait>::Lookup as StaticLookup>::Source));
        
        let(new_owner, _block_number) = Proofs::<Test>::get(&claim);
        assert_eq!(new_owner, 2 as <<mock::Test as system::Trait>::Lookup as StaticLookup>::Source);
    }
    )
}


// transfer_claim doesn't work due to claim not exsisting
#[test]
fn transfer_claim_fails_due_to_claim_not_existing(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let claim2 = vec![2,3];
        assert_noop!(Module::<Test>::transfer_claim(Origin::signed(1), claim2.clone(), 2 as <<mock::Test as system::Trait>::Lookup as StaticLookup>::Source), Error::<Test>::ClaimNotExist);
    
    }
    )
}


// transfer_claim doesn't work due to wrong owner
#[test]
fn transfer_claim_fails_due_to_wrong_owner(){
    new_test_ext().execute_with(|| {

        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_noop!(Module::<Test>::transfer_claim(Origin::signed(2), claim.clone(), 3 as <<mock::Test as system::Trait>::Lookup as StaticLookup>::Source), Error::<Test>::NotClaimOwner);
    
    }
    )
}