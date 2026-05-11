//! Variant: 16-entry fn-pointer table dispatch.
//!
//! Each handler is its own function. Dispatch through a static
//! `[fn; 16]` table indexed by `v & 15`. Single indirect branch
//! per step.
//!
//! Models the "refactor to fn-pointer table" alternative recommended
//! by match_arm_count for >12-variant hot dispatch.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

// Each handler has the same shape as one arm of the match variant.
fn h0(v: u64, acc: u64) -> u64 { acc.wrapping_add(v ^ 0xA1) }
fn h1(v: u64, acc: u64) -> u64 { acc ^ v.wrapping_mul(0xB1) }
fn h2(v: u64, acc: u64) -> u64 { acc.wrapping_sub(v ^ 0xC1) }
fn h3(v: u64, acc: u64) -> u64 { acc.rotate_left(1) ^ v }
fn h4(v: u64, acc: u64) -> u64 { acc.wrapping_mul((v | 1) ^ 0xD1) }
fn h5(v: u64, acc: u64) -> u64 { acc & v.wrapping_add(0xE1) }
fn h6(v: u64, acc: u64) -> u64 { acc | v.wrapping_add(0xF1) }
fn h7(v: u64, acc: u64) -> u64 { acc.rotate_right(2) ^ v }
fn h8(v: u64, acc: u64) -> u64 { acc.wrapping_add(v ^ 0xA2) }
fn h9(v: u64, acc: u64) -> u64 { acc ^ v.wrapping_mul(0xB2) }
fn h10(v: u64, acc: u64) -> u64 { acc.wrapping_sub(v ^ 0xC2) }
fn h11(v: u64, acc: u64) -> u64 { acc.rotate_left(3) ^ v }
fn h12(v: u64, acc: u64) -> u64 { acc.wrapping_mul((v | 1) ^ 0xD2) }
fn h13(v: u64, acc: u64) -> u64 { acc & v.wrapping_add(0xE2) }
fn h14(v: u64, acc: u64) -> u64 { acc | v.wrapping_add(0xF2) }
fn h15(v: u64, acc: u64) -> u64 { acc.rotate_right(4) ^ v }

static TABLE: [fn(u64, u64) -> u64; 16] = [
    h0, h1, h2, h3, h4, h5, h6, h7, h8, h9, h10, h11, h12, h13, h14, h15,
];

#[bench_variant("fpt_table_16", sizes = [256, 1024, 4096, 16384])]
fn run_fpt_table_16<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                let idx = (v & 15) as usize;
                acc = TABLE[idx](v, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
