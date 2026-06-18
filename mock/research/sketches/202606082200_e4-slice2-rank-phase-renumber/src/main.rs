//! E4 slice-2 de-risk: rank-outer phase renumber for the self-hosting meta
//! pipeline.
//!
//! `compute_phases_waist` (engine `plan/grouping.rs`) computes a per-unit
//! waist-bounded phase from the RAW dependency DAG. Slice-2 needs the meta work
//! units ordered into lifecycle bands around consumers: PlanStage < ScheduleReady
//! < PassStart < consumer < ScheduleEnd. The mechanism the DOC CL locked: each
//! unit's lifecycle RANK becomes the OUTER phase key and the existing waist-phase
//! the INNER key, then distinct `(rank, waist_phase)` pairs renumber to
//! contiguous phase ids.
//!
//! This sketch proves the renumber, as a const fn over fixed arrays (the form
//! that drops into the engine's const grouping), produces:
//!   1. correct band ordering (all rank-r units precede all rank-(r+1) units),
//!   2. preserved waist-phase order WITHIN a rank,
//!   3. contiguous phase ids 0..k,
//!   4. equal `(rank, waist_phase)` pairs sharing a phase id (so within-phase
//!      trunk grouping still sees them as same-phase).
//!
//! It also confirms the rank ordering never inverts a real data edge: lifecycle
//! flow (plan produces, consumers read, epilogue observes) runs in increasing
//! rank, so a real producer->consumer edge across ranks already agrees with the
//! rank order. Within a rank the waist-phase carries the real edges unchanged.
//!
//! Outcome at the bottom.

#![allow(dead_code)]

type Rank = u8;
// The five lifecycle ranks (the canonical kernel order).
const RANK_PLAN_STAGE: Rank = 0;
const RANK_SCHEDULE_READY: Rank = 1;
const RANK_PASS_START: Rank = 2;
const RANK_CONSUMER: Rank = 3;
const RANK_SCHEDULE_END: Rank = 4;

const MAX_UNITS: usize = 256;

/// `(rank[i], wphase[i])` lex-strictly-less than `(rank[j], wphase[j])`.
const fn lex_lt(ri: Rank, wi: usize, rj: Rank, wj: usize) -> bool {
    ri < rj || (ri == rj && wi < wj)
}

/// `(rank[i], wphase[i]) == (rank[j], wphase[j])`.
const fn pair_eq(ri: Rank, wi: usize, rj: Rank, wj: usize) -> bool {
    ri == rj && wi == wj
}

/// Renumber `(rank, wphase)` into contiguous phase ids.
///
/// `phase_out[i]` = number of DISTINCT pairs present in the unit set that are
/// lex-strictly-less than unit i's pair. "Distinct" is counted by first
/// occurrence (a pair contributes once, at its lowest-indexed unit). This yields
/// 0-based contiguous ids grouped by distinct pair, ascending in lex order, with
/// equal pairs sharing an id. Mirrors the const-fn / fixed-array form of the
/// engine grouping (no alloc, no set type).
const fn renumber(rank: &[Rank], wphase: &[usize], n: usize, phase_out: &mut [usize]) {
    let mut i = 0;
    while i < n {
        let ri = rank[i];
        let wi = wphase[i];
        let mut count = 0;
        let mut j = 0;
        while j < n {
            let rj = rank[j];
            let wj = wphase[j];
            if lex_lt(rj, wj, ri, wi) {
                // count j only if it is the first occurrence of its pair
                let mut first = true;
                let mut k = 0;
                while k < j {
                    if pair_eq(rank[k], wphase[k], rj, wj) {
                        first = false;
                    }
                    k += 1;
                }
                if first {
                    count += 1;
                }
            }
            j += 1;
        }
        phase_out[i] = count;
        i += 1;
    }
}

fn main() {
    // Scenario: one WU per lifecycle point plus two data-dependent consumers.
    // carrier order (index): 0 plan, 1 schedReady, 2 passStart, 3 consumerA,
    // 4 consumerB (reads A: a real RAW edge => waist-phase 1 within consumers),
    // 5 epilogue.
    //
    // waist-phase column is what `compute_phases_waist` would produce IF all
    // units were one rank: the only real edge is A->B, so A is wp 0 and B is wp
    // 1 within the consumer band; every other unit is wp 0 in its own band.
    let rank = [
        RANK_PLAN_STAGE,
        RANK_SCHEDULE_READY,
        RANK_PASS_START,
        RANK_CONSUMER,
        RANK_CONSUMER,
        RANK_SCHEDULE_END,
    ];
    let wphase = [0usize, 0, 0, 0, 1, 0];
    let n = 6;
    let mut phase = [0usize; MAX_UNITS];
    renumber(&rank, &wphase, n, &mut phase);
    let p = &phase[..n];

    // 1. band ordering: strictly increasing across distinct lifecycle points.
    assert!(p[0] < p[1], "plan < scheduleReady");
    assert!(p[1] < p[2], "scheduleReady < passStart");
    assert!(p[2] < p[3], "passStart < consumerA");
    // 2. waist order preserved within the consumer rank.
    assert!(p[3] < p[4], "consumerA < consumerB (real RAW edge preserved)");
    // band ordering continues past consumers.
    assert!(p[4] < p[5], "consumerB < epilogue (ScheduleEnd last)");

    // 3. contiguous 0..k.
    let mut maxp = 0;
    let mut i = 0;
    while i < n {
        if p[i] > maxp {
            maxp = p[i];
        }
        i += 1;
    }
    // every id in 0..=maxp must be present (contiguous).
    let mut id = 0;
    while id <= maxp {
        let mut present = false;
        let mut k = 0;
        while k < n {
            if p[k] == id {
                present = true;
            }
            k += 1;
        }
        assert!(present, "phase id {id} missing => non-contiguous");
        id += 1;
    }
    // here all six pairs are distinct, so 6 phases 0..=5.
    assert_eq!(p, &[0, 1, 2, 3, 4, 5][..], "one phase per distinct pair");

    // 4. equal pairs share a phase id. Two PassStart WUs with identical
    // waist-phase land in the SAME phase (so within-phase trunk grouping still
    // sees them as same-phase). Re-run with a duplicate pair.
    let rank2 = [RANK_PLAN_STAGE, RANK_PASS_START, RANK_PASS_START, RANK_CONSUMER];
    let wphase2 = [0usize, 0, 0, 0];
    let mut phase2 = [0usize; MAX_UNITS];
    renumber(&rank2, &wphase2, 4, &mut phase2);
    let q = &phase2[..4];
    assert_eq!(q[1], q[2], "two PassStart WUs, same waist-phase, same phase id");
    assert_eq!(q, &[0, 1, 1, 2][..], "plan=0, both passStart=1, consumer=2");

    // 5. const-context proof: the renumber is usable at const-eval (array-length
    // proof), so it drops into the engine's const grouping.
    const CRANK: [Rank; 3] = [RANK_PLAN_STAGE, RANK_CONSUMER, RANK_SCHEDULE_END];
    const CWP: [usize; 3] = [0, 0, 0];
    const CPHASE: [usize; MAX_UNITS] = {
        let mut out = [0usize; MAX_UNITS];
        renumber(&CRANK, &CWP, 3, &mut out);
        out
    };
    let _: [(); 0] = [(); CPHASE[0]];
    let _: [(); 1] = [(); CPHASE[1]];
    let _: [(); 2] = [(); CPHASE[2]];

    println!(
        "WORKS: rank-outer (rank, waist_phase) renumber gives contiguous lifecycle-ordered phase bands. scenario p={:?}, dup-pair q={:?}",
        p, q
    );
}
