//! Variant: Q0.32 saturating fixed-point EMA.
//!
//! Models the arvo Norm-based EMA formulation from round 202605101036
//! audit-2 M4. Alpha = 1/8 represented as `0x20000000` in Q0.32;
//! complement = 7/8 as `0xE0000000`. The math is:
//!
//!   acc = (acc * one_minus_alpha + sample * alpha) >> 32
//!
//! `acc` and `sample` are Q0.32; sample is the input byte shifted into
//! the high 8 bits of the Q0.32 word (value range [0, 0xFF000000]).
//!
//! Validates the cost of saturating fixed-point EMA against float and
//! integer-shift formulations.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_form_q32", sizes = [256, 1024, 4096, 16384])]
fn run_ema_form_q32<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            const ALPHA: u32 = 0x2000_0000;          // 1/8 in Q0.32
            const ONE_MINUS_ALPHA: u32 = 0xE000_0000; // 7/8 in Q0.32
            let mut acc: u32 = 0;
            for &byte in input.iter() {
                let sample_q: u32 = (byte as u32) << 24; // u8 -> Q0.32 high byte
                let prod_acc = (acc as u64).wrapping_mul(ONE_MINUS_ALPHA as u64);
                let prod_sample = (sample_q as u64).wrapping_mul(ALPHA as u64);
                acc = ((prod_acc.wrapping_add(prod_sample)) >> 32) as u32;
            }
            let final_val = acc as u64;
            output.copy_from_slice(&final_val.to_le_bytes());
        }
    }
}
