use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use super::*;

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExists,
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];

		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()), Error::<Test>::ClaimNotExist);
	})
}

// 非存证owner撤销存证操作
#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];
		let _ =	PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}


#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		//let _ = PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2);
		assert_ok!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2)
		);

		assert_eq!(
			Proofs::<Test>::get(&claim), 
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})	
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];

		assert_eq!(Proofs::<Test>::get(&claim), None);
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::ClaimNotExist);
	})
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());	

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn create_claim_works_by_proof_hash_lenght_limit() {
	new_test_ext().execute_with(|| {
		let claim = vec![1; ProofHashKeyLimit::get() as usize];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_proof_hash_lenght_overflow () {
	new_test_ext().execute_with(|| {
		let claim = vec![1; (ProofHashKeyLimit::get() + 1) as usize];

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofHashLenghtOverFlow,
		);
	})
}