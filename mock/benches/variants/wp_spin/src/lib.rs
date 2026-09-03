//! Variant: canonical bounded middle tier: budget 128 spins re-checking the word before the first park.
//!
//! One of the two wake_policy arms (deviation-3 evidence, round
//! 202607202310). The carrier lives in `wake_common`; the arms differ
//! only in the `spin_budget` handed to the frame waits. The harness
//! size `n` is the per-core record count of in-frame work; the timed
//! run block is exactly one publish-to-done frame round trip on the
//! persistent worker pool (spawned once per worker process in the
//! untimed warmup).

use std::cell::RefCell;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use wake_common::{seed_from_input, WaitPolicy, WakeCarrier};

thread_local! {
    static STATE: RefCell<Option<WakeCarrier>> = const { RefCell::new(None) };
}

#[bench_variant("wp_spin", sizes = [64, 1024, 8192])]
fn run_wp_spin<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.is_none() {
                *s = Some(WakeCarrier::new_with(WaitPolicy::Shipped { budget: 128 }));
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
