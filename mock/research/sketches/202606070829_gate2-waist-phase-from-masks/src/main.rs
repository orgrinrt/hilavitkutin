//! Sketch (GATE-2 chart S2 / R2-pre): can the engine derive the CANONICAL
//! waist-bounded phase axis at const time from per-unit access masks, via arvo's
//! `waist_detect_const`?
//!
//! Context: the shipped engine const grouping (`plan/grouping.rs::compute_phases`)
//! computes longest-read-after-write DEPTH and mislabels it "phase". The
//! course-correction established that the canonical phase axis is WAIST-bounded,
//! not depth (the runtime `compute_waists`, plan/steps.rs:252-327, is the
//! authoritative mapping: phase 0 starts at position 0, and each waist position
//! `p` opens a new phase at `p+1`, so the waist unit is the last of its phase).
//! R2 replaces the depth phase axis with a waist-bounded one fed by arvo's new
//! const `waist_detect_const` (R1b).
//!
//! This sketch proves the genuinely-new R2 chain const-evaluates end to end over
//! real arvo types:
//!   per-unit access masks
//!     -> unit x unit adjacency `[W; N]` (bit j of row i set iff reads[j]
//!        overlaps writes[i], i.e. RAW edge i -> j), built with const `BitLogic`
//!        / `BitSequence` (overlap) + const `BitAccess` (set)
//!     -> `waist_detect_const::<Dim<N>, W>(&adj, &identity_order)` -> waist flags
//!     -> canonical phase per position = count of waist flags strictly before it.
//!
//! The W word is `Bits<64, Hot, Unsigned>`, the engine's `D::AdjRow` default and
//! the exact `W: [const] BitAccess` R1b already proved const-evaluable. So this
//! sketch's risk is purely the mask->adjacency->phase composition, not the bit
//! word.
//!
//! Fixture is an hourglass where waist-phase and depth-phase DISAGREE, so a pass
//! proves the axis actually changed (not that the two happen to coincide).
//!
//! Outcome at the bottom + in FINDINGS.md.

#![allow(dead_code)]
#![feature(const_trait_impl)]

use arvo::{Bits, Bool, Hot, Unsigned};
use arvo::strategy::Identity;
use arvo::USize;
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_bitmask::NodeId;
use arvo_graph::waist_detect_const;
use arvo_tensor::Dim;

// Per-unit access mask over columns (6 columns: A..F at bits 0..5).
type Col = Bits<8, Hot, Unsigned>;
// Unit x unit adjacency row (one bit per unit, up to 64). The engine's
// `D::AdjRow` default; the const `BitAccess` word R1b proved.
type Adj = Bits<64, Hot, Unsigned>;

const N: usize = 6;

// Build a mask with the bits in `targets` set, const, via the const BitAccess
// contract. `targets` is a small bitmask of column / unit indices.
const fn mk_col(targets: u64) -> Col {
    let mut w = <Col as Identity>::ZERO;
    let mut j = 0usize;
    while j < 8 {
        if (targets >> j) & 1 == 1 {
            w = w.with_bit_set(USize(j));
        }
        j += 1;
    }
    w
}

// Two masks overlap iff their bitwise AND is non-zero. The const analog of the
// shipped `AccessMask::overlaps` (which composes the same const BitLogic +
// BitSequence contracts).
const fn overlaps(a: Col, b: Col) -> bool {
    !BitLogic::bitand(a, b).is_zero().0
}

// Unit x unit adjacency: row i has bit j set iff unit j reads a column unit i
// writes (RAW edge i -> j). Self-edges excluded.
const fn build_adj(reads: &[Col; N], writes: &[Col; N]) -> [Adj; N] {
    let mut adj = [<Adj as Identity>::ZERO; N];
    let mut i = 0usize;
    while i < N {
        let mut j = 0usize;
        while j < N {
            if i != j && overlaps(reads[j], writes[i]) {
                adj[i] = adj[i].with_bit_set(USize(j));
            }
            j += 1;
        }
        i += 1;
    }
    adj
}

