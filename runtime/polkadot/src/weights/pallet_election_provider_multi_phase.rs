// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_election_provider_multi_phase`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_election_provider_multi_phase.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_election_provider_multi_phase`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_election_provider_multi_phase::WeightInfo for WeightInfo<T> {
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CurrentPlannedSession (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Babe EpochIndex (r:1 w:0)
	// Storage: Babe GenesisSlot (r:1 w:0)
	// Storage: Babe CurrentSlot (r:1 w:0)
	// Storage: Staking ForceEra (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	fn on_initialize_nothing() -> Weight {
		(13_732_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_signed() -> Weight {
		(12_513_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	fn on_initialize_open_unsigned() -> Weight {
		(12_264_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	fn finalize_signed_phase_accept_solution() -> Weight {
		(26_169_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn finalize_signed_phase_reject_solution() -> Weight {
		(19_867_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, t: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((395_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 3_000
			.saturating_add((87_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:0 w:1)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, d: u32, ) -> Weight {
		(14_945_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((909_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 13_000
			.saturating_add((160_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	fn submit() -> Weight {
		(47_490_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_173_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 11_000
			.saturating_add((110_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((8_244_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 27_000
			.saturating_add((1_632_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_145_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 10_000
			.saturating_add((73_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 17_000
			.saturating_add((6_079_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 26_000
			.saturating_add((1_269_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
}
