//! Variant: aligned u64 loads via direct ptr cast.
//!
//! `*const u64` cast from `*const u8` and dereferenced. On aarch64
//! the hardware permits this without fault even if alignment is
//! technically violated (Rust language considers this UB, but the
//! ISA load instruction tolerates it). LLVM lowers to a plain LDR
//! with no alignment guard.
//!
//! Best case for per-element load throughput on a 64-bit word stream.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("load_aligned", sizes = [256, 1024, 4096, 16384])]
fn run_load_aligned<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let mut acc: u64 = 0xcbf29ce484222325;
            let base = input.as_ptr() as *const u64;
            for i in 0..words {
                let v = unsafe { *base.add(i) };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
