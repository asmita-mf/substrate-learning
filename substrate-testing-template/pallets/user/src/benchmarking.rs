//! Benchmarking setup for pallet-template

use super::*;
use scale_info::prelude::vec::Vec;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

const SEED: u32 = 0;

pub fn create_default_user<T: Config>(
    account: T::AccountId,
	name: Vec<u8>,
) -> (T::AccountId, Vec<u8>) {
	// let name: Vec<u8> = "alice1".as_bytes().to_vec();

	assert!(Pallet::<T>::register_user(
		RawOrigin::Signed(account.clone()).into(),
		name.clone(),
	)
	.is_ok());

	(account, name)
}

benchmarks! {
	register_user {
		let caller: T::AccountId = whitelisted_caller();
		let name = "alice".as_bytes().to_vec();
	}: _(RawOrigin::Signed(caller), name)
	
	signin {
		let caller: T::AccountId = whitelisted_caller();
		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller))

	signout {
		let caller: T::AccountId = whitelisted_caller();
		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller))
	
	update_user {
		let caller: T::AccountId = whitelisted_caller();
		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller), None, None, Some("address1".as_bytes().to_vec()))
	
	delete_user {
		let caller: T::AccountId = whitelisted_caller();
		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
	}: _(RawOrigin::Signed(caller))
	

	add_friend {
		let caller: T::AccountId = whitelisted_caller();
		let friend: T::AccountId = account("friend", 1, SEED);

		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
		create_default_user::<T>(friend.clone(), "alice2".as_bytes().to_vec());

	}: _(RawOrigin::Signed(caller), friend)

	delete_friend {
		let caller: T::AccountId = whitelisted_caller();
		let friend: T::AccountId = 	account("friend", 1, SEED);

		create_default_user::<T>(caller.clone(), "alice1".as_bytes().to_vec());
		create_default_user::<T>(friend.clone(), "alice2".as_bytes().to_vec());

	}: _(RawOrigin::Signed(caller), friend)


	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}



// Compile cli code: cargo build --release --features runtime-benchmarks
//   ./target/release/node-template benchmark pallet --pallet pallet_user --extrinsic "*" --steps=50 --repeat=20 --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=pallets/user/src/weights1.rs 
