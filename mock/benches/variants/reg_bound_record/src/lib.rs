//! Arm `reg_bound_record`: units and columns both come from run-time tables,
//! and nothing hoists the lookup. The loop calls a unit's body through the
//! pointer in its row once a record, and the body resolves its columns
//! through the binding table each time.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_bound_record", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_bound_record<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let layout = reg::layout(input);
    let bound = reg::bound_record::bind(&layout);
    let r = timed! {
        run {
            reg::bound_record::run(&bound);
        }
    };
    *output = reg::checksum(&layout);
    r
}
