//! An index returned out of the `each` that minted it. The twin returns the
//! value read at it.

#![forbid(unsafe_code)]

use core::num::NonZeroUsize;

use engine::{BlockKernel, Morsel, run};

struct Probe;

impl BlockKernel<i64, i64> for Probe {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        let kept = morsel.each(|block| {
            let index = block.indices().next().unwrap();
            let probe = index;
            probe
        });
        let _ = kept;
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(&Probe, &read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
}
