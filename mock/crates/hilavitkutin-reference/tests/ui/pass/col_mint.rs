//! Twin: outside the contracts a column handle is only asked its column.
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::num::{ColumnNo, Int};
use hilavitkutin_frozen_authors::Plane;

type L = Int<16>;

fn main() {
    let _ = Col::<Plane, L>::column;
    let _ = ColumnNo::from_host(0);
}
