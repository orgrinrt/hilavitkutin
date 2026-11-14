//! `engine_par`: the workload on the engine, `run_parallel` across the machine's cores.

use bench_arm::TaggedRecords;
use evs_support::{ArmKind, bench_call};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("evs_engine_par", sizes = [4096, 65536, 1048576, 4194304])]
fn run<const N: usize>(input: &TaggedRecords<N>, output: &mut u64) -> FfiBenchCall {
    bench_call(ArmKind::EnginePar, input, output)
}
