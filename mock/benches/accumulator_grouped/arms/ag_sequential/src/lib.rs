//! Arm `sequential`: one walker folds every record, whatever the section's
//! worker count. The canon's answer for a fold that is not commutative, and
//! the one arm whose result is the left fold of the records.

use accum_group_support::Acc;
use accum_group_support::arms::{SequentialArm, call};
use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(Acc, "sequential", sizes = [4096, 65536, 1048576])]
fn run_sequential<const N: usize>(
    input: &<Acc<N> as Routine>::Input,
    output: &mut <Acc<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<SequentialArm>(input, N, output)
}
