use crate::*;

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}
// pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
impl pallet_authorship::Config for Runtime {
	// type FindAuthor = <Self as frame_system::Config>::AccountId;// pallet_session::FindAccountFromAuthorIndex<Self, Aura>; 
    // Error: the trait `FindAuthor<AccountId32>` is not implemented for `AccountId32`

	type FindAuthor = (); //pallet_session::FindAccountFromAuthorIndex<Self, Aura>;

	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	// type EventHandler = (Staking, ImOnline);
	type EventHandler = ();

}