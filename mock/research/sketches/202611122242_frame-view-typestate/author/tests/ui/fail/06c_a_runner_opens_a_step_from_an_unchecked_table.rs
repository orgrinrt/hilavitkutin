// Data-bound twin of 06: the runner is handed the bare write table. The twin
// hands it the table through `Checked::bind`.
#![allow(unused)]
use engine::{Checked, Columns};

fn main() {
    let mut store = Columns::<i64, 2, 2>::new(0);
    let pos = store.bind().unwrap();
    let step = store.begin(&[&[pos][..]]);
}
