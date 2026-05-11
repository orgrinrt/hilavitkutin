//! Variant: `match` with 2 arms (binary discriminant).
//!
//! Lowers to a single conditional branch on aarch64. Baseline cost
//! for plan-stage enum dispatch when the enum has only two variants.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn step_2(v: u64, acc: u64) -> u64 {
    match v & 1 {
        0 => acc.wrapping_add(v),
        _ => acc ^ v,
    }
}

#[bench_variant("match_2_arms", sizes = [256, 1024, 4096, 16384])]
fn run_match_2_arms<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = step_2(v, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
