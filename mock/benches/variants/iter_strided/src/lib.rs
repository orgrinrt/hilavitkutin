//! Variant: strided iteration with stride 8 (cache-line skipping).
//!
//! Reads every 8th u64. On a 64-byte cache line (8 u64s per line), this
//! touches every cache line exactly once but skips 7 of 8 useful u64s
//! per line. Hardware prefetcher still helps (stride detection works on
//! constant strides) but wastes 7/8 of fetched bytes.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("iter_strided", sizes = [256, 1024, 4096, 16384])]
fn run_iter_strided<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let words = N / 8;
            let p = input.as_ptr();
            let mut i = 0usize;
            while i < words {
                let v = unsafe { (p.add(i * 8) as *const u64).read_unaligned() };
                acc = (acc ^ v).wrapping_mul(0x100000001b3);
                i += 8;
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
