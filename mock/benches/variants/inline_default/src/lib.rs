//! Variant: per-record step function with no inline attribute.
//!
//! Lets LLVM decide. Under release + fat LTO, the optimiser will
//! almost certainly inline a small leaf fn into a hot loop, so this
//! variant is expected to match `inline_always` in practice.
//!
//! Captures the cost (if any) of relying on the optimiser's heuristic
//! instead of asserting the inline. Tells the design whether
//! `#[inline]` is worth writing on per-record step fns or whether
//! LLVM auto-inlines reliably under release.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

fn step(acc: u64, byte: u8) -> u64 {
    (acc ^ (byte as u64)).wrapping_mul(0x100000001b3)
}

#[bench_variant("inline_default", sizes = [256, 1024, 4096, 16384])]
fn run_inline_default<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
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
