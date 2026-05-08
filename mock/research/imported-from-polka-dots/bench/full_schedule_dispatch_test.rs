// Test: full-schedule mega-dispatch, multi-threaded morsel dispatch,
// and intrinsics beyond prefetch (branch hints, non-temporal stores).
//
// Extends dispatch_optimization_test.rs:
// 1. All-partitions mega-dispatch: one function for the ENTIRE schedule
// 2. Multi-threaded morsel dispatch: std::thread workers with pre-partitioned ranges
// 3. Intrinsics: likely/unlikely branch hints, non-temporal stores
//
// Compile (nightly required for intrinsics):
//   rustc +nightly -C opt-level=3 -C lto=fat -C codegen-units=1 \
//         --edition 2021 full_schedule_dispatch_test.rs -o full_sched_test

#![allow(unused)]
#![feature(core_intrinsics)]

use std::sync::{Arc, Barrier};
use std::time::Instant;

// --- Column storage (shared across threads) ---

struct Columns {
    // partition 0: chain 0 (scale→add→norm) then chain 1 (combine→finalize)
    col_a: Vec<u64>,  // input
    col_b: Vec<u64>,  // intermediate (chain 0 write)
    col_c: Vec<u64>,  // chain 0 output / chain 1 input
    col_d: Vec<u64>,  // chain 1 second input
    col_e: Vec<u64>,  // chain 1 output

    // partition 1: completely independent columns
    col_f: Vec<u64>,  // input
    col_g: Vec<u64>,  // intermediate
    col_h: Vec<u64>,  // output
}

type EachFn = fn(&mut Columns, usize);

// =============================================================
// Partition 0 ops
// =============================================================

// chain 0: scale → add → normalize
#[inline(always)]
fn p0_op_scale(cols: &mut Columns, i: usize) {
    cols.col_b[i] = cols.col_a[i] * 3;
}
#[inline(always)]
fn p0_op_add(cols: &mut Columns, i: usize) {
    cols.col_b[i] = cols.col_b[i].wrapping_add(42);
}
#[inline(always)]
fn p0_op_norm(cols: &mut Columns, i: usize) {
    cols.col_c[i] = cols.col_b[i] >> 4;
}

// chain 1: combine → finalize
#[inline(always)]
fn p0_op_combine(cols: &mut Columns, i: usize) {
    cols.col_e[i] = cols.col_c[i].wrapping_add(cols.col_d[i]);
}
#[inline(always)]
fn p0_op_finalize(cols: &mut Columns, i: usize) {
    cols.col_e[i] = cols.col_e[i] ^ (cols.col_e[i] >> 7);
}

// =============================================================
// Partition 1 ops (independent columns, no overlap with P0)
// =============================================================

// chain 2: square → mask → store
#[inline(always)]
fn p1_op_square(cols: &mut Columns, i: usize) {
    cols.col_g[i] = cols.col_f[i].wrapping_mul(cols.col_f[i]);
}
#[inline(always)]
fn p1_op_mask(cols: &mut Columns, i: usize) {
    cols.col_g[i] = cols.col_g[i] & 0x0FFF_FFFF_FFFF_FFFF;
}
#[inline(always)]
fn p1_op_store(cols: &mut Columns, i: usize) {
    cols.col_h[i] = cols.col_g[i].wrapping_add(1);
}

// =============================================================
// Per-chain dispatch functions (inline(always) for composition)
// =============================================================

#[inline(always)]
fn chain0_dispatch(cols: &mut Columns, start: usize, end: usize) {
    let ops: &[EachFn] = &[p0_op_scale, p0_op_add, p0_op_norm];
    for i in start..end {
        for op in ops { op(cols, i); }
    }
}

#[inline(always)]
fn chain1_dispatch(cols: &mut Columns, start: usize, end: usize) {
    let ops: &[EachFn] = &[p0_op_combine, p0_op_finalize];
    for i in start..end {
        for op in ops { op(cols, i); }
    }
}

#[inline(always)]
fn chain2_dispatch(cols: &mut Columns, start: usize, end: usize) {
    let ops: &[EachFn] = &[p1_op_square, p1_op_mask, p1_op_store];
    for i in start..end {
        for op in ops { op(cols, i); }
    }
}

// =============================================================
// Dispatch approaches
// =============================================================

