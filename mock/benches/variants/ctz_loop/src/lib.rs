//! Variant: explicit shift-and-test loop.
//!
//! Naive bit-by-bit trailing-zero count. O(tz) work per word: best
//! case 1 iteration (low bit set), worst case 64 iterations. The
//! variant is data-dependent on the input distribution.
//!
//! Worst case for ctz computation. Models what a "portable, no
//! intrinsics, no clever tricks" implementation would produce.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn ctz_loop(mut v: u64) -> u64 {
    let mut count: u64 = 0;
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
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                let tz = ctz_loop(v | 1);
                acc = acc.wrapping_add(tz);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
