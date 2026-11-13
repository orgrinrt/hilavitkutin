//! The per-core walks over a bindings list for the threaded accumulator path:
//! rebasing each accumulator onto a core's record slice, collecting the live
//! and `wanted` counts a worker produced, and merging the per-core regions back
//! into one contiguous prefix.

use core::cell::Cell;
use core::marker::PhantomData;

use arvo::USize;

use super::{AccumBinding, BindingNil, ColKind, ColumnBinding, ResourceBinding, VirtualBinding};
use crate::resource::provenance::ColumnPtr;

/// Per-core rebase of the bindings for the GATE-2 deviation 9 threaded
/// accumulator path.
///
/// Each worker drives the unit-outer accumulator dispatch over its
/// record slice `[lo, lo+region_cap)` against a per-core COPY of the bindings:
/// every `AccumBinding` is offset past the records the leading bands accepted on
/// it, to `base + len + lo` clipped to its capacity, with fresh zero live-length
/// and `wanted` cells and capacity `region_cap` (its slice's worst-case room at
/// one append per record) clipped to the room left past that offset, while
/// column / resource / virtual nodes copy their (Copy) pointers unchanged. The
/// shared binding's `len` is read here and not written: the leading bands set it
/// before the publish, and nothing writes it until every worker has arrived. The
/// per-core bindings then drives the existing `RunFiber::run` unchanged:
/// disjoint per-core regions plus per-core cells mean no shared mutable state,
/// so the append path is sound across workers, and a region refuses past its cap
/// rather than writing into the next core's region or over the leading records.
/// The merge (`MergeAccums`) compacts the per-core regions afterwards.
///
/// The walk returns the SAME bindings type (each node maps to itself), so the
/// per-core value is a drop-in for `&self.bindings` at the dispatch call.
pub trait RebaseBindings {
    /// Build a per-core bindings copy with every accumulator offset to `lo` past
    /// its leading records and capped at `region_cap` within its capacity, fresh
    /// live and `wanted` cells, other nodes copied as-is.
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

impl<T, Tail: RebaseBindings, K: ColKind> RebaseBindings for ColumnBinding<T, Tail, K> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        // The copy points at the same reservations and at the mark of the
        // binding it was copied from, so a write on this core marks the one
        // store.
        ColumnBinding {
            ptr:   self.ptr,
            side:  K::dup_side(&self.side),
            count: self.count,
            mark:  self.mark.shared(),
            pos:   self.pos,
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

/// Where a core's region of an accumulator starts: after the `leading` records
/// the leading bands accepted, at the core's slice start `lo`, clipped to the
/// capacity. The rebase and the merge both place a region by this, so they
/// agree on it.
#[inline]
const fn region_start(leading: USize, lo: USize, cap: USize) -> USize {
    let start = leading.0.saturating_add(lo.0); // lint:allow(no-bare-numeric) reason: region offset past the leading records; tracked: #121
    USize(if start < cap.0 { start } else { cap.0 })
}

/// Stride between per-core accumulator regions: `ceil(total / ncores)`.
#[inline]
const fn accum_per(total: USize, ncores: USize) -> USize {
    let n = if ncores.0 == 0 { 1 } else { ncores.0 }; // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
    USize(total.0.div_ceil(n)) // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
}

/// One core's exclusive accumulator region as `(lo, len)`.
///
/// The single definition of the accumulator's record split. The worker
/// rebases its bindings onto this region and the merge reads core c's region
/// at this `lo`. Both sides must agree exactly or the merge reads a region a
/// different core wrote, so both call this and neither spells the arithmetic.
///
/// Regions tile `[0, total)` in core order, so the merge can concatenate them
/// in ascending core order without gaps. A surplus core (more cores than
/// records) gets `len == 0` and appends nothing.
#[inline]
pub(crate) const fn accum_slice(total: USize, ncores: USize, core: USize) -> (USize, USize) {
    let per = accum_per(total, ncores).0;
    let at = core.0 * per; // lint:allow(no-bare-numeric) reason: unclipped slice start; tracked: #121
    let lo = if at < total.0 { at } else { total.0 }; // lint:allow(no-bare-numeric) reason: slice start clipped to the record count; tracked: #121
    let end = lo + per; // lint:allow(no-bare-numeric) reason: unclipped slice end; tracked: #121
    let hi = if end < total.0 { end } else { total.0 }; // lint:allow(no-bare-numeric) reason: slice end clipped to the record count; tracked: #121
    (USize(lo), USize(hi - lo)) // lint:allow(no-bare-numeric) reason: slice length; tracked: #121
}

impl<T, Tail: RebaseBindings> RebaseBindings for AccumBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        // The region starts after the records the leading bands accepted on the
        // shared binding, and its cap is clipped to what is left of the capacity
        // past that start, so no region overlaps them or another region.
        let start = region_start(self.len.get(), lo, self.cap);
        let cap = USize(region_cap.0.min(self.cap.0 - start.0)); // lint:allow(no-bare-numeric) reason: region cap clipped to the room left, start <= cap; tracked: #121
        // SAFETY: `ptr` is the reserved-buffer base and `start <= cap`, so
        // `base + start` lands inside the reserved allocation or one past its
        // end; a region at the end has cap zero and never writes.
        let offset_ptr = unsafe {
            ColumnPtr::new_unchecked(self.ptr.as_ptr().add(start.0)) // lint:allow(no-bare-numeric) reason: element offset into reserved buffer; tracked: #121
        };
        AccumBinding {
            ptr: offset_ptr,
            len: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core live length; tracked: #121
            wanted: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core wanted count; tracked: #121
            cap,
            tail: self.tail.rebase_accums(lo, region_cap),
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

impl<T, Tail: CollectAccumCounts, K: ColKind> CollectAccumCounts for ColumnBinding<T, Tail, K> {
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
/// `wanted` counts for accumulator `a`; `ncores` and `total` give each core's
/// slice start through `accum_slice`, the definition the worker placed it by.
#[derive(Clone, Copy)]
pub struct PerCoreCounts<'a> {
    ncores: USize,
    total:  USize,
    live:   &'a [USize],
    wanted: &'a [USize],
    stride: USize,
}

impl<'a> PerCoreCounts<'a> {
    /// Bundle the split of `total` records over `ncores` cores with the
    /// cores' `live` and `wanted` rows of width `stride`.
    pub const fn new(
        ncores: USize,
        total: USize,
        live: &'a [USize],
        wanted: &'a [USize],
        stride: USize,
    ) -> Self {
        Self {
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
/// The leading bands appended on the shared binding before the workers ran, so
/// `[0, len)` holds their records, and each core's region sits at `[s_c, s_c +
/// live_c)` with `s_c = region_start(len, lo_c, cap)`, the start the rebase gave
/// it. This walk forward-compacts the regions in ascending core order after the
/// leading records, into `[len, len + sum live)`, and sets the binding's live
/// length to that end, so downstream readers see the leading records and then
/// the cores' in the order single-core `run()` appends them. `accum_idx` threads
/// the accumulator position through the walk.
///
/// The shared `wanted` becomes the merged live length plus every refused
/// append: the shared binding's own (`wanted - len` before the merge, from the
/// leading bands) and each core's `wanted_c - live_c`, saturating. So no record
/// any band accepted leaves the counts, the shared binding has refused exactly
/// when some append was refused anywhere, and `wanted` is the total the bodies
/// asked for, whatever the split.
///
/// The write cursor never exceeds `s_c` (`write_pos = len + sum of prior live_c
/// <= len + sum of prior slice sizes = len + lo_c`, and a region clipped to the
/// capacity has no room and nothing to copy), so each `ptr::copy` has `dst <=
/// src` and is forward-safe; order is preserved because cores own ascending
/// record slices.
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

impl<T, Tail: MergeAccums, K: ColKind> MergeAccums for ColumnBinding<T, Tail, K> {
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
        // The leading bands' records stay at `[0, leading)`; the regions were
        // placed after them, and so is the compaction cursor.
        let leading = self.len.get();
        // Refused appends of the leading bands, on the shared binding itself.
        let mut refused = self.wanted.get().0 - leading.0; // lint:allow(no-bare-numeric) reason: refused-append tally, wanted >= len by the append rule; tracked: #121
        let mut write_pos = leading.0; // lint:allow(no-bare-numeric) reason: compaction cursor; tracked: #121
        let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
        while c < counts.ncores.0 {
            let (lo, _) = accum_slice(counts.total, counts.ncores, USize(c)); // lint:allow(no-bare-numeric) reason: core index into the shared split; tracked: #121
            let src = region_start(leading, lo, self.cap).0; // lint:allow(no-bare-numeric) reason: the region start the rebase used; tracked: #121
            let slot = c * counts.stride.0 + a; // lint:allow(no-bare-numeric) reason: per-(core,accum) slot; tracked: #121
            let live_ca = counts.live[slot];
            refused = refused.saturating_add(counts.wanted[slot].0 - live_ca.0); // lint:allow(no-bare-numeric) reason: refused-append tally, wanted_c >= live_c by the append rule; tracked: #121
            if live_ca.0 > 0 && src != write_pos {
                // SAFETY: the region `[src, src + live_ca)` lies inside the
                // reserved buffer (the rebase capped it at the room left past
                // `src`), `write_pos <= src` (see the trait doc), and the
                // forward copy never overwrites source it has not read yet.
                unsafe {
                    core::ptr::copy(base.add(src), base.add(write_pos), live_ca.0); // lint:allow(no-bare-numeric) reason: compaction memmove offsets/len; tracked: #121
                }
            }
            write_pos += live_ca.0; // lint:allow(no-bare-numeric) reason: cursor advance; tracked: #121
            c += 1; // lint:allow(no-bare-numeric) reason: core index step; tracked: #121
        }
        self.len.set(USize(write_pos));
        self.wanted.set(USize(write_pos.saturating_add(refused))); // lint:allow(no-bare-numeric) reason: merged wanted = merged live + every refused append, saturating like the append; tracked: #121
        accum_idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum position step; tracked: #121
        self.tail.merge_accums(counts, accum_idx);
    }
}