/// Approach D: per-partition mega-dispatch
#[inline(never)]
fn partition0_dispatch(cols: &mut Columns, start: usize, end: usize) {
    chain0_dispatch(cols, start, end);
    chain1_dispatch(cols, start, end);
}

#[inline(never)]
fn partition1_dispatch(cols: &mut Columns, start: usize, end: usize) {
    chain2_dispatch(cols, start, end);
}

/// Approach E: full-schedule mega-dispatch — ALL partitions in ONE function.
/// Since partitions are independent, they can be sequenced in any order.
#[inline(never)]
fn schedule_mega_dispatch(cols: &mut Columns, start: usize, end: usize) {
    // partition 0
    chain0_dispatch(cols, start, end);
    chain1_dispatch(cols, start, end);
    // partition 1
    chain2_dispatch(cols, start, end);
}

/// Hand-fused baseline: everything manually inlined
#[inline(never)]
fn hand_fused_all(cols: &mut Columns, start: usize, end: usize) {
    // partition 0, chain 0
    for i in start..end {
        let b = cols.col_a[i] * 3;
        let b = b.wrapping_add(42);
        cols.col_b[i] = b;
        cols.col_c[i] = b >> 4;
    }
    // partition 0, chain 1
    for i in start..end {
        let e = cols.col_c[i].wrapping_add(cols.col_d[i]);
        cols.col_e[i] = e ^ (e >> 7);
    }
    // partition 1, chain 2
    for i in start..end {
        let g = cols.col_f[i].wrapping_mul(cols.col_f[i]);
        let g = g & 0x0FFF_FFFF_FFFF_FFFF;
        cols.col_h[i] = g.wrapping_add(1);
    }
}

/// Indirect per-partition (two indirect calls)
#[inline(never)]
fn dispatch_indirect_partitions(
    cols: &mut Columns,
    dispatchers: &[fn(&mut Columns, usize, usize)],
    start: usize,
    end: usize,
) {
    for d in dispatchers {
        d(cols, start, end);
    }
}

// =============================================================
// Intrinsics variants
// =============================================================

/// Branch hints: likely/unlikely on bounds checks.
/// Uses core::intrinsics::likely to tell LLVM the common path.
#[inline(never)]
fn schedule_mega_with_hints(cols: &mut Columns, start: usize, end: usize) {
    let n = cols.col_a.len();

    // partition 0, chain 0 — with branch hint on bounds
    for i in start..end {
        unsafe {
            if core::intrinsics::likely(i < n) {
                let b = *cols.col_a.get_unchecked(i) * 3;
                let b = b.wrapping_add(42);
                *cols.col_b.get_unchecked_mut(i) = b;
                *cols.col_c.get_unchecked_mut(i) = b >> 4;
            }
        }
    }
    // partition 0, chain 1
    for i in start..end {
        unsafe {
            if core::intrinsics::likely(i < n) {
                let e = cols.col_c.get_unchecked(i)
                    .wrapping_add(*cols.col_d.get_unchecked(i));
                *cols.col_e.get_unchecked_mut(i) = e ^ (e >> 7);
            }
        }
    }
    // partition 1, chain 2
    for i in start..end {
        unsafe {
            if core::intrinsics::likely(i < n) {
                let g = cols.col_f.get_unchecked(i)
                    .wrapping_mul(*cols.col_f.get_unchecked(i));
                let g = g & 0x0FFF_FFFF_FFFF_FFFF;
                *cols.col_h.get_unchecked_mut(i) = g.wrapping_add(1);
            }
        }
    }
}

/// Unchecked indexing (no bounds checks at all).
/// The scheduler guarantees start..end is within bounds.
#[inline(never)]
fn schedule_mega_unchecked(cols: &mut Columns, start: usize, end: usize) {
    // partition 0, chain 0
    for i in start..end {
        unsafe {
            let b = *cols.col_a.get_unchecked(i) * 3;
            let b = b.wrapping_add(42);
            *cols.col_b.get_unchecked_mut(i) = b;
            *cols.col_c.get_unchecked_mut(i) = b >> 4;
        }
    }
    // partition 0, chain 1
    for i in start..end {
        unsafe {
            let e = cols.col_c.get_unchecked(i)
                .wrapping_add(*cols.col_d.get_unchecked(i));
            *cols.col_e.get_unchecked_mut(i) = e ^ (e >> 7);
        }
    }
    // partition 1, chain 2
    for i in start..end {
        unsafe {
            let g = cols.col_f.get_unchecked(i)
                .wrapping_mul(*cols.col_f.get_unchecked(i));
            let g = g & 0x0FFF_FFFF_FFFF_FFFF;
            *cols.col_h.get_unchecked_mut(i) = g.wrapping_add(1);
        }
    }
}

