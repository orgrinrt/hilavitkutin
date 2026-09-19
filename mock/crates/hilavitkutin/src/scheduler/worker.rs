//! The GATE-2 persistent-pool worker mainloop, its unit-outer accumulator
//! dispatch, and `Scheduler::run_core_phase`, the per-(core,phase) primitive
//! only the worker mainloop calls.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). No behaviour change. `super::Scheduler` and `WorkerCtx`
//! stay defined directly in `scheduler` (`mod.rs`): this module is a
//! descendant, so it reaches their private fields without any widening.
//! `worker_main` is `pub(super)` because `scheduler::run_parallel` (a
//! sibling) spawns it; it in turn calls `s.carrier_unit_outer()`, `pub(super)`
//! in `scheduler::run_parallel`.

use core::sync::atomic::Ordering;

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_bitmask::{BitAccess, BitLogic};
use arvo_tensor::ConstCapacity;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;

use super::{Scheduler, WorkerCtx};
use crate::dispatch::core_mask::{phase_mask, phase_trunk_count};
use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_dispatch::RunTrunkDispatch;
use crate::plan::PlanDims;
use crate::plan::grouping::{
    BundleMasks,
    GATE2_MAX_ACCUMS,
    consumer_mask,
    consumer_phase_end,
    phase_count,
    plan_phase_count,
    pre_consumer_phase_count,
};
use crate::resource::bindings::{BindingsFor, CollectAccumLive, RebaseBindings};
use crate::thread::barrier::waist_barrier;
use crate::thread::frame::{frame_await, frame_done_arrive, frame_exit_arrive};

/// Persistent-pool worker mainloop (GATE-2 R4c). Spawned once per core at the
/// first `run_parallel`; parks on the frame `seq` between phases, runs its core's
/// dispatch for the published phase, and arrives at the frame done-barrier. The
/// `ctx` back-pointer is cast to the concrete `Scheduler` here (the monomorphic
/// turbofish at the spawn site supplies the types). Phase is derived from the seq
/// value: the main thread publishes one phase per `seq` bump, so
/// `phase = (seq - 1) % nphases`.
pub(super) fn worker_main<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
    ctx: *const WorkerCtx,
) where
    Cfg: RunCfg,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Clk: hilavitkutin_api::platform::ClockApi,
    WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
    WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
    WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: ConstCapacity,
    <D as PlanDims>::AdjRow: BitAccess + Identity<Additive>,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumLive,
{
    // SAFETY: `ctx` points into the pinned scheduler's `worker_ctxs`, valid until
    // `await_exit` runs at Drop (which joins before teardown).
    let core_id = unsafe { (*ctx).core_id };
    let sched = unsafe { (*ctx).sched } as *const Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>;
    // SAFETY: the pinned scheduler outlives every worker. This shared
    // reference is held live across every park, so the main thread must never
    // write any scheduler field through a plain `*mut` while it is alive: the
    // between-frame mutated, worker-visible fields (`first_frame`,
    // `virtual_epoch`, `store_dirty`, the `meta_block` cells) all carry
    // interior mutability, so the main thread writes through a shared
    // reference and never invalidates this borrow under the aliasing model.
    let s = unsafe { &*sched };
    let ncores = s.gate2_ncores;
    let nphases = s.gate2_nphases.0.max(1); // lint:allow(no-bare-numeric) reason: avoid modulo by zero; tracked: #121
    // E4 parity: leading plan-band phase count, skipped on a clean frame
    // (mirrors single-core dispatch_trunks).
    let plan_phases = plan_phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >()
    .0; // lint:allow(no-bare-numeric) reason: phase loop offset; tracked: #121
    let total = s.record_count;
    let msize = USize(Cfg::MORSEL_SIZE.0.max(1)); // lint:allow(no-bare-numeric) reason: morsel length guard; tracked: #121
    let mut last = <USize as Identity<Additive>>::IDENTITY;
    loop {
        last = frame_await(&s.pool, last);
        if s.pool.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(&s.pool, ncores);
            return;
        }
        if s.carrier_unit_outer().0 {
            // Deviation 9 threaded accumulator path: an accumulator-bearing
            // carrier runs unit-outer (each unit completes its full record
            // range). Each core takes its head+tail record slice `[lo, hi)`,
            // dispatches the whole carrier ONCE over a per-core bindings copy
            // whose accumulators are offset into the core's region with fresh
            // cells, then publishes its per-accumulator live counts for the
            // main-thread merge. No phase loop or waist barrier: cores are
            // independent over disjoint record ranges, joined only by the merge.
            worker_accum_unit_outer::<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
                s,
                USize(core_id),
                ncores,
                total,
            );
            frame_done_arrive(&s.pool, ncores);
            continue;
        }
        // One wake per frame: the worker runs ALL waist-bounded phases hot,
        // crossing each interior waist via the worker-side sense-reversing
        // barrier (the canonical worker-side sync; the main thread no longer
        // round-trips per phase). Phase order is the array order 0..nphases.
        // On a clean (not plan-dirty) frame the loop starts past the leading
        // plan band; `first_frame` is written between frames while every
        // worker is parked, so the read is stable under the publish/await
        // happens-before. All workers compute the same start, so the interior
        // waist-barrier counts stay matched.
        let mut p = if s.first_frame.load(Ordering::Relaxed) { 0 } else { plan_phases }; // lint:allow(no-bare-numeric) reason: phase loop start; tracked: #121
        while p < nphases {
            s.run_core_phase::<Witnesses, GW>(
                &s.gate2_phase,
                &s.gate2_trunk,
                s.gate2_n,
                USize(core_id),
                USize(p),
                ncores,
                total,
                msize,
            );
            if p + 1 < nphases {
                // every worker participates in each waist, even one that owned
                // no trunk this phase, so `expected` is the full core count. The
                // barrier times this core's follower park into the idle
                // accumulator using the scheduler's clock.
                waist_barrier(&s.pool, USize(core_id), ncores, || s.clock.now_ns());
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase loop step; tracked: #121
        }
        frame_done_arrive(&s.pool, ncores);
    }
}

