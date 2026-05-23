//! Variant: direct fn call. The baseline both bench groups compare against.
//!
//! The work payload (FNV1a-style mix of an 8-byte chunk into a u64 accumulator)
//! is identical across all five dispatch variants. Only the call shape varies.
//! Direct: no indirection. LLVM has full visibility, can inline freely.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn work(chunk: u64, acc: u64) -> u64 {
    // fnv1a-style mix: xor then multiply
    (acc ^ chunk).wrapping_mul(0x100000001b3)
}

#[bench_variant("dispatch_direct", sizes = [64, 256, 1024, 4096, 16384])]
fn run_dispatch_direct<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let in_ptr = input.as_ptr();
            for i in 0..chunks {
                let chunk =
                    unsafe { (in_ptr.add(i * 8) as *const u64).read_unaligned() };
                acc = work(chunk, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
