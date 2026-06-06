//! Sketch (E7 / #340, Phase E): dirty propagation -> incremental skip (Step 9).
//!
//! Consolidation Step 9 (:1418-1429): a runtime dirty-bit propagation pass
//! (`predecessors[N] & dirty_mask`, arvo-bitmask :191-194) skips clean WUs,
//! turning the pipeline from a batch processor (re-run everything) into an
//! incremental processor. The plan computes per-WU predecessor masks at plan time
//! (`compute_upward_rank_and_dirty`); E7 is the per-frame runtime consumption: given
//! which inputs changed this frame, mark every transitively-affected WU dirty and
//! SKIP the clean ones.
//!
//! Hypothesis (roadmap section 9): propagating a seed dirty set forward over the
//! predecessor masks (a WU is dirty if directly seeded OR any predecessor is dirty)
//! in topo order marks exactly the transitive descendants of the changed inputs;
//! the dispatch then runs only the dirty WUs and skips the rest. Asserted against
//! a hand-checked DAG: a change to one input dirties only its cone; an unrelated
//! subgraph stays clean and is skipped; a full-clean frame runs nothing; a
//! root-input change runs everything. Modeled with u64 bitsets (the engine's
//! arvo_bitmask::Mask64 wraps the same bit ops). Leeway (section 9): SOME-SHAPE.
//! Outcome at the bottom.

#![allow(dead_code)]

use arvo::USize;

// A plan WU node: its predecessor set as a bitmask (bit p set => WU p is a direct
// predecessor). The DAG below; bit i = WU i.
//
//   inputs:   0 (InA), 1 (InB), 2 (InC)   [no predecessors; seeded directly]
//   3 = derive from 0        (preds: {0})
//   4 = derive from 3        (preds: {3})       -> cone of InA: {3,4}
//   5 = join 1 and 4         (preds: {1,4})     -> depends on InB and the InA cone
//   6 = derive from 2        (preds: {2})       -> independent cone of InC: {6}
const N: usize = 7;
fn predecessor_masks() -> [u64; N] {
    let mut p = [0u64; N];
    p[3] = 1 << 0;
    p[4] = 1 << 3;
    p[5] = (1 << 1) | (1 << 4);
    p[6] = 1 << 2;
    p
}

// Topo order of the DAG (predecessors before dependents). 0,1,2 inputs first.
const TOPO: [usize; N] = [0, 1, 2, 3, 4, 6, 5];

// The per-frame dirty propagation pass. `seed` = inputs changed this frame (bit
// set). Returns the dirty mask: a WU is dirty iff seeded or any predecessor dirty.
// Walk in topo order so predecessors are decided before dependents (one pass).
fn propagate(seed: u64, preds: &[u64; N]) -> u64 {
    let mut dirty = seed;
    for &i in TOPO.iter() {
        // predecessors[i] & dirty != 0  =>  WU i is dirty.
        if preds[i] & dirty != 0 {
            dirty |= 1 << i;
        }
    }
    dirty
}

// Simulate the dispatch consuming the dirty mask: run only dirty WUs, skip clean.
// Returns (ran_set, skipped_set) as bitmasks.
fn dispatch_skip(dirty: u64) -> (u64, u64) {
    let mut ran = 0u64;
    let mut skipped = 0u64;
    for i in 0..N {
        if dirty & (1 << i) != 0 {
            ran |= 1 << i;
        } else {
            skipped |= 1 << i;
        }
    }
    (ran, skipped)
}

fn bit(i: usize) -> u64 {
    1 << i
}
fn set(bits: &[usize]) -> u64 {
    bits.iter().fold(0u64, |a, &i| a | bit(i))
}

