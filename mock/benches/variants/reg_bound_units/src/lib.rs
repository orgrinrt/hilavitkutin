//! Arm `reg_bound_units`: units and columns both come from run-time tables.
//! The morsel loop calls a unit's body through the pointer in its row once a
//! morsel, and the body resolves its columns through the binding table and
//! then runs a typed loop.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_bound_units", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_bound_units<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let layout = reg::layout(input);
    let bound = reg::bound_units::bind(&layout);
    let r = timed! {
        run {
            reg::bound_units::run(&bound);
        }
    };
    *output = reg::checksum(&layout);
    r
}
