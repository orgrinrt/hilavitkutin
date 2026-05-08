// Test: does LLVM devirtualize function pointers stored in a struct field,
// sliced, and iterated in a tight loop?
//
// This mirrors hilavitkutin's actual dispatch pattern:
// - ChainPlan stores ops: [EachFn; N] as a struct field
// - Dispatch takes a slice &ops[offset..offset+count]
// - The fused loop iterates: for i in range { for op in ops { op(stores, i) } }
//
// Compile:
//   rustc -C opt-level=3 -C lto=fat -C codegen-units=1 \
//         --edition 2021 struct_field_devirt_test.rs -o devirt_test
//
// Inspect assembly:
//   rustc -C opt-level=3 -C lto=fat -C codegen-units=1 \
//         --edition 2021 --emit asm struct_field_devirt_test.rs
//
// Look for: zero `blr` (aarch64) or `call *%r` (x86) in the hot loop.
// If devirtualized: arithmetic ops inlined directly in the loop body.

#![allow(unused)]

use std::time::Instant;

// --- Store simulation (minimal) ---

/// Simulates the StoreMap: contiguous column data.
/// In real hilavitkutin this is SlabColumn-backed, no heap.
/// For the test we use Vec to focus on the devirt question.
struct StoreMap {
    col_a: Vec<u64>,  // input column
    col_b: Vec<u64>,  // intermediate column
    col_c: Vec<u64>,  // output column
}

// --- Function pointer type (matches hilavitkutin's EachFn) ---

type EachFn = fn(&mut StoreMap, usize);

// --- Three ops that form a fusion chain ---

#[inline(always)]
fn op_scale(stores: &mut StoreMap, i: usize) {
    // multiply by 3 using add+shift (LLVM should emit add x, x, lsl #1)
    stores.col_b[i] = stores.col_a[i] * 3;
}

#[inline(always)]
fn op_add_offset(stores: &mut StoreMap, i: usize) {
    // add a constant offset
    stores.col_b[i] = stores.col_b[i].wrapping_add(42);
}

#[inline(always)]
fn op_normalize(stores: &mut StoreMap, i: usize) {
    // write to output column: shift right (cheap "normalize")
    stores.col_c[i] = stores.col_b[i] >> 4;
}

// --- The struct that holds function pointers (mirrors ChainPlan) ---

struct ChainPlan {
    /// All ops across all chains, packed contiguously.
    ops: [EachFn; 16],
    /// Per-chain: offset into ops array.
    op_offsets: [usize; 8],
    /// Per-chain: number of ops.
    op_counts: [usize; 8],
    chain_count: usize,
}

// --- Dispatch: the pattern we're testing ---

/// This mirrors the actual dispatch loop. The key question is whether
/// LLVM devirtualizes `op(stores, i)` when `ops` is a slice of a
/// struct field.
#[inline(never)]  // prevent the dispatch from being optimized away entirely
fn dispatch_fused(
    plan: &ChainPlan,
    stores: &mut StoreMap,
    chain_idx: usize,
    start: usize,
    end: usize,
) {
    let offset = plan.op_offsets[chain_idx];
    let count = plan.op_counts[chain_idx];
    let ops = &plan.ops[offset..offset + count];

    // THE FUSED LOOP — does LLVM devirtualize here?
    for i in start..end {
        for op in ops {
            op(stores, i);
        }
    }
}

// --- Hand-fused baseline for comparison ---

#[inline(never)]
fn dispatch_hand_fused(
    stores: &mut StoreMap,
    start: usize,
    end: usize,
) {
    for i in start..end {
        stores.col_b[i] = stores.col_a[i] * 3;
        stores.col_b[i] = stores.col_b[i].wrapping_add(42);
        stores.col_c[i] = stores.col_b[i] >> 4;
    }
}

// --- Mitigation 1: monomorphized dispatch per chain ---
// The plan stores a dispatch function per chain, not an array of ops.
// Each chain's dispatch function is a closure-like fn that captures
// the op list as a local slice. Generated at plan time via a generic
// helper.

/// Trait that chains implement. Each chain type knows its ops at
/// compile time.
trait ChainDispatch {
    fn dispatch(stores: &mut StoreMap, start: usize, end: usize);
}

/// Concrete chain: scale + add_offset + normalize.
struct Chain0;
impl ChainDispatch for Chain0 {
    #[inline(never)]
    fn dispatch(stores: &mut StoreMap, start: usize, end: usize) {
        let ops: &[EachFn] = &[op_scale, op_add_offset, op_normalize];
        for i in start..end {
            for op in ops {
                op(stores, i);
            }
        }
    }
}

// --- Mitigation 2: const-generic op count + inline registration fn ---
// The dispatch function is generic over N (op count), and the ops
// array is built inside the function via a registration callback.

#[inline(never)]
fn dispatch_const_generic<const N: usize>(
    stores: &mut StoreMap,
    ops: &[EachFn; N],
    start: usize,
    end: usize,
) {
    for i in start..end {
        for op in ops.iter() {
            op(stores, i);
        }
    }
}