/// One core's unit-outer accumulator dispatch over its head+tail record slice
/// (GATE-2 deviation 9). Builds a per-core bindings copy with every accumulator
/// offset to the slice start (fresh live cells, slice-sized cap), dispatches the
/// whole carrier once over `[lo, hi)`, and publishes the per-accumulator live
/// counts into the scheduler's `gate2_accum_live` row for this core (Relaxed; the
/// `frame_done_arrive` Release that follows publishes them to the merge). The
/// core's row is zeroed first so a non-participating core (surplus, or a
/// record-less frame's non-zero cores) contributes zeros to the merge.
fn worker_accum_unit_outer<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
    s: &Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>,
    core: USize,
    ncores: USize,
    total: USize,
) where
    Cfg: RunCfg,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Clk: hilavitkutin_api::platform::ClockApi,
    WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
    WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: ConstCapacity,
    <D as PlanDims>::AdjRow: BitAccess + Identity<Additive>,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumLive,
{
    let total0 = total.0; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
    let ncores0 = ncores.0.max(1); // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
    // Zero this core's publish row up front (every slot, so the merge reads clean
    // values regardless of this frame's accumulator count).
    let mut z = 0; // lint:allow(no-bare-numeric) reason: publish-row index; tracked: #121
    while z < GATE2_MAX_ACCUMS {
        s.gate2_accum_live[core.0 * GATE2_MAX_ACCUMS + z].store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-(core,accum) publish slot reset; tracked: #121
        z += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    // Head+tail record slice for this core (mirrors run_core_phase's split).
    let per = total0.div_ceil(ncores0); // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
    let lo = (core.0 * per).min(total0); // lint:allow(no-bare-numeric) reason: slice start; tracked: #121
    let hi = (lo + per).min(total0); // lint:allow(no-bare-numeric) reason: slice end; tracked: #121
    // A record-less frame runs the carrier once on core 0 only (a resource-only
    // unit must run exactly once, not once per core). With records, a surplus
    // core (`lo == hi`) appends nothing and is skipped.
    let run_this = if total0 == 0 { core.0 == 0 } else { lo < hi }; // lint:allow(no-bare-numeric) reason: participation guard; tracked: #121
    if !run_this {
        return;
    }
    let region = hi - lo; // lint:allow(no-bare-numeric) reason: slice length; tracked: #121
    let per_core = s.bindings.rebase_accums(USize(lo), USize(region));
    // E4 parity: meta units do not ride the per-core slice walk (the designated
    // thread dispatches them once per frame around the publish/await window).
    // A no-meta carrier keeps the ungated whole-carrier walk; the band counts
    // are const, so this branch folds at compile time.
    let pre = pre_consumer_phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    let cend = consumer_phase_end::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    let nphases = phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    if pre.0 == 0 && cend.0 == nphases.0 {
        s.wu_values.run(
            &per_core,
            &s.meta_block,
            MorselRange::new(USize(lo), USize(region)),
            USize(s.virtual_epoch.load(Ordering::Relaxed)),
        );
    } else {
        let cmask = consumer_mask::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >();
        s.wu_values.run_gated(
            &per_core,
            &s.meta_block,
            MorselRange::new(USize(lo), USize(region)),
            cmask,
            <USize as Identity<Additive>>::IDENTITY,
            USize(s.virtual_epoch.load(Ordering::Relaxed)),
        );
    }
    // Publish this core's per-accumulator live counts.
    let mut live = [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_ACCUMS];
    let mut idx = <USize as Identity<Additive>>::IDENTITY;
    per_core.collect_accum_live(&mut live, &mut idx);
    let mut a = 0; // lint:allow(no-bare-numeric) reason: accum index; tracked: #121
    while a < idx.0 {
        s.gate2_accum_live[core.0 * GATE2_MAX_ACCUMS + a].store(live[a].0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: publish per-(core,accum) live count; tracked: #121
        a += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
}

impl<
    Cfg: RunCfg,
    WuVals,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Stores,
    Clk: hilavitkutin_api::platform::ClockApi,
> Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
{
    fn run_core_phase<Witnesses, GW>(
        &self,
        phase: &[USize],
        trunk: &[USize],
        n: USize,
        core: USize,
        p: USize,
        ncores: USize,
        total: USize,
        msize: USize,
    ) where
        WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
        WuVals: RunTrunkDispatch<
                WuVals,
                <Vals as BindingsFor>::Bindings,
                Witnesses,
                GW,
                Stores,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
                0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
            >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity<Additive>,
    {
        // E4 slice 1: the worker reads the per-frame epoch set by `run_parallel`
        // before the publish (stable for the frame under the publish/await
        // happens-before).
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        let total = total.0; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
        let tphase = phase_trunk_count(phase, trunk, n, p);
        // All-ones dirty: run_parallel dispatches the pure-RAW path (no
        // incremental skip), so every owned member runs.
        let all = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY.bitnot();
        if tphase.0 == 1 && ncores.0 > 1 && total > 0 {
            // Head+tail convergence (spec :770): a single-trunk waist-bounded
            // phase is the serial bottleneck. Ownership there is by record slice,
            // not by trunk, so all cores walk the same one trunk (the whole-phase
            // mask) over a disjoint ceil-sized record slice, the union covering
            // [0, total) with no gap or overlap (surplus cores get lo == hi and
            // do nothing). Stays on the runtime-mask run_gated path; the per-trunk
            // dispatch_core walk cannot express a record-range split.
            let per = total.div_ceil(ncores.0); // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
            let lo = (core.0 * per).min(total); // lint:allow(no-bare-numeric) reason: slice start; tracked: #121
            let hi = (lo + per).min(total); // lint:allow(no-bare-numeric) reason: slice end; tracked: #121
            let mask = phase_mask::<<D as PlanDims>::AdjRow>(phase, n, p);
            let msize = msize.0; // lint:allow(no-bare-numeric) reason: morsel length; tracked: #121
            let mut start = lo; // lint:allow(no-bare-numeric) reason: morsel start; tracked: #121
            while start < hi {
                let len = msize.min(hi - start);
                self.wu_values.run_gated(
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(USize(start), USize(len)),
                    mask,
                    <USize as Identity<Additive>>::IDENTITY,
                    epoch,
                );
                start += len; // lint:allow(no-bare-numeric) reason: morsel step; tracked: #121
            }
        } else if total == 0 {
            // Record-less frame: one empty-morsel dispatch_core so a resource-only
            // trunk this core owns runs exactly once.
            let mut rank = <USize as Identity<Additive>>::IDENTITY;
            self.wu_values.dispatch_core(
                &self.wu_values,
                p,
                core,
                ncores,
                &mut rank,
                &self.bindings,
                &self.meta_block,
                MorselRange::new(
                    <USize as Identity<Additive>>::IDENTITY,
                    <USize as Identity<Additive>>::IDENTITY,
                ),
                all,
                epoch,
            );
        } else {
            // Ordinary trunk-rank ownership over the full range: per morsel,
            // dispatch_core fires the trunks this core owns as compiled per-trunk
            // monos (one runtime ownership branch per trunk-root, not per unit).
            let msize = msize.0; // lint:allow(no-bare-numeric) reason: morsel length; tracked: #121
            let mut start = 0; // lint:allow(no-bare-numeric) reason: morsel start; tracked: #121
            while start < total {
                let len = msize.min(total - start);
                let mut rank = <USize as Identity<Additive>>::IDENTITY;
                self.wu_values.dispatch_core(
                    &self.wu_values,
                    p,
                    core,
                    ncores,
                    &mut rank,
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(USize(start), USize(len)),
                    all,
                    epoch,
                );
                start += len; // lint:allow(no-bare-numeric) reason: morsel step; tracked: #121
            }
        }
    }
}
