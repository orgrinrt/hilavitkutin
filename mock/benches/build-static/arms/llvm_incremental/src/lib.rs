//! `build-static` arm `llvm_incremental`: the fixture built with the
//! toolchain's default backend and linker, incremental compilation on.

use build_call::Arm;
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("llvm_incremental", sizes = [25, 97, 169])]
fn run<const N: usize>(input: &[u8; 8], output: &mut u64) -> FfiBenchCall {
    build_call::bench_call(
        "build-static",
        "llvm_incremental",
        Arm::Build {
            incremental: true,
        },
        input,
        N,
        output,
    )
}
