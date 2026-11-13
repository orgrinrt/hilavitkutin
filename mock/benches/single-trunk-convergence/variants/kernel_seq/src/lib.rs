//! Arm `kernel_seq`: one walker on the bench's crew over the whole fiber,
//! the single-core anchor on the machinery the head-and-tail arms use.

use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use stc_support::Conv;
use stc_support::arms::{KernelSeq, call};

#[bench_variant(Conv, "kernel_seq", sizes = [16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 134217728])]
fn run_kernel_seq<const N: usize>(
    input: &<Conv<N> as Routine>::Input,
    output: &mut <Conv<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<KernelSeq>(input, N, output)
}
