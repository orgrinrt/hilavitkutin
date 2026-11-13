//! Resource replacement and the per-frame incremental-skip dirty mask.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). `super::Scheduler`'s fields stay private: this module is
//! a descendant of `scheduler`, where `Scheduler` is defined directly, so it
//! reaches them without any widening. `dirty_units` is `pub(super)` because
//! `scheduler::run` and `scheduler::run_fused` (siblings) call it.

use core::sync::atomic::Ordering;

use arvo_bitmask::{BitAccess, BitLogic, BitSequence};
use hilavitkutin_api::run_cfg::{PlanAffecting, RunCfg};
use hilavitkutin_api::store::{Replaceable, Resource};
use hilavitkutin_api::store_values::StoreValues;
use hilavitkutin_api::{ColumnStorage, ColumnValue};

use super::Scheduler;
use crate::dispatch::engine_ctx::Selector;
use crate::plan::PlanDims;
use crate::plan::project::{Locate, WitnessIndex};
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
    // Both swap methods below follow the swap specification, clauses S1, S2
    // and S4, which is implemented and benched and not ratified. They ship
    // under the maintainer's provisional stamp of 2026-11-13 ("Land as
    // provisional"), so they may be called and a ratification can still
    // change their shape. The two places this implementation differs from the
    // specification are marked where they are, FIXME(0ba7325f0216) below and
    // FIXME(b8637199eb8a) in `run_fused.rs`.
    /// Replace a plan-affecting resource and ask for a plan band.
    ///
    /// `T: PlanAffecting` routes the call onto the recompute path: the value
    /// is installed, its store is marked dirty, and its plan-dirty bit is set
    /// so the next frame opens on the leading plan band. Consumers that need
    /// a cheap value swap on a non-plan-affecting resource use
    /// `replace_value`. App-only: called between `build` and `run`, or
    /// between two frames.
    pub fn replace_resource<T: PlanAffecting + ColumnValue, Index, PtrIndex>(&mut self, new: T)
    where
        Stores: Locate<Resource<T>, Index>,
        Index: WitnessIndex,
        <Vals as BindingsFor>::Bindings: Selector<T, PtrIndex>,
    {
        // The identical witnessed blob install as `replace_value`: one
        // whole-value write through the binding pointer the drain wrote
        // through. Store values are `Copy` (drain contract), so the
        // overwrite drops nothing.
        let ptr = Selector::<T, PtrIndex>::get(&self.bindings);
        // SAFETY: the binding pointer names the one-record column reserved
        // and initialised for `T` at build; `&mut self` means no frame is in
        // flight, so no reader aliases the slot during the write.
        // FIXME(74e2b9918149): under `run_parallel` a parked worker still
        // holds a `&Scheduler` across this `&mut self`. No worker reads while
        // parked, so it is no data race, but it breaks the aliasing model
        // until the worker-visible state moves into a provider allocation.
        unsafe {
            core::ptr::write(ptr.as_ptr(), new);
        }
        // A swapped resource is a changed input, so mark its store dirty for
        // the next frame (domain-16 incremental skip seed).
        self.mark_dirty::<Resource<T>, Index>();
        // Domain-22 plan-recompute seed: set the plan-dirty bit for `T`,
        // keyed by the same store-position witness. The next frame's band
        // gate consumes and clears the bits. Positions past the
        // `D::PlanAffecting` capacity share the last slot, which loses
        // nothing while any set bit opens the band.
        // FIXME(0ba7325f0216): the swap specification keys this bit by the
        // plan-affecting id, not by store position, and with a capacity of
        // zero there is no slot, so the bit is dropped with no signal.
        let bits = self.plan_dirty.as_ref();
        let slot = Index::INDEX.0.min(bits.len().saturating_sub(1)); // lint:allow(no-bare-numeric) reason: bounded slice index from the store-position witness; tracked: #121
        if let Some(bit) = bits.get(slot) {
            bit.store(true, Ordering::Relaxed);
        }
    }

    /// Cheap value swap for a non-plan-affecting resource.
    ///
    /// `V: Replaceable` opts the type into runtime replacement without
    /// signalling a plan recompute: the value is installed and its store is
    /// marked dirty, so the units reading it run on the next frame. The
    /// `Replaceable` marker is consumer-driven per Topic 8 axis B
    /// (replaceable but not plan-affecting is the typical case for app-level
    /// state). App-only: called between `build` and `run`, or between two
    /// frames.
    pub fn replace_value<V: Replaceable + ColumnValue, Index, PtrIndex>(&mut self, new: V)
    where
        Stores: Locate<Resource<V>, Index>,
        Index: WitnessIndex,
        <Vals as BindingsFor>::Bindings: Selector<V, PtrIndex>,
    {
        // The specified install (swap spec S1, superseding the refuted
        // member-by-member wording of `202606210600`): one whole-blob write
        // through the `Selector` witness, the same binding pointer the drain
        // wrote through and the projection reads through. No other pointer
        // derivation exists on the swap path. Collection halves gain
        // per-record element writes with the #344 collection wiring (S3).
        let ptr = Selector::<V, PtrIndex>::get(&self.bindings);
        // SAFETY: the binding pointer names the one-record column reserved
        // and initialised for `V` at build; `&mut self` means no frame is in
        // flight, so no reader aliases the slot during the write. Store
        // values are `Copy` (drain contract), so the overwrite drops nothing.
        // FIXME(74e2b9918149): the same parked-worker aliasing as in
        // `replace_resource` above applies under `run_parallel`.
        unsafe {
            core::ptr::write(ptr.as_ptr(), new);
        }
        // A swapped value is a changed input: mark its store dirty for the
        // next frame so dependents re-run (domain-16 incremental skip). No
        // plan-recompute dirty, since the value swap is not structural.
        self.mark_dirty::<Resource<V>, Index>();
    }

    /// Decide this frame's plan band and publish the decision.
    ///
    /// Swap spec S2: the band opens on the first frame or when any plan-dirty
    /// bit was set by `replace_resource` since the last frame that consumed
    /// them. Consuming clears the bits, so one swap buys one plan band. The
    /// decision is stored in `plan_band` for the worker-side band start.
    /// Called between frames, before any worker wakes, by `run` and by
    /// `run_parallel`; both reach the bits and the flag through a shared
    /// reference, since they are atomics.
    pub(super) fn consume_plan_dirty(&self) -> arvo::Bool {
        let mut swapped = false; // lint:allow(no-bare-numeric) reason: fold over the atomic bit array; tracked: #121
        for bit in self.plan_dirty.as_ref() {
            if bit.swap(false, Ordering::Relaxed) {
                swapped = true;
            }
        }
        let dirty = self.first_frame.load(Ordering::Relaxed) || swapped;
        self.plan_band.store(dirty, Ordering::Relaxed);
        arvo::Bool(dirty)
    }

    /// Mark the store named by type `T` changed for the next frame.
    ///
    /// `T` is resolved to its position in the registered `Stores` access
    /// set via the same `Locate` witness the plan projection uses, so its
    /// bit lands in the Stores-list-position space the per-unit read masks
    /// index. The next `run` / `run_fused` seeds every unit reading `T` as
    /// dirty and propagates forward, so only `T`'s transitive cone runs.
    /// `Index` infers at the call site, so `scheduler.mark_dirty::<T>()`
    /// needs no turbofish on the index. The consumer calls this for an
    /// input it mutated directly (a host-populated column or a swapped
    /// resource value tracked elsewhere); `replace_resource` and
    /// `replace_value` call it internally.
    pub fn mark_dirty<T, Index>(&mut self)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        self.store_dirty
            .set(self.store_dirty.get().set(Index::INDEX));
    }

    /// This frame's dirty-unit mask for incremental skip (domain 16,
    /// canonical Step 9).
    ///
    /// Seeds every unit whose read set intersects the per-store change
    /// seed (or every unit, on the cold first frame), then propagates the
    /// seed forward over the predecessor masks in carrier (topological)
    /// order: a unit is dirty when directly seeded or any predecessor is
    /// dirty. Positions past the live unit count carry empty masks and stay
    /// clean. The walk over the array length runs the predecessors-before-
    /// dependents single pass because carrier position equals topological
    /// order (`build` validated it).
    pub(super) fn dirty_units(&self) -> D::AdjRow {
        if self.first_frame.load(Ordering::Relaxed) {
            // Cold frame: every unit dirty, so the first frame after build
            // executes the whole carrier.
            return D::AdjRow::default().bitnot();
        }
        let reads = self.read_masks.as_ref();
        let preds = self.predecessor_masks.as_ref();
        // Bound the seed and propagate passes to the live unit count, not the
        // full unit-capacity array length: positions past `topo_count` carry
        // empty masks and stay clean, so iterating them is pure per-frame
        // overhead on the hot path. The mask arrays are indexed by carrier
        // position (0..unit_count), and `topo_count` is that live count.
        let n = self.topo_count.0.min(reads.len()).min(preds.len());
        let mut dirty = D::AdjRow::default();
        let mut p = 0;
        while p < n {
            if reads[p].overlaps(&self.store_dirty.get()).0 {
                dirty = dirty.with_bit_set(arvo::USize(p)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct carrier position; tracked: #72
            }
            p += 1;
        }
        let mut p = 0;
        while p < n {
            if !preds[p].bitand(dirty).is_zero().0 {
                dirty = dirty.with_bit_set(arvo::USize(p)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct carrier position; tracked: #72
            }
            p += 1;
        }
        dirty
    }
}
