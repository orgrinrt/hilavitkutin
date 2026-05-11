//! Variant: `wrapping_add` reduction.
//!
//! Lowers to a single ADD on aarch64. Models the Hot strategy
//! overflow policy: silent wrap, accept the modulo-2^64 semantics.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("sat_wrap", sizes = [256, 1024, 4096, 16384])]
fn run_sat_wrap<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0;
            for &byte in input.iter() {
                acc = acc.wrapping_add((byte as u64).wrapping_mul(0x100000001b3));
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
