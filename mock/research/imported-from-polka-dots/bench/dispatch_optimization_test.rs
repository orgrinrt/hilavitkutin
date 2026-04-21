// Test: partition-level mega-dispatch, explicit intrinsics, software pipelining.
//
// Extends struct_field_devirt_test.rs findings. Explores:
// 1. Composing chain dispatches into a partition dispatch (does LTO inline?)
// 2. Explicit prefetch/non-temporal stores in the inner loop
// 3. Software pipelining (prefetch morsel N+1 while processing N)
//
// Compile:
//   rustc -C opt-level=3 -C lto=fat -C codegen-units=1 \
//         --edition 2021 dispatch_optimization_test.rs -o dispatch_opt_test
//
// Needs nightly for core::arch::aarch64 intrinsics:
//   rustc +nightly -C opt-level=3 -C lto=fat -C codegen-units=1 \
//         --edition 2021 dispatch_optimization_test.rs -o dispatch_opt_test

#![allow(unused)]
#![feature(core_intrinsics)]

use std::time::Instant;

// --- Store simulation ---

struct StoreMap {
    col_a: Vec<u64>,  // input
    col_b: Vec<u64>,  // intermediate
    col_c: Vec<u64>,  // output
    col_d: Vec<u64>,  // second chain input
    col_e: Vec<u64>,  // second chain output
}

type EachFn = fn(&mut StoreMap, usize);

// --- Chain 0 ops: scale → add → normalize ---

#[inline(always)]
fn op_scale(stores: &mut StoreMap, i: usize) {
    stores.col_b[i] = stores.col_a[i] * 3;
}

#[inline(always)]
fn op_add_offset(stores: &mut StoreMap, i: usize) {
    stores.col_b[i] = stores.col_b[i].wrapping_add(42);
}

#[inline(always)]
fn op_normalize(stores: &mut StoreMap, i: usize) {
    stores.col_c[i] = stores.col_b[i] >> 4;
}

// --- Chain 1 ops: reads col_c (chain 0 output), writes col_e ---

#[inline(always)]
fn op_combine(stores: &mut StoreMap, i: usize) {
    stores.col_e[i] = stores.col_c[i].wrapping_add(stores.col_d[i]);
}

#[inline(always)]
fn op_finalize(stores: &mut StoreMap, i: usize) {
    stores.col_e[i] = stores.col_e[i] ^ (stores.col_e[i] >> 7);
}

// =============================================================
// Baseline: hand-fused two chains
// =============================================================

#[inline(never)]
fn baseline_hand_fused(stores: &mut StoreMap, start: usize, end: usize) {
    // chain 0
    for i in start..end {
        let b = stores.col_a[i] * 3;
        let b = b.wrapping_add(42);
        stores.col_b[i] = b;
        stores.col_c[i] = b >> 4;
    }
    // chain 1 (depends on chain 0's col_c output)
    for i in start..end {
        let e = stores.col_c[i].wrapping_add(stores.col_d[i]);
        stores.col_e[i] = e ^ (e >> 7);
    }
}

// =============================================================
// Approach C from topic: indirect call per chain per morsel
// =============================================================

#[inline(never)]
fn chain0_dispatch(stores: &mut StoreMap, start: usize, end: usize) {
    let ops: &[EachFn] = &[op_scale, op_add_offset, op_normalize];
    for i in start..end {
        for op in ops { op(stores, i); }
    }
}

#[inline(never)]
fn chain1_dispatch(stores: &mut StoreMap, start: usize, end: usize) {
    let ops: &[EachFn] = &[op_combine, op_finalize];
    for i in start..end {
        for op in ops { op(stores, i); }
    }
}

type ChainFn = fn(&mut StoreMap, usize, usize);

#[inline(never)]
fn dispatch_indirect_per_chain(
    stores: &mut StoreMap,
    chains: &[ChainFn],
    start: usize,
    end: usize,
) {
    for chain in chains {
        chain(stores, start, end);
    }
}

// =============================================================
// Partition mega-dispatch: compose chain dispatches, inline all
// =============================================================

// chain dispatches marked inline(always) for composition
#[inline(always)]
fn chain0_dispatch_inlined(stores: &mut StoreMap, start: usize, end: usize) {
    let ops: &[EachFn] = &[op_scale, op_add_offset, op_normalize];
    for i in start..end {
        for op in ops { op(stores, i); }
    }
}

#[inline(always)]
fn chain1_dispatch_inlined(stores: &mut StoreMap, start: usize, end: usize) {
    let ops: &[EachFn] = &[op_combine, op_finalize];
    for i in start..end {
        for op in ops { op(stores, i); }
    }
}

/// Partition dispatch: ONE function, all chains inlined.
/// This is the "mega-dispatch" — called once per morsel range,
/// contains all chains' inner loops.
#[inline(never)]
fn partition_mega_dispatch(stores: &mut StoreMap, start: usize, end: usize) {
    chain0_dispatch_inlined(stores, start, end);
    chain1_dispatch_inlined(stores, start, end);
}

