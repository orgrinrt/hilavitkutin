//! Variant: data-dependent gather iteration.
//!
//! Each step picks the next index from the previous read's low bits. The
//! hardware prefetcher cannot predict the access; every load is a likely
//! cache miss at large N. Models the worst case for non-Column-shaped
//! storage (linked-list traversal, hash-table probing, indirection chains).
//!
//! Validates the Column<T> design's contiguous-access claim by showing the
//! gap vs sequential when the prefetcher loses.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("iter_gather", sizes = [256, 1024, 4096, 16384])]
fn run_iter_gather<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let words = N / 8;
            let mask = (words - 1) as u64; // assume power-of-2 N
            let p = input.as_ptr();
            // Seed the chain with a value derived from the first word.
            let mut idx = unsafe { (p as *const u64).read_unaligned() } & mask;
            for _ in 0..words {
                let v = unsafe { (p.add((idx as usize) * 8) as *const u64).read_unaligned() };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
                // Next index derived from current value's low bits, plus a stride
                // to avoid cycles in pathological inputs.
                idx = (v.wrapping_add(idx).wrapping_add(1)) & mask;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
