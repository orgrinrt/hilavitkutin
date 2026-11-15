//! Arm `reg_bound_columns`: the units stay typed and monomorphised, and their
//! columns are declared from data. Each unit resolves its three columns
//! through the run-time binding table once a morsel, with the shape checked
//! before the cast back.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_bound_columns", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_bound_columns<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let layout = reg::layout(input);
    let bound = reg::bound_columns::bind(&layout);
    let r = timed! {
        run {
            reg::bound_columns::run(&bound);
        }
    };
    *output = reg::checksum(&layout);
    r
}
