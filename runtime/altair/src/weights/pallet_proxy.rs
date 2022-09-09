//! Autogenerated weights for pallet_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_proxy.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_proxy::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_proxy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn proxy(p: u32) -> Weight {
		(40_684_000 as Weight) // Standard Error: 12_000
			.saturating_add((183_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}

	fn proxy_announced(a: u32, p: u32) -> Weight {
		(84_364_000 as Weight) // Standard Error: 14_000
			.saturating_add((349_000 as Weight).saturating_mul(a as Weight)) // Standard Error: 14_000
			.saturating_add((123_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn remove_announcement(a: u32, p: u32) -> Weight {
		(57_353_000 as Weight) // Standard Error: 8_000
			.saturating_add((443_000 as Weight).saturating_mul(a as Weight)) // Standard Error: 8_000
			.saturating_add((57_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn reject_announcement(a: u32, p: u32) -> Weight {
		(57_966_000 as Weight) // Standard Error: 9_000
			.saturating_add((429_000 as Weight).saturating_mul(a as Weight)) // Standard Error: 9_000
			.saturating_add((46_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn announce(a: u32, p: u32) -> Weight {
		(76_007_000 as Weight) // Standard Error: 10_000
			.saturating_add((392_000 as Weight).saturating_mul(a as Weight)) // Standard Error: 11_000
			.saturating_add((152_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}

	fn add_proxy(p: u32) -> Weight {
		(63_429_000 as Weight) // Standard Error: 12_000
			.saturating_add((245_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn remove_proxy(p: u32) -> Weight {
		(63_905_000 as Weight) // Standard Error: 13_000
			.saturating_add((260_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn remove_proxies(p: u32) -> Weight {
		(53_529_000 as Weight) // Standard Error: 9_000
			.saturating_add((198_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn anonymous(p: u32) -> Weight {
		(70_125_000 as Weight) // Standard Error: 17_000
			.saturating_add((204_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}

	fn kill_anonymous(p: u32) -> Weight {
		(56_171_000 as Weight) // Standard Error: 8_000
			.saturating_add((155_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
