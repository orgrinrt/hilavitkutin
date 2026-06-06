//! Sketch (E6 / #340, Phase E): N-core output == 1-core output (the oracle).
//!
//! The canon (synthesis 2.4, R6) makes the 1-core configuration the correctness
//! oracle the N-core configuration validates against: same engine, same plan, same
//! `Cfg::Out` for any core count. E6 (roadmap section 9) is a test-discipline step,
//! not new code: run the same pipeline at 1/2/7 cores, assert identical output.
//!
//! The pipeline here has both output shapes the engine produces: a per-record
//! Column (out[i] = g(f(i)), disjoint writes) and a commutative+associative
//! reduction (wrapping sum of the per-record values, the Accum path). Partitioning
//! the records across K cores must give bit-identical (column, sum) for any K, IFF
//! the reduction combine is associative/commutative (wrapping add is) and the
//! per-record op is pure. That is exactly what makes a fiber safe to parallelise;
//! E6 asserts the invariant holds across K in {1, 2, 7}.
//!
//! Hypothesis: for K in {1, 2, 7}, the (column, sum) Cfg::Out is bit-identical to
//! the serial reference. Leeway (section 9): EXACT (bit-identical output is the
//! oracle). A FAILURE here would mean the parallel partition is not output-
//! equivalent (a non-commutative reduction snuck in, or a partition overlap/gap).
//! Outcome at the bottom.

#![allow(dead_code)]

use std::sync::Arc;
use std::thread;

use arvo::USize;

const M1: u32 = 2654435761;
#[inline(always)]
fn f(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn g(a: u32) -> u32 {
    let b = a.wrapping_mul(2246822519).wrapping_add(1);
    (b >> 13) ^ b
}

// Serial reference: the 1-core oracle, computed without threads.
fn serial(n: usize) -> (Vec<u32>, u32) {
    let mut col = vec![0u32; n];
    let mut sum = 0u32;
    for i in 0..n {
        let v = g(f(i as u32));
        col[i] = v;
        sum = sum.wrapping_add(v);
    }
    (col, sum)
}

// K-core run: partition [0, n) into K contiguous ranges (one per core), each core
// fills its slice of the column and accumulates a partial sum; partials combine
// (wrapping add, associative+commutative) into the total. This is the canonical
// commutative-fiber parallelisation: disjoint column writes + a reduction.
fn k_core(n: usize, k: usize) -> (Vec<u32>, u32) {
    // Output column shared as raw cells (disjoint ranges => no aliasing of a cell).
    let col: Arc<Vec<std::sync::atomic::AtomicU32>> =
        Arc::new((0..n).map(|_| std::sync::atomic::AtomicU32::new(0)).collect());
    let per = n.div_ceil(k);
    let mut handles = Vec::new();
    for c in 0..k {
        let col = col.clone();
        let lo = c * per;
        let hi = (lo + per).min(n);
        handles.push(thread::spawn(move || {
            let mut partial = 0u32;
            for i in lo..hi {
                let v = g(f(i as u32));
                col[i].store(v, std::sync::atomic::Ordering::Relaxed);
                partial = partial.wrapping_add(v);
            }
            partial
        }));
    }
    // Combine partials. Wrapping add is associative+commutative, so the order of
    // combination does not change the total (the property that makes the reduction
    // core-count-independent).
    let mut sum = 0u32;
    for h in handles {
        sum = sum.wrapping_add(h.join().unwrap());
    }
    let col_v: Vec<u32> =
        col.iter().map(|a| a.load(std::sync::atomic::Ordering::Relaxed)).collect();
    (col_v, sum)
}

fn main() {
    let n = USize(1 << 18).0; // 262144 records

    // The 1-core oracle.
    let (ref_col, ref_sum) = serial(n);

    // N-core configs validate against it. 7 is intentionally non-divisible into n
    // (uneven last range) to exercise the remainder partition.
    for &k in &[1usize, 2, 7] {
        let (col, sum) = k_core(n, k);
        assert_eq!(sum, ref_sum, "k={k}: reduction sum must match the 1-core oracle");
        assert_eq!(col.len(), ref_col.len(), "k={k}: column length");
        // Bit-identical column.
        let mut mismatches = 0usize;
        for i in 0..n {
            if col[i] != ref_col[i] {
                mismatches += 1;
            }
        }
        assert_eq!(mismatches, 0, "k={k}: column must be bit-identical to the oracle");
    }

    println!(
        "WORKS: N-core == 1-core oracle. The same commutative pipeline (per-record column + \
         wrapping-sum reduction) over {n} records gives bit-identical (column, sum) Cfg::Out at \
         k=1, 2, 7 cores (7 exercises the uneven remainder partition). The 1-core configuration is \
         the correctness oracle; N cores validate against it."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// The same commutative pipeline (per-record column g(f(i)) + wrapping-sum
// reduction) over 262144 records gave bit-identical (column, sum) at k=1, 2, 7
// cores (k=7 exercises the uneven remainder partition); all matched the serial
// 1-core oracle exactly.
//
// WHAT THIS SETTLES (E6): the canon's correctness oracle (1-core output == N-core
// output, same Cfg::Out for any core count) holds for a correctly-partitioned
// commutative fiber: disjoint column writes are trivially core-count-independent,
// and the reduction is core-count-independent because wrapping add is
// associative+commutative so partial-combine order does not matter. This is the
// test-discipline gate the engine runs (run at 1/2/7, assert identical); a
// failure flags a non-commutative reduction or a partition overlap/gap.
//
// WHAT THIS DOES NOT SETTLE: it asserts the INVARIANT, not that the plan only
// parallelises genuinely-commutative fibers (the plan's commutativity analysis is
// upstream; a non-commutative fiber stays single-trunk). The oracle test is the
// safety net that catches a mis-classification.
// ---------------------------------------------------------------------
