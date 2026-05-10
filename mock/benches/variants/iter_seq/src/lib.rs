//! Variant: sequential forward iteration `for i in 0..N { acc += col[i] }`.
//!
//! Best case for the hardware prefetcher: stride-1 forward access. Models
//! the canonical Column<T> morsel-loop access pattern.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("iter_seq", sizes = [256, 1024, 4096, 16384])]
fn run_iter_seq<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let words = N / 8;
            let p = input.as_ptr();
            for i in 0..words {
                let v = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
