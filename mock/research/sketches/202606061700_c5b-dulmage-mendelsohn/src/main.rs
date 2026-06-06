//! Sketch (C5b / #340, Phase C): Dulmage-Mendelsohn non-trivial on real workloads.
//!
//! D4 (sketch 202606061600) proved internal-column fusion is the #664 enabler and
//! is worth ~2x. But fusion needs C7 to CLASSIFY columns Input/Output/Internal,
//! and C7 derives that from the plan's Dulmage-Mendelsohn decomposition (canonical
//! domain: D-M horizontal/vertical/square -> Output/Input/Internal). Today
//! `classify_columns` marks every store Internal (a stub). C5b's load-bearing
//! question (roadmap section 9): does arvo's D-M produce a NON-TRIVIAL
//! decomposition on the real gate workloads, so the split is real and at least one
//! column is non-Internal? If every workload collapses to all-square (core), C7 is
//! a no-op and D4's fusion never materialises.
//!
//! D-M (arvo-sparse, ff514a7) classifies each NODE of a directed graph:
//!   class 0 = horizontal (sink:   incoming edges, no outgoing)
//!   class 1 = vertical   (source/isolate: no incoming)
//!   class 2 = square     (core:   both incoming and outgoing)
//! `class_count` is ALWAYS 3 (carried for parity), so non-triviality is measured
//! by the DISTINCT classes actually used among the nodes, not by class_count.
//!
//! The plan's WU-DAG has an edge a->b when b reads a column a writes. A WU's class
//! fixes its columns: a SOURCE WU reads input columns (Input) and writes columns
//! consumed downstream (Internal); a CORE WU's writes are consumed downstream
//! (Internal); a SINK WU's writes are drained out (Output). A non-trivial node
//! decomposition gives a non-trivial column split.
//!
//! Hypothesis: the diamond (BranchX, BranchY -> JoinZ -> NormW) uses all 3 classes
//! (sources V, core S, sink H); the linear accumulator gate (S1 -> Tally) uses 2
//! (source + sink); an isolated node uses 1 (vertical). So the diamond column
//! split is Input (In) + Internal (Xv, Yv, Zv) + Output (Wv), NOT all-Internal.
//! Leeway (section 9): EXACT for "non-trivial output exists"; SOME-SHAPE for the
//! node->column mapping. Build + asserts are the test; outcome at the bottom.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_sparse::dulmage_mendelsohn;
use arvo_tensor::{Capacity, Dim};

const H: USize = USize(0); // horizontal: sink
const V: USize = USize(1); // vertical: source / isolate
const S: USize = USize(2); // square: core

fn nid(i: usize) -> NodeId {
    NodeId::new(USize(i))
}

// Count distinct class IDs actually used among the first `n` nodes. This is the
// real non-triviality signal (class_count is always 3 for this algorithm).
fn distinct_used<C: Capacity>(dm: &arvo_sparse::DulmageMendelsohn<C>, n: usize) -> usize {
    let cls = dm.class.as_ref();
    let mut seen = [false; 3];
    for i in 0..n {
        let c = cls[i].0;
        if c < 3 {
            seen[c] = true;
        }
    }
    seen.iter().filter(|&&b| b).count()
}

