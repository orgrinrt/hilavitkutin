//! Type-level phase and pipeline dispatch walks (domain 17, the levels above
//! `RunTrunk`).
//!
//! A phase is a set of trunks; `RunPhase` walks a value-carrying phase carrier
//! (`TrunkCons` / `TrunkNil`), running each trunk through `RunTrunk` and
//! recursing. The pipeline is the sequence of phases separated by waists;
//! `RunPipeline` walks the pipeline carrier (`PhaseCons` / `PhaseNil`), running
//! each phase through `RunPhase` then arriving at a waist barrier before the
//! next phase. The waist is the phase boundary, the point every trunk of a
//! phase reaches before the next phase begins.
//!
//! At a single core the waist barrier is a degenerate one-arriver (it never
//! spins when the expected arriver count is one); it is present so the waist
//! composes into the nest without breaking devirt. The load-bearing N-core
//! waist is the shipped `thread::barrier::phase_barrier_arrive` over a
//! `PoolFrame`, wired in a following round. The full
//! `RunPipeline -> RunPhase -> RunTrunk -> RunFiber` nest is a 3-deep witness
//! cons-list inferred at the call site with no turbofish, and the whole nest
//! plus the waist folds into one straight-line body that devirtualises under
//! fat LTO (sketch 202606070300). On a single core every level runs its
//! children sequentially and the waist is a no-op, so a pipeline walk is
//! output-equivalent to the flat `RunFiber` walk over the units concatenated in
//! the same order.

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{PhaseCons, PhaseNil, TrunkCons, TrunkNil};

use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_run::RunTrunk;
use crate::meta::MetaBlock;

/// Run a phase's trunks in carrier order, delegating each to `RunTrunk`.
///
/// `WL` is the per-trunk witness list: each element is one trunk's own witness
/// list (the `RunTrunk` witness), so the phase-over-trunk nest is one level of
/// a 3-deep cons-list inferred at the call site.
pub trait RunPhase<A, WL> {
    /// Run every trunk in the phase over `morsel`, head-first. `epoch` threads
    /// down for virtual firing (E4 slice 1).
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize);
}

impl<A> RunPhase<A, Empty> for TrunkNil {
    #[inline]
    fn run(&self, _bindings: &A, _meta_block: &MetaBlock, _morsel: MorselRange, _epoch: USize) {}
}

impl<A, T, Rest, TW, RestWL> RunPhase<A, Cons<TW, RestWL>> for TrunkCons<T, Rest>
where
    T: RunTrunk<A, TW>,
    Rest: RunPhase<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize) {
        // Single-core: the trunks run sequentially. At many cores each trunk is
        // pinned to a core and the trunks of this phase run concurrently with
        // zero cross-trunk sync (disjoint write columns); that is a following
        // round.
        self.trunk.run(bindings, meta_block, morsel, epoch);
        self.rest.run(bindings, meta_block, morsel, epoch);
    }
}

/// Run a pipeline's phases in carrier order, waist barrier between phases.
///
/// `WL` is the per-phase witness list: each element is one phase's own witness
/// list (the `RunPhase` witness), the outermost level of the 3-deep nest.
pub trait RunPipeline<A, WL> {
    /// Run every phase over `morsel`, arriving at `barrier` (expecting
    /// `expected` arrivers) between phases.
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, barrier: &AtomicUsize, expected: USize, epoch: USize);
}

impl<A> RunPipeline<A, Empty> for PhaseNil {
    #[inline]
    fn run(&self, _bindings: &A, _meta_block: &MetaBlock, _morsel: MorselRange, _barrier: &AtomicUsize, _expected: USize, _epoch: USize) {}
}

impl<A, P, Rest, PW, RestWL> RunPipeline<A, Cons<PW, RestWL>> for PhaseCons<P, Rest>
where
    P: RunPhase<A, PW>,
    Rest: RunPipeline<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, barrier: &AtomicUsize, expected: USize, epoch: USize) {
        self.phase.run(bindings, meta_block, morsel, epoch);
        // Waist: every trunk of this phase completes before the next phase
        // begins. Degenerate one-arriver at single core (never spins at
        // expected == 1); the load-bearing N-core barrier is the shipped
        // phase_barrier_arrive, wired in a following round.
        waist_barrier(barrier, expected);
        self.rest.run(bindings, meta_block, morsel, barrier, expected, epoch);
    }
}

/// Degenerate single-core waist barrier: release `fetch_add`, spin only while
/// fewer than `expected` arrivers have landed. At `expected == 1` the first
/// arriver passes immediately. The atomic counter operates on `usize`, the
/// width the atomic API fixes; the same boundary the shipped
/// `thread::barrier::phase_barrier_arrive` documents (tracked #121).
#[inline]
fn waist_barrier(counter: &AtomicUsize, expected: USize) {
    let arrived = counter.fetch_add(1, Ordering::AcqRel) + 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes usize increment; tracked: #121
    if arrived < expected.0 {
        while counter.load(Ordering::Acquire) < expected.0 {
            core::hint::spin_loop();
        }
    }
}
