//! Variant: `u64::trailing_zeros()` intrinsic.
//!
//! Lowers to RBIT + CLZ on aarch64 (no native CTZ instruction;
//! RBIT reverses bit order and CLZ counts leading zeros of the
//! reversed word = original trailing zeros). Two cycles on Apple
//! Silicon.
//!
//! Baseline. The canonical arvo-bitmask BitAccess::trailing_zeros
//! lowering.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ctz_intrinsic", sizes = [256, 1024, 4096, 16384])]
fn run_ctz_intrinsic<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                // OR in 1 to avoid trailing_zeros(0) = 64, which dominates the sum.
                let tz = (v | 1).trailing_zeros() as u64;
                acc = acc.wrapping_add(tz);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
