//! A column handle written as a literal: only the bind mints one, for a column it checked.
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::num::{ColumnNo, Int};
use hilavitkutin_frozen_authors::Plane;

type L = Int<16>;

fn column(col: Col<Plane, L>) -> ColumnNo {
    let _ = Col::<Plane, L> {
        column: ColumnNo::from_host(0),
    };
    col.column()
}

fn main() {
    let _ = column;
}
