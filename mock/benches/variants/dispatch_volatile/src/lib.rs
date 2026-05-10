//! Variant: opaque fn pointer dispatch.
//!
//! The fn pointer is loaded via `read_volatile` before each call. LLVM cannot
//! see through the load and is forced to emit an indirect call every time.
//! This represents the codegen pattern hilavitkutin's static-composition rule
//! is meant to avoid: dispatch where the compiler has no visibility.

use core::ptr;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(never)]
fn work(chunk: u64, acc: u64) -> u64 {
    (acc ^ chunk).wrapping_mul(0x100000001b3)
}

static MIX_SLOT: fn(u64, u64) -> u64 = work;

#[bench_variant("dispatch_volatile", sizes = [64, 256, 1024, 4096, 16384])]
fn run_dispatch_volatile<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let in_ptr = input.as_ptr();
            let slot_ptr: *const fn(u64, u64) -> u64 = &MIX_SLOT;
            for i in 0..chunks {
                let chunk =
                    unsafe { (in_ptr.add(i * 8) as *const u64).read_unaligned() };
                let mix: fn(u64, u64) -> u64 = unsafe { ptr::read_volatile(slot_ptr) };
                acc = mix(chunk, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
