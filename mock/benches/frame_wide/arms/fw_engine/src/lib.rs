//! `engine`: the frame as twelve work units on `Scheduler::run`, single core.

use frame_wide_support::{ArmKind, FrameInput, FrameOut, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("engine", sizes = [4096, 65536, 1048576, 4194304, 16777216])]
fn run<const N: usize>(input: &FrameInput<N>, output: &mut FrameOut) -> FfiBenchCall {
    bench_call(ArmKind::Engine, input, output)
}
