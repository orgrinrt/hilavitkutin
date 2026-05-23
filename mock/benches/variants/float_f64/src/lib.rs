//! Variant: f64 fused-multiply-add reduction.
//!
//! Same shape as `float_f32` (4-way ILP-broken) but in f64. NEON
//! 128-bit vector packs only 2 f64 lanes vs 4 f32 lanes, so peak
//! throughput per cycle halves at the SIMD level. Scalar VFMA has
//! identical latency for f32 and f64, so the gap surfaces only
//! when LLVM autovectorizes.
//!
//! Models the StrictFloat strategy: 53-bit mantissa precision at
//! the cost of half the SIMD throughput.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("float_f64", sizes = [256, 1024, 4096, 16384])]
fn run_float_f64<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut a0: f64 = 1.0;
            let mut a1: f64 = 1.0;
            let mut a2: f64 = 1.0;
            let mut a3: f64 = 1.0;
            let mut i = 0usize;
            while i + 4 <= N {
                let v0 = input[i] as f64;
                let v1 = input[i + 1] as f64;
                let v2 = input[i + 2] as f64;
                let v3 = input[i + 3] as f64;
                a0 = a0.mul_add(1.000001, v0);
                a1 = a1.mul_add(1.000001, v1);
                a2 = a2.mul_add(1.000001, v2);
                a3 = a3.mul_add(1.000001, v3);
                i += 4;
            }
            let acc = a0 + a1 + a2 + a3;
            let bits = acc.to_bits();
            output.copy_from_slice(&bits.to_le_bytes());
        }
    }
}
