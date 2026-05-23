//! Variant: branch taken ~94% of the time.
//!
//! `if byte > 16` is true for ~94% of uniformly-distributed u8 input.
//! Mirror of `bp_skew_high`: predictor saturates to "taken" within
//! a few iterations.
//!
//! Confirms the predictor handles both directions symmetrically.
//! Pairs with `bp_skew_high` to bracket the predictable end of the
//! spectrum.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bp_skew_low", sizes = [256, 1024, 4096, 16384])]
fn run_bp_skew_low<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                if byte > 16 {
                    acc = acc.wrapping_mul(0x100000001b3);
                } else {
                    acc ^= byte as u64;
                }
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
