//! `link-static` arm `default_relink`, quantity A: with every rlib fresh and
//! the facade's executable removed, cargo builds the facade again, which is
//! rustc on the facade and the link by the toolchain's default linker.

use build_call::Arm;
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("default_relink", sizes = [25, 97, 169])]
fn run<const N: usize>(input: &[u8; 8], output: &mut u64) -> FfiBenchCall {
    build_call::bench_call(
        "link-static",
        "default_relink",
        Arm::Relink,
        input,
        N,
        output,
    )
}