fn main() {
    let preds = predecessor_masks();
    let _ = USize(N); // tie record/unit count to the stack primitive

    // Case 1: only InA (0) changed. Cone = {0, 3, 4} and 5 (joins 4). 1,2,6 clean.
    let d = propagate(bit(0), &preds);
    let (ran, skipped) = dispatch_skip(d);
    assert_eq!(ran, set(&[0, 3, 4, 5]), "InA change dirties its cone incl the join");
    assert_eq!(skipped, set(&[1, 2, 6]), "unrelated WUs (InB, InC, InC-cone) skipped");

    // Case 2: only InC (2) changed. Cone = {2, 6}. Everything else clean.
    let d = propagate(bit(2), &preds);
    let (ran, skipped) = dispatch_skip(d);
    assert_eq!(ran, set(&[2, 6]), "InC change dirties only its independent cone");
    assert_eq!(skipped, set(&[0, 1, 3, 4, 5]), "the InA/InB subgraph stays clean");

    // Case 3: nothing changed. Incremental processor runs NOTHING.
    let d = propagate(0, &preds);
    let (ran, _) = dispatch_skip(d);
    assert_eq!(ran, 0, "a fully-clean frame runs no WUs (pure incremental skip)");

    // Case 4: InB (1) changed. Only 5 (join of 1 and 4) is downstream. 5 runs,
    // but NOT the InA cone (3,4) since InA is clean; the join runs because ONE of
    // its predecessors (InB) is dirty.
    let d = propagate(bit(1), &preds);
    let (ran, skipped) = dispatch_skip(d);
    assert_eq!(ran, set(&[1, 5]), "InB change dirties the join but not the clean InA cone");
    assert_eq!(skipped, set(&[0, 2, 3, 4, 6]), "InA cone + InC cone skipped");

    // Case 5: a root input + everything seeded = full re-run (batch fallback).
    let d = propagate(set(&[0, 1, 2]), &preds);
    let (ran, skipped) = dispatch_skip(d);
    assert_eq!(ran, set(&[0, 1, 2, 3, 4, 5, 6]), "all inputs changed -> full run");
    assert_eq!(skipped, 0, "nothing skipped when every input changed");

    println!(
        "WORKS: dirty-mask incremental skip. predecessors[N] & dirty propagation over the DAG \
         marks exactly the transitive cone of changed inputs each frame: InA->{{0,3,4,5}}, \
         InC->{{2,6}} (independent cone), InB->{{1,5}} (join only), clean-frame->{{}} (runs \
         nothing), all-inputs->full run. The dispatch skips clean WUs; the pipeline is an \
         incremental processor, not a batch one."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// The dirty propagation (predecessors[i] & dirty != 0 in topo order) marks exactly
// the transitive cone of changed inputs each frame, and the dispatch runs only
// dirty WUs: InA-change -> {0,3,4,5} (its cone + the join), InC-change -> {2,6}
// (independent cone, InA/InB subgraph stays clean), InB-change -> {1,5} (the join
// fires because ONE predecessor is dirty, but the clean InA cone does not),
// clean-frame -> {} (runs nothing), all-inputs -> full run. All hand-checked
// cases pass.
//
// WHAT THIS SETTLES (E7): the canonical Step-9 dirty-bit propagation + incremental
// skip works as a per-frame runtime pass over the plan-computed predecessor masks.
// One topo-order pass decides every WU (predecessors before dependents), and the
// dispatch skips clean WUs, making the engine an incremental processor (re-run
// only the cone of changed inputs) rather than a batch one. The bit ops are u64
// here; the engine uses arvo_bitmask::Mask64 (same ops, wider when N>64 via the
// multi-limb Mask). Fires within the E1 frame walk, gated by the E4 meta loop.
//
// WHAT THIS DOES NOT SETTLE: the plan-time computation of the predecessor masks
// (compute_upward_rank_and_dirty, shipped) and the change-DETECTION that seeds the
// dirty set (which inputs a frame actually mutated, a consumer/Resource-write
// signal); this proves the propagation + skip CONSUMPTION, the load-bearing part.
// ---------------------------------------------------------------------
