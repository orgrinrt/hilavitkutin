//! Variant: De Bruijn sequence trailing-zero count.
//!
//! Textbook portable bit-twiddling algorithm:
//!
//!   tz = TABLE[((v & -v) * MAGIC) >> 58]
//!
//! Where MAGIC is a 64-bit De Bruijn sequence and TABLE maps each
//! 6-bit window to the unique trailing-zero count. Five ops total
//! (and+neg, mul, shr, table-lookup) but each is a plain integer
//! instruction. No CLZ/RBIT dependency.
//!
//! Models what a portable / non-aarch64 platform would degrade to
//! if `core::intrinsics::cttz` failed to lower well.

use ctz_common::sum_trailing_zeros;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

const DEBRUIJN_MAGIC: u64 = 0x03F79D71B4CB0A89;
/// The window-to-count table of `DEBRUIJN_MAGIC`, derived from the magic at
/// compile time: the window of `1 << k` holds `k`. A table written out by
/// hand once disagreed with its magic, and the old `v | 1` input hid it.
const DEBRUIJN_TABLE: [u8; 64] = {
    let mut table = [0u8; 64];
    let mut k = 0;
    while k < 64 {
        table[((1u64 << k).wrapping_mul(DEBRUIJN_MAGIC) >> 58) as usize] = k as u8;
        k += 1;
    }
    table
};

/// The De Bruijn count. `v & -v` is zero for zero, which the table would read
/// as a count of 0, so zero is answered first.
#[inline(always)]
fn ctz_debruijn(v: u64) -> u32 {
    if v == 0 {
        return 64;
    }
    u32::from(DEBRUIJN_TABLE[((v & v.wrapping_neg()).wrapping_mul(DEBRUIJN_MAGIC) >> 58) as usize])
}

#[bench_variant("ctz_debruijn", sizes = [256, 1024, 4096, 16384])]
fn run_ctz_debruijn<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let acc = sum_trailing_zeros(input, ctz_debruijn);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use ctz_common::{agrees_with_std, sum_trailing_zeros, varies};

    #[test]
    fn the_count_agrees_with_std() {
        agrees_with_std(super::ctz_debruijn).unwrap();
    }

    #[test]
    fn the_kernel_does_not_time_a_constant() {
        assert!(varies(|i| sum_trailing_zeros(i, super::ctz_debruijn)));
    }
}
