//! Variant: const fn pointer dispatch.
//!
//! The fn pointer is const-known. LLVM should see through it and devirt to
//! a direct call. Tests whether `const FN: fn(...) = some_fn;` patterns
//! preserve the inlining path.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn work(chunk: u64, acc: u64) -> u64 {
    (acc ^ chunk).wrapping_mul(0x100000001b3)
}

const MIX: fn(u64, u64) -> u64 = work;

#[bench_variant("dispatch_const_ptr", sizes = [64, 256, 1024, 4096, 16384])]
fn run_dispatch_const_ptr<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let in_ptr = input.as_ptr();
            for i in 0..chunks {
                let chunk =
                    unsafe { (in_ptr.add(i * 8) as *const u64).read_unaligned() };
                acc = MIX(chunk, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
