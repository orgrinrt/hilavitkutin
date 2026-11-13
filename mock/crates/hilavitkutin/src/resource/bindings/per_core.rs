//! The per-core walks over a bindings list for the threaded accumulator path:
//! rebasing each accumulator onto a core's record slice, collecting the live
//! and `wanted` counts a worker produced, and merging the per-core regions back
//! into one contiguous prefix.

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
/// every `AccumBinding` is offset to `base + lo` with fresh zero live-length and
/// `wanted` cells and capacity `region_cap` (its slice's worst-case room at one
/// append per record), while column / resource / virtual nodes copy their
/// (Copy) pointers unchanged. The per-core bindings then drives the existing
/// `RunFiber::run` unchanged: disjoint per-core regions plus per-core cells mean
/// no shared mutable state, so the append path is sound across workers, and a
/// region refuses past `region_cap` rather than writing into the next core's
/// region. The merge (`MergeAccums`) compacts the per-core regions afterwards.
///
/// The walk returns the SAME bindings type (each node maps to itself), so the
/// per-core value is a drop-in for `&self.bindings` at the dispatch call.
pub trait RebaseBindings {
    /// Build a per-core bindings copy with every accumulator offset to `lo` and
    /// capped at `region_cap`, fresh live and `wanted` cells, other nodes copied
    /// as-is.
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
        // slice's worst case (one per record); past it the append is refused.
        let offset_ptr = unsafe {
            ColumnPtr::new_unchecked(self.ptr.as_ptr().add(lo.0)) // lint:allow(no-bare-numeric) reason: element offset into reserved buffer; tracked: #121
        };
        AccumBinding {
            ptr:    offset_ptr,
            len:    Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core live length; tracked: #121
            wanted: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core wanted count; tracked: #121
            cap:    region_cap,
            tail:   self.tail.rebase_accums(lo, region_cap),
        }
    }
}

/// Collect each accumulator's live and `wanted` counts in carrier-accum order.
///
/// After a worker's unit-outer dispatch over its per-core bindings, the merge
/// needs each accumulator's final live count and the appends it was asked for.
/// This walk writes them into `live[idx]` and `wanted[idx]`, advancing `idx`
/// past each `AccumBinding`; non-accumulator nodes are no-ops. The caller
/// publishes both to the shared merge arrays.
pub trait CollectAccumCounts {
    /// Write each accumulator's live length and `wanted` count, advancing `idx`.
    fn collect_accum_counts(&self, live: &mut [USize], wanted: &mut [USize], idx: &mut USize);
}

impl CollectAccumCounts for BindingNil {
    #[inline]
    fn collect_accum_counts(&self, _live: &mut [USize], _wanted: &mut [USize], _idx: &mut USize) {}
}

impl<T, Tail: CollectAccumCounts> CollectAccumCounts for ResourceBinding<T, Tail> {
    #[inline]
    fn collect_accum_counts(&self, live: &mut [USize], wanted: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_counts(live, wanted, idx);
    }
}

impl<T, Tail: CollectAccumCounts> CollectAccumCounts for ColumnBinding<T, Tail> {
    #[inline]
    fn collect_accum_counts(&self, live: &mut [USize], wanted: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_counts(live, wanted, idx);
    }
}

impl<T, Tail: CollectAccumCounts> CollectAccumCounts for VirtualBinding<T, Tail> {
    #[inline]
    fn collect_accum_counts(&self, live: &mut [USize], wanted: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_counts(live, wanted, idx);
    }
}

impl<T, Tail: CollectAccumCounts> CollectAccumCounts for AccumBinding<T, Tail> {
    #[inline]
    fn collect_accum_counts(&self, live: &mut [USize], wanted: &mut [USize], idx: &mut USize) {
        live[idx.0] = self.len.get(); // lint:allow(no-bare-numeric) reason: accum index into live array; tracked: #121
        wanted[idx.0] = self.wanted.get(); // lint:allow(no-bare-numeric) reason: accum index into wanted array; tracked: #121
        idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum index step; tracked: #121
        self.tail.collect_accum_counts(live, wanted, idx);
    }
}

/// The per-core counts the merge reads, one row of `stride` per core.
///
/// `live[c * stride + a]` and `wanted[c * stride + a]` are core `c`'s live and
/// `wanted` counts for accumulator `a`; `per`, `ncores` and `total` give each
/// core's slice start `lo_c = (c * per).min(total)`.
#[derive(Clone, Copy)]
pub struct PerCoreCounts<'a> {
    per:    USize,
    ncores: USize,
    total:  USize,
    live:   &'a [USize],
    wanted: &'a [USize],
    stride: USize,
}

