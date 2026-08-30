//! Shared workload for the fiber_theory bench (#644, the definitive
//! spectral-versus-greedy fiber comparison; seed governance item 3).
//!
//! Canon Step 7/8 assigns spectral to trunk formation and greedy (with
//! matrix-chain DP above 10 ops) to fiber grouping; the shipped plan
//! chain instead forms trunks from block-diagonal components and uses
//! spectral for fibers within wide blocks. This bench times the two
//! shipped algorithms (`plan::steps::group_fibers` walking the topo
//! order, and `plan::steps::spectral_partition` over the symmetric
//! Laplacian) on the same layered wide-fan-out DAG family, the regime
//! the role fork concerns. The timed body is one full grouping call on
//! a prepared graph; the harness size `n` is the unit count directly
//! (the engine's default unit capacity is 64).
//!
//! What this bench does and does not decide: dispatch follows the
//! compile-time carrier grouping, so execution under each grouping is
//! not steerable at runtime; the measurable halves are the plan-time
//! algorithm cost (here, harness-timed) and the grouping quality under
//! canon's own proxies (fiber count as parallelism grain, cut edges
//! between fibers as bridge pressure), which the companion quality test
//! in this crate computes and the findings memo records. Outputs are
//! the assignment hash; the two algorithms legitimately produce
//! different groupings, so the bench declares `may_differ`.

use hilavitkutin::plan::steps::{group_fibers, spectral_partition, topo_sort};
use hilavitkutin::plan::{DefaultPlanDims, DependencyGraph};

type D = DefaultPlanDims;

/// Deterministic layered wide-fan-out DAG: `units` nodes over layers of
/// width eight, each unit feeding one to three units of the next layer
/// (seed-driven), the shape where fiber grouping has real choices
/// (fan-in forces fences, sharing rewards grouping). Edges are added in
/// ascending source order, which the graph's CSR builder requires.
pub fn build_graph(seed: u64, units: usize) -> DependencyGraph<D> {
    let mut g: DependencyGraph<D> = DependencyGraph::new();
    let mut s = seed | 1;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let width = 8usize;
    for u in 0..units {
        let layer = u / width;
        let succ_start = (layer + 1) * width;
        if succ_start >= units {
            continue;
        }
        let succ_end = ((layer + 2) * width).min(units);
        let outs = 1 + (next() as usize % 3);
        for _ in 0..outs {
            let t = succ_start + (next() as usize % (succ_end - succ_start));
            g.add_edge(arvo::USize(u), arvo::USize(t));
        }
    }
    // Trailing sink-only units carry no out-edges; the CSR builder only
    // advances unit_count on sources, so close their empty rows at the
    // final edge offset (what add_edge would have done) and state the
    // true count.
    while g.unit_count.0 < units {
        let uc = g.unit_count.0;
        g.row_offsets.as_mut()[uc] = g.edge_count;
        g.unit_count = arvo::USize(uc + 1);
    }
    g
}

/// Which grouping theory the arm runs.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Theory {
    /// Canon Step 8's fiber grouper: greedy walk over the topo order.
    Greedy,
    /// The shipped wide-block fiber former: Fiedler bisection over the
    /// symmetric Laplacian.
    Spectral,
}

/// Run one grouping and fold the assignment into a hash for validation.
pub fn run_theory(theory: Theory, g: &DependencyGraph<D>) -> u64 {
    let grouping = match theory {
        Theory::Greedy => {
            let (topo, _count) = topo_sort::<D>(g);
            group_fibers::<D>(g, &topo)
        }
        Theory::Spectral => spectral_partition::<D>(g),
    };
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let n = g.unit_count.0;
    let asn = grouping.assignment.as_ref();
    for a in asn.iter().take(n) {
        h = (h ^ a.index().0 as u64).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h ^ (grouping.fiber_count.0 as u64).rotate_left(32)
}

/// Quality proxies for one grouping on one graph: (fiber count, cut
/// edges between fibers). Canon's Step 8 cost is record walks times
/// union column bytes per fiber; without a column model on the bare
/// DAG, fiber count stands proxy for parallelism grain and the cut for
/// bridge (fence) pressure. The findings memo carries the caveat.
pub fn quality(theory: Theory, g: &DependencyGraph<D>) -> (usize, usize) {
    let grouping = match theory {
        Theory::Greedy => {
            let (topo, _count) = topo_sort::<D>(g);
            group_fibers::<D>(g, &topo)
        }
        Theory::Spectral => spectral_partition::<D>(g),
    };
    let asn = grouping.assignment.as_ref();
    let mut cut = 0usize;
    for u in 0..g.unit_count.0 {
        let start = g.row_offsets.as_ref()[u].0;
        let end = if u + 1 < g.unit_count.0 {
            g.row_offsets.as_ref()[u + 1].0
        } else {
            g.edge_count.0
        };
        for e in start..end {
            let v = g.col_indices.as_ref()[e].index().0;
            if asn[u] != asn[v] {
                cut += 1;
            }
        }
    }
    (grouping.fiber_count.0, cut)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The quality companion: prints both theories' proxies across seeds
    /// at the engine's full default capacity, for the findings memo.
    #[test]
    fn quality_proxies_across_seeds() {
        for seed in [3u64, 17, 51, 101] {
            let g = build_graph(seed, 64);
            let (gf, gc) = quality(Theory::Greedy, &g);
            let (sf, sc) = quality(Theory::Spectral, &g);
            std::println!(
                "seed {seed}: greedy fibers={gf} cut={gc}; spectral fibers={sf} cut={sc}"
            );
            assert!(gf > 0 && sf > 0, "both theories must produce fibers");
        }
    }

    /// Both theories are deterministic per (seed, units).
    #[test]
    fn groupings_are_deterministic() {
        let g = build_graph(42, 64);
        assert_eq!(
            run_theory(Theory::Greedy, &g),
            run_theory(Theory::Greedy, &g)
        );
        assert_eq!(
            run_theory(Theory::Spectral, &g),
            run_theory(Theory::Spectral, &g)
        );
    }
}
