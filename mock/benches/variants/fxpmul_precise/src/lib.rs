//! Variant: fixed-point multiplication with i128 intermediate (Precise).
//!
//! Each multiply widens to i128 to capture full product, then arithmetic-right-
//! shifts to recover the fixed-point result. Loses no precision, costs more
//! than i64 intrinsic mul. This is arvo's `Strategy::Precise` codegen path.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const FBITS: u32 = 16; // Q16 fixed-point

#[inline(never)]
fn fxp_mul_precise(a: i64, b: i64) -> i64 {
    let prod: i128 = (a as i128) * (b as i128);
    (prod >> FBITS) as i64
}

#[bench_variant("fxpmul_precise", sizes = [64, 256, 1024, 4096, 16384])]
fn run_fxpmul_precise<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let blocks = N / 16;
            let p = input.as_ptr();
            let mut acc: i64 = 1 << FBITS; // 1.0 in Q16
            for i in 0..blocks {
                let a = unsafe { (p.add(i * 16) as *const i64).read_unaligned() };
                let b = unsafe { (p.add(i * 16 + 8) as *const i64).read_unaligned() };
                acc = fxp_mul_precise(acc.wrapping_add(a), b);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
