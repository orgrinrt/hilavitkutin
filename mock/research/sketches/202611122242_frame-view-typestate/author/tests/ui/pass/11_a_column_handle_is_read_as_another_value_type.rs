// A handle minted for a `u32` column reads an `i64` view. The twin reads it
// with the handle minted for the `i64` column.
#![allow(unused)]
use engine::{Checked, Columns, Step};

fn main() {
    let mut wide = Columns::<i64, 1, 2>::new(0);
    let mut narrow = Columns::<u32, 1, 2>::new(0);
    let heat = wide.bind().unwrap();
    let count = narrow.bind().unwrap();
    let Step {
        view,
        ..
    } = wide.begin(&Checked::bind([&[heat][..]]).unwrap());
    let row = view.rows().next().unwrap();
    let read = view.get(heat, row);
}
