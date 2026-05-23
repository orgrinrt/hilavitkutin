//! Variant: two-way interleaved fold.
//!
//! Two independent accumulators alternate over even/odd indices.
//! Each accumulator's dep chain runs in parallel; the multiplier
//! is ~3 cycles latency but throughput is 1 per cycle, so two
//! independent chains should hit ~1.5 cycles per iteration.
//! Final combine merges the two accumulators.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("fold_paired", sizes = [256, 1024, 4096, 16384])]
fn run_fold_paired<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc0: u64 = 0xcbf29ce484222325;
            let mut acc1: u64 = 0xcbf29ce484222325;
            let mut i = 0usize;
            while i + 2 <= words {
                let v0 = unsafe { *p.add(i) };
                let v1 = unsafe { *p.add(i + 1) };
                acc0 = (acc0 ^ v0).wrapping_mul(0x100000001b3);
                acc1 = (acc1 ^ v1).wrapping_mul(0x100000001b3);
                i += 2;
            }
            if i < words {
                let v = unsafe { *p.add(i) };
                acc0 = (acc0 ^ v).wrapping_mul(0x100000001b3);
            }
            let acc = acc0 ^ acc1;
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