// =============================================================
// Partition mega-dispatch with explicit prefetch
// =============================================================

#[inline(never)]
fn partition_mega_dispatch_prefetch(stores: &mut StoreMap, start: usize, end: usize) {
    const PREFETCH_DISTANCE: usize = 8; // elements ahead

    // chain 0: scale → add → normalize
    {
        let ops: &[EachFn] = &[op_scale, op_add_offset, op_normalize];
        for i in start..end {
            // prefetch input data for upcoming elements
            if i + PREFETCH_DISTANCE < end {
                unsafe {
                    let ptr_a = stores.col_a.as_ptr().add(i + PREFETCH_DISTANCE) as *const u8;
                    core::intrinsics::prefetch_read_data::<_, 3>(ptr_a);
                }
            }
            for op in ops { op(stores, i); }
        }
    }

    // chain 1: combine → finalize
    {
        let ops: &[EachFn] = &[op_combine, op_finalize];
        for i in start..end {
            if i + PREFETCH_DISTANCE < end {
                unsafe {
                    let ptr_c = stores.col_c.as_ptr().add(i + PREFETCH_DISTANCE) as *const u8;
                    let ptr_d = stores.col_d.as_ptr().add(i + PREFETCH_DISTANCE) as *const u8;
                    core::intrinsics::prefetch_read_data::<_, 3>(ptr_c);
                    core::intrinsics::prefetch_read_data::<_, 3>(ptr_d);
                }
            }
            for op in ops { op(stores, i); }
        }
    }
}

// =============================================================
// Software-pipelined morsel dispatch
// =============================================================

/// Process morsels with software pipelining: prefetch morsel N+1's
/// data while processing morsel N.
#[inline(never)]
fn partition_mega_pipelined(
    stores: &mut StoreMap,
    total_start: usize,
    total_end: usize,
    morsel_size: usize,
) {
    let mut m_start = total_start;
    while m_start < total_end {
        let m_end = core::cmp::min(m_start + morsel_size, total_end);

        // prefetch NEXT morsel's input columns into L2
        let next_m_start = m_end;
        if next_m_start < total_end {
            let next_m_end = core::cmp::min(next_m_start + morsel_size, total_end);
            unsafe {
                // prefetch first cache lines of next morsel
                for offset in (0..core::cmp::min(next_m_end - next_m_start, 64)).step_by(8) {
                    let idx = next_m_start + offset;
                    if idx < stores.col_a.len() {
                        core::intrinsics::prefetch_read_data::<_, 2>(
                            stores.col_a.as_ptr().add(idx) as *const u8
                        );
                    }
                }
            }
        }

        // process current morsel — fully inlined chains
        chain0_dispatch_inlined(stores, m_start, m_end);
        chain1_dispatch_inlined(stores, m_start, m_end);

        m_start = m_end;
    }
}

// =============================================================
// Hand-fused with explicit prefetch baseline
// =============================================================

#[inline(never)]
fn baseline_hand_fused_prefetch(stores: &mut StoreMap, start: usize, end: usize) {
    const PD: usize = 8;
    // chain 0
    for i in start..end {
        if i + PD < end {
            unsafe {
                core::intrinsics::prefetch_read_data::<_, 3>(
                    stores.col_a.as_ptr().add(i + PD) as *const u8
                );
            }
        }
        let b = stores.col_a[i] * 3;
        let b = b.wrapping_add(42);
        stores.col_b[i] = b;
        stores.col_c[i] = b >> 4;
    }
    // chain 1
    for i in start..end {
        if i + PD < end {
            unsafe {
                core::intrinsics::prefetch_read_data::<_, 3>(
                    stores.col_c.as_ptr().add(i + PD) as *const u8
                );
                core::intrinsics::prefetch_read_data::<_, 3>(
                    stores.col_d.as_ptr().add(i + PD) as *const u8
                );
            }
        }
        let e = stores.col_c[i].wrapping_add(stores.col_d[i]);
        stores.col_e[i] = e ^ (e >> 7);
    }
}

// =============================================================
// Fully unrolled partition: no fn pointers at all, just inline code
// =============================================================

#[inline(never)]
fn partition_fully_inlined(stores: &mut StoreMap, start: usize, end: usize) {
    // chain 0: manually inlined
    for i in start..end {
        stores.col_b[i] = stores.col_a[i] * 3;
        stores.col_b[i] = stores.col_b[i].wrapping_add(42);
        stores.col_c[i] = stores.col_b[i] >> 4;
    }
    // chain 1: manually inlined
    for i in start..end {
        let e = stores.col_c[i].wrapping_add(stores.col_d[i]);
        stores.col_e[i] = e ^ (e >> 7);
    }
}

// --- Benchmark harness ---

fn black_box<T>(x: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&x);
        std::mem::forget(x);
        ret
    }
}

