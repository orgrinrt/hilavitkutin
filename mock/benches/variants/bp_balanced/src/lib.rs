//! Variant: branch taken ~50% of the time.
//!
//! `if byte > 128` splits uniformly. Worst case for the branch
//! predictor: no skew lets the saturating counters converge, and
//! pattern history depends on input distribution rather than
//! per-branch behavior. Modern aarch64 predictors typically resolve
//! such branches via cmov / csel patterns when LLVM can prove
//! branchless equivalent (see `branch_pattern` bench).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bp_balanced", sizes = [256, 1024, 4096, 16384])]
fn run_bp_balanced<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                if byte > 128 {
                    acc = acc.wrapping_mul(0x100000001b3);
                } else {
                    acc ^= byte as u64;
                }
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
