//! Arm `per_core`: the engine's ceil slice with one partial per core, the
//! shape the engine runs today and the price's baseline. Its result moves
//! with the worker count.

use accum_group_support::Acc;
use accum_group_support::arms::{PerCoreArm, call};
use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Acc, "per_core", sizes = [4096, 65536, 1048576])]
fn run_per_core<const N: usize>(
    input: &<Acc<N> as Routine>::Input,
    output: &mut <Acc<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<PerCoreArm>(input, N, output)
}
