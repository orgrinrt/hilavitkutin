//! Per-trunk dispatch entries: `run_one_trunk`, `run_one_trunk_windowed`,
//! `run_all_trunks`, and their shared `dispatch_trunks` phase-loop core.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). No behaviour change. `super::Scheduler`'s fields stay
//! private: this module is a descendant of `scheduler`, where `Scheduler` is
//! defined directly, so it reaches them without any widening.

use core::sync::atomic::Ordering;

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_bitmask::{BitAccess, BitLogic};
use arvo_tensor::ConstCapacity;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;

use super::Scheduler;
use crate::dispatch::engine_ctx::Here;
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::trunk_dispatch::RunTrunkDispatch;
use crate::dispatch::trunk_gate::RunGatedTrunk;
use crate::plan::PlanDims;
use crate::plan::grouping::{BundleMasks, phase_count, plan_phase_count};
use crate::resource::bindings::BindingsFor;

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
    /// Dispatch only the units of one phase and trunk (GATE-2, round 2a).
    ///
    /// Walks the carrier through `RunGatedTrunk`, running just the members of
    /// `(PHASE, TRUNK)` over the whole record range; every other carrier
    /// position folds away, so this monomorphisation is that trunk's member-only
    /// program. The per-trunk entry the round-2b dispatcher loops across every
    /// `(phase, trunk)` in phase order, and the unit each core runs at G2-N.
    /// `Witnesses` is the per-unit projection list and `GW` the grouping witness
    /// list, both inferred at the call site.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn run_one_trunk<Witnesses, GW, const TRUNK: usize>(&mut self) // lint:allow(no-bare-numeric) reason: const-generic trunk selector; tracked: #121
    where
        WuVals: RunGatedTrunk<
                WuVals,
                <Vals as BindingsFor>::Bindings,
                Witnesses,
                GW,
                Stores,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
                TRUNK,
                Here, // the walk starts at carrier position zero (Peano Here)
            >,
    {
        let total = self.record_count.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel length; tracked: #121
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block
            .metrics
            .pass_count
            .set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // No-skip entry: every member of the trunk runs (all-ones dirty mask).
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        self.wu_values.run_trunk(
            &self.bindings,
            &self.meta_block,
            MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(total)),
            all,
            epoch,
        );
    }

    /// Dispatch one trunk's monomorphised program fiber-outer/morsel-inner: walk
    /// `[0, record_count)` in windows of `window` records, one whole-fiber
    /// `run_trunk` call per window (all-ones dirty, no-skip). For a single-fiber
    /// trunk this is per-fiber morsel windowing (spec domain 12, "multiple morsels
    /// per fiber"): the fiber's whole unit sequence runs over one morsel before the
    /// next, keeping its co-located columns L1-hot for the window's duration.
    ///
    /// The per-record body is the same `run_trunk` projection `run_one_trunk`
    /// dispatches whole-range, so the const-DCE and devirtualisation are
    /// unchanged: `MorselRange` is a runtime argument that no compile-time DCE site
    /// (`IsRoot`/`PhaseAt`/`Member`/`GateWith`) reads, so distinct per-window ranges
    /// cannot perturb the monomorphised dispatch. A zero `window` floors to 1.
    /// At `record_count == 0` the loop runs zero windows (vs `run_one_trunk`'s one
    /// call over `[0, 0)`); both are no-ops because the per-record body never fires
    /// at zero records, so the behaviours coincide.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn run_one_trunk_windowed<Witnesses, GW, const TRUNK: usize>(&mut self, window: USize) // lint:allow(no-bare-numeric) reason: const-generic trunk selector; tracked: #121
    where
        WuVals: RunGatedTrunk<
                WuVals,
                <Vals as BindingsFor>::Bindings,
                Witnesses,
                GW,
                Stores,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
                TRUNK,
                Here,
            >,
    {
        let total = self.record_count.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel length; tracked: #121
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block
            .metrics
            .pass_count
            .set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        let w = window.0.max(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: window floor; tracked: #121
        let mut start = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel cursor; tracked: #121
        while start < total {
            let len = w.min(total - start); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: clamped window length; tracked: #121
            self.wu_values.run_trunk(
                &self.bindings,
                &self.meta_block,
                MorselRange::new(USize(start), USize(len)),
                all,
                epoch,
            );
            start += len; // lint:allow(no-bare-numeric) reason: advance cursor; tracked: #121
        }
    }

    /// Dispatch every trunk in phase order over `morsel`, single-core (round 2b).
    ///
    /// The outer driver: for each phase pass `0..phase_count` it walks the
    /// carrier through `trunk_dispatch::RunTrunkDispatch`, dispatching each
    /// trunk-root's per-trunk mono whose compile-time phase equals the pass. Each
    /// trunk's members run in carrier (RCM-reordered topological) order; phases
    /// run in waist order; so the result is output-equivalent to the flat
    /// `RunFiber` walk, while every trunk is an independently monomorphised
    /// program (the unit a core runs at G2-N). Whole-range, no-skip entry (every
    /// member runs); the morsel-windowed, dirty-skipping form drives the
    /// incremental `run` path. `Witnesses` (per-unit projection list) and `GW`
    /// (grouping witness list) infer at the call site.
    pub fn run_all_trunks<Witnesses, GW>(&mut self)
    where
        WuVals: RunTrunkDispatch<
                WuVals,
                <Vals as BindingsFor>::Bindings,
                Witnesses,
                GW,
                Stores,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
                0, // the walk starts at carrier position zero // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
            >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity<Additive>,
    {
        let total = self.record_count.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel length; tracked: #121
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block
            .metrics
            .pass_count
            .set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // No-skip: every member runs (all-ones dirty mask).
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        // No-skip entry: run every band including the plan band (plan_dirty=true).
        self.dispatch_trunks::<Witnesses, GW, _>(
            MorselRange::new(<USize as Identity<Additive>>::IDENTITY, USize(total)),
            all,
            epoch,
            arvo::Bool::TRUE,
        );
    }

    /// Phase-loop core of the per-trunk dispatch: for each phase pass walk the
    /// carrier through `RunTrunkDispatch` over `morsel`, skipping members clear in
    /// `dirty`. `run_all_trunks` (whole-range, all-ones) and the incremental
    /// `run` path (per-morsel, real dirty) both delegate here.
    fn dispatch_trunks<Witnesses, GW, M: BitAccess>(
        &self,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
        plan_dirty: arvo::Bool,
    ) where
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
        // Phase-loop bound = the const grouping's phase count, the same axis the
        // dispatcher's per-trunk phase gate reads, so every trunk-root fires in
        // exactly one pass.
        let nphases = phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: phase-loop bound; tracked: #121
        // E4 slice 2 (self-hosting meta pipeline): the rank-outer grouping places
        // `OnMeta<PlanStage>` units in the leading plan band (phases
        // `0..plan_phase_count`). On a clean frame (not plan-dirty) the kernel skips
        // that band, so plan-stage meta units run only when the plan is recomputed;
        // the schedule-ready / pass-start / consumer / schedule-end bands always
        // dispatch. This is the kernel's lifecycle sequencing: the band order is the
        // canonical PlanStage < ScheduleReady < PassStart < consumer < ScheduleEnd.
        let start = if plan_dirty.0 {
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
            .0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: skip the leading plan band on a clean frame; tracked: #121
        };
        // E8 adapt, per-phase timing: `dispatch_trunks` runs once per morsel, so
        // ADD each phase's per-morsel duration into the per-frame accumulator;
        // `run` folds the per-frame total into `phase_ema` once at frame end (so
        // the EMA is per-frame, not per-morsel). Engine-internal; feeds the
        // eventual `select_adapt_config`.
        let mut p = start; // lint:allow(no-bare-numeric) reason: phase-pass index; tracked: #121
        while p < nphases {
            let t0 = self.clock.now_ns().to_raw(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos for the duration delta; tracked: #121
            self.wu_values.dispatch(
                &self.wu_values,
                USize(p),
                &self.meta_block,
                &self.bindings,
                morsel,
                dirty,
                epoch,
            );
            let dur = self.clock.now_ns().to_raw().saturating_sub(t0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: monotonic phase-slice delta; tracked: #121
            if p < self.phase_accum.len() {
                let slot = &self.phase_accum[p];
                slot.set(hilavitkutin_api::platform::Nanos::from_raw(
                    slot.get().to_raw().saturating_add(dur),
                )); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame phase-duration sum; tracked: #121
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase-pass step; tracked: #121
        }
    }
}
