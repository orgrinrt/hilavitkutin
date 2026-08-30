//! Dependency graph: CSR-backed adjacency between WU nodes (domains
//! 11, 15).
//!
//! Per Topic 9 axis B bench finding: CSR is canonical at every N, no
//! threshold. The dense matrix shape that preceded this is gone; the
//! workspace rule `no-legacy-shims-pre-1.0.md` precludes leaving a
//! shim. CSR's three columnar arrays are aligned to arvo-sparse
//! conventions:
//!
//! - `row_offsets[i]`: first edge index for unit `i`. Edges for unit
//!   `i` occupy `row_offsets[i]..end_for(i)`, where `end_for(i)` is
//!   `row_offsets[i + 1]` for `i < unit_count - 1`, or `edge_count`
//!   for the last populated unit.
//! - `col_indices[k]`: the destination `UnitId` for edge `k`.
//! - `edge_kinds[k]`: classification (`Read`, `Write`, `Control`).
//!
//! The unit capacity (`D::Units`) is the row cap; the edge capacity
//! (`D::Edges`) is the edge cap. The `add_edge` path assumes edges are
//! appended in row-major order (all edges for unit 0, then unit 1, then
//! ...). The plan-stage algorithm chain populates the graph in topo
//! order, which already guarantees that ordering.

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use arvo_bitmask::NodeId;
use arvo_sparse::{Csr, CsrBidirectional};
use arvo_tensor::{Capacity, cap_size};
use core::fmt;

use hilavitkutin_api::UnitId;

use crate::plan::dims::PlanDims;

/// Classification of one dependency edge.
///
/// The dispatch stage routes edges differently per kind: `Read` and
/// `Write` participate in column classification + dirty propagation,
/// while `Control` only orders dispatch without contributing to the
/// data-flow analysis.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum EdgeKind {
    /// `to` reads what `from` wrote (RAW). Drives data dependency.
    Read,
    /// `to` writes after `from` wrote (WAW). Drives ordering.
    Write,
    /// Explicit ordering with no data interaction (synchronisation
    /// barrier, side-effect ordering).
    Control,
}

impl Default for EdgeKind {
    fn default() -> Self {
        Self::Read
    }
}

/// CSR-backed dependency graph.
///
/// Sized by two capacities projected from one `D: PlanDims`: the unit
/// capacity (`D::Units`, the row cap) and the edge capacity
/// (`D::Edges`, the edge cap). The current populated counts live in
/// `unit_count` and `edge_count`.
pub struct DependencyGraph<D: PlanDims> {
    /// `row_offsets[i]` = first edge index for unit `i`.
    pub row_offsets: <D::Units as Capacity>::Array<USize>,
    /// Destination unit per edge.
    pub col_indices: <D::Edges as Capacity>::Array<UnitId>,
    /// Kind per edge.
    pub edge_kinds: <D::Edges as Capacity>::Array<EdgeKind>,
    /// Number of units actually populated.
    pub unit_count: USize,
    /// Number of edges actually populated.
    pub edge_count: USize,
}

impl<D: PlanDims> DependencyGraph<D> {
    /// Empty graph (no units, no edges).
    pub fn new() -> Self {
        Self {
            row_offsets: <D::Units as Capacity>::filled(USize::ZERO),
            col_indices: <D::Edges as Capacity>::filled(UnitId::ZERO),
            edge_kinds: <D::Edges as Capacity>::filled(EdgeKind::Read),
            unit_count: USize::ZERO,
            edge_count: USize::ZERO,
        }
    }

    /// End-of-row index for unit `i` (exclusive). For unit `i` not
    /// the last, this is `row_offsets[i + 1]`; for the last unit
    /// it's `edge_count`. Visible to sibling plan modules so the
    /// 13-step chain can scan a single row without reimplementing
    /// the boundary logic at every call site.
    pub(super) fn end_for(&self, i: usize) -> usize {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal indexing; rust grammar requires usize; tracked: #72
        let next = i + 1;
        let count = self.unit_count.0;
        if next < count {
            self.row_offsets.as_ref()[next].0
        } else {
            self.edge_count.0
        }
    }

    /// True iff an edge `from to` exists. False if either index is
    /// out of range or no matching edge is recorded. Linear scan
    /// within the row, which is small (typical fan-out is `O(1)`).
    pub fn has_edge(&self, from: USize, to: USize) -> Bool {
        let f = from.0;
        let t = to.0;
        if f >= cap_size(<D::Units as Capacity>::CAP) || t >= cap_size(<D::Units as Capacity>::CAP)
        {
            return Bool::FALSE;
        }
        if f >= self.unit_count.0 {
            return Bool::FALSE;
        }
        let start = self.row_offsets.as_ref()[f].0;
        let end = self.end_for(f);
        let cols = self.col_indices.as_ref();
        let mut k = start;
        while k < end {
            // Compare via the typed index accessor. Equality here is
            // value equality on the array index.
            if cols[k].index().0 == t {
                return Bool::TRUE;
            }
            k += 1;
        }
        Bool::FALSE
    }