// --- Mitigation 3: unrolled dispatch (no slice at all) ---
// For small chain sizes (≤8 ops), generate an unrolled dispatch
// function per chain. No function pointer array. Direct calls.

#[inline(never)]
fn dispatch_unrolled_3(
    stores: &mut StoreMap,
    op0: EachFn,
    op1: EachFn,
    op2: EachFn,
    start: usize,
    end: usize,
) {
    for i in start..end {
        op0(stores, i);
        op1(stores, i);
        op2(stores, i);
    }
}

// --- Local slice baseline (known to devirtualize) ---

#[inline(never)]
fn dispatch_local_slice(
    stores: &mut StoreMap,
    start: usize,
    end: usize,
) {
    let ops: &[EachFn] = &[op_scale, op_add_offset, op_normalize];
    for i in start..end {
        for op in ops {
            op(stores, i);
        }
    }
}

// --- Benchmark harness ---

fn black_box<T>(x: T) -> T {
    // prevent optimization of the result
    unsafe {
        let ret = std::ptr::read_volatile(&x);
        std::mem::forget(x);
        ret
    }
}

fn main() {
    const N: usize = 1 << 20; // 1M elements
    const ITERS: usize = 100;

    let mut stores = StoreMap {
        col_a: vec![7u64; N],
        col_b: vec![0u64; N],
        col_c: vec![0u64; N],
    };

    // build the plan (mirrors scheduler plan construction)
    let mut plan = ChainPlan {
        ops: [op_scale as EachFn; 16], // filled with placeholder
        op_offsets: [0; 8],
        op_counts: [0; 8],
        chain_count: 1,
    };
    // chain 0: three ops at offset 0
    plan.ops[0] = op_scale;
    plan.ops[1] = op_add_offset;
    plan.ops[2] = op_normalize;
    plan.op_offsets[0] = 0;
    plan.op_counts[0] = 3;

    // build const-generic ops array
    let const_ops: [EachFn; 3] = [op_scale, op_add_offset, op_normalize];

    // warm up all variants
    dispatch_fused(&plan, &mut stores, 0, 0, N);
    dispatch_hand_fused(&mut stores, 0, N);
    dispatch_local_slice(&mut stores, 0, N);
    Chain0::dispatch(&mut stores, 0, N);
    dispatch_const_generic(&mut stores, &const_ops, 0, N);
    dispatch_unrolled_3(&mut stores, op_scale, op_add_offset, op_normalize, 0, N);

    // --- benchmarks ---

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_fused(&plan, &mut stores, 0, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_struct = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_hand_fused(&mut stores, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_hand = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_local_slice(&mut stores, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_local = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        Chain0::dispatch(&mut stores, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_mono = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_const_generic(&mut stores, &const_ops, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_const = t0.elapsed();

    let t0 = Instant::now();
    for _ in 0..ITERS {
        dispatch_unrolled_3(&mut stores, op_scale, op_add_offset, op_normalize, 0, N);
        black_box(&stores.col_c[0]);
    }
    let dt_unrolled = t0.elapsed();

    let elements = N * ITERS;
    let ns = |d: std::time::Duration| d.as_nanos() as f64 / elements as f64;
    let ms = |d: std::time::Duration| d.as_secs_f64() * 1000.0;

    println!("=== devirtualization test: struct field fn ptr array ===");
    println!();
    println!("elements per iter: {N}");
    println!("iterations:        {ITERS}");
    println!();
    println!("hand-fused:              {:.2} ns/elem  ({:.1} ms)", ns(dt_hand), ms(dt_hand));
    println!("local slice (known):     {:.2} ns/elem  ({:.1} ms)", ns(dt_local), ms(dt_local));
    println!("monomorphized trait:     {:.2} ns/elem  ({:.1} ms)", ns(dt_mono), ms(dt_mono));
    println!("const-generic [N]:       {:.2} ns/elem  ({:.1} ms)", ns(dt_const), ms(dt_const));
    println!("unrolled (3 params):     {:.2} ns/elem  ({:.1} ms)", ns(dt_unrolled), ms(dt_unrolled));
    println!("struct-field slice:      {:.2} ns/elem  ({:.1} ms)", ns(dt_struct), ms(dt_struct));
    println!();

    let baseline = ns(dt_hand);
    let report = |name: &str, t: std::time::Duration| {
        let ratio = ns(t) / baseline;
        let status = if ratio < 1.15 { "PASS" } else if ratio < 2.0 { "WARN" } else { "FAIL" };
        println!("  {status}: {name:<25} {ratio:.1}x hand-fused");
    };
    report("local slice", dt_local);
    report("monomorphized trait", dt_mono);
    report("const-generic [N]", dt_const);
    report("unrolled (3 params)", dt_unrolled);
    report("struct-field slice", dt_struct);

    // verify correctness
    dispatch_fused(&plan, &mut stores, 0, 0, N);
    let expected = ((7u64 * 3).wrapping_add(42)) >> 4;
    assert_eq!(stores.col_c[0], expected, "correctness check failed");
    assert_eq!(stores.col_c[N - 1], expected, "correctness check failed (last)");
    println!();
    println!("correctness: OK (col_c[0] = {}, expected = {})", stores.col_c[0], expected);
}
