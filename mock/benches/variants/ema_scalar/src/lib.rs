//! Variant: hand-rolled scalar reference path.
//!
//! Identical math to the NEON variant: 4 EMA lanes, sample i updates
//! lane (i % 4) with `acc = (acc*7 + sample) >> 3`. Each sample is
//! processed by name and the loop is structured to discourage
//! autovectorisation (read into a temporary, write back to the
//! same lane explicitly). Output: first two lanes as 8 bytes.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_scalar", sizes = [64, 256, 1024, 4096, 16384])]
fn run_ema_scalar<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let samples = N / 4;
            let mut acc0: u32 = 0;
            let mut acc1: u32 = 0;
            let mut acc2: u32 = 0;
            let mut acc3: u32 = 0;
            let in_ptr = input.as_ptr();
            let mut i: usize = 0;
            // Process groups of 4 explicitly (matches NEON lane layout).
            while i + 4 <= samples {
                let base = i * 4;
                let s0 = unsafe { (in_ptr.add(base) as *const u32).read_unaligned() };
                let s1 = unsafe { (in_ptr.add(base + 4) as *const u32).read_unaligned() };
                let s2 = unsafe { (in_ptr.add(base + 8) as *const u32).read_unaligned() };
                let s3 = unsafe { (in_ptr.add(base + 12) as *const u32).read_unaligned() };
                acc0 = (acc0.wrapping_mul(7).wrapping_add(s0)) >> 3;
                acc1 = (acc1.wrapping_mul(7).wrapping_add(s1)) >> 3;
                acc2 = (acc2.wrapping_mul(7).wrapping_add(s2)) >> 3;
                acc3 = (acc3.wrapping_mul(7).wrapping_add(s3)) >> 3;
                i += 4;
            }
            // Tail (only for sizes where N/4 is not a multiple of 4).
            while i < samples {
                let s = unsafe { (in_ptr.add(i * 4) as *const u32).read_unaligned() };
                acc0 = (acc0.wrapping_mul(7).wrapping_add(s)) >> 3;
                i += 1;
            }
            output[0..4].copy_from_slice(&acc0.to_le_bytes());
            output[4..8].copy_from_slice(&acc1.to_le_bytes());
            // suppress unused-warning by black-boxing acc2/acc3:
            core::hint::black_box(&acc2);
            core::hint::black_box(&acc3);
        }
    }
}
