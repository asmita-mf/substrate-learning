
#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
		[pallet_usd_rate, UsdRate]
		[pallet_vtbdex, VtbDex]
		[pallet_cross_chain, CrossChainActions]
		[pallet_vtbt, VtbTErc20]
	);
}