//! Sketch (C5a / #340, Phase C): block-diagonal block detection non-trivial.
//!
//! C5a (roadmap section 9): wire `arvo_sparse::block_diagonal` to the plan graph
//! and verify real plan shapes produce a non-trivial block decomposition, so the
//! engine's block-detection (`block_diagonalise`, plan/steps.rs:335, "connected-
//! component block detection to trunk skeletons") distinguishes independent
//! components and the feasibility-error paths keyed on block structure become
//! reachable. block_diagonal returns (block_count, per_node_block_id);
//! block_count is the number of distinct connected components (undirected
//! reachability), block IDs start at 0.
//!
//! Hypothesis: the connected diamond (BranchX, BranchY -> JoinZ -> NormW, all
//! linked through JoinZ) is ONE block (block_count == 1); two independent fibers
//! sharing no column (fiber1 0->1, fiber2 2->3, no cross edge) are TWO blocks
//! (block_count == 2, distinct IDs). So block detection is non-trivial: it fires
//! >1 on multi-component plans (each component a trunk skeleton, feeding C6) and
//! collapses to 1 on a connected plan. Leeway (section 9): SOME-SHAPE. The build +
//! asserts are the test; outcome at the bottom.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::block_diagonal;
use arvo_tensor::Dim;

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

fn main() {
    type Bw = Bits<64, Hot, Unsigned>;

    // ---- Connected diamond: 0=BranchX 1=BranchY 2=JoinZ 3=NormW. All four are in
    // one connected component (JoinZ links the branches and NormW). ----
    let mut diamond: BitMatrix<Bw, Dim<4>> = BitMatrix::<Bw, Dim<4>>::empty();
    diamond.set_edge(nid(0), nid(2));
    diamond.set_edge(nid(1), nid(2));
    diamond.set_edge(nid(2), nid(3));
    let (blocks_d, _ids_d) = block_diagonal(&diamond);
    assert_eq!(blocks_d, USize(1), "connected diamond is a single block");

    // ---- Two independent fibers: 0->1 and 2->3, no edge between {0,1} and {2,3}.
    // Two disconnected components -> two blocks -> two trunk skeletons. ----
    let mut split: BitMatrix<Bw, Dim<4>> = BitMatrix::<Bw, Dim<4>>::empty();
    split.set_edge(nid(0), nid(1));
    split.set_edge(nid(2), nid(3));
    let (blocks_s, ids_s) = block_diagonal(&split);
    assert_eq!(blocks_s, USize(2), "two independent fibers are two blocks");
    let ids = ids_s.as_ref();
    // Nodes 0,1 share a block; 2,3 share the other; the two blocks differ.
    assert_eq!(ids[0], ids[1], "fiber-1 nodes co-block");
    assert_eq!(ids[2], ids[3], "fiber-2 nodes co-block");
    assert_ne!(ids[0], ids[2], "the two fibers are distinct blocks");

    // ---- Three independent isolates: no edges -> three blocks (each node alone).
    // Confirms the count tracks components, not a fixed value. ----
    let isolates: BitMatrix<Bw, Dim<3>> = BitMatrix::<Bw, Dim<3>>::empty();
    let (blocks_i, _ids_i) = block_diagonal(&isolates);
    assert_eq!(blocks_i, USize(3), "three isolates are three blocks");

    println!(
        "WORKS: block_diagonal non-trivial. Connected diamond -> 1 block; two independent fibers \
         -> 2 blocks (0,1 | 2,3 distinct); three isolates -> 3 blocks. Block detection tracks \
         connected components, distinguishing independent trunk skeletons for C6 and making the \
         block-structure feasibility paths reachable."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// arvo `block_diagonal` returns a block_count that tracks connected components:
//   - connected diamond -> 1 block (all four nodes reachable through JoinZ).
//   - two independent fibers (0->1, 2->3) -> 2 blocks; per-node IDs co-block
//     {0,1} and {2,3} into distinct components.
//   - three isolates (no edges) -> 3 blocks.
//
// WHAT THIS SETTLES (C5a): block-diagonal fires non-trivially on real plan shapes.
// The engine's `block_diagonalise` (plan/steps.rs:335) wraps this over the plan
// DependencyGraph via the to_csr_bidirectional adapter; the primitive distinguishes
// independent components, which become the trunk skeletons C6 forms, and makes the
// block-structure-keyed feasibility-error paths reachable (a plan whose detected
// block structure mismatches the expected phase alignment can be flagged). The
// `Bool::TRUE` stub in `block_diagonalise` had a real signal available all along.
//
// WHAT THIS DOES NOT SETTLE: the exact PlanError::PhaseAlignmentMismatch /
// FeasibilityCheckFailed wiring inside the engine (mechanical: compare detected
// block/phase structure against expected, error on mismatch); and the
// block-diagonal-vs-D-M division of labor (block_diagonal = components/trunks,
// D-M = column Input/Output/Internal, C5b). Both are engine wiring given these
// primitives, not open feasibility questions.
// ---------------------------------------------------------------------
