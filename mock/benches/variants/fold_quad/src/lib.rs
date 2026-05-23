//! Variant: four-way interleaved fold.
//!
//! Four independent accumulators. Should saturate the aarch64
//! multiplier throughput (1 per cycle). Best case for ILP if LLVM
//! doesn't already extract it from the sequential variant.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("fold_quad", sizes = [256, 1024, 4096, 16384])]
fn run_fold_quad<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc0: u64 = 0xcbf29ce484222325;
            let mut acc1: u64 = 0xcbf29ce484222325;
            let mut acc2: u64 = 0xcbf29ce484222325;
            let mut acc3: u64 = 0xcbf29ce484222325;
            let mut i = 0usize;
            while i + 4 <= words {
                let v0 = unsafe { *p.add(i) };
                let v1 = unsafe { *p.add(i + 1) };
                let v2 = unsafe { *p.add(i + 2) };
                let v3 = unsafe { *p.add(i + 3) };
                acc0 = (acc0 ^ v0).wrapping_mul(0x100000001b3);
                acc1 = (acc1 ^ v1).wrapping_mul(0x100000001b3);
                acc2 = (acc2 ^ v2).wrapping_mul(0x100000001b3);
                acc3 = (acc3 ^ v3).wrapping_mul(0x100000001b3);
                i += 4;
            }
            while i < words {
                let v = unsafe { *p.add(i) };
                acc0 = (acc0 ^ v).wrapping_mul(0x100000001b3);
                i += 1;
            }
            let acc = acc0 ^ acc1 ^ acc2 ^ acc3;
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
