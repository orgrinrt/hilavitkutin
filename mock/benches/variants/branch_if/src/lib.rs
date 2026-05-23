//! Variant: branchful if/else dispatch.
//!
//! For each byte, if even-valued, accumulate one way; if odd-valued, another.
//! Tests the branch-predictor cost on adversarial (data-dependent) input.
//! Compares vs branchless select.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("branch_if", sizes = [64, 256, 1024, 4096, 16384])]
fn run_branch_if<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for b in input.iter() {
                if (*b & 1) == 0 {
                    acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
                } else {
                    acc = (acc.wrapping_add(*b as u64)).rotate_left(13);
                }
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
