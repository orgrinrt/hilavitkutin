//! Axis E arm fs_packed: every thread writes its own slot, the slots packed into one cache line.
//! See `rsb_kernels::false_sharing`. N sets the writes per thread, `N * 1024`;
//! the input bytes are not read.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::false_sharing;

#[bench_variant("fs_packed", sizes = [1024, 16384, 65536])]
fn run_fs_packed<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let _ = input;
    let mut sum = 0u64;
    let r = timed! { run { sum = false_sharing::pass(true, (N as u64) * 1024); } };
    output.copy_from_slice(&sum.to_le_bytes());
    r
}
