//! Variant: the shipped wide-block fiber former: Fiedler bisection over the symmetric Laplacian.
//!
//! One arm of the fiber_theory bench (#644): the plan-time cost of one
//! full grouping call on the shared layered wide-fan-out DAG family.
//! The harness size `n` is the unit count directly. The graph builds
//! untimed per call; the timed body is exactly the grouping.

use fiber_common::{Theory, build_graph, run_theory};
use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xCBF29CE484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001B3);
    }
    acc
}

#[bench_variant("fib_spectral", sizes = [8, 16, 32, 64])]
fn run_fib_spectral<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        setup {
            let g = build_graph(seed_from_input(input), N);
        }
        run {
            let h = run_theory(Theory::Spectral, &g);
            output.copy_from_slice(&h.to_le_bytes());
        }
    }
}
