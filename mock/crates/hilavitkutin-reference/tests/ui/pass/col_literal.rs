//! Twin: a column handle is only read, by its column.
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::num::{ColumnNo, Int};
use hilavitkutin_frozen_authors::Plane;

type L = Int<16>;

fn column(col: Col<Plane, L>) -> ColumnNo {
    let _ = ColumnNo::from_host(0);
    col.column()
}

fn main() {
    let _ = column;
}
