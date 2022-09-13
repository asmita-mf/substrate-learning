use codec::{Decode, Encode};
use scale_info::TypeInfo;
use scale_info::prelude::vec::Vec;

use codec::Codec;

// use sp_std::{
// 	prelude::*, str,
// 	collections::{btree_map::BTreeMap}
// };
// use serde::__private::ToString;
use frame_support::pallet_prelude::DispatchResult;
use frame_support::pallet_prelude::*;
use crate::*;
// use frame_system::Config;

#[derive( Encode, Decode, Debug, Clone, PartialEq, Eq, Default, TypeInfo)]
pub struct UserData {
	pub name: Vec<u8>,
    pub email_id: Option<Vec<u8>>,
    pub phone_number: Option<Vec<u8>>,
    pub address: Option<Vec<u8>>,
	pub active: bool,
}

impl UserData {
    pub fn new(name: Vec<u8>) -> UserData {
        UserData {
            name: name,
            email_id: None,
            phone_number: None,
            address: None,
            active: false
        }
    }
}

pub struct UserStorage<T>(T);

impl<T: Config> UserStorage<T> {

    pub(crate) fn insert_new(accountid: &T::AccountId, name: Vec<u8>) -> DispatchResult {
		let data = UserData::new(name);
		<User<T>>::insert(accountid, data);

        Ok(())
    }

    pub(crate) fn update_user_data(
        accountid: &T::AccountId,
        email_id: Option<Vec<u8>>, 
        phone_number: Option<Vec<u8>>, 
        address: Option<Vec<u8>>, 
    ) -> DispatchResult {
        User::<T>::mutate(accountid, |user_record| {
            user_record.email_id = email_id;

            user_record.phone_number = phone_number;

            user_record.address = address;
        });
        Ok(())
    }
}