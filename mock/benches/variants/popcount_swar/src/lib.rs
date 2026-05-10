//! Variant: SWAR (SIMD within a register) popcount.
//!
//! Branchless, constant-time. Uses the standard 7-instruction "1101010101" mask
//! sequence. Compares vs hardware POPCNT and Kernighan loop. Tests whether
//! LLVM will recognise the SWAR pattern and lower to POPCNT anyway (it usually
//! does — this is also useful evidence about LLVM's pattern-matching strength).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn popcount_swar(x: u64) -> u32 {
    let m1: u64 = 0x5555555555555555;
    let m2: u64 = 0x3333333333333333;
    let m4: u64 = 0x0f0f0f0f0f0f0f0f;
    let h01: u64 = 0x0101010101010101;
    let mut x = x;
    x -= (x >> 1) & m1;
    x = (x & m2) + ((x >> 2) & m2);
    x = (x.wrapping_add(x >> 4)) & m4;
    ((x.wrapping_mul(h01)) >> 56) as u32
}

#[bench_variant("popcount_swar", sizes = [64, 256, 1024, 4096, 16384])]
fn run_popcount_swar<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut total: u64 = 0;
            let words = N / 8;
            let p = input.as_ptr();
            for i in 0..words {
                let w = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                total = total.wrapping_add(popcount_swar(w) as u64);
            }
            output.copy_from_slice(&total.to_le_bytes());
        }
    }
}
