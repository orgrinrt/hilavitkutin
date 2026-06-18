//! Sketch (GATE-2 chart S3 / R3a-pre): can the engine enumerate the distinct
//! (phase, trunk) pairs of a grouping at compile time and dispatch one
//! `run_one_trunk::<PHASE, TRUNK>` mono per pair, over the FLAT carrier, without
//! the forbidden type-level partition, without a mono explosion, and without a
//! `{N+1}` const-generic-recursion overflow?
//!
//! Context: R3a re-points `Scheduler::run` to the canonical per-(phase,trunk)
//! dispatcher. op's mandate "single-core = the 1-core degenerate, NO special
//! path" means single-core must use the SAME structured dispatcher as N-core
//! (one core runs ALL (phase,trunk) monos in phase order), so the (phase,trunk)
//! enumeration is on R3a's critical path, not deferrable. The `PhaseCons` /
//! `TrunkCons` value-nest cannot be the vehicle (building it from the flat
//! carrier needs a forbidden N-way type-level partition). So the enumeration
//! must run over the flat carrier via `run_one_trunk::<PHASE,TRUNK>`.
//!
//! The candidate mechanism (this sketch):
//!   1. The const grouping yields a const `PAIRS: [(usize, usize); K]` of the
//!      distinct (phase, trunk) pairs in phase order (K = trunk count, NOT
//!      GATE2_MAX_UNITS, so no mono explosion).
//!   2. A const-generic recursion enumerates `PAIRS[0..K]`, calling
//!      `run_one::<{PAIRS[I].0}, {PAIRS[I].1}>` per pair. Termination is the
//!      bool-dispatch pattern: `RunStep<const I, const CONT: bool>` with a base
//!      impl (`CONT = false`, I past the end) and a recursive impl (`CONT =
//!      true`) that recurses to `RunStep<{I+1}, {I+1 < K}>`. Monomorphisation is
//!      finite (instantiates I = 0..=K), so no `{N+1}` runaway.
//!   3. `{I+1}` and `{PAIRS[I].0}` / `{PAIRS[I].1}` are GCE in const-generic
//!      position (the engine enables `generic_const_exprs`); the `{POS+1}` form
//!      already worked clean in sketches 070800 / 071230 (no cap_size WF bound).
//!
//! `run_one::<P,T>` here uses a runtime compare for the membership gate; the
//! compile-time-`const{}`-gated DCE that collapses each mono to member-only is
//! already proven (sketch 071230). What is unproven and what THIS sketch must
//! show: the ENUMERATION recursion compiles + monomorphizes + dispatches each
//! unit exactly once in phase-grouped order (output-equivalent to the flat
//! walk).
//!
//! Outcome at the bottom + in FINDINGS.md.

#![feature(generic_const_exprs)]
#![allow(incomplete_features, dead_code)]

// Hourglass fixture: 6 units, (phase, trunk) per unit id (the R2 grouping's
// output for the hourglass: phase [0,0,0,0,1,1], trunk [0,0,0,0,4,5]).
const UNITS: [(usize, usize); 6] = [
    (0, 0), // U0
    (0, 0), // U1
    (0, 0), // U2
    (0, 0), // U3
    (1, 4), // U4
    (1, 5), // U5
];
const NUNITS: usize = 6;

// Distinct (phase, trunk) pairs in phase order. In the engine these come from a
// const grouping fn; here they are the hand-computed distinct pairs of UNITS.
// K = 3 (three trunks), NOT NUNITS^2: no mono explosion.
const PAIRS: [(usize, usize); 3] = [(0, 0), (1, 4), (1, 5)];
const NPAIRS: usize = 3;

// One trunk's member-only program: walk the flat carrier, run (log) every unit
// whose (phase, trunk) == (PHASE, TRUNK), in carrier order. Models
// `run_one_trunk::<PHASE,TRUNK>`; the const-gated DCE form is sketch 071230.
fn run_one<const PHASE: usize, const TRUNK: usize>(log: &mut Vec<usize>) {
    let mut i = 0;
    while i < NUNITS {
        if UNITS[i].0 == PHASE && UNITS[i].1 == TRUNK {
            log.push(i);
        }
        i += 1;
    }
}

// Alt B: free-fn const-generic recursion with an in-body bound guard. Each
// instantiation runs pair I's mono, then recurses to I+1 only while I+1 < NPAIRS.
// The recursion is a fn call (no trait where-clause), so termination is by the
// `if` guard rather than abstract impl WF.
fn run_step<const I: usize>(log: &mut Vec<usize>)
where
    [(); NPAIRS - I]:,
{
    run_one::<{ PAIRS[I].0 }, { PAIRS[I].1 }>(log);
    if I + 1 < NPAIRS {
        run_step::<{ I + 1 }>(log);
    }
}

// Entry: dispatch every (phase, trunk) pair in phase order.
fn run_pairs(log: &mut Vec<usize>) {
    if NPAIRS > 0 {
        run_step::<0>(log);
    }
}

// The flat-walk reference: every unit once, in carrier order.
fn flat_walk(log: &mut Vec<usize>) {
    let mut i = 0;
    while i < NUNITS {
        log.push(i);
        i += 1;
    }
}

fn main() {
    let mut paired = Vec::new();
    run_pairs(&mut paired);
    println!("pairs-enumerated order = {paired:?}");

    let mut flat = Vec::new();
    flat_walk(&mut flat);
    println!("flat-walk order        = {flat:?}");

    // Each unit dispatched exactly once.
    assert_eq!(paired.len(), NUNITS, "every unit runs exactly once");

    // Phase-grouped order: pair (0,0) -> U0..U3, (1,4) -> U4, (1,5) -> U5.
    assert_eq!(paired, [0, 1, 2, 3, 4, 5], "phase-then-trunk grouped order");

    // Output-equivalent to the flat walk (here the orders coincide because
    // waist-phase is monotonic in carrier order; the enumeration runs the same
    // units the same number of times).
    assert_eq!(paired, flat, "pairs-enumeration is output-equivalent to flat walk");

    println!("S3 SKETCH: WORKS");
}
