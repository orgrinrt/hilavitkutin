//! `Scheduler::run_fused` (the within-fiber linear-fusion entry), the E8
//! adapt tuning decision, and the `#[doc(hidden)]` white-box test accessors.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). No behaviour change. `super::Scheduler`'s fields stay
//! private: this module is a descendant of `scheduler`, where `Scheduler` is
//! defined directly, so it reaches them without any widening.
//! `select_adapt_config` is `pub(super)` because `scheduler::run` (a sibling)
//! calls it at the end of `run()`.

use core::sync::atomic::Ordering;

use arvo::Bool;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::platform::Nanos;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};

use super::{PlanHandle, Scheduler};
use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::fusion::{ChainWu, FuseCarrier};
use crate::dispatch::morsel::MorselRange;
use crate::meta::fold_ema;
use crate::plan::{AccessMask, PlanDims};
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
    /// Dispatch the retained carrier fused: fold its `RecordOp` work units into
    /// one `ChainWu` and walk that, keeping the chain's intermediate columns
    /// register-resident.
    ///
    /// This is the within-fiber linear fusion entry (the canonical spec's
    /// deep-single-fiber rust-pipe). It applies when the retained carrier is a
    /// linear read-after-write chain of opt-in `RecordOp` maps: `FuseCarrier`
    /// folds the carrier into the matching `OpChain` at the type level, and the
    /// fused `ChainWu` reads the chain's input column, runs the maps with every
    /// intermediate in a register, and writes only the chain's output column.
    /// Dead-store elimination under fat LTO removes the intermediate-column
    /// traffic, matching a hand-fused loop.
    ///
    /// The engine performs the fold: a consumer registers natural separate
    /// `RecordOp` work units plus their columns and calls `run_fused`; it never
    /// hand-authors the chain. The choice between this and the general per-WU
    /// `run` is an explicit entry rather than a transparent `run` auto-detection,
    /// which is not expressible on the toolchain (the fused projection witness
    /// would be an unconstrained specializing-impl parameter, and
    /// `min_specialization` does not permit specializing on the `FuseCarrier`
    /// bound). `W2` is the fused carrier's projection-witness list, inferred at
    /// the call site exactly as `run`'s `Witnesses` is, so `scheduler.run_fused()`
    /// needs no turbofish.
    ///
    /// A fusible chain writes no accumulator, so it dispatches morsel-outer (the
    /// runtime morsel loop wraps the whole-chain walk so the intermediates stay
    /// register-resident per morsel); a record-less frame runs the chain once
    /// over an empty morsel.
    pub fn run_fused<W2>(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
        WuVals: FuseCarrier,
        WuCons<ChainWu<<WuVals as FuseCarrier>::Chain>, WuNil>:
            RunFiber<<Vals as BindingsFor>::Bindings, W2>,
    {
        let _ = (
            &self.plan_dirty,
            &self.plan_cache,
            &self.topo_order,
            &self.topo_count,
        );
        // E8 adapt: sample the frame start; the cold-start state is the EMA
        // seed flag (the first frame stores its raw duration).
        let frame_start = self.clock.now_ns();
        let ema_seed = Bool(self.first_frame.load(Ordering::Relaxed));
        let fused = WuCons {
            head: ChainWu::new(self.wu_values.fuse()),
            tail: WuNil,
        };
        // Incremental skip for the fused chain: the chain is one unit at
        // carrier position 0, and a linear chain's only external input is
        // its root, so the chain runs iff dirty bit 0 is set (its input
        // changed, or the cold frame). A clean frame skips the whole chain,
        // leaving its output column untouched.
        let dirty = self.dirty_units();
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block
            .metrics
            .pass_count
            .set(arvo::USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = arvo::USize(self.virtual_epoch.load(Ordering::Relaxed));
        let msize = Cfg::MORSEL_SIZE.0.max(1);
        let total = self.record_count.0;
        if total == 0 {
            fused.run_gated(
                &self.bindings,
                &self.meta_block,
                MorselRange::new(
                    <arvo::USize as arvo::strategy::Identity<arvo::strategy::Additive>>::IDENTITY,
                    <arvo::USize as arvo::strategy::Identity<arvo::strategy::Additive>>::IDENTITY,
                ),
                dirty,
                <arvo::USize as arvo::strategy::Identity<arvo::strategy::Additive>>::IDENTITY,
                epoch,
            );
        } else {
            let mut start = 0;
            while start < total {
                let len = msize.min(total - start);
                fused.run_gated(
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(arvo::USize(start), arvo::USize(len)),
                    dirty,
                    <arvo::USize as arvo::strategy::Identity<arvo::strategy::Additive>>::IDENTITY,
                    epoch,
                );
                start += len;
            }
        }
        // Capture the change_class signal before the seed is consumed.
        let stores_changed = !self.store_dirty.get().is_empty().0;
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
                .set(arvo::USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
        }
        Cfg::Out::default()
    }

    /// Borrow the resource bindings. Hidden test accessor: lets in-crate
    /// and integration tests walk the bindings nodes to confirm the
    /// moved-in resource values. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __bindings(&self) -> &<Vals as BindingsFor>::Bindings {
        &self.bindings
    }

    /// Set `store_dirty` to a non-empty mask. Hidden test accessor: the only
    /// public trigger for `store_dirty` is `replace_resource<T: PlanAffecting>`,
    /// and `PlanAffecting` is sealed, so a white-box test for the change_class
    /// signal sets the dirty mask directly. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __mark_store_dirty(&self) {
        self.store_dirty
            .set(AccessMask::empty().set(arvo::USize(0))); // lint:allow(no-bare-numeric) reason: store index zero; tracked: #121
    }

    /// Read the core-idle adapt metric (`SchedulerMetrics::idle_ns`) after a
    /// frame. Hidden test accessor: an accumulator-bearing carrier takes the
    /// unit-outer no-barrier path (zero idle by design), so a barrier-driven
    /// signal cannot be read back through an accumulator append. Not part of the
    /// supported surface; consumers read it through an `OnMeta<ScheduleEnd>` hook.
    #[doc(hidden)]
    pub fn __idle_ns(&self) -> Nanos {
        self.meta_block.metrics.idle_ns.get()
    }

    /// Read the per-fiber morsel window size on the dispatch descriptor at
    /// dispatch-order index `i`. Hidden test accessor: the descriptor's
    /// `morsel_size` is populated from `plan.morsel_windows` and consumed by
    /// the fiber-outer dispatch loop (A2b); a white-box test asserts the field
    /// carries the plan's per-fiber value.
    #[doc(hidden)]
    pub fn __fiber_morsel_size(&self, i: arvo::USize) -> arvo::USize {
        self.fiber_dispatch.as_ref()[i.0].morsel_size
    }

    /// Read the engine-internal per-phase duration EMA for phase `p`. Hidden
    /// test accessor: per-phase EMA is engine-internal (it feeds the eventual
    /// `select_adapt_config`), with no `OnMeta` consumer read, so a white-box
    /// test asserts the recorded per-phase durations directly.
    #[doc(hidden)]
    pub fn __phase_ema(&self, p: arvo::USize) -> Nanos {
        self.phase_ema[p.0].get()
    }

    /// E8 adapt tuning decision (domain-22, R5): scan the per-phase EMA and set
    /// the phase-imbalance reconfigure trigger when one active phase dominates the
    /// least active phase. Active phases are the slots with a nonzero EMA; the
    /// trigger fires only with at least two active phases and `max > FACTOR * min`.
    /// Pure read of engine-internal state; the actuation that acts on the trigger
    /// is a follow-up. `BALANCE_FACTOR` is a tunable default (consumer-tunable per
    /// the caps-are-defaults discipline).
    ///
    /// `pub(super)`: `scheduler::run` (a sibling) calls it at the end of `run()`.
    pub(super) fn select_adapt_config(&self) {
        const BALANCE_FACTOR: u64 = 2; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: imbalance ratio default; tracked: #121
        let mut active = 0usize; // lint:allow(no-bare-numeric) reason: active-phase counter; tracked: #121
        let mut max = 0u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw-nanos max over active phases; tracked: #121
        let mut min = u64::MAX; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw-nanos min over active phases; tracked: #121
        let mut p = 0usize; // lint:allow(no-bare-numeric) reason: phase-scan index; tracked: #121
        while p < self.phase_ema.len() {
            let e = self.phase_ema[p].get().to_raw();
            if e > 0 {
                active += 1; // lint:allow(no-bare-numeric) reason: count active phases; tracked: #121
                if e > max {
                    max = e;
                }
                if e < min {
                    min = e;
                }
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase-scan step; tracked: #121
        }
        let imbalanced = active >= 2 && max > min.saturating_mul(BALANCE_FACTOR); // lint:allow(no-bare-numeric) reason: imbalance predicate; tracked: #121
        self.adapt_reconfigure.set(Bool(imbalanced));
    }

    /// Read the phase-imbalance reconfigure trigger. Hidden test accessor:
    /// engine-internal, no consumer read yet (actuation is a follow-up).
    #[doc(hidden)]
    pub fn __adapt_reconfigure(&self) -> Bool {
        self.adapt_reconfigure.get()
    }

    /// Set a per-phase EMA slot directly. Hidden test accessor: lets a test drive
    /// `select_adapt_config` with a chosen balance state without depending on
    /// wall-clock timing.
    #[doc(hidden)]
    pub fn __set_phase_ema(&self, p: arvo::USize, ns: Nanos) {
        self.phase_ema[p.0].set(ns);
    }

    /// Run the adapt tuning decision. Hidden test accessor for the decision logic.
    #[doc(hidden)]
    pub fn __select_adapt_config(&self) {
        self.select_adapt_config();
    }

    /// Borrow the backing store. Hidden test accessor mirroring
    /// `__bindings`: lets tests inspect reserved columns. The field is also
    /// held for its `Drop`, which frees every reserved resource column.
    /// Not part of the supported surface.
    #[doc(hidden)]
    pub fn __storage(&self) -> &CS {
        &self.storage
    }

    /// The store-backed plan locator. Hidden accessor: the dispatch consumer
    /// (and tests) read the plan columns out of `storage` through this handle.
    /// Not part of the supported surface until the dispatch reader lands.
    #[doc(hidden)]
    pub fn __plan_handle(&self) -> PlanHandle {
        self.plan_handle
    }
}
