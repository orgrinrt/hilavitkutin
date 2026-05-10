//! Variant: fixed-point multiplication with i64-only intermediate (Hot).
//!
//! No widening; accepts overflow at the high end. Cheapest possible mul-shift.
//! This is arvo's `Strategy::Hot` codegen path: fast but lossy on extreme
//! values. MAY_DIFFER=true vs Precise variant (output diverges on overflow).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const FBITS: u32 = 16;

#[inline(never)]
fn fxp_mul_hot(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b) >> FBITS
}

#[bench_variant("fxpmul_hot", sizes = [64, 256, 1024, 4096, 16384])]
fn run_fxpmul_hot<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let blocks = N / 16;
            let p = input.as_ptr();
            let mut acc: i64 = 1 << FBITS;
            for i in 0..blocks {
                let a = unsafe { (p.add(i * 16) as *const i64).read_unaligned() };
                let b = unsafe { (p.add(i * 16 + 8) as *const i64).read_unaligned() };
                acc = fxp_mul_hot(acc.wrapping_add(a), b);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
