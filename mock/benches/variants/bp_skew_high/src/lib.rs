//! Variant: branch taken ~6% of the time.
//!
//! `if byte > 240` is true for ~6% of uniformly-distributed u8 input.
//! The branch predictor saturates to "not taken" within a few
//! iterations and pays near-zero misprediction cost from then on.
//!
//! Best case for predictor success. Floor for branchful inner loop.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bp_skew_high", sizes = [256, 1024, 4096, 16384])]
fn run_bp_skew_high<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                if byte > 240 {
                    acc = acc.wrapping_mul(0x100000001b3);
                } else {
                    acc ^= byte as u64;
                }
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
