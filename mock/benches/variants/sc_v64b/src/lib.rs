//! Variant: 64-byte payload: the timed body is one replace_value install of a 16-word value.
//!
//! One of three arms of the swap_cost install bench (spec S7, round
//! 202607200500): the witnessed blob write cost across value sizes.
//! The preparation lives in `swap_common`; the timed body is exactly
//! one `replace_value` call (argument construction is untimed setup).

use std::cell::RefCell;

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;
use swap_common::prepare_cost_64b;

type Prep = (Box<dyn FnMut(u64)>, Box<dyn FnMut() -> u64>);

thread_local! {
    static STATE: RefCell<Option<Prep>> = const { RefCell::new(None) };
}

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
    }
    acc
}

#[bench_variant("sc_v64b", sizes = [64])]
fn run_sc_v64b<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            if s.is_none() {
                *s = Some(prepare_cost_64b(seed_from_input(input)));
            }
        });
        setup {
            // Pure function of the seed: the determinism check calls the
            // arm twice per seed, and re-installing the same value still
            // pays the full witnessed blob write.
            let x = seed_from_input(input) ^ 0x5150_C0DE_5150_C0DE;
        }
        run {
            STATE.with(|s| (s.borrow_mut().as_mut().unwrap().0)(x));
        }
        STATE.with(|s| {
            let h = (s.borrow_mut().as_mut().unwrap().1)();
            output.copy_from_slice(&h.to_le_bytes());
        });
    }
}
