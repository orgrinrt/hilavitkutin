//! Arm `kernel_nway`: the engine's N-way ceil slice on `W` walkers of the
//! bench's crew, so the split compares with head-and-tail on equal machinery.

use mockspace_bench_core::{FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;
use stc_support::Conv;
use stc_support::arms::{KernelNway, call};

#[bench_variant(Conv, "kernel_nway", sizes = [16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576, 4194304, 16777216, 134217728])]
fn run_kernel_nway<const N: usize>(
    input: &<Conv<N> as Routine>::Input,
    output: &mut <Conv<N> as Routine>::Output,
) -> FfiBenchCall {
    call::<KernelNway>(input, N, output)
}
