//! Variant: explicit shift-and-test loop.
//!
//! Naive bit-by-bit trailing-zero count. O(tz) work per word: best
//! case 1 iteration (low bit set), worst case 64 iterations. The
//! variant is data-dependent on the input distribution.
//!
//! Worst case for ctz computation. Models what a "portable, no
//! intrinsics, no clever tricks" implementation would produce.

use ctz_common::sum_trailing_zeros;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

/// The shift-and-test count, stopping at 64 so that zero ends.
#[inline(always)]
fn ctz_loop(mut v: u64) -> u32 {
    let mut count = 0u32;
    while v & 1 == 0 && count < 64 {
        v >>= 1;
        count += 1;
    }
    count
}

#[bench_variant("ctz_loop", sizes = [256, 1024, 4096, 16384])]
fn run_ctz_loop<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let acc = sum_trailing_zeros(input, ctz_loop);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use ctz_common::{agrees_with_std, sum_trailing_zeros, varies};

    #[test]
    fn the_count_agrees_with_std() {
        agrees_with_std(super::ctz_loop).unwrap();
    }

    #[test]
    fn the_kernel_does_not_time_a_constant() {
        assert!(varies(|i| sum_trailing_zeros(i, super::ctz_loop)));
    }
}
