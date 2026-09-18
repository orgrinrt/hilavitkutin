//! Step 1: build the CSR `DependencyGraph` from `AccessMask` overlap, and
//! the registration-order back-edge check that reuses it.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo_tensor::{Capacity, cap_size};
use notko::Maybe;

use super::{DependencyGraph, EdgeKind, PlanDims, PlanInputs};

/// Step 1: build the CSR `DependencyGraph` from `AccessMask` overlap.
///
/// RAW edges are order-independent: for every ordered pair `(s, t)`
/// with `s != t`, if the reader `t` reads what the writer `s` wrote,
/// append a `Read` edge `s to t` (the writer sorts before the reader,
/// regardless of input order, so a reader registered before its writer
/// still gets the dependency). WAW conflicts serialise
/// deterministically: a `Write` edge `s to t` only when `s < t` (the
/// lower input index first), so two writers of one store never produce
/// a back-edge and a spurious cycle. The CSR append-order invariant is
/// preserved because the outer loop walks the source `s` in ascending
/// order. WAR anti-dependencies are out of scope (a tracked plan-chain
/// follow-up).
pub fn build_dag<D: PlanDims>(inputs: &PlanInputs<D::Units, D::Stores>) -> DependencyGraph<D> {
    let mut g: DependencyGraph<D> = DependencyGraph::new();
    let n = inputs.unit_count.0;
    let reads = inputs.reads.as_ref();
    let writes = inputs.writes.as_ref();
    // Outer loop walks the source `s` in ascending order, so every
    // appended edge has a source no smaller than the previous one,
    // satisfying `add_edge_kind`'s CSR append-order invariant.
    let mut s = 0;
    while s < n {
        let mut t = 0;
        while t < n {
            if t != s {
                // RAW: `t` reads what `s` wrote, so the writer `s` runs
                // before the reader `t`. Checked for every ordered pair
                // (not only `s < t`), so a reader registered ahead of its
                // writer still gets the dependency.
                if reads[t].overlaps(&writes[s]).0 {
                    g.add_edge_kind(USize(s), USize(t), EdgeKind::Read); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
                }
                // WAW: both write the same store. Serialise
                // deterministically, the lower input index first, so the
                // back-direction is never added (two writers cannot form a
                // spurious cycle).
                if s < t && writes[t].overlaps(&writes[s]).0 {
                    g.add_edge_kind(USize(s), USize(t), EdgeKind::Write); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
                }
            }
            t += 1;
        }
        s += 1;
    }
    // Ensure every input unit has a row entry, even units with zero
    // out-degree. row_offsets for empty rows equals edge_count
    // (consistent with the CSR invariant: empty row = start == end).
    while g.unit_count.0 < n && g.unit_count.0 < cap_size(<D::Units as Capacity>::CAP) {
        let uc = g.unit_count.0;
        g.row_offsets.as_mut()[uc] = g.edge_count;
        g.unit_count = USize(g.unit_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    }
    g
}

/// First carrier-space back-edge in the registration order, if any.
///
/// The dispatch walk follows the carrier order (registration order) directly:
/// it visits slot 0, 1, 2, ... so the carrier order must already be a
/// topological order of the dependency DAG, or the walk runs a reader before
/// its writer. This builds the canonical dag via `build_dag` (one definition of
/// "what is a dependency edge") and returns the first edge whose source slot
/// index is not strictly less than its destination slot index, as
/// `Maybe::Is((source, destination))` in carrier-slot space; `Maybe::Isnt` when
/// the registration order is topological. WAW edges are appended only for
/// `source < destination`, so they never back-edge; only a RAW edge (a reader
/// registered before its writer) can.
pub fn first_back_edge<D: PlanDims>(
    inputs: &PlanInputs<D::Units, D::Stores>,
) -> Maybe<(USize, USize)> {
    let g = build_dag::<D>(inputs);
    let cols = g.col_indices.as_ref();
    let row_offsets = g.row_offsets.as_ref();
    let mut from = 0;
    let n = g.unit_count.0;
    while from < n && from < cap_size(<D::Units as Capacity>::CAP) {
        let start = row_offsets[from].0;
        let end_excl = g.end_for(from);
        let mut k = start;
        while k < end_excl && k < g.edge_count.0 {
            let to = cols[k].index().0;
            // A back-edge: the source slot is not strictly before the
            // destination slot, so the carrier order would dispatch the
            // destination (reader) before the source (writer).
            if from >= to {
                return Maybe::Is((USize(from), USize(to))); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-wrap internal slot indices; tracked: #72
            }
            k += 1;
        }
        from += 1;
    }
    Maybe::Isnt
}
