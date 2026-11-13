//! Arm `std_parallel`: scoped std threads over `ceil(n / W)` chunks, spawned
//! and joined every frame, the fair parallel bar.

use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use stc_support::Conv;
use stc_support::arms::{StdParallel, call};

#[bench_variant(Conv, "std_parallel", sizes = [16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 134217728])]
fn run_std_parallel<const N: usize>(
    input: &<Conv<N> as Routine>::Input,
    output: &mut <Conv<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<StdParallel>(input, N, output)
}
