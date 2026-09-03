//! Variant: interleave 0,6,1,7,2,8,3,9,4,10,5,11 in which no consecutive pair shares an edge column (bandwidth 9, total 65, 0/11): the worst-case control.
//!
//! One of five arms of the rcm_rivals ordering-theory bench. The 3x4
//! grid workload lives in `grid_common`; only the dispatch order
//! differs between arms. The harness size `n` is seed-material length;
//! the record count is `n * 1024`.
//!
//! The scheduler is expensive to build, so it is cached per worker
//! process keyed by N (the framework guarantees at least one untimed
//! warmup call, which is where the lazy build lands); the timed run
//! block is exactly one marked-dirty engine frame.

use std::cell::RefCell;

use grid_common::{prepare, GridOrder, Prepared};
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

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

#[bench_variant("riv_scr", sizes = [64, 256, 1024, 2048, 4096, 8192, 16384])]
fn run_riv_scr<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.as_ref().map(|(n, _)| *n) != Some(N) {
                *s = Some((N, prepare(GridOrder::Scr, seed_from_input(input), N * 1024)));
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
