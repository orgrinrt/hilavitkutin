//! Variant: bit-trick branchless min/max.
//!
//! Uses signed-overflow subtract trick: cast to i64, take the sign
//! bit of the difference, use it as a mask to blend the two values.
//! Avoids CSEL by using arithmetic-and-shift only. On aarch64 the
//! optimiser may convert this back to CSEL anyway; the bench tells.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn min_branchless(a: u64, b: u64) -> u64 {
    // Treat both as i64 to get sign of (a - b); use that to mask between a and b.
    // Works without overflow concerns because we mask before extracting.
    let diff = (a as i128) - (b as i128);
    let pick_a = (diff >> 127) as u64; // all-ones if a<b, else zero
    (a & pick_a) | (b & !pick_a)
}

#[inline(always)]
fn max_branchless(a: u64, b: u64) -> u64 {
    let diff = (a as i128) - (b as i128);
    let pick_a = ((!diff) >> 127) as u64;
    (a & pick_a) | (b & !pick_a)
}

#[bench_variant("minmax_branchless", sizes = [256, 1024, 4096, 16384])]
fn run_minmax_branchless<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut mn: u64 = u64::MAX;
            let mut mx: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                mn = min_branchless(mn, v);
                mx = max_branchless(mx, v);
            }
            let acc = mn ^ mx;
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