fn main() {
    type Bw = Bits<64, Hot, Unsigned>;

    // ---- Workload 1: the diamond. Nodes 0=BranchX 1=BranchY 2=JoinZ 3=NormW.
    // Edges (writer -> reader): BranchX->JoinZ (Xv), BranchY->JoinZ (Yv),
    // JoinZ->NormW (Zv). ----
    let mut diamond: BitMatrix<Bw, Dim<4>> = BitMatrix::<Bw, Dim<4>>::empty();
    diamond.set_edge(nid(0), nid(2)); // BranchX -> JoinZ
    diamond.set_edge(nid(1), nid(2)); // BranchY -> JoinZ
    diamond.set_edge(nid(2), nid(3)); // JoinZ   -> NormW

    let dm = dulmage_mendelsohn(&diamond);
    let dc = dm.class.as_ref();
    assert_eq!(dc[0], V, "BranchX is a source (vertical)");
    assert_eq!(dc[1], V, "BranchY is a source (vertical)");
    assert_eq!(dc[2], S, "JoinZ is the core (square)");
    assert_eq!(dc[3], H, "NormW is the sink (horizontal)");
    assert_eq!(distinct_used(&dm, 4), 3, "diamond uses all 3 classes (non-trivial)");

    // Node classes -> column classes for the diamond:
    //   In : read by sources from outside           -> INPUT
    //   Xv : BranchX(source) -> JoinZ(core)          -> INTERNAL
    //   Yv : BranchY(source) -> JoinZ(core)          -> INTERNAL
    //   Zv : JoinZ(core)     -> NormW(sink)          -> INTERNAL
    //   Wv : NormW(sink), drained out                -> OUTPUT
    // Non-trivial: 1 Input + 3 Internal + 1 Output. There ARE internal columns
    // (Xv, Yv, Zv) for D4 to fuse.

    // ---- Workload 2: linear accumulator gate. 0=S1 1=Tally. Edge S1->Tally. ----
    let mut linear: BitMatrix<Bw, Dim<2>> = BitMatrix::<Bw, Dim<2>>::empty();
    linear.set_edge(nid(0), nid(1)); // S1 -> Tally
    let dm2 = dulmage_mendelsohn(&linear);
    let dc2 = dm2.class.as_ref();
    assert_eq!(dc2[0], V, "S1 is a source");
    assert_eq!(dc2[1], H, "Tally is a sink");
    assert_eq!(distinct_used(&dm2, 2), 2, "linear gate uses 2 classes (non-trivial)");

    // ---- Negative control: an isolated node uses ONLY the vertical class. So
    // D-M DOES collapse on a trivial graph; the non-trivial results above are real,
    // not an always-3 artifact. ----
    let isolated: BitMatrix<Bw, Dim<1>> = BitMatrix::<Bw, Dim<1>>::empty();
    let dm3 = dulmage_mendelsohn(&isolated);
    assert_eq!(dm3.class.as_ref()[0], V, "isolated node is vertical");
    assert_eq!(distinct_used(&dm3, 1), 1, "isolated node uses 1 class (degenerate)");

    println!(
        "WORKS: D-M non-trivial on real workloads. Diamond -> 2 sources(V) + 1 core(S) + 1 \
         sink(H), 3 distinct classes; column split = 1 Input + 3 Internal + 1 Output (NOT \
         all-Internal). Linear accumulator gate -> source + sink (2 classes). Isolated node -> \
         vertical only (degenerate collapses to 1), so the non-trivial result is real. C7 \
         classification + D4 fusion have a real Internal set to work on."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// arvo's `dulmage_mendelsohn` produces a NON-TRIVIAL decomposition on the real
// gate workloads:
//   - Diamond (BranchX, BranchY -> JoinZ -> NormW): node classes V, V, S, H -> 3
//     distinct classes used. Maps to columns: In = Input, Xv/Yv/Zv = Internal,
//     Wv = Output. NOT all-Internal; three internal columns for D4 to fuse.
//   - Linear accumulator gate (S1 -> Tally): V, H -> 2 distinct classes.
//   - Isolated node: V only -> 1 class. The negative control: D-M genuinely
//     collapses on a trivial graph, so the non-trivial results are real (not an
//     always-3 artifact; class_count is always 3 but distinct-USED varies 1/2/3).
//
// WHAT THIS SETTLES (C5b, the fusion precondition): the load-bearing worry (every
// workload collapses to all-square, making C7 a no-op and D4's fusion vacuous) is
// REFUTED. Real workloads produce a genuine source/core/sink split, which maps to
// a real Input/Internal/Output column classification. So C7 (classify_columns
// from D-M, currently a stub marking everything Internal) has a real signal to
// compute, and D4's internal-column fusion (proven 2.09x, 202606061600) has a
// real Internal set. The fusion path is unblocked end to end.
//
// WHAT THIS DOES NOT SETTLE: the exact node-class -> column-class MAPPING in C7
// (reasoned here, not yet an algorithm); and C5a (block_diagonal wiring + reachable
// feasibility-error path), the sibling. Both are mechanical given this non-trivial
// D-M output; neither is an open feasibility question after this result.
// ---------------------------------------------------------------------
