//! Arm `head_tail`: two walkers from opposite ends, meeting at the fixed
//! `RecordRange` midpoint, on the bench's crew. The scheduler never emits the
//! head and tail ranges, so this arm runs outside it.

use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use stc_support::Conv;
use stc_support::arms::{HeadTail, call};

#[bench_variant(Conv, "head_tail", sizes = [16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 134217728])]
fn run_head_tail<const N: usize>(
    input: &<Conv<N> as Routine>::Input,
    output: &mut <Conv<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<HeadTail>(input, N, output)
}
