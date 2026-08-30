//! Variant: reverse column-adjacent dispatch order (control arm).
//!
//! WUs dispatch 7..0. Every consecutive pair still shares one input
//! column (WU k and k-1 share C{k}), so adjacency is fully preserved
//! under a different numbering. If the mechanism behind `rcm_adj`'s win
//! is column adjacency rather than the specific 0..8 numbering, this arm
//! matches `rcm_adj` at every size. The harness size `n` is
//! seed-material length; the record count is `n * 1024`. The workload
//! lives in `rcm_common`; only the order differs.
//!
//! The scheduler is expensive to build, so it is cached per worker
//! process keyed by N (the framework guarantees at least one untimed
//! warmup call, which is where the lazy build lands); the timed run
//! block is exactly one marked-dirty engine frame.

use std::cell::RefCell;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use rcm_common::{prepare, Order, Prepared};

thread_local! {
    static STATE: RefCell<Option<(usize, Prepared)>> = const { RefCell::new(None) };
}

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
    }
    acc
}

#[bench_variant("rcm_rev", sizes = [64, 256, 1024, 2048, 4096, 8192, 16384])]
fn run_rcm_rev<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.as_ref().map(|(n, _)| *n) != Some(N) {
                *s = Some((N, prepare(Order::Rev, seed_from_input(input), N * 1024)));
            }
        });
        run {
            STATE.with(|s| (s.borrow_mut().as_mut().unwrap().1.run_frame)());
        }
        STATE.with(|s| {
            let h = (s.borrow_mut().as_mut().unwrap().1.finish)();
            output.copy_from_slice(&h.to_le_bytes());
        });
    }
}
