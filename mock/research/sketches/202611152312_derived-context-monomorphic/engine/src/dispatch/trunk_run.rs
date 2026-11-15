//! Type-level trunk dispatch walk (domain 17, the level above `RunFiber`).
//!
//! A trunk is a sequence of fibers; `RunTrunk` is the monomorphic walk over a
//! value-carrying trunk carrier (`FiberCons` / `FiberNil`), running each fiber
//! through the unchanged `RunFiber` and recursing on the rest. `FiberCons`
//! cells carry one fiber (itself a `WuCons` unit list) plus the tail of
//! remaining fibers; `FiberNil` terminates. The per-fiber witness list is
//! carried as the head of a parallel cons-list (`WL`), so the trunk-over-fiber
//! nest is a 2-deep witness cons-list inferred at the call site with no
//! turbofish (proven by sketch 202606061400; the full
//! phase-over-trunk-over-fiber nest by sketch 202606070300).
//!
//! On a single core the fibers run sequentially, the same order a flat
//! concatenation of their unit lists would produce, so a `RunTrunk` walk is
//! output-equivalent to a flat `RunFiber` walk over the concatenated units.
//! The delegation introduces no indirect call, so the trunk walk devirtualises
//! under fat LTO exactly as the fiber walk does. At many cores each trunk, not
//! each fiber, becomes the core-pinned unit and a phase's trunks run
//! concurrently with zero cross-trunk synchronisation (their write columns are
//! disjoint); that core-pinning is a following round. `Scheduler::run`
//! continues to drive the flat single-core walk; building the nested
//! phase-trunk-fiber carrier from the plan and routing the run through it is a
//! following round.

use arvo::USize;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{FiberCons, FiberNil};

use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::morsel::MorselRange;
use crate::meta::MetaBlock;

/// Run a trunk's fibers in carrier order, delegating each to `RunFiber`.
///
/// `A` is the scheduler bindings every fiber's `RunFiber` projects from.
/// `WL` is the parallel per-fiber witness list: each element is one fiber's
/// own per-unit witness list (the `RunFiber` witness), so the whole nest is a
/// 2-deep cons-list inferred at the call site.
pub trait RunTrunk<A, WL> {
    /// Run every fiber in the trunk over `morsel`, head-first. `epoch` threads
    /// into each fiber's `RunFiber::run` for virtual firing (E4 slice 1).
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize);
}

impl<A> RunTrunk<A, Empty> for FiberNil {
    #[inline]
    fn run(&self, _bindings: &A, _meta_block: &MetaBlock, _morsel: MorselRange, _epoch: USize) {}
}

impl<A, F, Rest, FW, RestWL> RunTrunk<A, Cons<FW, RestWL>> for FiberCons<F, Rest>
where
    F: RunFiber<A, FW>,
    Rest: RunTrunk<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize) {
        // Single-core: the fibers run sequentially, the same order a flat
        // concatenation of their unit lists would produce. At many cores each
        // trunk is pinned to a core and the trunks of a phase run concurrently
        // with zero cross-trunk sync (disjoint write columns); that is a
        // following round.
        self.fiber.run(bindings, meta_block, morsel, epoch);
        self.rest.run(bindings, meta_block, morsel, epoch);
    }
}
