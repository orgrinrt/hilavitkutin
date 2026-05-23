//! Variant: modulo by an opaque (non-const) divisor.
//!
//! Divisor lives in a `#[inline(never)]` getter so LLVM cannot fold
//! it to a magic-number multiply or to a mask. Each `%` lowers to
//! a real `udiv` + `msub` (aarch64) sequence with full divider
//! latency.
//!
//! Models the cost a runtime-loaded cap would pay (e.g. if MAX_FIBERS
//! came from a `Resource<RunCfg>` field read each iteration instead
//! of being a const generic on PoolFrame).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn opaque_divisor() -> u64 {
    // The function body computes 64 at runtime in a way LLVM cannot
    // see through. `#[inline(never)]` blocks the call from inlining;
    // the multiplication keeps the value out of constant prop.
    let a = core::hint::black_box(8_u64);
    let b = core::hint::black_box(8_u64);
    a * b
}

#[bench_variant("mod_var", sizes = [256, 1024, 4096, 16384])]
fn run_mod_var<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let modulus = opaque_divisor();
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = acc.wrapping_mul(0x100000001b3) ^ (byte as u64);
                acc = acc % modulus;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
