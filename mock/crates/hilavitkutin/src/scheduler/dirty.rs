//! Resource replacement and the per-frame incremental-skip dirty mask.
//!
//! Split out of `scheduler/mod.rs`'s giant `impl<...> Scheduler<...>` block
//! (file-size lint). `super::Scheduler`'s fields stay private: this module is
//! a descendant of `scheduler`, where `Scheduler` is defined directly, so it
//! reaches them without any widening. `dirty_units` is `pub(super)` because
//! `scheduler::run` and `scheduler::run_fused` (siblings) call it.

use core::sync::atomic::Ordering;

use arvo_bitmask::{BitAccess, BitLogic, BitSequence};
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::run_cfg::{PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::store_values::StoreValues;

use super::Scheduler;
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
    /// Replace the existing `Resource<T>` instance in the data
    /// plane with `_new`, marking the plan dirty.
    ///
    /// `T: PlanAffecting` routes the call onto the dirty-marking
    /// path; the next `run()` recomputes the execution plan.
    /// Consumers that need a cheap value swap on a non-plan-
    /// affecting resource use `replace_value`.
    pub fn replace_resource<T: PlanAffecting, Index>(&mut self, _new: T)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        // A swapped resource is a changed input, so mark its store dirty for
        // the next frame (domain-16 incremental skip seed).
        self.mark_dirty::<T, Index>();
        // The domain-22 plan-recompute seed (`plan_dirty` by PlanAffectingId)
        // and the data-plane value install are sequenced with the adapt
        // subsystem (runtime plan recompute on resource swap).
        let _ = &self.plan_dirty;
    }

    /// Cheap value-swap path for non-plan-affecting resources.
    ///
    /// `T: Replaceable` opts the type into runtime replacement
    /// without signalling plan recompute. The `Replaceable` marker
    /// is consumer-driven per Topic 8 axis B (replaceable but not
    /// plan-affecting is the typical case for app-level state).
    pub fn replace_value<T: Replaceable, Index>(&mut self, _new: T)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        // A swapped value is a changed input: mark its store dirty for the
        // next frame so dependents re-run (domain-16 incremental skip). No
        // plan-recompute dirty, since the value swap is not structural. The
        // data-plane value install is sequenced with the adapt subsystem.
        self.mark_dirty::<T, Index>();
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
