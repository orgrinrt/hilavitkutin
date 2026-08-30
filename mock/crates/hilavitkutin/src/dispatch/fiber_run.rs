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
use hilavitkutin_api::{HasSchedule, WorkUnit};

use crate::dispatch::engine_ctx::{
    AccumProject, ColProject, EngineCtx, GateWith, MetaPtrFor, Project, VirtualProject,
};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::wu_fn::invoke_wu_in_fiber;
use crate::meta::MetaBlock;

/// Run a unit cons-list in carrier order, projecting and executing each unit.
///
/// `A` is the scheduler bindings the walk projects each unit's `EngineCtx`
/// from. `Witnesses` is the parallel per-unit projection-index list: each
/// element is the sextuple `(RIdx, RCIdx, WCIdx, WAIdx, WVIdx, GI)` (the
/// resource-projection index for the Read set, the column-projection indices for
/// the Read and Write sets, the accumulator-projection index for the Write set,
/// the virtual-projection index for the Write set, and the schedule-gate index
/// `GI` the trunk walk reads for `On<V>` gating), all inferred at the call site.
/// `RunFiber` itself does not gate (the trunk walk does); it carries `GI` only
/// to pin it via the `GateWith` bound so the index infers alongside the others.
/// `epoch` is the current pass epoch threaded into each `EngineCtx` so a firer
/// WU's `ctx.fire::<V>()` stamps its `Virtual<V>` cell with it.
pub trait RunFiber<A, Witnesses> {
    /// Project and execute every unit in the list over `morsel`, head-first.
    ///
    /// `meta_block` is the engine-owned meta state (E4 slice 3): an `OnMeta`
    /// unit's Ctx captures a `MetaRef` into it; a consumer unit ignores it
    /// (`MetaNil`).
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize);

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
    fn run_gated<M: BitAccess>(
        &self,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        pos: USize,
        epoch: USize,
    );

    /// Project and execute only this cell's head unit over `morsel`; do not
    /// recurse the tail.
    ///
    /// The per-unit step of `run`, factored out so a const-gated walk (the
    /// GATE-2 per-trunk dispatch) can run a chosen subset of carrier positions
    /// without duplicating the projection. `WuNil` is a no-op.
    fn run_head(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize);
}

impl<A> RunFiber<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _bindings: &A, _meta_block: &MetaBlock, _morsel: MorselRange, _epoch: USize) {}

    #[inline]
    fn run_gated<M: BitAccess>(
        &self,
        _bindings: &A,
        _meta_block: &MetaBlock,
        _morsel: MorselRange,
        _dirty: M,
        _pos: USize,
        _epoch: USize,
    ) {
    }

    #[inline]
    fn run_head(
        &self,
        _bindings: &A,
        _meta_block: &MetaBlock,
        _morsel: MorselRange,
        _epoch: USize,
    ) {
    }
}

impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WVIdx, GI, WTail>
    RunFiber<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx, WVIdx, GI), WTail>> for WuCons<W, Tail>
where
    // E4 slice 1: recover the unit's schedule (`Always` or `On<V>`) so a mixed
    // carrier dispatches; `<W as HasSchedule>::Sched` feeds back as the WorkUnit
    // parameter. The blanket `impl<W: WorkUnit<Always>> HasSchedule` keeps every
    // existing Always WU unchanged. The `GateWith` bound pins the schedule-gate
    // index `GI` (the 6th witness-tuple element) so it infers alongside the
    // projection indices; the trunk walk reads it, `RunFiber` does not gate.
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
    <W as HasSchedule>::Sched: GateWith<A, GI>,
    // E4 slice 3: the unit's schedule determines its Ctx meta pointer (the 9th
    // `EngineCtx` param): `MetaNil` for `Always` / `On<V>`, `MetaRef<'f>` for
    // `OnMeta<V>`. The dispatch projects it from the engine-owned meta block.
    for<'f> <W as HasSchedule>::Sched: MetaPtrFor<'f>,
    A: Project<<W as WorkUnit<<W as HasSchedule>::Sched>>::Read, RIdx>,
    A: ColProject<<W as WorkUnit<<W as HasSchedule>::Sched>>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WCIdx>,
    for<'f> A: AccumProject<'f, <W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WAIdx>,
    for<'f> A: VirtualProject<'f, <W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WVIdx>,
    // Tie each unit's Ctx GAT to the projection of its Read set (resources), its
    // Read / Write sets (columns), its Write set (accumulators), and its Write
    // set (virtuals) over the shared bindings, for all frame lifetimes. A
    // resource-only unit projects empty column / accumulator / virtual bundles; a
    // column-, accumulator-, or virtual-bearing unit projects its real handles.
    // The accumulator and virtual bundles are lifetime-tied, so the 7th and 8th
    // Ctx params vary with `'f`.
    for<'f> W: WorkUnit<
        <W as HasSchedule>::Sched,
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Read,
            <W as WorkUnit<<W as HasSchedule>::Sched>>::Write,
            <A as Project<<W as WorkUnit<<W as HasSchedule>::Sched>>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit<<W as HasSchedule>::Sched>>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WCIdx>>::Out,
            <A as AccumProject<'f, <W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WAIdx>>::Out,
            <A as VirtualProject<'f, <W as WorkUnit<<W as HasSchedule>::Sched>>::Write, WVIdx>>::Out,
            <<W as HasSchedule>::Sched as MetaPtrFor<'f>>::Ptr,
        >,
    >,
    Tail: RunFiber<A, WTail>,
{
    #[inline]
    fn run(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize) {
        // The bindings are the resource source, the column source, the
        // accumulator source, and the virtual source (Shape A): all `project`
        // arguments are the same bindings. The resource and column pointers are
        // read out (Copy) at projection time; the accumulator and virtual bundles
        // retain a borrow of the bindings' cells for the dispatch frame. The meta
        // block is the engine-owned meta state (E4 slice 3).
        let ctx: <W as WorkUnit<<W as HasSchedule>::Sched>>::Ctx<'_> =
            EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx, WVIdx>(bindings, bindings, meta_block, epoch, morsel);
        invoke_wu_in_fiber(&self.head, &ctx);
        self.tail.run(bindings, meta_block, morsel, epoch);
    }

    #[inline]
    fn run_gated<M: BitAccess>(
        &self,
        bindings: &A,
        meta_block: &MetaBlock,
        morsel: MorselRange,
        dirty: M,
        pos: USize,
        epoch: USize,
    ) {
        // Skip the projection and execute entirely when this unit's
        // carrier-position bit is clean. The bit test is the only cost a
        // skipped unit incurs; the branch is predicated, adding no call.
        if dirty.bit(pos).0 {
            let ctx: <W as WorkUnit<<W as HasSchedule>::Sched>>::Ctx<'_> =
                EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx, WVIdx>(bindings, bindings, meta_block, epoch, morsel);
            invoke_wu_in_fiber(&self.head, &ctx);
        }
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: carrier-position successor; tracked: #72
        self.tail.run_gated(bindings, meta_block, morsel, dirty, USize(pos.0 + 1), epoch);
    }

    #[inline]
    fn run_head(&self, bindings: &A, meta_block: &MetaBlock, morsel: MorselRange, epoch: USize) {
        let ctx: <W as WorkUnit<<W as HasSchedule>::Sched>>::Ctx<'_> =
            EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx, WVIdx>(bindings, bindings, meta_block, epoch, morsel);
        invoke_wu_in_fiber(&self.head, &ctx);
    }
}
