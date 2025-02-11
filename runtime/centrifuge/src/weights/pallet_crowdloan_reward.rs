//! Autogenerated weights for pallet_crowdloan_reward
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_crowdloan_reward
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_crowdloan_reward.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_crowdloan_reward::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_crowdloan_reward using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn initialize() -> Weight {
		(30_990_000 as Weight).saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn set_vesting_start() -> Weight {
		(27_518_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn set_vesting_period() -> Weight {
		(27_468_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn set_direct_payout_ratio() -> Weight {
		(27_675_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
