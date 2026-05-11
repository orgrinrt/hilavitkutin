//! Variant: const-amount right shift.
//!
//! `acc >>= 13` with 13 const. Lowers to `LSR xN, xN, #13` on
//! aarch64 — single immediate-form shift, one cycle. Baseline.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("shr_const", sizes = [256, 1024, 4096, 16384])]
fn run_shr_const<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = acc ^ (v >> 13);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
