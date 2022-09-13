//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-13, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("./scripts/../res/genesis/testnet/frequency-spec-rococo-testnet.json"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet
// pallet_utility
// --extrinsic
// *
// --chain=./scripts/../res/genesis/testnet/frequency-spec-rococo-testnet.json
// --execution
// wasm
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 10
// --output=./scripts/../runtime/common/src/weights/pallet_utility.rs
// --template=./scripts/../.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		(21_395_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((5_201_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(7_551_000 as Weight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		(23_548_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((5_445_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(17_761_000 as Weight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		(18_410_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((5_208_000 as Weight).saturating_mul(c as Weight))
	}
}
