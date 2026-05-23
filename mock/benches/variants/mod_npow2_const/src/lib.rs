//! Variant: modulo by a const non-power-of-2.
//!
//! `idx % 60` where 60 is const. LLVM can fold this to a magic-number
//! multiply (libgcc-style reciprocal trick), but it still costs more
//! than the AND mask for power-of-2 moduli.
//!
//! Models the cost a non-power-of-2 design choice would pay vs the
//! pow2 choice the engine actually makes for MAX_FIBERS, MAX_CORES,
//! MAX_UNITS, MICRO_MORSEL_INTERVAL, etc.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("mod_npow2_const", sizes = [256, 1024, 4096, 16384])]
fn run_mod_npow2_const<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            const MOD: u64 = 60;
            let mut acc: u64 = 0xcbf29ce484222325;
            for &byte in input.iter() {
                acc = acc.wrapping_mul(0x100000001b3) ^ (byte as u64);
                acc = acc % MOD;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
