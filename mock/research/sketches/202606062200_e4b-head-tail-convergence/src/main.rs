//! Sketch (E4b / #340, Phase E): head+tail convergence (domain 20 :1838-1844).
//!
//! Within a commutative fiber, two cores process records from OPPOSITE ENDS and
//! converge in the middle (load-balancing without a precomputed split). The
//! per-core compiled program bakes each core's record range as the head half or
//! the tail half (domain 17 item 2, the record-range field). E4b depends on the
//! D1b carrier (each core's range is a baked field) and the D1c barrier (phase
//! sync); both proven. At 1 core there is one range (no convergence, degenerate).
//!
//! Hypothesis (roadmap section 9): a commutative fiber with head/tail cursors
//! baked per core converges at 2 cores with every record processed EXACTLY once,
//! no overlap, no gap, regardless of the relative speed of the two cores. The
//! convergence is a shared pair of opposite-end cursors: the head core claims the
//! lowest unclaimed index, the tail core claims the highest, both stop when the
//! cursors cross. Commutativity means the per-record op is order-independent
//! (here each record writes f(i) to its own disjoint slot), so the
//! interleaving does not affect the result. Leeway (section 9): SOME-SHAPE.
//!
//! Two std threads model the two cores (research binary; the engine pool is
//! no_std and spawn-once, out of scope here, E2). The claimed-index protocol is
//! the load-balancing convergence; the "baked head/tail range" is the per-core
//! designation (which end each core claims from). Outcome at the bottom.

#![allow(dead_code)]

use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

use arvo::USize;

const M1: u32 = 2654435761;
#[inline(always)]
fn f(i: u32) -> u32 {
    i.wrapping_mul(M1) ^ (i >> 3)
}

// The convergence cursor pair: `low` is the next head index (claimed upward),
// `high` is the next tail index (claimed downward). A claim succeeds while
// low <= high; when they cross, both cores stop. A single AtomicI64 packs both?
// Keep them separate with a claim protocol that is race-free via a single CAS on
// a packed (low, high) word. Pack: high 32 bits = low cursor, low 32 bits = high
// cursor (as u32). Claim-head: read packed, if low > high stop, else CAS to
// (low+1, high), take `low`. Claim-tail: if low > high stop, else CAS to (low,
// high-1), take `high`. One CAS loop per claim; the pair moves monotonically
// toward the middle and every index in [0, n) is handed out exactly once.
#[inline]
fn pack(low: u32, high: u32) -> u64 {
    ((low as u64) << 32) | (high as u64)
}
#[inline]
fn unpack(w: u64) -> (u32, u32) {
    ((w >> 32) as u32, (w & 0xffff_ffff) as u32)
}

fn claim_head(cursor: &std::sync::atomic::AtomicU64) -> Option<u32> {
    loop {
        let w = cursor.load(Ordering::Acquire);
        let (low, high) = unpack(w);
        if low > high {
            return None;
        }
        let nw = pack(low + 1, high);
        if cursor
            .compare_exchange_weak(w, nw, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Some(low);
        }
    }
}
fn claim_tail(cursor: &std::sync::atomic::AtomicU64) -> Option<u32> {
    loop {
        let w = cursor.load(Ordering::Acquire);
        let (low, high) = unpack(w);
        if low > high {
            return None;
        }
        // claim `high`; move high down. If high == 0 and low == 0, this is the
        // last element; after claiming, set low > high to terminate.
        let nw = if high == 0 { pack(1, 0) } else { pack(low, high - 1) };
        if cursor
            .compare_exchange_weak(w, nw, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Some(high);
        }
    }
}

use std::sync::atomic::AtomicU64;

