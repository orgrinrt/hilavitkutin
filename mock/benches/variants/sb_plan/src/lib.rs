//! Variant: plan-swap arm: an untimed replace_resource precedes each timed frame, so the frame pays the PlanAffecting cone plus the leading plan band.
//!
//! One of three arms of the swap_band asymmetry bench (spec S7, round
//! 202607200500). The carrier lives in `swap_common`; only the untimed
//! per-call setup differs between arms. The harness size `n` is
//! seed-material length; the record count is `n * 1024`. The scheduler
//! is cached per worker process keyed by N (the framework guarantees an
//! untimed warmup call where the lazy build lands); the timed run block
//! is exactly one engine frame.

use std::cell::RefCell;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use swap_common::{prepare_band, PreparedBand, SwapMode};

thread_local! {
    static STATE: RefCell<Option<(usize, PreparedBand)>> = const { RefCell::new(None) };
}

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
    }
    acc
}

#[bench_variant("sb_plan", sizes = [64, 1024, 8192])]
fn run_sb_plan<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.as_ref().map(|(n, _)| *n) != Some(N) {
                *s = Some((N, prepare_band(SwapMode::Plan, seed_from_input(input), N * 1024)));
            }
        });
        setup {
            STATE.with(|s| (s.borrow_mut().as_mut().unwrap().1.swap)());
        }
        run {
            STATE.with(|s| (s.borrow_mut().as_mut().unwrap().1.run_frame)());
        }
        STATE.with(|s| {
            let h = (s.borrow_mut().as_mut().unwrap().1.finish)();
            output.copy_from_slice(&h.to_le_bytes());
        });
    }
}
