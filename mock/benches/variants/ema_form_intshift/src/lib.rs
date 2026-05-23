//! Variant: integer-shift EMA (lossy but cheap).
//!
//! `acc = acc - (acc >> 3) + (sample >> 3)` is the textbook IIR
//! formulation. Two shifts, one subtract, one add. No multiply at
//! all. The downside: each step loses three bits of `acc` precision,
//! and the input is also pre-quantised by `>> 3`, so for u8 samples
//! the effective input range shrinks to 5 bits.
//!
//! Worst-precision floor for the EMA-formulation bench. Sets the
//! lower bound on per-step cost; the Q0.32 and float variants pay
//! for higher precision.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("ema_form_intshift", sizes = [256, 1024, 4096, 16384])]
fn run_ema_form_intshift<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            // Widen sample by 16 bits so the >> 3 step doesn't crush
            // it down to zero; final scaling preserves an apples-to-
            // apples final-value range vs the Q0.32 variant.
            let mut acc: u64 = 0;
            for &byte in input.iter() {
                let sample: u64 = (byte as u64) << 16;
                acc = acc - (acc >> 3) + (sample >> 3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