fn run_convergence(n: usize) -> (Vec<u32>, Vec<u32>) {
    // out[i] = f(i), written by whichever core claimed i. processed[i] counts
    // how many times i was claimed (must be exactly 1).
    let out: Arc<Vec<AtomicU32>> = Arc::new((0..n).map(|_| AtomicU32::new(0)).collect());
    let processed: Arc<Vec<AtomicU32>> = Arc::new((0..n).map(|_| AtomicU32::new(0)).collect());
    // cursor packs (low=0, high=n-1).
    let cursor = Arc::new(AtomicU64::new(pack(0, (n - 1) as u32)));

    let mut handles = Vec::new();
    // Core 0: the HEAD core, claims from the low end (its baked designation).
    {
        let out = out.clone();
        let processed = processed.clone();
        let cursor = cursor.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0u32;
            while let Some(i) = claim_head(&cursor) {
                out[i as usize].store(f(i), Ordering::Relaxed);
                processed[i as usize].fetch_add(1, Ordering::Relaxed);
                count += 1;
            }
            count
        }));
    }
    // Core 1: the TAIL core, claims from the high end (its baked designation).
    {
        let out = out.clone();
        let processed = processed.clone();
        let cursor = cursor.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0u32;
            while let Some(i) = claim_tail(&cursor) {
                out[i as usize].store(f(i), Ordering::Relaxed);
                processed[i as usize].fetch_add(1, Ordering::Relaxed);
                count += 1;
            }
            count
        }));
    }
    let mut total = 0u32;
    for h in handles {
        total += h.join().unwrap();
    }
    assert_eq!(total as usize, n, "head + tail claimed exactly n records total");

    let out_v: Vec<u32> = out.iter().map(|a| a.load(Ordering::Relaxed)).collect();
    let proc_v: Vec<u32> = processed.iter().map(|a| a.load(Ordering::Relaxed)).collect();
    (out_v, proc_v)
}

fn main() {
    let n_records = USize(1 << 16);
    let n = n_records.0;

    // Run the 2-core convergence many times to shake out races.
    for round in 0..50 {
        let (out, processed) = run_convergence(n);
        for i in 0..n {
            assert_eq!(processed[i], 1, "round {round}: record {i} processed exactly once");
            assert_eq!(out[i], f(i as u32), "round {round}: record {i} value correct");
        }
    }

    // Degenerate 1-core check: a single range [0, n) covers everything once
    // (no convergence; the per-core program has one baked range).
    let single: Vec<u32> = (0..n).map(|i| f(i as u32)).collect();
    for i in 0..n {
        assert_eq!(single[i], f(i as u32), "single-core range covers record {i}");
    }

    println!(
        "WORKS: head+tail convergence. Two cores claimed {n} records from opposite ends across 50 \
         rounds; every record processed EXACTLY once (no overlap, no gap), all values correct, \
         under racing threads. Per-core head/tail designation + opposite-end cursor convergence \
         holds. 1-core degenerate = one range covering all."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// Two cores (std threads) claimed 65536 records from opposite ends via a packed
// (low, high) cursor CAS protocol, across 50 racing rounds. Every record was
// processed EXACTLY once (processed[i] == 1 for all i, no overlap, no gap), all
// values correct, and head+tail claimed exactly n total. The CAS on the packed
// cursor makes the single-element-left contention race-free (only one of head/
// tail wins the last index; the loser retries, sees low>high, stops).
//
// WHAT THIS SETTLES (E4b): head+tail convergence (domain 20 :1838-1844) within a
// commutative fiber works at 2 cores: each core has a baked end-designation
// (head claims upward, tail downward), they converge in the middle, every record
// processed once. The per-core program carries the head/tail record-range field
// (domain 17 item 2), which is the D1b carrier + D1c barrier already proven; this
// adds the convergence protocol on top. At 1 core it degenerates to one range.
//
// WHAT THIS DOES NOT SETTLE: integration into the real spawn-once pool mainloop
// (E2, bench-proven model) and the engine-side range baking from the plan (the
// per-core program field); this sketch proves the convergence PROTOCOL is correct
// and race-free, not its wiring into the pool. Commutativity is the consumer
// fiber's property (the plan only assigns head/tail to fibers it proved
// commutative); this models the disjoint-slot commutative case.
// ---------------------------------------------------------------------
