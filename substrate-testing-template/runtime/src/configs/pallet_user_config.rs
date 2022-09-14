
use crate::*;

/// Configure the pallet-template in pallets/template.
impl pallet_user::Config for Runtime {
	type Event = Event;
	type WeightInfo = pallet_user::weights::WeightInfoStruct<Runtime>;
}