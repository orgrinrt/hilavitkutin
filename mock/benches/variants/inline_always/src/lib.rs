//! Variant: per-record step function with `#[inline(always)]`.
//!
//! Forces the step to inline into the morsel loop. The loop body
//! folds into a single arithmetic chain with no call boundary.
//!
//! Baseline for the inline-strategy bench. Validates the Topic 7
//! morsel-loop assumption that per-record steps inline into the
//! caller and produce single-arithmetic-chain codegen.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn step(acc: u64, byte: u8) -> u64 {
    (acc ^ (byte as u64)).wrapping_mul(0x100000001b3)
}

#[bench_variant("inline_always", sizes = [256, 1024, 4096, 16384])]
fn run_inline_always<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = step(acc, byte);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
