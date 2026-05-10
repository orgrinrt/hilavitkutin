//! Variant: Brian Kernighan's loop popcount.
//!
//! Loops `n &= n - 1` until n == 0, counting iterations. Branch-heavy; cheap
//! when bits are sparse, expensive when dense. Useful comparison vs hardware
//! POPCNT to see how much the dedicated instruction matters.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn popcount_kernighan(mut x: u64) -> u32 {
    let mut c: u32 = 0;
    while x != 0 {
        x &= x.wrapping_sub(1);
        c += 1;
    }
    c
}

#[bench_variant("popcount_handrolled", sizes = [64, 256, 1024, 4096, 16384])]
fn run_popcount_handrolled<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut total: u64 = 0;
            let words = N / 8;
            let p = input.as_ptr();
            for i in 0..words {
                let w = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                total = total.wrapping_add(popcount_kernighan(w) as u64);
            }
            output.copy_from_slice(&total.to_le_bytes());
        }
    }
}
