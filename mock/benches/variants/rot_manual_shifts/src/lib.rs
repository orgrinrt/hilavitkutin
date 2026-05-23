//! Variant: hand-rolled `(v << K) | (v >> (64-K))`.
//!
//! Tests whether LLVM recognises the textbook rotate idiom and
//! lowers it to the same `ROR` instruction as the intrinsic. If
//! it does: identical performance to `rot_intrinsic_const`. If it
//! does not: three instructions (LSL + LSR + ORR) and roughly 2x
//! cycles per step.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn rotl_manual(v: u64, k: u32) -> u64 {
    (v << k) | (v >> (64 - k))
}

#[bench_variant("rot_manual_shifts", sizes = [256, 1024, 4096, 16384])]
fn run_rot_manual_shifts<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = rotl_manual(acc, 13) ^ v;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
