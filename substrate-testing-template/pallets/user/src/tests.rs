use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::register_user(Origin::signed(1), "alice".as_bytes().to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});

	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::register_user(Origin::signed(2), "bob".as_bytes().to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});

	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::register_user(Origin::signed(1), "alice".as_bytes().to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::add_friend(Origin::signed(1), 2), Error::<Test>::AddressNotRegistered);
	});
}
