//! Arm `head_tail_dyn`: two walkers from opposite ends claiming morsel
//! blocks from one shared cursor, the meeting point floating, on the bench's
//! crew.

use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use stc_support::Conv;
use stc_support::arms::{HeadTailDynamic, call};

#[bench_variant(Conv, "head_tail_dyn", sizes = [16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 134217728])]
fn run_head_tail_dyn<const N: usize>(
    input: &<Conv<N> as Routine>::Input,
    output: &mut <Conv<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<HeadTailDynamic>(input, N, output)
}