impl<'a> PerCoreCounts<'a> {
    /// Bundle `per` records per core slice over `ncores` cores and `total`
    /// records, with the cores' `live` and `wanted` rows of width `stride`.
    pub const fn new(
        per: USize,
        ncores: USize,
        total: USize,
        live: &'a [USize],
        wanted: &'a [USize],
        stride: USize,
    ) -> Self {
        Self {
            per,
            ncores,
            total,
            live,
            wanted,
            stride,
        }
    }
}

/// Merge the per-core accumulator regions into the shared buffer's live prefix.
///
/// After the threaded unit-outer phase, each accumulator's per-core regions sit
/// at `[lo_c, lo_c + live_c)` of the shared reserved buffer. This walk
/// forward-compacts them in ascending core order into the shared binding's
/// `[0, sum live)` prefix and sets the binding live length, so downstream
/// readers see the same contiguous prefix single-core `run()` would produce
/// when nothing was refused. `accum_idx` threads the accumulator position
/// through the walk.
///
/// The shared `wanted` becomes the merged live length plus every refused
/// append: the shared binding's own (`wanted - len` before the merge, from the
/// leading bands) and each core's `wanted_c - live_c`. So the shared binding has
/// refused exactly when some append was refused anywhere, and `wanted` is the
/// total the bodies asked for, whatever the split.
///
/// The write cursor never exceeds `lo_c` (`write_pos = sum of prior live_c <=
/// sum of prior slice sizes = lo_c`), so each `ptr::copy` has `dst <= src` and is
/// forward-safe; order is preserved because cores own ascending record slices.
pub trait MergeAccums {
    /// Forward-compact each accumulator's per-core regions and set its counts.
    fn merge_accums(&self, counts: PerCoreCounts<'_>, accum_idx: &mut USize);
}

impl MergeAccums for BindingNil {
    #[inline]
    fn merge_accums(&self, _counts: PerCoreCounts<'_>, _accum_idx: &mut USize) {}
}

impl<T, Tail: MergeAccums> MergeAccums for ResourceBinding<T, Tail> {
    #[inline]
    fn merge_accums(&self, counts: PerCoreCounts<'_>, accum_idx: &mut USize) {
        self.tail.merge_accums(counts, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for ColumnBinding<T, Tail> {
    #[inline]
    fn merge_accums(&self, counts: PerCoreCounts<'_>, accum_idx: &mut USize) {
        self.tail.merge_accums(counts, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for VirtualBinding<T, Tail> {
    #[inline]
    fn merge_accums(&self, counts: PerCoreCounts<'_>, accum_idx: &mut USize) {
        self.tail.merge_accums(counts, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for AccumBinding<T, Tail> {
    #[inline]
    fn merge_accums(&self, counts: PerCoreCounts<'_>, accum_idx: &mut USize) {
        let a = accum_idx.0; // lint:allow(no-bare-numeric) reason: this accumulator's position; tracked: #121
        let base = self.ptr.as_ptr();
        // Refused appends of the leading bands, on the shared binding itself.
        let mut refused = self.wanted.get().0 - self.len.get().0; // lint:allow(no-bare-numeric) reason: refused-append tally, wanted >= len by the append rule; tracked: #121
        let mut write_pos = 0; // lint:allow(no-bare-numeric) reason: compaction cursor; tracked: #121
        let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
        while c < counts.ncores.0 {
            let lo = (c * counts.per.0).min(counts.total.0); // lint:allow(no-bare-numeric) reason: core slice start; tracked: #121
            let slot = c * counts.stride.0 + a; // lint:allow(no-bare-numeric) reason: per-(core,accum) slot; tracked: #121
            let live_ca = counts.live[slot];
            refused += counts.wanted[slot].0 - live_ca.0; // lint:allow(no-bare-numeric) reason: refused-append tally, wanted_c >= live_c by the append rule; tracked: #121
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
        self.wanted.set(USize(write_pos + refused)); // lint:allow(no-bare-numeric) reason: merged wanted = merged live + every refused append; tracked: #121
        accum_idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum position step; tracked: #121
        self.tail.merge_accums(counts, accum_idx);
    }
}
