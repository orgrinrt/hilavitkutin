//! `Scheduler::run`: the flat schedule-mega dispatch entry.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). No behaviour change. `super::Scheduler`'s fields stay
//! private: this module is a descendant of `scheduler`, where `Scheduler` is
//! defined directly, so it reaches them without any widening. Calls
//! `self.dirty_units()` (`pub(super)` in `scheduler::dirty`) and
//! `self.select_adapt_config()` (`pub(super)` in `scheduler::run_fused`),
//! both siblings of this module.

use core::sync::atomic::Ordering;

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::{BitAccess, BitLogic, BitSequence};
use arvo_tensor::ConstCapacity;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;

use super::Scheduler;
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_dispatch::RunTrunkDispatch;
use crate::meta::fold_ema;
use crate::plan::grouping::{
    BundleMasks,
    consumer_mask,
    consumer_phase_end,
    phase_count,
    plan_phase_count,
    pre_consumer_phase_count,
};
use crate::plan::{AccessMask, PlanDims};
use crate::resource::bindings::{BindingsFor, ResetAccumulators};

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
    /// Dispatch the retained WorkUnit carrier in carrier order, windowing the
    /// record range into morsels, then return `Cfg::Out::default()`.
    ///
    /// The retained `wu_values` carrier is walked as one type-level `RunFiber`
    /// recursion in carrier (registration) order, which `build()` validated is a
    /// topological order: a consumer registers producer-before-consumer, and an
    /// anti-topological carrier is rejected at `build()`
    /// (`BuildError::NonTopologicalRegistration`). Each `RunFiber` step projects
    /// that unit's `EngineCtx` from the bindings (resources, columns, and
    /// accumulators alike) and runs `execute`; no unit dispatches through a
    /// stored function pointer, so the whole walk monomorphises into one
    /// straight-line body that devirtualises under fat LTO. This is the flat
    /// schedule-mega dispatch (spec Approach E); the per-fiber and per-phase
    /// sub-carrier nesting is a later refinement.
    ///
    /// Drive shape (A2b): phase-outer over the const grouping's phase axis,
    /// with the consumer passes fiber-outer/morsel-inner. The meta lifecycle
    /// bands dispatch once per frame over the whole range (the plan band
    /// skipped on clean frames). Within a consumer pass the per-fiber dispatch
    /// descriptors are walked in plan order: a `morsel_local` fiber windows
    /// the record range by its own plan-baked `FiberDispatch::morsel_size`
    /// (the domain-12 L1 window; `RunCfg::MORSEL_SIZE` is the fallback for a
    /// degenerate zero window), members selected by the fiber's carrier-
    /// position mask composed with the consumer mask and the incremental-skip
    /// dirty mask; an accumulator fiber dispatches once over the whole range
    /// ungated (the per-frame append path must always re-run), which is also
    /// the record-less-frame form (each fiber runs once over an empty morsel
    /// so a resource-only unit runs exactly once). The `Witnesses` parameter
    /// is the per-unit projection-index list, inferred at the call site, so
    /// `scheduler.run()` needs no turbofish.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn run<Witnesses, GW>(&mut self) -> Cfg::Out // lint:allow(no-bare-numeric) reason: const-generic dispatch entry position; tracked: #121
    where
        Cfg::Out: Default,
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
        <Vals as BindingsFor>::Bindings: ResetAccumulators,
    {
        // E8 adapt: sample the frame start; the cold-start state is the EMA
        // seed flag (the first frame stores its raw duration).
        let frame_start = self.clock.now_ns();
        let ema_seed = Bool(self.first_frame.load(Ordering::Relaxed));
        // Schedule-once-reuse: zero every accumulator live-length at frame
        // start so this frame appends into a fresh buffer rather than
        // continuing from the prior frame's live offset. No-op for an
        // accumulator-free carrier.
        self.bindings.reset_accumulators();
        // E4 slice 1: advance the virtual epoch once per pass. A fire this pass
        // stamps cells with the new value; last pass's stamps no longer match, so
        // a stale fire gates its `On<V>` consumer shut (epoch-based reset).
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block
            .metrics
            .pass_count
            .set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // E4 slice 2 (self-hosting meta pipeline): a plan-dirty frame runs the
        // leading plan band (`OnMeta<PlanStage>` units recompute the plan); a clean
        // frame skips it. The first frame is always plan-dirty (the plan is computed
        // once); the `replace_resource`-driven `plan_dirty` bit-array re-dirty is
        // the domain-22 recompute, sequenced with the adapt subsystem (slice 3). For
        // a carrier with no plan-stage meta unit, `plan_phase_count` is zero, so this
        // is a no-op and dispatch is byte-identical to before.
        let plan_dirty = Bool(self.first_frame.load(Ordering::Relaxed));
        // `plan_dirty` array / `plan_cache` are the domain-22 plan-recompute seed
        // and cache (set by `replace_resource`); rebuilding the plan from them is the
        // adapt subsystem's job, sequenced later. The domain-16 incremental-skip seed
        // is `store_dirty`, consumed here.
        let _ = (&self.plan_dirty, &self.plan_cache);
        // Per-frame incremental skip: the dirty-unit mask names which units
        // this frame must run (their input cone changed); the rest are
        // skipped, producing identical output to running them.
        let dirty = self.dirty_units();
        let msize = Cfg::MORSEL_SIZE.0.max(1);
        let total = self.record_count.0;
        // A2b fiber-outer dispatch. Phase passes run in const-grouping order
        // (barrier semantics unchanged); the meta lifecycle bands dispatch once
        // per frame around the consumer work (run_parallel's designated-thread
        // band shape); within a consumer pass the fiber descriptors are walked
        // in plan order, each morsel_local fiber windowing the record range by
        // its own plan-baked L1 window. A fiber whose units sit in another
        // phase contributes nothing in this pass (the const phase gate on trunk
        // roots filters it), so no runtime fiber-to-phase map is needed.
        let nphases = phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: phase-loop bound; tracked: #121
        let pre = pre_consumer_phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: leading-band bound; tracked: #121
        let cend = consumer_phase_end::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: trailing-band start; tracked: #121
        let cmask = consumer_mask::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >();
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        // A plan-dirty frame runs the leading plan band; a clean frame skips it
        // (E4 kernel band skipping, unchanged from dispatch_trunks).
        let band_start = if plan_dirty.0 {
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
            .0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: clean frame skips the plan band; tracked: #121
        };
        let descriptors = self.fiber_dispatch.as_ref();
        let fcount = self.fiber_dispatch_count.0.min(descriptors.len());
        let order = self.topo_order.as_ref();
        // A built carrier with live units always has fiber descriptors (the
        // dispatch-order derivation asserts every unit lands in exactly one);
        // a zero fiber count with live units would silently dispatch nothing.
        debug_assert!(
            self.topo_count.0 == 0 || fcount > 0, // lint:allow(no-bare-numeric) reason: emptiness guard; tracked: #121
            "run: live units but no fiber descriptors; the plan's fiber partition never reached the scheduler"
        );
        let mut p = band_start; // lint:allow(no-bare-numeric) reason: phase-pass index; tracked: #121
        while p < nphases {
            let t0 = self.clock.now_ns().to_raw(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos for the duration delta; tracked: #121
            if p < pre || p >= cend {
                // Meta lifecycle band pass: whole range, every member, once per
                // frame. Band members are the meta units of this pass's rank;
                // the all-ones mask never skips them (lifecycle hooks are not
                // input-cone-gated).
                self.wu_values.dispatch(
                    &self.wu_values,
                    USize(p),
                    &self.meta_block,
                    &self.bindings,
                    MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(total)),
                    all,
                    epoch,
                );
            } else {
                // Consumer pass: fiber-outer/morsel-inner. Each descriptor
                // completes its full window sequence before the next fiber, so
                // a producer fiber's whole range lands before its consumer
                // fiber's first read.
                let mut fi = 0; // lint:allow(no-bare-numeric) reason: descriptor cursor; tracked: #121
                while fi < fcount {
                    let desc = descriptors[fi];
                    // The fiber's member mask over carrier positions: build
                    // validated registration order is topological, so
                    // topo_order values are carrier positions and the mask
                    // composes directly with the dirty mask.
                    // FIXME: rebuilt per (phase, fiber); plan-bake the member
                    // masks (and a fiber-to-const-phase map to skip out-of-phase
                    // fibers' window walks) at build() per schedule-once-reuse;
                    // tracked #340.
                    let mut fmask = <D as PlanDims>::AdjRow::default();
                    let kend = (desc.start.0 + desc.len.0).min(order.len());
                    let mut k = desc.start.0;
                    while k < kend {
                        fmask = fmask.with_bit_set(order[k]);
                        k += 1; // lint:allow(no-bare-numeric) reason: unit-slice cursor; tracked: #121
                    }
                    // Meta units never ride the fiber walk (their runtime-plan
                    // phase is unconstrained; the bands above own them), and
                    // incremental skip applies only to the windowed RAW path:
                    // an accumulator fiber is reset and re-appended each frame,
                    // so skipping it would leave it reset-but-empty. A
                    // record-less frame runs each fiber once over the empty
                    // range so resource-only units run exactly once.
                    let windowed = desc.morsel_local.0 && total != 0; // lint:allow(no-bare-numeric) reason: record-less frame test; tracked: #121
                    let mut members = fmask.bitand(cmask);
                    if windowed {
                        members = members.bitand(dirty);
                    }
                    if !members.is_zero().0 {
                        if windowed {
                            // The per-fiber L1 window (plan step 9, baked into
                            // the descriptor at build); zero falls back to the
                            // uniform Cfg default.
                            let w =
                                if desc.morsel_size.0 != 0 { desc.morsel_size.0 } else { msize }; // lint:allow(no-bare-numeric) reason: window fallback test; tracked: #121
                            let mut s = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel cursor; tracked: #121
                            while s < total {
                                let len = w.min(total - s); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: clamped window length; tracked: #121
                                self.wu_values.dispatch(
                                    &self.wu_values,
                                    USize(p),
                                    &self.meta_block,
                                    &self.bindings,
                                    MorselRange::new(USize(s), USize(len)),
                                    members,
                                    epoch,
                                );
                                s += len; // lint:allow(no-bare-numeric) reason: advance cursor; tracked: #121
                            }
                        } else {
                            self.wu_values.dispatch(
                                &self.wu_values,
                                USize(p),
                                &self.meta_block,
                                &self.bindings,
                                MorselRange::new(
                                    <USize as Identity<Additive>>::IDENTITY,
                                    USize(total),
                                ),
                                members,
                                epoch,
                            );
                        }
                    }
                    fi += 1; // lint:allow(no-bare-numeric) reason: descriptor step; tracked: #121
                }
            }
            // E8 adapt, per-phase timing: one start/stop pair per phase pass
            // per frame; `run` folds the per-frame total into `phase_ema` below.
            let dur = self.clock.now_ns().to_raw().saturating_sub(t0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: monotonic phase-slice delta; tracked: #121
            if p < self.phase_accum.len() {
                let slot = &self.phase_accum[p];
                slot.set(hilavitkutin_api::platform::Nanos::from_raw(
                    slot.get().to_raw().saturating_add(dur),
                )); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame phase-duration sum; tracked: #121
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase-pass step; tracked: #121
        }
        // Capture the change_class signal before the seed is consumed: a
        // non-empty store_dirty means an input change was seen this frame.
        let stores_changed = !self.store_dirty.get().is_empty().0;
        // The frame consumed the change seed; clear it and leave cold-start.
        self.store_dirty.set(AccessMask::empty());
        self.first_frame.store(false, Ordering::Relaxed);
        // E8 adapt: fold this frame's duration into the pass-duration EMA.
        // Between frames, so the write needs no synchronisation.
        let m = &self.meta_block.metrics;
        m.ema_pass_duration_ns.set(fold_ema(
            m.ema_pass_duration_ns.get(),
            self.clock.now_ns() - frame_start,
            ema_seed,
        ));
        m.last_record_count.set(self.record_count);
        if stores_changed {
            m.change_seen_count
                .set(USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
        }
        // E8 adapt, per-phase EMA: fold each phase's per-frame total (summed
        // across this frame's morsels by `dispatch_trunks`) into its EMA with the
        // same seed, then zero the accumulator for the next frame. Per-frame, so
        // a multi-morsel frame folds once, not once per morsel.
        let nph = phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: phase-fold bound; tracked: #121
        let mut pe = 0; // lint:allow(no-bare-numeric) reason: phase-fold index; tracked: #121
        while pe < nph && pe < self.phase_ema.len() {
            let acc = self.phase_accum[pe].get();
            self.phase_ema[pe].set(fold_ema(self.phase_ema[pe].get(), acc, ema_seed));
            self.phase_accum[pe].set(hilavitkutin_api::platform::Nanos::from_raw(0)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame accumulator reset; tracked: #121
            pe += 1; // lint:allow(no-bare-numeric) reason: phase-fold step; tracked: #121
        }
        // E8 adapt tuning: read the just-folded per-phase EMA and set the
        // reconfigure trigger when the frame's phases are imbalanced.
        self.select_adapt_config();
        Cfg::Out::default()
    }
}
