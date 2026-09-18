//! `Scheduler::run_parallel`: the GATE-2 persistent-pool N-core dispatch, plus
//! `carrier_unit_outer`, the accumulator-carrier test both this and the
//! worker mainloop (`scheduler::worker`) need.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). No behaviour change. `super::Scheduler`, `WorkerCtx`,
//! `SendCtxPtr` and `empty_pool_frame` stay defined directly in `scheduler`
//! (`mod.rs`): this module is a descendant, so it reaches their private
//! fields without any widening. `carrier_unit_outer` is `pub(super)` because
//! `scheduler::worker` (a sibling) calls it on the worker mainloop's fast
//! path.

use core::pin::Pin;
use core::sync::atomic::Ordering;

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::{BitAccess, BitLogic};
use arvo_tensor::ConstCapacity;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::platform::{OnePointerClosure, ThreadPoolApi};
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;

use super::{Scheduler, SendCtxPtr, UnitsFitGate2, WorkerCtx};
use crate::dispatch::core_mask::grouping_arrays;
use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_dispatch::RunTrunkDispatch;
use crate::meta::fold_ema;
use crate::plan::grouping::{
    BundleMasks,
    GATE2_MAX_ACCUMS,
    GATE2_MAX_UNITS,
    consumer_phase_end,
    plan_phase_count,
    pre_consumer_phase_count,
};
use crate::plan::{AccessMask, PlanDims};
use crate::resource::bindings::{
    BindingsFor,
    CollectAccumLive,
    MergeAccums,
    RebaseBindings,
    ResetAccumulators,
};
use crate::thread::class::runnable_worker_count;
use crate::thread::frame::{frame_await_done, frame_publish};

