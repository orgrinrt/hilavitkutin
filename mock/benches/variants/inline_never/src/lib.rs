//! Variant: per-record step function with `#[inline(never)]`.
//!
//! Forces a real function call per record. The loop body becomes a
//! call boundary: prologue, BL, epilogue per iteration. Models what
//! a WorkUnit-as-dyn-fn shape would cost; also models what a generic
//! WorkUnit whose monomorphisation barrier prevents inlining would
//! degrade to.
//!
//! Worst case for the inline-strategy bench. Quantifies the gap that
//! `#[inline]` on hot step fns prevents.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn step(acc: u64, byte: u8) -> u64 {
    (acc ^ (byte as u64)).wrapping_mul(0x100000001b3)
}

#[bench_variant("inline_never", sizes = [256, 1024, 4096, 16384])]
fn run_inline_never<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = step(acc, byte);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
