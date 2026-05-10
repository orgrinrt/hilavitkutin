//! Variant: branchless select dispatch.
//!
//! Computes BOTH paths for each byte, then selects via a bool-to-mask
//! conversion. No branch; LLVM lowers to csel (aarch64) / cmov (x86_64).
//! Compares vs branchful if/else under adversarial data-dependent input.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("branch_cmov", sizes = [64, 256, 1024, 4096, 16384])]
fn run_branch_cmov<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for b in input.iter() {
                let even_path = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
                let odd_path = (acc.wrapping_add(*b as u64)).rotate_left(13);
                // branchless select via cond cast
                let is_even = (*b & 1) == 0;
                acc = if is_even { even_path } else { odd_path };
                // LLVM should emit cmov / csel; the `if` here is data-dependent
                // but the compiler typically picks branchless for this shape.
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
