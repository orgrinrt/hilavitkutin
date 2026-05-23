//! Variant: sequential single-accumulator fold.
//!
//! Long dependency chain: each multiply waits for the previous one
//! to complete. The aarch64 multiplier has ~3 cycle latency; with a
//! single accumulator, the loop is latency-bound at ~3 cycles per
//! iteration.
//!
//! Baseline. Models a naive reduction that doesn't break dep chains.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("fold_sequential", sizes = [256, 1024, 4096, 16384])]
fn run_fold_sequential<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
