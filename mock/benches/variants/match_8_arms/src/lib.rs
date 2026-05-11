//! Variant: `match` with 8 arms (dense small-range).
//!
//! Lowers to a jump table on aarch64 (ADR + LDR + BR). Constant-time
//! dispatch regardless of which arm is selected. Models plan-stage
//! enum dispatch when the enum has handful of variants (Phase / Stage
//! enums with 4-8 variants).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn step_8(v: u64, acc: u64) -> u64 {
    match v & 7 {
        0 => acc.wrapping_add(v),
        1 => acc ^ v,
        2 => acc.wrapping_sub(v),
        3 => acc.rotate_left(7) ^ v,
        4 => acc.wrapping_mul(v | 1),
        5 => acc & v,
        6 => acc | v,
        _ => acc.rotate_right(11) ^ v,
    }
}

#[bench_variant("match_8_arms", sizes = [256, 1024, 4096, 16384])]
fn run_match_8_arms<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = step_8(v, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
