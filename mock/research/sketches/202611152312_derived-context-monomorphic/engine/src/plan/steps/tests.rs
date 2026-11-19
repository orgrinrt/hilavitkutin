//! Tests for the plan step chain.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

#[cfg(test)]
mod predecessor_mask_tests {
    use arvo::USize;
    use arvo_bitmask::BitAccess;

    use super::super::DependencyGraph;
    use super::super::step8_rank::compute_predecessor_masks;
    use crate::plan::DefaultPlanDims;

    // Diamond DAG: 0 -> {1, 2} -> 3. Predecessors: unit 0 has none, units 1
    // and 2 each have {0}, unit 3 has {1, 2}. Edges are added in
    // ascending-source order to satisfy the CSR append-order invariant; the
    // sink (unit 3) gets an explicit empty row, mirroring `build_dag`'s
    // row-fill tail.
    #[test]
    fn diamond_predecessor_masks() {
        let mut g: DependencyGraph<DefaultPlanDims> = DependencyGraph::new();
        g.add_edge(USize(0), USize(1)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test DAG literal indices; tracked: #72
        g.add_edge(USize(0), USize(2)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test DAG literal indices; tracked: #72
        g.add_edge(USize(1), USize(3)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test DAG literal indices; tracked: #72
        g.add_edge(USize(2), USize(3)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test DAG literal indices; tracked: #72
        // Sink row for unit 3 (no out-edges), as `build_dag` would fill.
        while g.unit_count.0 < 4 {
            let uc = g.unit_count.0;
            g.row_offsets.as_mut()[uc] = g.edge_count;
            g.unit_count = USize(g.unit_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test count advance; tracked: #72
        }
        let masks = compute_predecessor_masks::<DefaultPlanDims>(&g);
        let m = masks.as_ref();
        // unit 0: root, no predecessors.
        assert!(
            !m[0].bit(USize(0)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[0].bit(USize(1)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[0].bit(USize(2)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[0].bit(USize(3)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        );
        // unit 1: predecessor {0}.
        assert!(m[1].bit(USize(0)).0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        assert!(
            !m[1].bit(USize(1)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[1].bit(USize(2)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[1].bit(USize(3)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        );
        // unit 2: predecessor {0}.
        assert!(m[2].bit(USize(0)).0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        assert!(
            !m[2].bit(USize(1)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[2].bit(USize(2)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[2].bit(USize(3)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        );
        // unit 3: predecessors {1, 2}, not {0, 3}.
        assert!(m[3].bit(USize(1)).0 && m[3].bit(USize(2)).0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test indices; tracked: #72
        assert!(
            !m[3].bit(USize(0)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
                && !m[3].bit(USize(3)).0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test index; tracked: #72
        );
    }
}
