//! GATE-2 outer per-trunk dispatcher (round 2b, domain 17).
//!
//! `RunGatedTrunk` (round 2a) runs the members of ONE trunk. This is the outer
//! driver that runs EVERY trunk, in phase order, single-core, over the flat
//! `WuCons` / `WuNil` carrier: a const-gated walk that, at each carrier position
//! that is a trunk-root (`trunk_of(POS) == POS`, a compile-time test that DCEs
//! the non-root positions away), dispatches that trunk's per-trunk mono
//! (`RunGatedTrunk` for `TRUNK = POS`). The position is threaded as a `const POS`
//! (the recursion is `{ POS + 1 }`), so `POS` reaches the inner `TRUNK` by
//! identity: there is no generic-const-expression in const-argument position.
//! Sketch `202606080200` proved this compiles and orders correctly with the real
//! `RunGatedTrunk` instantiated under the grouping bound at each step.
//!
//! Phase ordering is the caller's runtime phase loop, not a const on this walk: a
//! trunk-root fires only in the pass whose runtime `p` equals its compile-time
//! `phase_of(POS)`. Phases run in dependency (waist) order, so a phase-`p+1`
//! reader sees every record a phase-`p` writer produced; within a phase trunks
//! touch disjoint columns, so their order is immaterial; each trunk runs its
//! members in carrier (RCM-reordered topological) order. The result is
//! output-equivalent to the flat `RunFiber` walk, while every trunk is an
//! independently monomorphised program ready for the core-pinning of G2-N.
//!
//! `Witnesses` (the per-unit `RunFiber` projection list) and `GW` (the grouping
//! witness list) are both for the whole carrier `Full` and stay fixed through the
//! recursion, exactly as `Full` does; only `POS` advances. `dirty` is the
//! incremental-skip mask, threaded into each `RunGatedTrunk` so a clean member is
//! skipped exactly as the flat `run_gated` path skips it. The no-skip path passes
//! an all-ones mask.

use core::marker::PhantomData;

use arvo::strategy::Identity;
use arvo::USize;
use arvo_bitmask::BitAccess;
use arvo_tensor::{Capacity, ConstCapacity};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};

use crate::dispatch::engine_ctx::Here;
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_gate::RunGatedTrunk;
use crate::meta::MetaBlock;
use crate::plan::grouping::{phase_of, trunk_of, BundleMasks};

/// Whether carrier position `POS` is a trunk-root (its own component-min id), as
/// an associated const so the gate reads `IsRoot::<..>::IS` rather than an inline
/// `const { trunk_of(..) == POS }` block, which the generic-constant complexity
/// limit rejects when the grouping carries this many parameters (the same reason
/// `trunk_gate::Member` exists).
struct IsRoot<Full, Stores, GW, CU, CS, Adj, const POS: usize>(PhantomData<(Full, Stores, GW, CU, CS, Adj)>); // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121

impl<Full, Stores, GW, CU: Capacity + const ConstCapacity, CS: Capacity, Adj: const BitAccess + Identity, const POS: usize> IsRoot<Full, Stores, GW, CU, CS, Adj, POS> // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121
where
    Full: const BundleMasks<Stores, GW, CS>,
{
    const IS: bool = trunk_of::<Full, Stores, GW, CU, CS, Adj>(USize(POS)).0 == POS; // lint:allow(no-bare-numeric) reason: trunk-root compile-time test; tracked: #121
}

/// The waist-bounded phase of carrier position `POS`, as an associated const (the
/// `const { phase_of(..) }` block hits the same complexity limit as `IsRoot`).
struct PhaseAt<Full, Stores, GW, CU, CS, Adj, const POS: usize>(PhantomData<(Full, Stores, GW, CU, CS, Adj)>); // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121

impl<Full, Stores, GW, CU: Capacity + const ConstCapacity, CS: Capacity, Adj: const BitAccess + Identity, const POS: usize> PhaseAt<Full, Stores, GW, CU, CS, Adj, POS> // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121
where
    Full: const BundleMasks<Stores, GW, CS>,
{
    const VAL: usize = phase_of::<Full, Stores, GW, CU, CS, Adj>(USize(POS)).0; // lint:allow(no-bare-numeric) reason: compile-time phase of position; tracked: #121
}

/// Drive every trunk-root at or after carrier position `POS` for one phase pass.
///
/// `Full` is the whole carrier type (fixed through the recursion); `A` the
/// scheduler bindings; `Witnesses` the per-unit `RunFiber` projection list and
/// `GW` the grouping witness list (both for `Full`, fixed); `Stores` the global
/// store set; `CU` / `CS` the unit / store capacities; `Adj` the adjacency row
/// word. `POS` is the head's carrier position (a `const`, threaded `{POS+1}`).
pub trait RunTrunkDispatch<Full, A, Witnesses, GW, Stores, CU: Capacity, CS: Capacity, Adj, const POS: usize> { // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121
    /// Dispatch every trunk-root from `POS` on whose phase equals `p`, over
    /// `morsel`, skipping members clear in `dirty`. `epoch` gates `On<V>` members.
    fn dispatch<M: BitAccess>(&self, full: &Full, p: USize, meta_block: &MetaBlock, bindings: &A, morsel: MorselRange, dirty: M, epoch: USize);

