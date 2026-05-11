//! Variant: plain `u64` accumulator.
//!
//! Baseline. The accumulator lives in a register through the entire
//! hot loop; no memory traffic per step.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("acc_plain", sizes = [256, 1024, 4096, 16384])]
fn run_acc_plain<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = (acc ^ (byte as u64)).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
