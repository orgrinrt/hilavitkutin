//! Variant: float EMA.
//!
//! `acc = 0.875 * acc + 0.125 * sample`. Modern hardware lowers this
//! to one fused-multiply-add plus one multiply. The float hardware
//! beats fixed-point on multiply throughput but loses on precision
//! (f32 mantissa is 23 bits vs Q0.32's 32 bits).
//!
//! Baseline comparison for the Q0.32 saturating-fixed-point
//! formulation under round 202605101036 audit-2 M4.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_form_float", sizes = [256, 1024, 4096, 16384])]
fn run_ema_form_float<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: f32 = 0.0;
            for &byte in input.iter() {
                acc = 0.875_f32 * acc + 0.125_f32 * (byte as f32);
            }
            // pack final f32 + zero pad as 8 little-endian bytes
            let bits = acc.to_bits() as u64;
            output.copy_from_slice(&bits.to_le_bytes());
        }
    }
}
