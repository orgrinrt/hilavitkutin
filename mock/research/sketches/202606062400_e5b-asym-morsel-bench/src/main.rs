//! Sketch (E5b / #340, Phase E): asymmetric morsel sizing (domain 20 :1810-1822).
//!
//! Given the P/E topology (E5a proven), the plan should give P-cores larger record
//! shares and E-cores proportionally smaller, so all cores finish together
//! (minimised makespan) instead of the fast cores idling while the slow ones
//! drain an equal share. E5b (roadmap section 9) is BENCH-decided (Kind 2):
//! correctness is trivial (any partition covers all records once); the open
//! question is whether the asymmetric split actually lowers makespan.
//!
//! macOS does not let a thread pin to a specific P or E core (no
//! sched_setaffinity; only QoS hints), so a real on-core bench is not possible
//! here. The heterogeneity is modeled with a per-record work multiplier on the
//! "E" threads: a P-thread does W units/record, an E-thread does W*SLOW units/
//! record (SLOW ~ the measured P/E speed ratio, ~2.5x on Apple Silicon). This is
//! a faithful proxy for the load-balancing CLAIM, which is about work
//! distribution vs core speed, not about the absolute on-core cost. The real
//! perf number belongs on pinned hardware (Linux sched_setaffinity) or the #664
//! suite; this bench answers "does proportional-to-speed distribution beat equal
//! distribution under heterogeneity" (the Kind-2 fork), not the absolute delta.
//!
//! Hypothesis: equal distribution (every core the same record count) has makespan
//! bounded by the SLOWEST core's share; proportional distribution (records split
//! in proportion to core speed) finishes all cores at ~the same time, lowering
//! makespan. Expect proportional makespan < equal makespan by a clear margin.
//! Outcome at the bottom.

#![allow(dead_code)]

use std::sync::Arc;
use std::thread;
use std::time::Instant;

use arvo::USize;

// Per-record synthetic work. A tight integer kernel; iters scales the cost.
#[inline(never)]
fn work(seed: u32, iters: u32) -> u32 {
    let mut x = seed;
    for _ in 0..iters {
        x = x.wrapping_mul(2654435761).wrapping_add(1);
        x ^= x >> 13;
    }
    x
}

const P_CORES: usize = 4;
const E_CORES: usize = 4;
const TOTAL_CORES: usize = P_CORES + E_CORES;
// Measured Apple-Silicon P/E throughput ratio is roughly 2-3x; model E as 1/SLOW
// the speed of P. The plan reads this from adapt metrics in the real engine.
const SLOW: u32 = 3;
const BASE_ITERS: u32 = 64;

// Run a partition: `shares[c]` = record count for core c. Core c < P_CORES is a
// P-core (BASE_ITERS/record); else an E-core (BASE_ITERS*SLOW/record). Returns
// the makespan (max per-core wall time) in nanos.
fn run_partition(shares: &[usize; TOTAL_CORES], base_seed: u32) -> u128 {
    let shares = Arc::new(*shares);
    let mut handles = Vec::new();
    for c in 0..TOTAL_CORES {
        let shares = shares.clone();
        handles.push(thread::spawn(move || {
            let iters = if c < P_CORES { BASE_ITERS } else { BASE_ITERS * SLOW };
            let n = shares[c];
            let t = Instant::now();
            let mut acc = 0u32;
            for i in 0..n {
                acc = acc.wrapping_add(work(base_seed ^ (i as u32) ^ (c as u32), iters));
            }
            std::hint::black_box(acc);
            t.elapsed().as_nanos()
        }));
    }
    let mut makespan = 0u128;
    for h in handles {
        makespan = makespan.max(h.join().unwrap());
    }
    makespan
}

fn bench_min(warmup: usize, iters: usize, shares: &[usize; TOTAL_CORES]) -> u128 {
    for _ in 0..warmup {
        run_partition(shares, 0x9e37);
    }
    let mut best = u128::MAX;
    for r in 0..iters {
        best = best.min(run_partition(shares, 0x9e37 ^ r as u32));
    }
    best
}

