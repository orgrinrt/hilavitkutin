//! Step 2: topological sort via Kahn's algorithm.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::UnitId;

use super::{DependencyGraph, PlanDims};

/// Sentinel value marking an already-placed unit in the in-degree
/// counter array used by `topo_sort`. Distinguished from a real
/// in-degree count (which is bounded by the edge capacity) by being
/// set to `usize::MAX`, which no valid in-degree can ever reach.
const CONSUMED: USize = USize(usize::MAX); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel definition; rust grammar requires raw usize literal here; tracked: #72

/// Step 2: topological sort via Kahn's algorithm.
///
/// Returns the units in topo order and the count of units that were
/// placed. The placed-count is the cycle-detection signal: when
/// `placed < graph.unit_count`, the input contains a cycle. The
/// runner (`compute_execution_plan`) is responsible for translating
/// that into `PlanError::Cycle`. Trailing entries in the returned
/// array (indices `placed..unit capacity`) are left as `UnitId::ZERO`
/// (the array's initial fill); they are NOT the cycle members. The
/// caller must use the placed count to slice the valid prefix.
pub fn topo_sort<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> (<D::Units as Capacity>::Array<UnitId>, USize) {
    let mut out: <D::Units as Capacity>::Array<UnitId> =
        <D::Units as Capacity>::filled(UnitId::ZERO);
    let n = graph.unit_count.0;
    if n == 0 {
        return (out, <USize as Identity<Additive>>::IDENTITY);
    }
    // In-degree counter.
    let mut in_degree: <D::Units as Capacity>::Array<USize> =
        <D::Units as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let cols = graph.col_indices.as_ref();
    let row_offsets = graph.row_offsets.as_ref();
    let mut e = 0;
    while e < graph.edge_count.0 {
        let d = cols[e].index().0;
        if d < cap_size(<D::Units as Capacity>::CAP) {
            let id = in_degree.as_mut();
            id[d] = USize(id[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        e += 1;
    }
    // Simple queue replacement: a placement cursor over a fixed array.
    // The outer loop is a fixed-point iteration over zero-in-degree
    // units. Cycles cause an iteration with no progress, at which
    // point the loop exits with `placed < n`; the runner reads the
    // count and produces `PlanError::Cycle`.
    let mut placed: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal placement cursor; rust grammar requires usize; tracked: #72
    let mut progress = true;
    while progress && placed < n {
        progress = false;
        let mut i = 0;
        while i < n {
            // Skip already-placed units (in_degree set to CONSUMED).
            if in_degree.as_ref()[i].0 == 0 {
                let id = UnitId::from_index(USize(i));
                out.as_mut()[placed] = id;
                placed += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal cursor increment; tracked: #72
                in_degree.as_mut()[i] = CONSUMED;
                progress = true;
                // Decrement successors of unit `i`.
                let start = row_offsets[i].0;
                let end_excl = graph.end_for(i);
                let mut k = start;
                while k < end_excl {
                    let d = cols[k].index().0;
                    let deg = in_degree.as_ref()[d];
                    if d < cap_size(<D::Units as Capacity>::CAP) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel + bound check on USize internal field; tracked: #72
                        && deg.0 != CONSUMED.0
                        && deg.0 > 0
                    {
                        in_degree.as_mut()[d] = USize(deg.0 - 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
                    }
                    k += 1;
                }
            }
            i += 1;
        }
    }
    (out, USize(placed)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-wrap internal cursor; tracked: #72
}
