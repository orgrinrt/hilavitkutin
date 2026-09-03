//! Variant: no-hint spin, budget 8192 plain re-loads (about 3 us window, long enough to catch a small frame without parking).
//!
//! A round-2 wake_policy arm (the spin-shape re-examination after the
//! ISB-cost confound was identified in the round-1 pair). The carrier
//! lives in `wake_common`; arms differ only in the wait policy. The
//! harness size `n` is the per-core record count of in-frame work; the
//! timed run block is one publish-to-done frame round trip on the
//! persistent worker pool.

use std::cell::RefCell;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use wake_common::{seed_from_input, WaitPolicy, WakeCarrier};

thread_local! {
    static STATE: RefCell<Option<WakeCarrier>> = const { RefCell::new(None) };
}

#[bench_variant("wp_nh8k", sizes = [64, 1024, 8192])]
fn run_wp_nh8k<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.is_none() {
                *s = Some(WakeCarrier::new_with(WaitPolicy::Local { budget: 8192, hint: false }));
            }
        });
        let seed = seed_from_input(input);
        setup {
            let _ = seed;
        }
        run {
            STATE.with(|s| {
                let h = s.borrow().as_ref().unwrap().frame(N, seed);
                output.copy_from_slice(&h.to_le_bytes());
            });
        }
    }
}
