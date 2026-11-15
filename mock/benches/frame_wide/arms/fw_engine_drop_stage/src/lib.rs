//! `engine_drop_stage`: the engine negative control, the engine builder call
//! sequence with the first pipeline's second `Map` omitted. It exists to be
//! rejected by validation.

use frame_wide_support::{ArmKind, FrameInput, FrameOut, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("engine_drop_stage", sizes = [4096, 65536, 1048576, 4194304, 16777216])]
fn run<const N: usize>(input: &FrameInput<N>, output: &mut FrameOut) -> FfiBenchCall {
    bench_call(ArmKind::EngineDropStage, input, output)
}
