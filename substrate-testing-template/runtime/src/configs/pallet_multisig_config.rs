use crate::*;

parameter_types! {
	pub const DepositBase: u64 = 0;
	pub const DepositFactor: u64 = 1;
	pub const MaxSignatories: u16 = 10;
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = ();
}