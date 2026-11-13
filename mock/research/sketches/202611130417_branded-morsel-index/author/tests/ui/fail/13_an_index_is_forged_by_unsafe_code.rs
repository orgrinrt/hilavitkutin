//! An index made by `mem::zeroed` in a crate that forbids `unsafe`. The twin
//! is the same file without the `forbid` line.

#![forbid(unsafe_code)]

use core::num::NonZeroUsize;

use engine::{BlockKernel, Index, Morsel, run};

struct Probe;

impl BlockKernel<i64, i64> for Probe {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            // SAFETY: all zeros is a valid `usize` and a valid zero-sized
            // brand, so this is a valid value; it is row 0 of every morsel,
            // and every morsel here has a row 0. The point is that it is made
            // without `each`.
            let probe: Index<'_> = unsafe { core::mem::zeroed() };
            block.write(probe, 1);
        });
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(&Probe, &read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
    assert_eq!(write, [1, 0, 1, 0, 1]);
}
