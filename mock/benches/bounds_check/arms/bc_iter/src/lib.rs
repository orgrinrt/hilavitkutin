//! Variant: `for &byte in bytes`, the iterator, which needs no check.
//!
//! Reads the input through `bc_common::opaque`, whose length the compiler
//! cannot see, and sums its bytes with independent adds (see `bc_common`).

use bc_common::opaque;
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

/// The byte sum, read the way this arm reads.
#[inline(always)]
fn kernel<const N: usize>(input: &[u8; N]) -> u64 {
    let bytes = opaque(input);
    let mut acc: u64 = 0;
    for &byte in bytes {
        acc += u64::from(byte);
    }
    acc
}

#[bench_variant("bc_iter", sizes = [256, 1024, 4096, 16384])]
fn run_bc_iter<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let acc = kernel(input);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_kernel_sums_its_input() {
        bc_common::check(super::kernel).unwrap();
    }
}