    /// Append edge `from to` with the given kind. No-op if either
    /// index is out of range, if the edge capacity is exhausted, or if
    /// `from` is less than the last appended row (CSR append-order
    /// invariant).
    pub fn add_edge_kind(&mut self, from: USize, to: USize, kind: EdgeKind) {
        let f = from.0;
        let t = to.0;
        if f >= cap_size(<D::Units as Capacity>::CAP) || t >= cap_size(<D::Units as Capacity>::CAP)
        {
            return;
        }
        if self.edge_count.0 >= cap_size(<D::Edges as Capacity>::CAP) {
            return;
        }
        // CSR append-order: `from` may not be smaller than the
        // current frontier. The plan-stage chain walks units in
        // topo order, which satisfies this naturally.
        let frontier = if self.unit_count.0 == 0 {
            0
        } else {
            self.unit_count.0 - 1
        };
        if f < frontier {
            return;
        }
        // Advance the frontier: every unit `g` in `frontier..=f`
        // gets `row_offsets[g] = edge_count`. The first time we add
        // an edge for unit `g`, that sets its row's start.
        while self.unit_count.0 <= f {
            let uc = self.unit_count.0;
            self.row_offsets.as_mut()[uc] = self.edge_count;
            self.unit_count = USize(self.unit_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        // Append the edge to the columnar arrays. Reconstruct the
        // destination UnitId from the array-index value via the typed
        // accessor.
        let k = self.edge_count.0;
        let dest = UnitId::from_index(USize(t));
        self.col_indices.as_mut()[k] = dest;
        self.edge_kinds.as_mut()[k] = kind;
        self.edge_count = USize(self.edge_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    }

    /// Convenience: append a `Read` edge (the common case).
    pub fn add_edge(&mut self, from: USize, to: USize) {
        self.add_edge_kind(from, to, EdgeKind::Read);
    }

    /// Number of outgoing edges from unit `i`. Zero if `i` is past
    /// the populated range.
    pub fn out_degree(&self, i: USize) -> USize {
        let idx = i.0;
        if idx >= self.unit_count.0 {
            return USize::ZERO;
        }
        let start = self.row_offsets.as_ref()[idx].0;
        let end = self.end_for(idx);
        USize(end - start) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    }

    /// Convert to a bidirectional CSR for the arvo-sparse structural
    /// algorithms.
    ///
    /// The forward adjacency is this graph's own CSR arrays copied
    /// within the live `unit_count` / `edge_count`; `with_transpose`
    /// then builds the predecessor index that rcm, block-diagonal, and
    /// spectral all consume. The two capacities pass straight through
    /// to arvo's `R` / `NNZ` positions, so the backing arrays are the
    /// same length and the copy is element-for-element within the live
    /// range. Live counts are set via `with_live_counts` so the slack
    /// tail past `unit_count` / `edge_count` is never iterated and never
    /// contributes a phantom edge into the default `NodeId(0)` slot.
    pub fn to_csr_bidirectional(&self) -> CsrBidirectional<D::Units, D::Edges, EdgeKind>
    where
        <D::Units as Capacity>::Array<USize>: Copy,
        <D::Edges as Capacity>::Array<NodeId>: Copy,
    {
        let uc = self.unit_count.0;
        let ec = self.edge_count.0;
        let mut csr: Csr<D::Units, D::Edges, EdgeKind> =
            Csr::with_live_counts(self.unit_count, self.edge_count);
        // forward row offsets and edge kinds carry over verbatim.
        csr.row_ptr.as_mut()[..uc].copy_from_slice(&self.row_offsets.as_ref()[..uc]);
        csr.values.as_mut()[..ec].copy_from_slice(&self.edge_kinds.as_ref()[..ec]);
        // destinations convert from the typed UnitId to the arvo NodeId.
        for (dst, src) in csr.col_idx.as_mut()[..ec]
            .iter_mut()
            .zip(self.col_indices.as_ref()[..ec].iter())
        {
            *dst = NodeId::new(src.index());
        }
        csr.with_transpose()
    }
}

impl<D: PlanDims> Copy for DependencyGraph<D>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<UnitId>: Copy,
    <D::Edges as Capacity>::Array<EdgeKind>: Copy,
{
}

impl<D: PlanDims> Clone for DependencyGraph<D>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<UnitId>: Copy,
    <D::Edges as Capacity>::Array<EdgeKind>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for DependencyGraph<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> fmt::Debug for DependencyGraph<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DependencyGraph")
            .field("unit_cap", &cap_size(<D::Units as Capacity>::CAP))
            .field("edge_cap", &cap_size(<D::Edges as Capacity>::CAP))
            .field("units", &self.unit_count.0)
            .field("edges", &self.edge_count.0)
            .finish()
    }
}
