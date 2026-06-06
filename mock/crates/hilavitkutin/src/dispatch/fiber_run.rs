//! Type-level fiber dispatch walk (domain 17, the devirtualising dispatch core).
//!
//! `RunFiber` is the sequential walk that constructs each unit's `EngineCtx`
//! and runs `execute` over a value-carrying unit list, in cons-list
//! (registration) order. `WuCons` cells carry a unit instance plus the tail,
//! `WuNil` terminates. Per unit it projects that unit's `EngineCtx` from the
//! scheduler bindings (resources via `Project`, read and write columns via
//! `ColProject`, accumulators via `AccumProject`), runs `execute` through the
//! `invoke_wu_in_fiber` shim, and recurses on the tail. The per-unit
//! projection indices are carried as parallel witness lists and inferred at the
//! call site, so `Scheduler::run` needs no turbofish.
//!
//! No unit dispatches through a stored function pointer: each `execute` is a
//! call to a statically-known concrete type's method, so the walk monomorphises
//! into one straight-line body that devirtualises under fat LTO (proven under a
//! real runtime morsel loop by sketch 202606081200, where the same workload
//! also auto-vectorised). This is the only dispatch path; there is no
//! type-erased per-unit dispatch slot.

use arvo::USize;
use arvo_bitmask::BitAccess; // re-exported through arvo-bitmask L2 per #76
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_api::WorkUnit;

use crate::dispatch::engine_ctx::{AccumProject, ColProject, EngineCtx, Project};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::wu_fn::invoke_wu_in_fiber;

/// Run a unit cons-list in carrier order, projecting and executing each unit.
///
/// `A` is the scheduler bindings the walk projects each unit's `EngineCtx`
/// from. `Witnesses` is the parallel per-unit projection-index list: each
/// element is the quad `(RIdx, RCIdx, WCIdx, WAIdx)` (the resource-projection
/// index for the Read set, the column-projection indices for the Read and Write
/// sets, and the accumulator-projection index for the Write set), all inferred
/// at the call site.
pub trait RunFiber<A, Witnesses> {
    /// Project and execute every unit in the list over `morsel`, head-first.
    fn run(&self, bindings: &A, morsel: MorselRange);

    /// Like `run`, but skip any unit whose dirty bit (at its carrier
    /// position) is clear.
    ///
    /// `dirty` carries one bit per carrier position (the runtime
    /// incremental-skip mask); `pos` is the head unit's position, threaded
    /// forward as the walk recurses. A skipped unit's projection and
    /// `execute` are both elided, so a clean unit costs only the bit test.
    /// The gate is a predicated branch around the same project plus invoke
    /// as `run`; it introduces no indirect call, so the walk still
    /// devirtualises under fat LTO (sketch 202606111000).
    fn run_gated<M: BitAccess>(&self, bindings: &A, morsel: MorselRange, dirty: M, pos: USize);
}

impl<A> RunFiber<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _bindings: &A, _morsel: MorselRange) {}

    #[inline]
    fn run_gated<M: BitAccess>(&self, _bindings: &A, _morsel: MorselRange, _dirty: M, _pos: USize) {}
}

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunFiber<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx>,
    // Tie each unit's Ctx GAT to the projection of its Read set (resources), its
    // Read / Write sets (columns), and its Write set (accumulators) over the
    // shared bindings, for all frame lifetimes. A resource-only unit projects
    // empty column and accumulator bundles; a column- or accumulator-bearing
    // unit projects its real pointers. The accumulator bundle is lifetime-tied,
    // so the 7th Ctx param varies with `'f`.
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
            <A as AccumProject<'f, <W as WorkUnit>::Write, WAIdx>>::Out,
        >,
    >,
    Tail: RunFiber<A, WTail>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        // The bindings are the resource source, the column source, and the
        // accumulator source (Shape A): both `project` arguments are the same
        // bindings. The resource and column pointers are read out (Copy) at
        // projection time; the accumulator bundle retains a borrow of the
        // bindings' live-length cells for the dispatch frame.
        let ctx: <W as WorkUnit>::Ctx<'_> =
            EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
        invoke_wu_in_fiber(&self.head, &ctx);
        self.tail.run(bindings, morsel);
    }

    #[inline]
    fn run_gated<M: BitAccess>(&self, bindings: &A, morsel: MorselRange, dirty: M, pos: USize) {
        // Skip the projection and execute entirely when this unit's
        // carrier-position bit is clean. The bit test is the only cost a
        // skipped unit incurs; the branch is predicated, adding no call.
        if dirty.bit(pos).0 {
            let ctx: <W as WorkUnit>::Ctx<'_> =
                EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
            invoke_wu_in_fiber(&self.head, &ctx);
        }
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: carrier-position successor; tracked: #72
        self.tail.run_gated(bindings, morsel, dirty, USize(pos.0 + 1));
    }
}
