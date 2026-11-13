//! A kernel stashing an index in its own state, for the next morsel to find.
//! The twin stashes the value read at it, and its kernel's state holds an
//! `i64` instead of an index.

#![forbid(unsafe_code)]

use core::cell::Cell;
use core::num::NonZeroUsize;

use engine::{BlockKernel, Index, Morsel, run};

struct Probe<T>(Cell<Option<T>>);

impl<'k> BlockKernel<i64, i64> for Probe<Index<'k>> {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|block| {
            for index in block.indices() {
                let probe = index;
                self.0.set(Some(probe));
            }
        });
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(
        &Probe(Cell::new(None)),
        &read,
        &mut write,
        NonZeroUsize::new(2).unwrap(),
    )
    .unwrap();
}
