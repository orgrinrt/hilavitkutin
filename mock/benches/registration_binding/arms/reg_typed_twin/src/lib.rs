//! Arm `reg_typed_twin`: the typed arm a second time, line for line. It
//! measures nothing of its own. The distance between it and `reg_typed` is
//! what this host and this harness put between two runs of one program, and
//! a difference between two other arms that is no larger than that distance
//! is not a difference the bench has shown.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;
use registration_kernels as reg;

#[bench_variant("reg_typed_twin", sizes = [4096, 16384, 65536, 262144])]
fn run_reg_typed_twin<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
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
