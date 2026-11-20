//! `link-static` arm `default_replay`, quantity B: the link line rustc used
//! for the facade, recorded once with its objects kept, run again as a
//! process with the executable removed. The linker and its driver alone. The
//! set names no linker, so the toolchain's own links.

use build_call::Replay;
use hilavitkutin_build::{ChecksumFreshness, NoIncremental};
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("default_replay", sizes = [25, 97, 169])]
fn run<const N: usize>(input: &[u8; 8], output: &mut u64) -> FfiBenchCall {
    build_call::bench_call::<(ChecksumFreshness, NoIncremental), Replay>(
        "link-static",
        "default_replay",
        input,
        N,
        output,
    )
}
