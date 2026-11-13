// Data-bound twin of 06: a checked table is written out by hand, so no
// disjointness check ran. The twin builds it with `Checked::bind`.
#![allow(unused)]
use engine::{Checked, Columns};

fn main() {
    let mut store = Columns::<i64, 2, 2>::new(0);
    let pos = store.bind().unwrap();
    let checked: Checked<i64, 2> = Checked::bind([&[pos][..], &[][..]]).unwrap();
    let step = store.begin(&checked);
}
