
use crate::*;
use frame_system::{ EnsureRoot, EnsureSignedBy };
use frame_support::PalletId;

// use frame_support::TestRandomness;
// parameter_types! {
// 	pub const SocietyPalletId: PalletId = PalletId(*b"py/socie");
// 	pub BlockWeights: frame_system::limits::BlockWeights =
// 		frame_system::limits::BlockWeights::simple_max(frame_support::weights::Weight::from_ref_time(1024));
// }

// ord_parameter_types! {
// 	pub const FounderSetAccount: u128 = 1;
// 	pub const SuspensionJudgementSetAccount: u128 = 2;
// }

// impl pallet_society::Config for Runtime {
// 	type Event = Event;
// 	type Currency = pallet_balances::Pallet<Self>;
// 	type Randomness = TestRandomness<Self>;
// 	type CandidateDeposit = ConstU64<25>;
// 	type WrongSideDeduction = ConstU64<2>;
// 	type MaxStrikes = ConstU32<2>;
// 	type PeriodSpend = ConstU64<1000>;
// 	type MembershipChanged = ();
// 	type RotationPeriod = ConstU64<4>;
// 	type MaxLockDuration = ConstU64<100>;
// 	type FounderSetOrigin = EnsureSignedBy<FounderSetAccount, u128>;
// 	type SuspensionJudgementOrigin = EnsureSignedBy<SuspensionJudgementSetAccount, u128>;
// 	type ChallengePeriod = ConstU64<8>;
// 	type MaxCandidateIntake = ConstU32<10>;
// 	type PalletId = SocietyPalletId;
// }
parameter_types! {
	pub const CandidateDeposit: Balance = 10 * DOLLARS;
	pub const WrongSideDeduction: Balance = 2 * DOLLARS;
	pub const MaxStrikes: u32 = 10;
	pub const RotationPeriod: BlockNumber = 80 * HOURS;
	pub const PeriodSpend: Balance = 500 * DOLLARS;
	pub const MaxLockDuration: BlockNumber = 36 * 30 * DAYS;
	pub const ChallengePeriod: BlockNumber = 7 * DAYS;
	pub const MaxCandidateIntake: u32 = 10;
	pub const SocietyPalletId: PalletId = PalletId(*b"py/socie");
   // pub const FounderSetAccount: u128 = 1;
}

ord_parameter_types! {
	pub const FounderSetAccount: u128 = 1;
	//pub const SuspensionJudgementSetAccount: u128 = 2;
}

impl pallet_society::Config for Runtime {
	type Event = Event;
	type PalletId = SocietyPalletId;
	type Currency = Balances;
	type Randomness = RandomnessCollectiveFlip;
	type CandidateDeposit = CandidateDeposit;
	type WrongSideDeduction = WrongSideDeduction;
	type MaxStrikes = MaxStrikes;
	type PeriodSpend = PeriodSpend;
	type MembershipChanged = ();
	type RotationPeriod = RotationPeriod;
	type MaxLockDuration = MaxLockDuration;
    type FounderSetOrigin = EnsureRoot<AccountId>; // EnsureSignedBy<AccountId, u128>;
	// type FounderSetOrigin =
	// 	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>;
	type SuspensionJudgementOrigin = pallet_society::EnsureFounder<Runtime>;
	type MaxCandidateIntake = MaxCandidateIntake;
	type ChallengePeriod = ChallengePeriod;
}