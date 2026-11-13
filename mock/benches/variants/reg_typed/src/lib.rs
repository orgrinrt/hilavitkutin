//! Arm `reg_typed`: typed registration. The two unit bodies are monomorphised
//! into the morsel loop and their column pointers were resolved once. The
//! baseline of the registration bench.
//!
//! The columns are built from the seeded input before the timed block, and
//! the 8-byte output is the FNV-1a checksum of the `out` column, which the
//! harness compares byte for byte across the arms.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_typed", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_typed<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let layout = reg::layout(input);
    let bound = reg::typed::bind(&layout);
    let r = timed! {
        run {
            reg::typed::run(&bound);
        }
    };
    *output = reg::checksum(&layout);
    r
}
