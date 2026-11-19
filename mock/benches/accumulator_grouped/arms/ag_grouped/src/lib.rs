//! Arm `grouped`: option 3, the accumulator grouped by the data. Groups of
//! the section's width from record zero, one partial per group, each core's
//! slice rounded up to whole groups. Its result is the same at every worker
//! count.

use accum_group_support::Acc;
use accum_group_support::arms::{GroupedArm, call};
use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Acc, "grouped", sizes = [4096, 65536, 1048576])]
fn run_grouped<const N: usize>(
    input: &<Acc<N> as Routine>::Input,
    output: &mut <Acc<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<GroupedArm>(input, N, output)
}
