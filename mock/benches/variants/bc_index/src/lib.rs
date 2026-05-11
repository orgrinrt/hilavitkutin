//! Variant: explicit `input[i]` indexed loop.
//!
//! Safe Rust indexing in a `0..N` range. LLVM should recognise that
//! `i < N` is the loop condition and elide the per-element bounds
//! check. If the elision fails, this variant pays an extra compare +
//! conditional-branch per iteration.
//!
//! Models the consumer-WorkUnit pattern where the morsel index is an
//! integer carried by the loop, not an iterator.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[bench_variant("bc_index", sizes = [256, 1024, 4096, 16384])]
fn run_bc_index<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..N {
                let byte = input[i];
                acc = (acc ^ (byte as u64)).wrapping_mul(0x100000001b3);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