    /// Core-aware variant of `dispatch`: fire only the in-phase-`p` trunk-roots
    /// this `core` owns. Ownership is the within-phase trunk rank modulo
    /// `ncores` (the R4a rule, `core_mask::core_phase_mask`): `rank` threads
    /// through the recursion counting in-phase-`p` roots seen so far, the root
    /// fires iff `rank % ncores == core`, and `rank` advances for every in-phase
    /// root whether owned or not. Single-core (`ncores == 1`) makes every rank
    /// `% 1 == 0`, so core 0 owns every trunk (the degenerate full walk).
    fn dispatch_core<M: BitAccess>(
        &self,
        full: &Full,
        p: USize,
        core: USize,
        ncores: USize,
        rank: &mut USize,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
    );
}

impl<Full, A, Witnesses, GW, Stores, CU: Capacity, CS: Capacity, Adj, const POS: usize> RunTrunkDispatch<Full, A, Witnesses, GW, Stores, CU, CS, Adj, POS> for WuNil { // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121
    #[inline]
    fn dispatch<M: BitAccess>(&self, _full: &Full, _p: USize, _meta_block: &MetaBlock, _bindings: &A, _morsel: MorselRange, _dirty: M, _epoch: USize) {}

    #[inline]
    fn dispatch_core<M: BitAccess>(
        &self,
        _full: &Full,
        _p: USize,
        _core: USize,
        _ncores: USize,
        _rank: &mut USize,
        _bindings: &A,
        _meta_block: &MetaBlock,
        _morsel: MorselRange,
        _dirty: M,
        _epoch: USize,
    ) {
    }
}

impl<Full, A, W, Tail, Witnesses, GW, Stores, CU: Capacity + const ConstCapacity, CS: Capacity, Adj: const BitAccess + Identity, const POS: usize> RunTrunkDispatch<Full, A, Witnesses, GW, Stores, CU, CS, Adj, POS> for WuCons<W, Tail> // lint:allow(no-bare-numeric) reason: const-generic carrier position; tracked: #121
where
    Tail: RunTrunkDispatch<Full, A, Witnesses, GW, Stores, CU, CS, Adj, { POS + 1 }>,
    Full: const BundleMasks<Stores, GW, CS>,
    Full: RunGatedTrunk<Full, A, Witnesses, GW, Stores, CU, CS, Adj, POS, Here>,
{
    #[inline]
    fn dispatch<M: BitAccess>(&self, full: &Full, p: USize, meta_block: &MetaBlock, bindings: &A, morsel: MorselRange, dirty: M, epoch: USize) {
        // Is carrier position POS a trunk-root (its own component min)? Compile
        // time; non-roots fold away. If so, run it only in its phase's pass.
        if IsRoot::<Full, Stores, GW, CU, CS, Adj, POS>::IS {
            if PhaseAt::<Full, Stores, GW, CU, CS, Adj, POS>::VAL == p.0 { // lint:allow(no-bare-numeric) reason: phase-pass match; tracked: #121
                <Full as RunGatedTrunk<Full, A, Witnesses, GW, Stores, CU, CS, Adj, POS, Here>>::run_trunk(full, bindings, meta_block, morsel, dirty, epoch);
            }
        }
        self.tail.dispatch(full, p, meta_block, bindings, morsel, dirty, epoch);
    }

    #[inline]
    fn dispatch_core<M: BitAccess>(
        &self,
        full: &Full,
        p: USize,
        core: USize,
        ncores: USize,
        rank: &mut USize,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
    ) {
        // Compile-time: is POS a trunk-root, and in phase p this pass? Non-roots
        // and off-phase roots fold away.
        if IsRoot::<Full, Stores, GW, CU, CS, Adj, POS>::IS {
            if PhaseAt::<Full, Stores, GW, CU, CS, Adj, POS>::VAL == p.0 { // lint:allow(no-bare-numeric) reason: phase-pass match; tracked: #121
                // This core owns the root iff its within-phase rank (in-phase
                // roots seen so far) modulo core count equals the core id.
                if ncores.0 != 0 && rank.0 % ncores.0 == core.0 { // lint:allow(no-bare-numeric) reason: round-robin trunk ownership; tracked: #121
                    <Full as RunGatedTrunk<Full, A, Witnesses, GW, Stores, CU, CS, Adj, POS, Here>>::run_trunk(full, bindings, meta_block, morsel, dirty, epoch);
                }
                // Advance rank for every in-phase root (owned or not), matching
                // core_phase_mask's "count of in-phase roots below".
                rank.0 += 1; // lint:allow(no-bare-numeric) reason: within-phase trunk rank; tracked: #121
            }
        }
        self.tail.dispatch_core(full, p, core, ncores, rank, bindings, meta_block, morsel, dirty, epoch);
    }
}
