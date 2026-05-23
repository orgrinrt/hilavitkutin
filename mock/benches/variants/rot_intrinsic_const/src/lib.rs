//! Variant: `rotate_left` intrinsic with const amount.
//!
//! On aarch64 with const amount, LLVM emits a single `ROR` (rotate
//! right) with the negated amount. One instruction, one cycle.
//! Baseline for the rotate bench.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("rot_intrinsic_const", sizes = [256, 1024, 4096, 16384])]
fn run_rot_intrinsic_const<const N: usize>(
    input: &[u8; N],
    output: &mut [u8; 8],
) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = acc.rotate_left(13) ^ v;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
