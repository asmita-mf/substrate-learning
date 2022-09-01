use crate::*;

use pallet_vtbt;
impl pallet_vtbt::Config for Runtime {
	type Event = Event;
	type TokenBalance = u128;
	type AssetId = u32;
	type WeightInfo = pallet_vtbt::weights::SubstrateWeight<Runtime>;
}