fn main() {
    const N: usize = 1 << 20; // 1M elements
    const ITERS: usize = 100;
    const MORSEL: usize = 1024;

    let mut stores = StoreMap {
        col_a: vec![7u64; N],
        col_b: vec![0u64; N],
        col_c: vec![0u64; N],
        col_d: vec![11u64; N],
        col_e: vec![0u64; N],
    };

    let chains: [ChainFn; 2] = [chain0_dispatch, chain1_dispatch];

    // warm up
    baseline_hand_fused(&mut stores, 0, N);
    partition_mega_dispatch(&mut stores, 0, N);
    dispatch_indirect_per_chain(&mut stores, &chains, 0, N);
    partition_mega_dispatch_prefetch(&mut stores, 0, N);
    partition_mega_pipelined(&mut stores, 0, N, MORSEL);
    baseline_hand_fused_prefetch(&mut stores, 0, N);
    partition_fully_inlined(&mut stores, 0, N);

    // --- benchmarks ---

    let bench = |name: &str, f: &dyn Fn()| -> std::time::Duration {
        let t0 = Instant::now();
        for _ in 0..ITERS {
            f();
            black_box(&stores.col_e[0]);
        }
        let dt = t0.elapsed();
        dt
    };

    // we can't use closures that mutate stores in a shared &dyn Fn,
    // so we do it manually

    let t0 = Instant::now();
    for _ in 0..ITERS {
        baseline_hand_fused(&mut stores, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_hand = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        partition_fully_inlined(&mut stores, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_inlined = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_indirect_per_chain(&mut stores, &chains, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_indirect = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        partition_mega_dispatch(&mut stores, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_mega = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        baseline_hand_fused_prefetch(&mut stores, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_hand_pf = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        partition_mega_dispatch_prefetch(&mut stores, 0, N);
        black_box(&stores.col_e[0]);
    }
    let dt_mega_pf = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        partition_mega_pipelined(&mut stores, 0, N, MORSEL);
        black_box(&stores.col_e[0]);
    }
    let dt_pipelined = t0.elapsed();

    let elements = N * ITERS;
    let ns = |d: std::time::Duration| d.as_nanos() as f64 / elements as f64;
    let ms = |d: std::time::Duration| d.as_secs_f64() * 1000.0;

    println!("=== dispatch optimization test: 2 chains, 5 ops total ===");
    println!();
    println!("elements per iter: {N}");
    println!("iterations:        {ITERS}");
    println!("morsel size:       {MORSEL} (for pipelined only)");
    println!();
    println!("--- no intrinsics ---");
    println!("hand-fused baseline:     {:.2} ns/elem  ({:.1} ms)", ns(dt_hand), ms(dt_hand));
    println!("fully inlined (no fn):   {:.2} ns/elem  ({:.1} ms)", ns(dt_inlined), ms(dt_inlined));
    println!("partition mega-dispatch:  {:.2} ns/elem  ({:.1} ms)", ns(dt_mega), ms(dt_mega));
    println!("indirect per-chain:      {:.2} ns/elem  ({:.1} ms)", ns(dt_indirect), ms(dt_indirect));
    println!();
    println!("--- with prefetch ---");
    println!("hand-fused + prefetch:   {:.2} ns/elem  ({:.1} ms)", ns(dt_hand_pf), ms(dt_hand_pf));
    println!("mega-dispatch + prefetch: {:.2} ns/elem  ({:.1} ms)", ns(dt_mega_pf), ms(dt_mega_pf));
    println!("mega + morsel pipeline:  {:.2} ns/elem  ({:.1} ms)", ns(dt_pipelined), ms(dt_pipelined));
    println!();

    let baseline = ns(dt_hand);
    let report = |name: &str, t: std::time::Duration| {
        let ratio = ns(t) / baseline;
        let status = if ratio < 1.05 { "===" }
                     else if ratio < 1.15 { " + " }
                     else if ratio < 2.0 { "---" }
                     else { "!!!" };
        let faster = if ratio < 0.95 { " FASTER" } else if ratio > 1.05 { " slower" } else { "" };
        println!("  [{status}] {name:<30} {ratio:.2}x{faster}");
    };
    println!("vs hand-fused baseline:");
    report("fully inlined (no fn ptr)", dt_inlined);
    report("partition mega-dispatch", dt_mega);
    report("indirect per-chain", dt_indirect);
    report("hand-fused + prefetch", dt_hand_pf);
    report("mega-dispatch + prefetch", dt_mega_pf);
    report("mega + morsel pipeline", dt_pipelined);

    // correctness
    baseline_hand_fused(&mut stores, 0, N);
    let expected_c = ((7u64 * 3).wrapping_add(42)) >> 4;
    let expected_e = (expected_c.wrapping_add(11)) ^ ((expected_c.wrapping_add(11)) >> 7);
    assert_eq!(stores.col_c[0], expected_c, "chain 0 failed");
    assert_eq!(stores.col_e[0], expected_e, "chain 1 failed");

    partition_mega_dispatch(&mut stores, 0, N);
    assert_eq!(stores.col_c[0], expected_c, "mega chain 0 failed");
    assert_eq!(stores.col_e[0], expected_e, "mega chain 1 failed");

    println!();
    println!("correctness: OK");
}
