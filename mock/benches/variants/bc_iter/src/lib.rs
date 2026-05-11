//! Variant: idiomatic `.iter()` loop.
//!
//! Iterator-protocol traversal. Rust guarantees no bounds checks
//! because the iterator itself owns the position. This is the
//! canonical morsel-loop shape for consumer WorkUnits.
//!
//! Baseline. The other two variants compare against this.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bc_iter", sizes = [256, 1024, 4096, 16384])]
fn run_bc_iter<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
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
