//! Variant: `u64::min` / `u64::max` intrinsic on each step.
//!
//! Tracks running min and max of N u64 samples. The intrinsics
//! lower to CSEL (compare-and-select) on aarch64 - one cycle.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("minmax_intrinsic", sizes = [256, 1024, 4096, 16384])]
fn run_minmax_intrinsic<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut mn: u64 = u64::MAX;
            let mut mx: u64 = 0;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                mn = mn.min(v);
                mx = mx.max(v);
            }
            let acc = mn ^ mx;
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
