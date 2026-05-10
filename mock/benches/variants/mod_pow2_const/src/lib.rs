//! Variant: modulo by a const power-of-2.
//!
//! `idx % 64` where 64 is a const. LLVM lowers this to `idx & 63`
//! (one AND instruction). This is the empirical floor cost the
//! workspace's pervasive power-of-2 cap convention pays for.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("mod_pow2_const", sizes = [256, 1024, 4096, 16384])]
fn run_mod_pow2_const<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            const MOD: u64 = 64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = acc.wrapping_mul(0x100000001b3) ^ (byte as u64);
                acc = acc % MOD;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
