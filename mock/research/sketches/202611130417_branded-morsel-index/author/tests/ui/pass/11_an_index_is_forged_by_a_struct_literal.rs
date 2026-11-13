//! An index written out as a struct literal. The twin takes the one `each`
//! minted.

#![forbid(unsafe_code)]

use core::marker::PhantomData;
use core::num::NonZeroUsize;

use engine::{BlockKernel, Index, Morsel, run};

struct Probe;

impl BlockKernel<i64, i64> for Probe {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            for index in block.indices() {
                let probe: Index<'_> = index;
                let _ = PhantomData::<()>;
                block.write(probe, 1);
            }
        });
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(&Probe, &read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
}
