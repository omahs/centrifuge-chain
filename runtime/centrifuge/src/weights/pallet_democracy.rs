//! Autogenerated weights for pallet_democracy
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
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_democracy.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_democracy::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_democracy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn propose() -> Weight {
		(116_040_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn second(s: u32) -> Weight {
		(69_532_000 as Weight) // Standard Error: 4_000
			.saturating_add((330_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn vote_new(r: u32) -> Weight {
		(86_222_000 as Weight) // Standard Error: 5_000
			.saturating_add((436_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn vote_existing(r: u32) -> Weight {
		(84_827_000 as Weight) // Standard Error: 4_000
			.saturating_add((440_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn emergency_cancel() -> Weight {
		(44_097_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn blacklist(p: u32) -> Weight {
		(128_464_000 as Weight) // Standard Error: 9_000
			.saturating_add((612_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}

	fn external_propose(v: u32) -> Weight {
		(24_844_000 as Weight) // Standard Error: 0
			.saturating_add((60_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn external_propose_majority() -> Weight {
		(9_584_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn external_propose_default() -> Weight {
		(9_687_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn fast_track() -> Weight {
		(44_806_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn veto_external(v: u32) -> Weight {
		(46_584_000 as Weight) // Standard Error: 2_000
			.saturating_add((92_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn cancel_proposal(p: u32) -> Weight {
		(101_156_000 as Weight) // Standard Error: 5_000
			.saturating_add((635_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}

	fn cancel_referendum() -> Weight {
		(30_798_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn cancel_queued(r: u32) -> Weight {
		(51_624_000 as Weight) // Standard Error: 6_000
			.saturating_add((3_278_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn on_initialize_base(r: u32) -> Weight {
		(6_358_000 as Weight) // Standard Error: 7_000
			.saturating_add((7_021_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn on_initialize_base_with_launch_period(r: u32) -> Weight {
		(15_193_000 as Weight) // Standard Error: 8_000
			.saturating_add((7_097_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn delegate(r: u32) -> Weight {
		(81_876_000 as Weight) // Standard Error: 42_000
			.saturating_add((9_719_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}

	fn undelegate(r: u32) -> Weight {
		(43_755_000 as Weight) // Standard Error: 12_000
			.saturating_add((9_679_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}

	fn clear_public_proposals() -> Weight {
		(12_057_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn note_preimage(b: u32) -> Weight {
		(64_129_000 as Weight) // Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn note_imminent_preimage(b: u32) -> Weight {
		(44_936_000 as Weight) // Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn reap_preimage(b: u32) -> Weight {
		(61_077_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn unlock_remove(r: u32) -> Weight {
		(56_504_000 as Weight) // Standard Error: 3_000
			.saturating_add((217_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn unlock_set(r: u32) -> Weight {
		(53_543_000 as Weight) // Standard Error: 3_000
			.saturating_add((365_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}

	fn remove_vote(r: u32) -> Weight {
		(34_660_000 as Weight) // Standard Error: 3_000
			.saturating_add((331_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn remove_other_vote(r: u32) -> Weight {
		(34_959_000 as Weight) // Standard Error: 3_000
			.saturating_add((333_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
