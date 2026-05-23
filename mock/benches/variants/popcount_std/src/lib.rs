//! Variant: std `u64::count_ones()` (lowers to POPCNT on x86_64 / NEON CNT on aarch64).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("popcount_std", sizes = [64, 256, 1024, 4096, 16384])]
fn run_popcount_std<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut total: u64 = 0;
            let words = N / 8;
            let p = input.as_ptr();
            for i in 0..words {
                let w = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                total = total.wrapping_add(w.count_ones() as u64);
            }
            output.copy_from_slice(&total.to_le_bytes());
        }
    }
}
