//! Variant: `saturating_add` reduction.
//!
//! Lowers to ADDS + CSEL on aarch64: add-with-flags, then select
//! u64::MAX if carry, else the sum. Two cycles vs one for wrapping.
//! Models the Precise strategy overflow policy: clamp at boundary
//! instead of wrapping.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("sat_saturate", sizes = [256, 1024, 4096, 16384])]
fn run_sat_saturate<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0;
            for &byte in input.iter() {
                acc = acc.saturating_add((byte as u64).wrapping_mul(0x100000001b3));
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
