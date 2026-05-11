//! Variant: f32 fused-multiply-add reduction.
//!
//! Quad-way ILP-broken accumulator (per fold_strategy bench finding)
//! to expose throughput rather than latency. NEON can pack 4 f32
//! lanes per 128-bit vector; on aarch64 each VFMA is 4-cycle latency
//! / 1 per cycle throughput.
//!
//! Models the FastFloat strategy: accept reduced precision (24-bit
//! mantissa) for higher throughput.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("float_f32", sizes = [256, 1024, 4096, 16384])]
fn run_float_f32<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut a0: f32 = 1.0;
            let mut a1: f32 = 1.0;
            let mut a2: f32 = 1.0;
            let mut a3: f32 = 1.0;
            let mut i = 0usize;
            while i + 4 <= N {
                let v0 = input[i] as f32;
                let v1 = input[i + 1] as f32;
                let v2 = input[i + 2] as f32;
                let v3 = input[i + 3] as f32;
                a0 = a0.mul_add(1.000001, v0);
                a1 = a1.mul_add(1.000001, v1);
                a2 = a2.mul_add(1.000001, v2);
                a3 = a3.mul_add(1.000001, v3);
                i += 4;
            }
            let acc = a0 + a1 + a2 + a3;
            let bits = acc.to_bits() as u64;
            output.copy_from_slice(&bits.to_le_bytes());
        }
    }
}
