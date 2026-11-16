//! Variant: `u64::trailing_zeros()` intrinsic.
//!
//! Lowers to RBIT + CLZ on aarch64 (no native CTZ instruction;
//! RBIT reverses bit order and CLZ counts leading zeros of the
//! reversed word = original trailing zeros). Two cycles on Apple
//! Silicon.
//!
//! Baseline. The canonical arvo-bitmask BitAccess::trailing_zeros
//! lowering.

use ctz_common::sum_trailing_zeros;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

/// The trailing-zero count as std lowers it.
#[inline(always)]
fn ctz_intrinsic(v: u64) -> u32 {
    v.trailing_zeros()
}

#[bench_variant("ctz_intrinsic", sizes = [256, 1024, 4096, 16384])]
fn run_ctz_intrinsic<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let acc = sum_trailing_zeros(input, ctz_intrinsic);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use ctz_common::{agrees_with_std, sum_trailing_zeros, varies};

    #[test]
    fn the_count_agrees_with_std() {
        agrees_with_std(super::ctz_intrinsic).unwrap();
    }

    #[test]
    fn the_kernel_does_not_time_a_constant() {
        assert!(varies(|i| sum_trailing_zeros(i, super::ctz_intrinsic)));
    }
}
