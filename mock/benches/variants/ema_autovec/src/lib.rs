//! Variant: autovec-hopeful path.
//!
//! Idiomatic loop, no intrinsics, no explicit unrolling. If LLVM
//! decides to autovectorise the body, we get the SIMD win for free;
//! if not, we measure the gap to the explicit NEON variant. Same
//! math and lane layout as the scalar variant: 4 EMA accumulators,
//! sample i updates lane i%4.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_autovec", sizes = [64, 256, 1024, 4096, 16384])]
fn run_ema_autovec<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let samples = N / 4;
            // Reinterpret the input bytes as u32 samples. Safe because
            // [u8; N] is byte-aligned and N is always a multiple of 4
            // for our manifest sizes; we read unaligned to stay portable.
            let mut acc = [0u32; 4];
            for i in 0..samples {
                let s = unsafe {
                    (input.as_ptr().add(i * 4) as *const u32).read_unaligned()
                };
                let lane = i % 4;
                acc[lane] = (acc[lane].wrapping_mul(7).wrapping_add(s)) >> 3;
            }
            output[0..4].copy_from_slice(&acc[0].to_le_bytes());
            output[4..8].copy_from_slice(&acc[1].to_le_bytes());
        }
    }
}
