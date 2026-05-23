//! Variant: integer fixed-point multiply-accumulate at u64 width.
//!
//! Same algorithm shape (4-way ILP-broken FMA) but using integer
//! arithmetic with a fixed scaling constant. This is the path arvo
//! UFixed<I, F, Hot> takes: scale + integer multiply + add, no
//! FPU. NEON packs 2 u64 lanes per vector (same as f64) but the
//! integer multiplier is sometimes faster than FMA on Apple Silicon.
//!
//! Models the case where arvo's fixed-point primitives substitute
//! for float ones entirely.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("float_i64_scaled", sizes = [256, 1024, 4096, 16384])]
fn run_float_i64_scaled<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            // Fixed-point: scaling factor representing 1.000001 in Q32.32.
            // 1.000001 * 2^32 ~ 4_294_972_590
            const MUL_FIXED: u64 = 4_294_972_590;
            const ONE_FIXED: u64 = 1u64 << 32;
            let mut a0: u64 = ONE_FIXED;
            let mut a1: u64 = ONE_FIXED;
            let mut a2: u64 = ONE_FIXED;
            let mut a3: u64 = ONE_FIXED;
            let mut i = 0usize;
            while i + 4 <= N {
                let v0 = (input[i] as u64) << 32;
                let v1 = (input[i + 1] as u64) << 32;
                let v2 = (input[i + 2] as u64) << 32;
                let v3 = (input[i + 3] as u64) << 32;
                a0 = a0.wrapping_mul(MUL_FIXED).wrapping_shr(32).wrapping_add(v0);
                a1 = a1.wrapping_mul(MUL_FIXED).wrapping_shr(32).wrapping_add(v1);
                a2 = a2.wrapping_mul(MUL_FIXED).wrapping_shr(32).wrapping_add(v2);
                a3 = a3.wrapping_mul(MUL_FIXED).wrapping_shr(32).wrapping_add(v3);
                i += 4;
            }
            let acc = a0.wrapping_add(a1).wrapping_add(a2).wrapping_add(a3);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