// Canonical waist -> phase mapping (runtime compute_waists, steps.rs:314-326):
// phase 0 starts at position 0; each waist position opens a new phase at the
// next position. So phase[k] = number of waist flags strictly before k.
const fn phases_from_flags(flags: &[Bool; N]) -> [usize; N] {
    let mut ph = [0usize; N];
    let mut count = 0usize;
    let mut k = 0usize;
    while k < N {
        ph[k] = count;
        if flags[k].0 {
            count += 1;
        }
        k += 1;
    }
    ph
}

// Hourglass over columns A=0,B=1,C=2,D=3,E=4,F=5. Registration order
// U0..U5 is a valid topo order (every producer precedes its consumers).
//   U0: write A
//   U1: read A, write B      U2: read A, write C      (parallel, depth 1)
//   U3: read B,C, write D                              (the waist, depth 2)
//   U4: read D, write E      U5: read D, write F       (parallel, depth 3)
// RAW edges: 0->1,0->2, 1->3,2->3, 3->4,3->5.
// Level widths by depth: d0=1, d1=2, d2=1, d3=2 -> depth 2 (U3) is a strict
// local minimum = the one waist, at topo position 3.
const READS: [Col; N] = [
    mk_col(0),          // U0
    mk_col(1 << 0),     // U1 reads A
    mk_col(1 << 0),     // U2 reads A
    mk_col((1 << 1) | (1 << 2)), // U3 reads B,C
    mk_col(1 << 3),     // U4 reads D
    mk_col(1 << 3),     // U5 reads D
];
const WRITES: [Col; N] = [
    mk_col(1 << 0),     // U0 writes A
    mk_col(1 << 1),     // U1 writes B
    mk_col(1 << 2),     // U2 writes C
    mk_col(1 << 3),     // U3 writes D
    mk_col(1 << 4),     // U4 writes E
    mk_col(1 << 5),     // U5 writes F
];

const ADJ: [Adj; N] = build_adj(&READS, &WRITES);

const ORDER: [NodeId; N] = [
    NodeId(USize(0)),
    NodeId(USize(1)),
    NodeId(USize(2)),
    NodeId(USize(3)),
    NodeId(USize(4)),
    NodeId(USize(5)),
];

// The whole chain, forced through const evaluation.
const FLAGS: [Bool; N] = waist_detect_const::<Dim<N>, Adj>(&ADJ, &ORDER);
const PHASES: [usize; N] = phases_from_flags(&FLAGS);

// The depth axis the shipped grouping computes today, for contrast.
const fn depth_phases(reads: &[Col; N], writes: &[Col; N]) -> [usize; N] {
    let mut d = [0usize; N];
    let mut pass = 0usize;
    while pass < N {
        let mut j = 0usize;
        while j < N {
            let mut i = 0usize;
            while i < N {
                if i != j && overlaps(reads[j], writes[i]) {
                    let cand = d[i] + 1;
                    if d[j] < cand {
                        d[j] = cand;
                    }
                }
                i += 1;
            }
            j += 1;
        }
        pass += 1;
    }
    d
}
const DEPTH: [usize; N] = depth_phases(&READS, &WRITES);

fn main() {
    println!("waist flags  = {:?}", FLAGS.map(|b| b.0));
    println!("waist phases = {:?}", PHASES);
    println!("depth phases = {:?}", DEPTH);

    // Exactly one waist, at U3's topo position (3).
    assert_eq!(FLAGS.map(|b| b.0), [false, false, false, true, false, false]);

    // Canonical waist-bounded phase: U0..U3 in phase 0 (U3 the waist is the LAST
    // of phase 0), U4/U5 in phase 1.
    assert_eq!(PHASES, [0, 0, 0, 0, 1, 1], "canonical waist-bounded phase axis");

    // The waist axis is genuinely different from the depth axis the shipped
    // grouping computes (else the sketch would prove nothing).
    assert_eq!(DEPTH, [0, 1, 1, 2, 3, 3], "depth axis, for contrast");
    assert_ne!(PHASES, DEPTH, "waist-bounded phase must differ from depth");

    println!("S2 SKETCH: WORKS");
}