// =============================================================
// Multi-threaded dispatch
// =============================================================

/// Pre-spawned thread pool with barrier synchronization.
/// Threads are created once. Each iteration: release barrier → work → join barrier.
fn bench_multithreaded(
    cols: &mut Columns,
    n: usize,
    worker_count: usize,
    dispatch_fn: fn(&mut Columns, usize, usize),
    iters: usize,
) -> std::time::Duration {
    use std::sync::atomic::{AtomicBool, Ordering};

    let per_worker = n / worker_count;
    let remainder = n % worker_count;
    let cols_ptr = cols as *mut Columns as usize;

    let start_barrier = Arc::new(Barrier::new(worker_count + 1));
    let done_barrier = Arc::new(Barrier::new(worker_count + 1));
    let stop = Arc::new(AtomicBool::new(false));

    // pre-spawn workers
    let mut handles = Vec::new();
    let mut offset = 0;
    for w in 0..worker_count {
        let count = per_worker + if w < remainder { 1 } else { 0 };
        let my_start = offset;
        let my_end = offset + count;
        offset = my_end;

        let sb = Arc::clone(&start_barrier);
        let db = Arc::clone(&done_barrier);
        let st = Arc::clone(&stop);
        let ptr = cols_ptr;

        handles.push(std::thread::spawn(move || {
            loop {
                sb.wait(); // wait for "go"
                if st.load(Ordering::Relaxed) { break; }
                let cols = unsafe { &mut *(ptr as *mut Columns) };
                dispatch_fn(cols, my_start, my_end);
                db.wait(); // signal "done"
            }
        }));
    }

    // run iterations
    let t0 = Instant::now();
    for _ in 0..iters {
        start_barrier.wait(); // release workers
        done_barrier.wait();  // wait for completion
    }
    let elapsed = t0.elapsed();

    // stop workers
    stop.store(true, Ordering::Relaxed);
    start_barrier.wait();
    for h in handles { let _ = h.join(); }

    elapsed
}

// =============================================================
// Benchmark
// =============================================================

fn black_box<T>(x: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&x);
        std::mem::forget(x);
        ret
    }
}