/// Hand `f` to the executor after forcing the api's one-pointer gate on
/// it, so the closure the engine builds for a worker fails the build at
/// monomorphisation if it ever grows past one pointer, rather than
/// reaching an executor that cannot carry it without allocating.
fn spawn_one_pointer<P, F>(pool: &P, f: F)
where
    P: ThreadPoolApi,
    F: FnOnce() + Send + 'static,
{
    let () = OnePointerClosure::<F>::FITS;
    pool.spawn(f);
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
    /// Dispatch the carrier as per-core trunk programs joined by waist barriers
    /// (GATE-2 N-core dispatch) on the executor `pool`.
    ///
    /// op's runtime-mask mechanism: the canonical waist-bounded phase axis (R2)
    /// and per-phase round-robin trunk-to-core ownership (R4a `core_mask`)
    /// select, for each `(core, phase)`, the carrier positions that core owns in
    /// that phase. Each selection is a `run_gated` walk over the flat carrier, so
    /// unit bodies devirtualise exactly as the single-core walk does; only the
    /// per-unit ownership test is a runtime branch. It is output-equivalent to
    /// `run` for a pure read-after-write carrier: phases run in waist order, so a
    /// phase-`p+1` reader sees every record a phase-`p` writer produced; trunks
    /// within a phase touch disjoint columns, so their order is immaterial; each
    /// trunk's units run in carrier (topological) order. Single-core
    /// (`ncores == 1`) is the degenerate case with one core owning every trunk
    /// per phase, not a separate path.
    ///
    /// The first call computes the `phase` / `trunk` arrays once and hands the
    /// executor one closure per worker, on the executor's `worker_count`
    /// clamped into `1..=MAX_CORES` (`runnable_worker_count`). The workers
    /// persist and park between frames. Each frame the calling thread runs the
    /// leading meta bands, publishes the frame, waits while the workers run
    /// every phase and cross each waist on their own barrier, then merges the
    /// accumulator regions and runs the trailing bands.
    ///
    /// `Witnesses` is the per-unit projection list (for the carrier walk) and
    /// `GW` the grouping witness list (for the const grouping that fills the
    /// `phase` / `trunk` arrays), both inferred at the call site.
    pub fn run_parallel<Witnesses, GW, P>(self: Pin<&mut Self>, pool: &P) -> Cfg::Out
    where
        Cfg::Out: Default,
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
        <Vals as BindingsFor>::Bindings:
            ResetAccumulators + RebaseBindings + CollectAccumLive + MergeAccums,
        P: ThreadPoolApi,
    {
        // Cross-cap guard (#690): the grouping producer is sized by `D::Units`,
        // but the gate2_phase / gate2_trunk scratch is still sized by
        // GATE2_MAX_UNITS. A `D` whose `Units` capacity exceeds that ceiling
        // would index past those arrays. Forcing this trait's assoc const fails
        // the build per monomorphisation with a clear message rather than an
        // out-of-bounds panic. Removed when #690 lifts the parallel scratch onto
        // `Units`. (A named assoc const, not an inline `const {}`, because the
        // latter is an anon generic constant the GCE grammar rejects.)
        let () = <D as UnitsFitGate2>::ASSERT_UNITS_FIT;
        // SAFETY: the scheduler is pinned (the receiver is `Pin<&mut Self>`), so
        // its address is fixed for the workers' raw pointers. Take a raw pointer
        // so no live `&mut` aliases the workers' `*const Self`; the `&mut`
        // reborrows below are confined to moments when every worker is parked
        // (before the first publish and between frames), matching the proven
        // sketch discipline (202606071930).
        let me: *mut Self = unsafe { self.get_unchecked_mut() };

        // First call: compute the const grouping into the gate2_* fields and spawn
        // the persistent pool once. Workers park immediately on `seq == 0`.
        let already = unsafe { (*me).spawned.0 };
        if !already {
            let mut phase = [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_UNITS];
            let mut trunk = [<USize as Identity<Additive>>::IDENTITY; GATE2_MAX_UNITS];
            let n = grouping_arrays::<
                WuVals,
                Stores,
                GW,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
            >(&mut phase, &mut trunk);
            let count = n.0; // lint:allow(no-bare-numeric) reason: live unit count; tracked: #121
            let mut nphases = 0; // lint:allow(no-bare-numeric) reason: phase count accumulator; tracked: #121
            let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
            while u < count {
                if phase[u].0 + 1 > nphases {
                    nphases = phase[u].0 + 1; // lint:allow(no-bare-numeric) reason: phase successor; tracked: #121
                }
                u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            // The engine runs on the executor's count clamped into
            // `1..=MAX_CORES`; every later read takes this stored value.
            let ncores = runnable_worker_count(pool.worker_count());
            // SAFETY: no workers running yet; exclusive setup of pinned fields.
            unsafe {
                (*me).gate2_phase = phase;
                (*me).gate2_trunk = trunk;
                (*me).gate2_n = n;
                (*me).gate2_nphases = USize(nphases);
                (*me).gate2_ncores = ncores;
            }
            let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
            while c < ncores.0 {
                // SAFETY: pinned, stable address; the ctx outlives every worker
                // (Drop joins via await_exit before teardown).
                unsafe {
                    (*me).worker_ctxs[c] = WorkerCtx {
                        sched:   me as *const (),
                        core_id: c,
                    };
                }
                let cp = SendCtxPtr(unsafe { &(*me).worker_ctxs[c] as *const WorkerCtx });
                spawn_one_pointer(pool, move || {
                    let cp = cp; // capture the Send wrapper whole, not the raw field
                    super::worker::worker_main::<
                        Cfg,
                        WuVals,
                        Vals,
                        CS,
                        D,
                        Stores,
                        Clk,
                        Witnesses,
                        GW,
                    >(cp.0);
                });
                c += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            // SAFETY: setup complete; mark spawned.
            unsafe {
                (*me).spawned = Bool::TRUE;
            }
        }

        // E8 adapt: sample the frame start on the main thread; the cold-start
        // state is the EMA seed flag. SAFETY: every worker is parked between
        // frames; exclusive field reads.
        let frame_start = unsafe { (*me).clock.now_ns() };
        let ema_seed = Bool(unsafe { (*me).first_frame.load(Ordering::Relaxed) });
        // Frame start: zero accumulators (every worker is parked).
        // SAFETY: between frames, no worker is dereferencing the bindings.
        unsafe {
            (*me).bindings.reset_accumulators();
        }
        // E4 slice 1: advance the virtual epoch once per frame, before the
        // publish, while every worker is parked. Workers read it after the
        // publish under the frame happens-before; a stale fire from last frame no
        // longer matches this frame's epoch (epoch-based reset).
        // SAFETY: every worker is parked between frames; exclusive field write.
        unsafe {
            (*me).virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-frame epoch successor; tracked: #121
            (*me)
                .meta_block
                .metrics
                .pass_count
                .set(USize((*me).meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-frame meta pass_count; tracked: #121
        }
        // E4 parity, unit-outer path: the main thread is the designated core for
        // the meta bands, with the frame publish/await pair as the two ordering
        // barriers. The leading bands (plan, skipped on a clean frame, then the
        // remaining pre-consumer bands) dispatch here, before the publish, so
        // every worker's consumer slice work happens-after them; the trailing
        // bands dispatch after the await plus merge below. `core = 0, ncores =
        // 1` makes this thread own every trunk in the dispatched phases. The
        // band ranges are const and empty for a no-meta carrier.
        let unit_outer = unsafe { (*me).carrier_unit_outer() }.0;
        let pre_phases = pre_consumer_phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) reason: leading-band loop bound; tracked: #121
        if unit_outer && pre_phases > 0 {
            let start = if unsafe { (*me).first_frame.load(Ordering::Relaxed) } {
                0 // lint:allow(no-bare-numeric) reason: plan-dirty frame runs the plan band; tracked: #121
            } else {
                plan_phase_count::<
                    WuVals,
                    Stores,
                    GW,
                    <D as PlanDims>::Units,
                    <D as PlanDims>::Stores,
                    <D as PlanDims>::AdjRow,
                >()
                .0 // lint:allow(no-bare-numeric) reason: clean frame skips the plan band; tracked: #121
            };
            let all = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY.bitnot();
            let epoch = USize(unsafe { (*me).virtual_epoch.load(Ordering::Relaxed) });
            let mut p = start;
            while p < pre_phases {
                let mut rank = <USize as Identity<Additive>>::IDENTITY;
                // SAFETY: every worker is parked (pre-publish); exclusive frame
                // access to the bindings and the meta block.
                unsafe {
                    (*me).wu_values.dispatch_core(
                        &(*me).wu_values,
                        USize(p),
                        <USize as Identity<Additive>>::IDENTITY,
                        USize(1), // lint:allow(no-bare-numeric) reason: designated thread owns every trunk; tracked: #121
                        &mut rank,
                        &(*me).bindings,
                        &(*me).meta_block,
                        MorselRange::new(
                            <USize as Identity<Additive>>::IDENTITY,
                            <USize as Identity<Additive>>::IDENTITY,
                        ),
                        all,
                        epoch,
                    );
                }
                p += 1; // lint:allow(no-bare-numeric) reason: phase step; tracked: #121
            }
        }
        let ncores = unsafe { (*me).gate2_ncores };
        // One publish/await per frame. Workers run every waist-bounded phase hot
        // and cross each interior waist via the worker-side sense-reversing
        // `waist_barrier`; the main thread no longer round-trips per phase. It
        // publishes the frame once and waits for every worker to finish all
        // phases (the per-frame waist barrier is now worker-side, not here).
        // SAFETY: `pool` is a pinned field at a stable address; the frame
        // helpers only touch its atomics.
        let pool_frame = unsafe { &(*me).pool };
        frame_publish(pool_frame);
        frame_await_done(pool_frame, ncores);
        // Deviation 9: for the unit-outer accumulator carrier, each worker
        // appended into its own per-core region of the reserved buffer and
        // published its per-accumulator live counts. Forward-compact those
        // regions into each accumulator's `[0, sum)` prefix and set the binding
        // live length, so downstream readers see the same contiguous prefix
        // single-core `run()` would produce. The `frame_await_done` Acquire
        // paired with the workers' `frame_done_arrive` Release publishes the live
        // counts; load them Relaxed under that happens-before.
        // SAFETY: all workers re-parked; exclusive access to the bindings + array.
        if unsafe { (*me).carrier_unit_outer() }.0 {
            let total0 = unsafe { (*me).record_count.0 }; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
            let ncores0 = ncores.0.max(1); // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
            let per = total0.div_ceil(ncores0); // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
            let mut live = [<USize as Identity<Additive>>::IDENTITY;
                crate::thread::class::MAX_CORES * GATE2_MAX_ACCUMS];
            let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
            while c < ncores0 {
                let mut a = 0; // lint:allow(no-bare-numeric) reason: accum index; tracked: #121
                while a < GATE2_MAX_ACCUMS {
                    let slot = c * GATE2_MAX_ACCUMS + a; // lint:allow(no-bare-numeric) reason: flat (core,accum) index; tracked: #121
                    live[slot] =
                        USize(unsafe { (*me).gate2_accum_live[slot].load(Ordering::Relaxed) }); // lint:allow(no-bare-numeric) reason: load published live count; tracked: #121
                    a += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
                }
                c += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            let mut accum_idx = <USize as Identity<Additive>>::IDENTITY;
            unsafe {
                (*me).bindings.merge_accums(
                    USize(per),
                    ncores,
                    USize(total0),
                    &live,
                    USize(GATE2_MAX_ACCUMS),
                    &mut accum_idx,
                );
            }
        }
        // E4 parity, unit-outer path: the trailing meta bands (the schedule-end
        // epilogue) dispatch on the main thread after the await plus merge, so
        // they happen-after all consumer work and an epilogue hook's appends
        // land after the merged consumer data (single-core buffer order).
        if unit_outer {
            let cend = consumer_phase_end::<
                WuVals,
                Stores,
                GW,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
            >()
            .0; // lint:allow(no-bare-numeric) reason: trailing-band loop start; tracked: #121
            let nphases = unsafe { (*me).gate2_nphases.0 }; // lint:allow(no-bare-numeric) reason: trailing-band loop bound; tracked: #121
            let all = <<D as PlanDims>::AdjRow as Identity<Additive>>::IDENTITY.bitnot();
            let epoch = USize(unsafe { (*me).virtual_epoch.load(Ordering::Relaxed) });
            let mut p = cend;
            while p < nphases {
                let mut rank = <USize as Identity<Additive>>::IDENTITY;
                // SAFETY: every worker re-parked (post-await); exclusive frame
                // access to the bindings and the meta block.
                unsafe {
                    (*me).wu_values.dispatch_core(
                        &(*me).wu_values,
                        USize(p),
                        <USize as Identity<Additive>>::IDENTITY,
                        USize(1), // lint:allow(no-bare-numeric) reason: designated thread owns every trunk; tracked: #121
                        &mut rank,
                        &(*me).bindings,
                        &(*me).meta_block,
                        MorselRange::new(
                            <USize as Identity<Additive>>::IDENTITY,
                            <USize as Identity<Additive>>::IDENTITY,
                        ),
                        all,
                        epoch,
                    );
                }
                p += 1; // lint:allow(no-bare-numeric) reason: phase step; tracked: #121
            }
        }
        // Capture the change_class signal before the seed is consumed: a
        // non-empty store_dirty means an input change was seen this frame. The
        // increment runs here on the main thread after every worker re-parks, so
        // it shares the single-core fold's discipline. A consumer's own append on
        // a worker thread that over-runs an accumulator panics there (the append
        // path's capacity assert); a worker panic can stall the join rather than
        // abort cleanly, which is the accepted failure mode for that contract
        // violation (the over-capacity should_panic test runs single-core).
        // SAFETY: all phases done, every worker re-parked.
        let stores_changed = unsafe { !(*me).store_dirty.get().is_empty().0 };
        // The frame consumed the change seed; clear it and leave cold-start.
        // SAFETY: all phases done, every worker re-parked.
        unsafe {
            (*me).store_dirty.set(AccessMask::empty());
            (*me).first_frame.store(false, Ordering::Relaxed);
        }
        // E8 adapt: fold this frame's duration into the pass-duration EMA on
        // the main thread, after the await plus merge plus trailing bands.
        // SAFETY: every worker re-parked; between-frames write, same
        // discipline as virtual_epoch and pass_count.
        unsafe {
            let m = &(*me).meta_block.metrics;
            m.ema_pass_duration_ns.set(fold_ema(
                m.ema_pass_duration_ns.get(),
                (*me).clock.now_ns() - frame_start,
                ema_seed,
            ));
            m.last_record_count.set((*me).record_count);
            if stores_changed {
                m.change_seen_count
                    .set(USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
            }
            // E8 adapt, core-idle axis: reduce this frame's per-core barrier
            // idle (filled by the waist barrier follower parks) to the worst
            // core, then zero the accumulators for the next frame. Worst-core,
            // not sum, because the adapt trigger is "is some core starved".
            // Bounded by the slot count, which the clamped worker count
            // never exceeds.
            let acc = &(*me).pool.idle_accumulator;
            let mut worst = 0u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos max reduction; tracked: #121
            let mut c = 0; // lint:allow(no-bare-numeric) reason: slot index; tracked: #121
            while c < acc.len() {
                let v = acc[c].swap(0, Ordering::AcqRel); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: read-and-reset accumulator; tracked: #121
                if v > worst {
                    worst = v;
                }
                c += 1; // lint:allow(no-bare-numeric) reason: slot index step; tracked: #121
            }
            m.idle_ns
                .set(hilavitkutin_api::platform::Nanos::from_raw(worst));
        }
        Cfg::Out::default()
    }

    /// Whether the carrier is unit-outer (accumulator-bearing): any fiber whose
    /// `morsel_local` bit is false. Mirrors the decision `run` makes. An
    /// accumulator fiber stays unit-outer (each unit completes its full record
    /// range), so the threaded path routes the whole carrier through the per-core
    /// bindings rebase (deviation 9) rather than the morsel-local phase walk.
    ///
    /// The shared per-(core,phase) primitive: the single-threaded `run_parallel`
    /// sweep and the threaded worker mainloop (`scheduler::worker`) both read it,
    /// hence `pub(super)` rather than private.
    pub(super) fn carrier_unit_outer(&self) -> Bool {
        let descriptors = self.fiber_dispatch.as_ref();
        let fcount = self.fiber_dispatch_count.0.min(descriptors.len()); // lint:allow(no-bare-numeric) reason: fiber descriptor count; tracked: #121
        let mut unit_outer = false; // lint:allow(no-bare-numeric) reason: local accumulator flag; tracked: #121
        let mut fi = 0; // lint:allow(no-bare-numeric) reason: fiber index; tracked: #121
        while fi < fcount {
            if !descriptors[fi].morsel_local.0 {
                unit_outer = true; // lint:allow(no-bare-numeric) reason: local flag set; tracked: #121
            }
            fi += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        Bool(unit_outer)
    }
}
