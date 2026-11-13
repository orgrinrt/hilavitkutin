//! Two indices added. The twin adds the values read at them.

#![forbid(unsafe_code)]

use core::num::NonZeroUsize;

use engine::{BlockKernel, Morsel, run};

struct Probe;

impl BlockKernel<i64, i64> for Probe {
    fn block(&self, mut morsel: Morsel<'_, i64, i64>) {
        morsel.each(|mut block| {
            for index in block.indices() {
                let probe = index;
                let got = probe + probe;
                block.write(index, got);
            }
        });
    }
}

fn main() {
    let read = [1, 2, 3, 4, 5];
    let mut write = [0; 5];
    run(&Probe, &read, &mut write, NonZeroUsize::new(2).unwrap()).unwrap();
}
