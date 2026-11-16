//! Arm `reg_bound_strided`: as `reg_bound_units`, and a body does not assume
//! its columns are packed. It steps each one by the stride the binding
//! declares, which is the addressing a column needs when it is a member of a
//! wider shaped store.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_bound_strided", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_bound_strided<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let layout = reg::layout(input);
    let bound = reg::bound_strided::bind(&layout);
    let r = timed! {
        run {
            reg::bound_strided::run(&bound);
        }
    };
    *output = reg::checksum(&layout);
    r
}