fn main() {
    let n_records = USize(1 << 20); // ~1M records
    let n = n_records.0;

    // EQUAL distribution: every core the same count.
    let mut equal = [0usize; TOTAL_CORES];
    let per = n / TOTAL_CORES;
    for c in 0..TOTAL_CORES {
        equal[c] = per;
    }
    equal[0] += n - per * TOTAL_CORES; // remainder onto core 0

    // PROPORTIONAL distribution: weight P-cores SLOW-times an E-core. Total weight
    // = P_CORES*SLOW + E_CORES*1; each core's share is round(n * weight_c / total).
    let total_weight = (P_CORES as u32 * SLOW + E_CORES as u32) as usize;
    let mut prop = [0usize; TOTAL_CORES];
    let mut assigned = 0usize;
    for c in 0..TOTAL_CORES {
        let w = if c < P_CORES { SLOW as usize } else { 1 };
        let share = n * w / total_weight;
        prop[c] = share;
        assigned += share;
    }
    prop[0] += n - assigned; // remainder onto a P-core

    // Both partitions must cover exactly n records (correctness is trivial but
    // assert it: no records lost or double-counted).
    assert_eq!(equal.iter().sum::<usize>(), n, "equal partition covers n");
    assert_eq!(prop.iter().sum::<usize>(), n, "proportional partition covers n");

    let warmup = 5;
    let iters = 30;
    let equal_ns = bench_min(warmup, iters, &equal);
    let prop_ns = bench_min(warmup, iters, &prop);
    let speedup = equal_ns as f64 / prop_ns as f64;

    println!(
        "equal-split makespan = {} us, proportional-split makespan = {} us, speedup = {:.3}x \
         (P={} cores @ {} iters/rec, E={} cores @ {} iters/rec, SLOW={}x, {} records)",
        equal_ns / 1000,
        prop_ns / 1000,
        speedup,
        P_CORES,
        BASE_ITERS,
        E_CORES,
        BASE_ITERS * SLOW,
        SLOW,
        n
    );
    assert!(
        speedup > 1.20,
        "proportional distribution should clearly beat equal under heterogeneity (got {speedup:.3}x)"
    );
    println!(
        "WORKS: asymmetric morsel sizing lowers makespan under P/E heterogeneity. Proportional-\
         to-speed record distribution finished ~{speedup:.2}x faster than equal distribution \
         (equal is bounded by the slow E-cores draining an oversized share). Correctness trivial \
         (both cover all records once). Real on-core delta belongs on pinned hardware / #664."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, Apple Silicon, simulated heterogeneity).
//
// Equal-split makespan ~40.7 ms vs proportional-split makespan ~22.4 ms = 1.81x
// faster, with 4 P-threads (64 iters/rec) + 4 E-threads (192 iters/rec = 3x
// slower) over ~1M records. Equal distribution is bounded by the slow E-cores
// draining an oversized equal share while the P-cores idle; proportional-to-speed
// distribution finishes all cores together. Both partitions cover all records
// exactly once (correctness trivial, asserted).
//
// WHAT THIS SETTLES (E5b, the Kind-2 bench fork): asymmetric morsel sizing
// (P-cores get a larger record share proportional to speed) clearly lowers
// makespan under P/E heterogeneity. The plan's morsel-to-core affinity (R6
// adaptive param) should weight shares by the E5a-detected core speeds. The
// mechanism is trivial (topology x weight); the perf WIN is confirmed in
// direction and rough magnitude (~1.8x at a 3x speed ratio).
//
// WHAT THIS DOES NOT SETTLE (and why): macOS cannot pin threads to specific P/E
// cores, so this models heterogeneity with a per-record work multiplier rather
// than running on real E-cores. The ABSOLUTE on-core delta (real cache/frequency
// effects, the exact weight to use) belongs on pinned hardware (Linux
// sched_setaffinity) or the #664 perf suite once the pool is real. The DIRECTION
// (proportional beats equal) is the Kind-2 decision and it is settled here.
// ---------------------------------------------------------------------