fn main() {
    const N: usize = 1 << 22; // 4M elements (~256 MB working set)
    const ITERS: usize = 50;

    let mut cols = Columns {
        col_a: vec![7u64; N],
        col_b: vec![0u64; N],
        col_c: vec![0u64; N],
        col_d: vec![11u64; N],
        col_e: vec![0u64; N],
        col_f: vec![5u64; N],
        col_g: vec![0u64; N],
        col_h: vec![0u64; N],
    };

    let partition_fns: [fn(&mut Columns, usize, usize); 2] =
        [partition0_dispatch, partition1_dispatch];

    // warm up
    hand_fused_all(&mut cols, 0, N);
    schedule_mega_dispatch(&mut cols, 0, N);
    dispatch_indirect_partitions(&mut cols, &partition_fns, 0, N);
    schedule_mega_with_hints(&mut cols, 0, N);
    schedule_mega_unchecked(&mut cols, 0, N);

    // --- single-threaded benchmarks ---

    let t0 = Instant::now();
    for _ in 0..ITERS {
        hand_fused_all(&mut cols, 0, N);
        black_box(cols.col_e[0]);
        black_box(cols.col_h[0]);
    }
    let dt_hand = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        schedule_mega_dispatch(&mut cols, 0, N);
        black_box(cols.col_e[0]);
        black_box(cols.col_h[0]);
    }
    let dt_mega = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_indirect_partitions(&mut cols, &partition_fns, 0, N);
        black_box(cols.col_e[0]);
        black_box(cols.col_h[0]);
    }
    let dt_indirect = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        schedule_mega_with_hints(&mut cols, 0, N);
        black_box(cols.col_e[0]);
        black_box(cols.col_h[0]);
    }
    let dt_hints = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        schedule_mega_unchecked(&mut cols, 0, N);
        black_box(cols.col_e[0]);
        black_box(cols.col_h[0]);
    }
    let dt_unchecked = t0.elapsed();

    // --- multi-threaded benchmarks ---

    let core_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    // test various thread counts
    let thread_counts: Vec<usize> = {
        let mut v = vec![1, 2, 4];
        if core_count >= 6 { v.push(core_count); }
        v
    };

    let mut mt_results: Vec<(usize, &str, std::time::Duration)> = Vec::new();

    for &tc in &thread_counts {
        let dt = bench_multithreaded(&mut cols, N, tc, schedule_mega_dispatch, ITERS);
        mt_results.push((tc, "mega-dispatch", dt));
        let dt = bench_multithreaded(&mut cols, N, tc, schedule_mega_unchecked, ITERS);
        mt_results.push((tc, "mega unchecked", dt));
        let dt = bench_multithreaded(&mut cols, N, tc, hand_fused_all, ITERS);
        mt_results.push((tc, "hand-fused", dt));
    }

    // --- results ---

    let elements = N * ITERS;
    let ns = |d: std::time::Duration| d.as_nanos() as f64 / elements as f64;
    let ms = |d: std::time::Duration| d.as_secs_f64() * 1000.0;
    let baseline = ns(dt_hand);

    println!("=== full-schedule dispatch test: 2 partitions, 3 chains, 8 ops ===");
    println!();
    println!("elements:     {N}");
    println!("iterations:   {ITERS}");
    println!("cores:        {core_count}");
    println!();

    println!("--- single-threaded ---");
    let entries: Vec<(&str, std::time::Duration)> = vec![
        ("hand-fused (baseline)", dt_hand),
        ("schedule mega-dispatch", dt_mega),
        ("indirect per-partition", dt_indirect),
        ("mega + likely/unchecked", dt_hints),
        ("mega fully unchecked", dt_unchecked),
    ];
    for (name, dt) in &entries {
        let ratio = ns(*dt) / baseline;
        let tag = if ratio < 0.95 { ">>>" }
                  else if ratio < 1.05 { "===" }
                  else if ratio < 1.15 { " + " }
                  else { "!!!" };
        let note = if ratio < 0.95 { " FASTER" }
                   else if ratio > 1.05 { " slower" }
                   else { "" };
        println!("  [{tag}] {name:<28} {:.2} ns/elem  {ratio:.2}x{note}",
                 ns(*dt));
    }

    println!();
    println!("--- multi-threaded (pre-spawned pool) ---");
    println!("  {:>7} {:>20} {:>12} {:>8} {:>8}", "threads", "dispatch", "ns/elem", "vs 1T", "speedup");

    // group by dispatch type, find 1-thread baseline per type
    for dispatch_name in &["mega-dispatch", "mega unchecked", "hand-fused"] {
        let st_time = mt_results.iter()
            .find(|(tc, name, _)| *tc == 1 && *name == *dispatch_name)
            .map(|(_, _, dt)| ns(*dt))
            .unwrap_or(1.0);

        for &(tc, name, dt) in &mt_results {
            if name != *dispatch_name { continue; }
            let per_elem = ns(dt);
            let vs_baseline = per_elem / baseline;
            let speedup = st_time / per_elem;
            println!("  {:>7} {:>20} {:>10.2} {:>7.2}x {:>7.1}x",
                     tc, name, per_elem, vs_baseline, speedup);
        }
    }

    // correctness
    hand_fused_all(&mut cols, 0, N);
    let expected_c = ((7u64 * 3).wrapping_add(42)) >> 4;
    let expected_e = (expected_c.wrapping_add(11)) ^ ((expected_c.wrapping_add(11)) >> 7);
    let expected_h = (5u64.wrapping_mul(5) & 0x0FFF_FFFF_FFFF_FFFF).wrapping_add(1);
    assert_eq!(cols.col_c[0], expected_c);
    assert_eq!(cols.col_e[0], expected_e);
    assert_eq!(cols.col_h[0], expected_h);

    schedule_mega_dispatch(&mut cols, 0, N);
    assert_eq!(cols.col_c[0], expected_c);
    assert_eq!(cols.col_e[0], expected_e);
    assert_eq!(cols.col_h[0], expected_h);

    schedule_mega_unchecked(&mut cols, 0, N);
    assert_eq!(cols.col_c[0], expected_c);
    assert_eq!(cols.col_e[0], expected_e);
    assert_eq!(cols.col_h[0], expected_h);

    println!();
    println!("correctness: OK");
}
