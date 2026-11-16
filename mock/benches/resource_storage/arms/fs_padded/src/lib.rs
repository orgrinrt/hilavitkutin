//! Axis E arm fs_padded: every thread writes its own slot, the slots each on a line of its own.
//! See `rsb_kernels::false_sharing`. N sets the writes per thread, `N * 1024`;
//! the input bytes are not read.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use rsb_kernels::false_sharing;

#[bench_variant("fs_padded", sizes = [1024, 16384, 65536])]
fn run_fs_padded<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let _ = input;
    let mut sum = 0u64;
    let r = timed! { run { sum = false_sharing::pass(false, (N as u64) * 1024); } };
    output.copy_from_slice(&sum.to_le_bytes());
    r
}
