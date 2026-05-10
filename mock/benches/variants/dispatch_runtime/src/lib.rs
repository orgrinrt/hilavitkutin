//! Variant: input-dependent fn pointer table dispatch.
//!
//! Each iteration selects a fn pointer from a 4-entry table indexed by the low
//! 2 bits of the input chunk. The table contains four mixers that all produce
//! equivalent output for the bench's purposes (different prime, different
//! shift, etc.). LLVM cannot devirt because the index is data-dependent.
//!
//! This is the worst-case "we have N different work shapes and pick at runtime"
//! pattern. The branch predictor may help (input bytes are pseudo-random), but
//! the call itself is unavoidably indirect.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn mix0(chunk: u64, acc: u64) -> u64 {
    (acc ^ chunk).wrapping_mul(0x100000001b3)
}

#[inline(never)]
fn mix1(chunk: u64, acc: u64) -> u64 {
    acc.wrapping_add(chunk).wrapping_mul(0x9e3779b97f4a7c15)
}

#[inline(never)]
fn mix2(chunk: u64, acc: u64) -> u64 {
    (acc.rotate_left(13) ^ chunk).wrapping_mul(0xbf58476d1ce4e5b9)
}

#[inline(never)]
fn mix3(chunk: u64, acc: u64) -> u64 {
    (acc.wrapping_sub(chunk)) ^ (chunk.rotate_right(7))
}

static TABLE: [fn(u64, u64) -> u64; 4] = [mix0, mix1, mix2, mix3];

#[bench_variant("dispatch_runtime", sizes = [64, 256, 1024, 4096, 16384])]
fn run_dispatch_runtime<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let in_ptr = input.as_ptr();
            for i in 0..chunks {
                let chunk =
                    unsafe { (in_ptr.add(i * 8) as *const u64).read_unaligned() };
                // data-dependent index: LLVM cannot devirt.
                let idx = (chunk & 0b11) as usize;
                let mix = TABLE[idx];
                acc = mix(chunk, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
