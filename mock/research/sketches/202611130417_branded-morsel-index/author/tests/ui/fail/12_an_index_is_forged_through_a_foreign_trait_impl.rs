//! The orphan-rule route: a foreign trait, `From`, implemented for the
//! foreign `Index` with a local type as its parameter, which the orphan rule
//! admits. The body then has to build an index. The twin keeps the impl and
//! builds nothing in its body.

#![forbid(unsafe_code)]

use core::marker::PhantomData;
use core::num::NonZeroUsize;

use engine::{BlockKernel, Index, Morsel, run};

struct Mine;

impl<'b> From<Mine> for Index<'b> {
    fn from(_: Mine) -> Self {
        Index {
            at:    0,
            brand: PhantomData,
        }
    }
}

struct Probe;

impl BlockKernel<i64, i64> for Probe {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            for index in block.indices() {
                block.write(index, 1);
            }
        });
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(&Probe, &read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
}
