//! Variant: 16-arm `match` dispatch.
//!
//! Control. Validates the match_arm_count finding's slope between
//! 8 arms and 64 arms. Each arm body computes a distinct mix of
//! ops to defeat LLVM arm-folding.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn step_match_16(v: u64, acc: u64) -> u64 {
    match v & 15 {
        0 => acc.wrapping_add(v ^ 0xA1),
        1 => acc ^ v.wrapping_mul(0xB1),
        2 => acc.wrapping_sub(v ^ 0xC1),
        3 => acc.rotate_left(1) ^ v,
        4 => acc.wrapping_mul((v | 1) ^ 0xD1),
        5 => acc & v.wrapping_add(0xE1),
        6 => acc | v.wrapping_add(0xF1),
        7 => acc.rotate_right(2) ^ v,
        8 => acc.wrapping_add(v ^ 0xA2),
        9 => acc ^ v.wrapping_mul(0xB2),
        10 => acc.wrapping_sub(v ^ 0xC2),
        11 => acc.rotate_left(3) ^ v,
        12 => acc.wrapping_mul((v | 1) ^ 0xD2),
        13 => acc & v.wrapping_add(0xE2),
        14 => acc | v.wrapping_add(0xF2),
        _ => acc.rotate_right(4) ^ v,
    }
}

#[bench_variant("fpt_match_16", sizes = [256, 1024, 4096, 16384])]
fn run_fpt_match_16<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = step_match_16(v, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
