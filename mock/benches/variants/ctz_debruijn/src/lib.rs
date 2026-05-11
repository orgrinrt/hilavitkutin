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

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const DEBRUIJN_MAGIC: u64 = 0x03f79d71b4cb0a89;
const DEBRUIJN_TABLE: [u8; 64] = [
    0, 1, 56, 2, 57, 49, 28, 3, 61, 58, 42, 50, 38, 29, 17, 4,
    62, 47, 59, 36, 45, 43, 51, 22, 53, 39, 33, 30, 24, 18, 12, 5,
    63, 55, 48, 27, 60, 41, 37, 16, 46, 35, 44, 21, 52, 32, 23, 11,
    54, 26, 40, 15, 34, 20, 31, 10, 25, 14, 19, 9, 13, 8, 7, 6,
];

#[inline(always)]
fn ctz_debruijn(v: u64) -> u64 {
    DEBRUIJN_TABLE[(((v & v.wrapping_neg()).wrapping_mul(DEBRUIJN_MAGIC)) >> 58) as usize] as u64
}

#[bench_variant("ctz_debruijn", sizes = [256, 1024, 4096, 16384])]
fn run_ctz_debruijn<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                let tz = ctz_debruijn(v | 1);
                acc = acc.wrapping_add(tz);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
