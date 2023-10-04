
//! Autogenerated weights for `pallet_collator_allowlist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collator_allowlist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_collator_allowlist.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collator_allowlist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_allowlist::WeightInfo for WeightInfo<T> {
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorAllowlist Allowlist (r:1 w:1)
	/// Proof: CollatorAllowlist Allowlist (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	fn add() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `556`
		//  Estimated: `5538`
		// Minimum execution time: 26_079 nanoseconds.
		Weight::from_parts(27_280_000, 5538)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorAllowlist Allowlist (r:1 w:1)
	/// Proof: CollatorAllowlist Allowlist (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	fn remove() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `2507`
		// Minimum execution time: 19_596 nanoseconds.
		Weight::from_parts(20_027_000, 2507)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
