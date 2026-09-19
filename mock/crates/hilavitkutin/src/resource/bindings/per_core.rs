//! The per-core walks over a bindings list for the threaded accumulator path:
//! rebasing each accumulator onto a core's record slice, collecting the live
//! lengths a worker produced, and merging the per-core regions back into one
//! contiguous prefix.

use core::cell::Cell;
use core::marker::PhantomData;

use arvo::USize;

use super::{AccumBinding, BindingNil, ColumnBinding, ResourceBinding, VirtualBinding};
use crate::resource::provenance::ColumnPtr;

/// Per-core rebase of the bindings for the GATE-2 deviation 9 threaded
/// accumulator path.
///
/// Each worker drives the unit-outer accumulator dispatch over its head+tail
/// record slice `[lo, lo+region_cap)` against a per-core COPY of the bindings:
/// every `AccumBinding` is offset to `base + lo` with a fresh zero live-length
/// cell and capacity `region_cap` (its slice's worst-case room at one append per
/// record), while column / resource / virtual nodes copy their (Copy) pointers
/// unchanged. The per-core bindings then drives the existing `RunFiber::run`
/// unchanged: disjoint per-core regions plus per-core cells mean no shared
/// mutable state, so the append path is sound across workers. The merge
/// (`MergeAccums`) compacts the per-core regions afterwards.
///
/// The walk returns the SAME bindings type (each node maps to itself), so the
/// per-core value is a drop-in for `&self.bindings` at the dispatch call.
pub trait RebaseBindings {
    /// Build a per-core bindings copy with every accumulator offset to `lo` and
    /// capped at `region_cap`, fresh live cells, other nodes copied as-is.
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self;
}

impl RebaseBindings for BindingNil {
    #[inline]
    fn rebase_accums(&self, _lo: USize, _region_cap: USize) -> Self {
        BindingNil
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for ResourceBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        ResourceBinding {
            erased: self.erased,
            shape:  self.shape,
            _ty:    PhantomData,
            tail:   self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for ColumnBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        ColumnBinding {
            ptr:   self.ptr,
            count: self.count,
            tail:  self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for VirtualBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        VirtualBinding {
            _marker: PhantomData,
            stamp:   Cell::new(self.stamp.get()),
            tail:    self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for AccumBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        // SAFETY: `ptr` is the reserved-buffer base; `lo` is this core's record-
        // slice start (`lo < total <= reserved capacity`), so `base + lo` lands
        // inside the reserved allocation. `region_cap` caps appends to the
        // slice's worst case (one per record).
        let offset_ptr = unsafe {
            ColumnPtr::new_unchecked(self.ptr.as_ptr().add(lo.0)) // lint:allow(no-bare-numeric) reason: element offset into reserved buffer; tracked: #121
        };
        AccumBinding {
            ptr:  offset_ptr,
            len:  Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core live length; tracked: #121
            cap:  region_cap,
            tail: self.tail.rebase_accums(lo, region_cap),
        }
    }
}

/// Collect each accumulator's live length into `out` in carrier-accum order.
///
/// After a worker's unit-outer dispatch over its per-core bindings, the merge
/// needs each accumulator's final live count. This walk writes them into
/// `out[idx]`, advancing `idx` past each `AccumBinding`; non-accumulator nodes
/// are no-ops. The caller publishes `out` to the shared merge array.
pub trait CollectAccumLive {
    /// Write each accumulator live length into `out`, advancing `idx`.
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize);
}

impl CollectAccumLive for BindingNil {
    #[inline]
    fn collect_accum_live(&self, _out: &mut [USize], _idx: &mut USize) {}
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for ResourceBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for ColumnBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for VirtualBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for AccumBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        out[idx.0] = self.len.get(); // lint:allow(no-bare-numeric) reason: accum index into live array; tracked: #121
        idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum index step; tracked: #121
        self.tail.collect_accum_live(out, idx);
    }
}

/// Merge the per-core accumulator regions into the shared buffer's live prefix.
///
/// After the threaded unit-outer phase, each accumulator's per-core regions sit
/// at `[lo_c, lo_c + live_c)` of the shared reserved buffer (`lo_c = (c*per).min
/// (total)`). This walk forward-compacts them in ascending core order into the
/// shared binding's `[0, sum live)` prefix and sets the binding live length, so
/// downstream readers see the same contiguous prefix single-core `run()` would
/// produce. `live[c * stride + a]` is core `c`'s live count for accumulator `a`;
/// `accum_idx` threads the accumulator position through the walk.
///
/// The write cursor never exceeds `lo_c` (`write_pos = sum of prior live_c <=
/// sum of prior slice sizes = lo_c`), so each `ptr::copy` has `dst <= src` and is
/// forward-safe; order is preserved because cores own ascending record slices.
pub trait MergeAccums {
    /// Forward-compact each accumulator's per-core regions and set its live length.
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    );
}

impl MergeAccums for BindingNil {
    #[inline]
    fn merge_accums(
        &self,
        _per: USize,
        _ncores: USize,
        _total: USize,
        _live: &[USize],
        _stride: USize,
        _accum_idx: &mut USize,
    ) {
    }
}

impl<T, Tail: MergeAccums> MergeAccums for ResourceBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for ColumnBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for VirtualBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for AccumBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        let a = accum_idx.0; // lint:allow(no-bare-numeric) reason: this accumulator's position; tracked: #121
        let base = self.ptr.as_ptr();
        let mut write_pos = 0; // lint:allow(no-bare-numeric) reason: compaction cursor; tracked: #121
        let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
        while c < ncores.0 {
            let lo = (c * per.0).min(total.0); // lint:allow(no-bare-numeric) reason: core slice start; tracked: #121
            let live_ca = live[c * stride.0 + a]; // lint:allow(no-bare-numeric) reason: per-(core,accum) live count; tracked: #121
            if live_ca.0 > 0 && lo != write_pos {
                // SAFETY: `base + lo` and `base + write_pos` are both within the
                // reserved buffer (`lo, write_pos < total <= capacity`), the two
                // ranges of `live_ca` elements are in-bounds, and `write_pos <=
                // lo` makes the forward copy non-destructive of unread source.
                unsafe {
                    core::ptr::copy(base.add(lo), base.add(write_pos), live_ca.0); // lint:allow(no-bare-numeric) reason: compaction memmove offsets/len; tracked: #121
                }
            }
            write_pos += live_ca.0; // lint:allow(no-bare-numeric) reason: cursor advance; tracked: #121
            c += 1; // lint:allow(no-bare-numeric) reason: core index step; tracked: #121
        }
        self.len.set(USize(write_pos));
        accum_idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum position step; tracked: #121
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}
