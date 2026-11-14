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
use notko::Maybe;

use super::{Scheduler, WorkerCtx};
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
use crate::plan::reach_cut::{CarrierCuts, cut_segments};
use crate::resource::bindings::{BindingsFor, CollectAccumCounts, RebaseBindings, accum_slice};
use crate::thread::barrier::waist_barrier;
use crate::thread::frame::{frame_await, frame_done_arrive, frame_exit_arrive};

/// The cuts of the carrier `Wus` at the plan dimensions `D`.
type Cuts<Wus, Stores, GW, D> = CarrierCuts<
    Wus,
    Stores,
    GW,
    <D as PlanDims>::Units,
    <D as PlanDims>::Stores,
    <D as PlanDims>::AdjRow,
>;

/// Persistent-pool worker mainloop (GATE-2 R4c). Spawned once per core at the
/// first `run_parallel`; parks on the frame `seq` between phases, runs its core's
/// dispatch for the published phase, and arrives at the frame done-barrier. The
/// `ctx` back-pointer is cast to the concrete `Scheduler` here (the monomorphic
/// turbofish at the spawn site supplies the types). Phase is derived from the seq
/// value: the main thread publishes one phase per `seq` bump, so
/// `phase = (seq - 1) % nphases`.
///
/// `extern "C"`, so an unwind out of the worker's own code between units aborts
/// the process rather than ending the executor's thread short of the done
/// barrier, which would leave the frame waiting for good. A unit body is
/// stopped earlier, at `invoke_wu_in_fiber`.
pub(super) extern "C" fn worker_main<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
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
    WuVals: const BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: const ConstCapacity,
    <D as PlanDims>::AdjRow: const BitAccess + Identity<Additive>,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumCounts,
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
        last = frame_await(&s.pool, last, Cfg::WAKE_SPIN_BUDGET);
        if s.pool.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(&s.pool, ncores);
            return;
        }
        if s.carrier_unit_outer().0 {
            // Deviation 9 threaded accumulator path: an accumulator-bearing
            // carrier runs unit-outer (each unit completes its full record
            // range). Each core takes its own record slice `[lo, hi)` of the N-way tiling,
            // dispatches the whole carrier ONCE over a per-core bindings copy
            // whose accumulators are offset into the core's region with fresh
            // cells, then publishes its per-accumulator live counts for the
            // main-thread merge. No phase loop: the cores walk disjoint record
            // ranges and are joined by the merge, and they meet at a barrier
            // only ahead of a unit that reads with a reach behind a writer.
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
        // plan band; `plan_band` is published between frames while every
        // worker is parked (first frame OR a consumed `replace_resource`
        // plan-dirty bit), so the read is stable under the publish/await
        // happens-before. All workers compute the same start, so the interior
        // waist-barrier counts stay matched.
        let mut p = if s.plan_band.load(Ordering::Relaxed) { 0 } else { plan_phases }; // lint:allow(no-bare-numeric) reason: phase loop start; tracked: #121
        while p < nphases {
            s.run_core_phase::<Witnesses, GW>(USize(core_id), USize(p), ncores, total, msize);
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

/// One core's unit-outer accumulator dispatch over its record slice of the N-way tiling
/// (GATE-2 deviation 9). Builds a per-core bindings copy with every accumulator
/// offset to the slice start (fresh live and `wanted` cells, slice-sized cap),
/// dispatches the whole carrier once over `[lo, hi)`, and publishes the
/// per-accumulator live and `wanted` counts into the scheduler's
/// `gate2_accum_live` and `gate2_accum_wanted` rows for this core (Relaxed; the
/// `frame_done_arrive` Release that follows publishes them to the merge). The
/// core's row is zeroed first so a non-participating core (surplus, or a
/// record-less frame's non-zero cores) contributes zeros to the merge.
///
/// A carrier whose consumer units include a read with a reach behind a writer
/// is walked in segments cut ahead of each such reader, with the phase barrier
/// crossed by every core between two segments.
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
    WuVals: const BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: const ConstCapacity,
    <D as PlanDims>::AdjRow: const BitAccess + Identity<Additive>,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumCounts,
{
    let total0 = total.0; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
    let ncores0 = ncores.0.max(1); // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
    // Zero this core's publish row up front (every slot, so the merge reads clean
    // values regardless of this frame's accumulator count).
    let mut z = 0; // lint:allow(no-bare-numeric) reason: publish-row index; tracked: #121
    while z < GATE2_MAX_ACCUMS {
        s.gate2_accum_live[core.0 * GATE2_MAX_ACCUMS + z].store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-(core,accum) publish slot reset; tracked: #121
        s.gate2_accum_wanted[core.0 * GATE2_MAX_ACCUMS + z].store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-(core,accum) wanted slot reset; tracked: #121
        z += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    // This core's exclusive accumulator region: each core appends into its
    // own region, merged after the workers rejoin in append order. Derived
    // from the shared `accum_slice` so the boundaries match the offsets the
    // merge reads.
    let (lo, region) = accum_slice(USize(total0), USize(ncores0), core);
    // A record-less frame runs the carrier once on core 0 only (a resource-only
    // unit must run exactly once, not once per core). With records, a surplus
    // core (an empty region) appends nothing and is skipped.
    let run_this = if total0 == 0 { core.0 == 0 } else { region.0 > 0 }; // lint:allow(no-bare-numeric) reason: participation guard; tracked: #121
    // The reach boundary on this path: where a consumer unit reads a plain
    // column with a reach behind a writer of it, the walk is cut ahead of that
    // unit and every core meets at a barrier there, so the writer's whole
    // range has landed on every slice before any core starts the reader. The
    // cuts are a fold over the registered units, so a carrier with no such
    // reader takes the uncut walk below and crosses nothing. They are
    // constants of the carrier's types, evaluated once when those are fixed.
    let (cuts, cut) = Cuts::<WuVals, Stores, GW, D>::CUTS;
    if cut.0 {
        let consumers = Cuts::<WuVals, Stores, GW, D>::CONSUMERS;
        let units = Cuts::<WuVals, Stores, GW, D>::UNITS;
        // A core that walks nothing still meets the others at every barrier:
        // the barrier counts every core, and the segments are the same on each.
        let per_core = if run_this {
            Maybe::Is(s.bindings.rebase_accums(lo, region))
        } else {
            Maybe::Isnt
        };
        let epoch = USize(s.virtual_epoch.load(Ordering::Relaxed));
        cut_segments(cuts, consumers, units, |segment, last| {
            if let Maybe::Is(per_core) = &per_core {
                s.wu_values.run_gated(
                    per_core,
                    &s.meta_block,
                    MorselRange::new(lo, region),
                    segment,
                    <USize as Identity<Additive>>::IDENTITY,
                    epoch,
                );
            }
            if !last.0 {
                waist_barrier(&s.pool, core, ncores, || s.clock.now_ns());
            }
        });
        if let Maybe::Is(per_core) = &per_core {
            publish_accum_counts(s, core, per_core);
        }
        return;
    }
    if !run_this {
        return;
    }
    let per_core = s.bindings.rebase_accums(lo, region);
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
            MorselRange::new(lo, region),
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
            MorselRange::new(lo, region),
            cmask,
            <USize as Identity<Additive>>::IDENTITY,
            USize(s.virtual_epoch.load(Ordering::Relaxed)),
        );
    }
    publish_accum_counts(s, core, &per_core);
}

/// Publish one core's per-accumulator live and `wanted` counts from its
/// per-core bindings copy. Only this worker writes its row, and only before
/// its `frame_done_arrive`.
fn publish_accum_counts<Cfg, WuVals, Vals, CS, D, Stores, Clk>(
    s: &Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>,
    core: USize,
    per_core: &<Vals as BindingsFor>::Bindings,
) where
    Cfg: RunCfg,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Clk: hilavitkutin_api::platform::ClockApi,
    <Vals as BindingsFor>::Bindings: CollectAccumCounts,
{
    let mut live = [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_ACCUMS];
    let mut wanted = [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_ACCUMS];
    let mut idx = <USize as Identity<Additive>>::IDENTITY;
    per_core.collect_accum_counts(&mut live, &mut wanted, &mut idx);
    let mut a = 0; // lint:allow(no-bare-numeric) reason: accum index; tracked: #121
    while a < idx.0 {
        let slot = core.0 * GATE2_MAX_ACCUMS + a; // lint:allow(no-bare-numeric) reason: per-(core,accum) slot; tracked: #121
        s.gate2_accum_live[slot].store(live[a].0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: publish per-(core,accum) live count; tracked: #121
        s.gate2_accum_wanted[slot].store(wanted[a].0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: publish per-(core,accum) wanted count; tracked: #121
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
        // A meta band is dispatched once in a frame by one core over the whole
        // record range, with no consumer unit beside it. Core 0 runs the band's
        // phase as the owner of every trunk in it (`ncores = 1`), and the other
        // cores go straight on to the waist barrier, so a maintenance unit does
        // not ride the per-core trunk split or the morsel windows. The band
        // bounds are const, so a carrier with no meta unit folds the test away.
        let pre = pre_consumer_phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) reason: leading-band bound; tracked: #121
        let cend = consumer_phase_end::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) reason: trailing-band start; tracked: #121
        if p.0 < pre || p.0 >= cend {
            if core.0 == 0 {
                let all = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY.bitnot();
                let mut rank = <USize as Identity<Additive>>::IDENTITY;
                self.wu_values.dispatch_core(
                    &self.wu_values,
                    p,
                    <USize as Identity<Additive>>::IDENTITY,
                    USize(1), // lint:allow(no-bare-numeric) reason: designated core owns every trunk; tracked: #121
                    &mut rank,
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(total)),
                    all,
                    epoch,
                );
            }
            return;
        }
        if total == 0 {
            // Record-less frame: one empty-morsel dispatch_core so a resource-only
            // trunk this core owns runs exactly once. All-ones dirty:
            // run_parallel dispatches the pure-RAW path (no incremental skip),
            // so every owned member runs.
            let all = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY.bitnot();
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
            // fiber-outer / morsel-inner (A4): dispatch_core fires the trunks this
            // core owns as compiled per-trunk monos, and the walk is driven per
            // fiber so each one windows the record range by its own plan-baked L1
            // size rather than by one scalar shared across the phase. The fiber's
            // member mask rides the `dirty` position, which is the per-unit member
            // gate `RunGatedTrunk` already consults, so a fiber-restricted mask
            // runs exactly that fiber's units.
            // Ownership survives the per-fiber calls because `rank` advances for
            // every in-phase root independently of the mask and is reset per call,
            // so each core resolves the same trunk set every time.
            let descriptors = self.fiber_dispatch.as_ref();
            let fcount = self.fiber_dispatch_count.0.min(descriptors.len()); // lint:allow(no-bare-numeric) reason: fiber descriptor count; tracked: #121
            let order = self.topo_order.as_ref();
            // Same emptiness guard `run` carries: the dispatch mask is the union
            // of the per-fiber masks, so live units but no fiber descriptors
            // would silently dispatch nothing rather than run everything.
            debug_assert!(
                self.topo_count.0 == 0 || fcount > 0, // lint:allow(no-bare-numeric) reason: emptiness guard; tracked: #121
                "run_core_phase: live units but no fiber descriptors; the plan formed no fibers the scheduler could see"
            );
            let msize = msize.0; // lint:allow(no-bare-numeric) reason: zero-window fallback length; tracked: #121
            let mut fi = 0; // lint:allow(no-bare-numeric) reason: fiber index; tracked: #121
            while fi < fcount {
                let desc = descriptors[fi];
                // FIXME(placeholders_tagged_2a77ac57ef94): the member mask is rebuilt per (phase, fiber)
                // here and in `run`; plan-baking the masks plus a fiber-to-phase map
                // would also skip the walks for fibers outside this phase.
                let mut fmask = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY;
                let kend = (desc.start.0 + desc.len.0).min(order.len()); // lint:allow(no-bare-numeric) reason: fiber slice end; tracked: #121
                let mut k = desc.start.0; // lint:allow(no-bare-numeric) reason: fiber slice start; tracked: #121
                while k < kend {
                    fmask = fmask.with_bit_set(order[k]);
                    k += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
                }
                let w = if desc.morsel_local.0 && desc.morsel_size.0 != 0 {
                    desc.morsel_size.0
                } else {
                    msize
                }; // lint:allow(no-bare-numeric) reason: window selection; tracked: #121
                let mut start = 0; // lint:allow(no-bare-numeric) reason: morsel start; tracked: #121
                while start < total {
                    let len = w.min(total - start);
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
                        fmask,
                        epoch,
                    );
                    start += len; // lint:allow(no-bare-numeric) reason: morsel step; tracked: #121
                }
                fi += 1; // lint:allow(no-bare-numeric) reason: fiber index step; tracked: #121
            }
        }
    }
}
