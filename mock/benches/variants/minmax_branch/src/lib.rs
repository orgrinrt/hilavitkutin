//! Variant: explicit if/else min/max.
//!
//! LLVM is expected to lower the if/else to the same CSEL as the
//! intrinsic, but the bench confirms whether the user-side syntax
//! choice has any throughput consequence.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("minmax_branch", sizes = [256, 1024, 4096, 16384])]
fn run_minmax_branch<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut mn: u64 = u64::MAX;
            let mut mx: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                mn = if v < mn { v } else { mn };
                mx = if v > mx { v } else { mx };
            }
            let acc = mn ^ mx;
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
