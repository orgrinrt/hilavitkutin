//! Variant: NEON intrinsics path for EMA streaming.
//!
//! Reads N input bytes as N/4 u32 samples, processes them in groups
//! of 4 through a v4u32 NEON EMA update (`mla.4s` + `ushr.4s`), and
//! writes the first two u32s of the final 4-lane accumulator state
//! as the 8-byte output. All three variants produce bitwise-identical
//! output for identical input because the underlying integer math
//! does not vary.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_neon", sizes = [64, 256, 1024, 4096, 16384])]
fn run_ema_neon<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            unsafe {
                use core::arch::aarch64::*;
                let samples = N / 4;
                let mut acc = vdupq_n_u32(0);
                let seven = vdupq_n_u32(7);
                let mut i: usize = 0;
                let in_ptr = input.as_ptr();
                while i + 4 <= samples {
                    let s = vld1q_u32(in_ptr.add(i * 4) as *const u32);
                    // ema: new = (old*7 + sample) >> 3
                    let s_plus_7old = vmlaq_u32(s, acc, seven);
                    acc = vshrq_n_u32(s_plus_7old, 3);
                    i += 4;
                }
                // tail samples (when N/4 % 4 != 0); fold into lane 0
                while i < samples {
                    let s = (in_ptr.add(i * 4) as *const u32).read_unaligned();
                    let lane0 = vgetq_lane_u32(acc, 0);
                    let new0 = (lane0.wrapping_mul(7).wrapping_add(s)) >> 3;
                    acc = vsetq_lane_u32(new0, acc, 0);
                    i += 1;
                }
                // write first two u32s of acc as output bytes
                let l0 = vgetq_lane_u32(acc, 0);
                let l1 = vgetq_lane_u32(acc, 1);
                output[0..4].copy_from_slice(&l0.to_le_bytes());
                output[4..8].copy_from_slice(&l1.to_le_bytes());
            }
            #[cfg(not(all(target_arch = "aarch64", target_feature = "neon")))]
            {
                // Fallback: scalar 4-lane update so the cdylib still
                // builds on non-NEON targets. Not the path we measure.
                let samples = N / 4;
                let mut acc = [0u32; 4];
                let mut i: usize = 0;
                let in_ptr = input.as_ptr();
                while i < samples {
                    let s = unsafe {
                        (in_ptr.add(i * 4) as *const u32).read_unaligned()
                    };
                    let lane = i % 4;
                    acc[lane] = (acc[lane].wrapping_mul(7).wrapping_add(s)) >> 3;
                    i += 1;
                }
                output[0..4].copy_from_slice(&acc[0].to_le_bytes());
                output[4..8].copy_from_slice(&acc[1].to_le_bytes());
            }
        }
    }
}
