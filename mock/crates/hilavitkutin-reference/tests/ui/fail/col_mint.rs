//! A column handle minted outside the contracts: minting is the bind's alone.
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::num::{ColumnNo, Int};
use hilavitkutin_frozen_authors::Plane;

type L = Int<16>;

fn main() {
    let _ = Col::<Plane, L>::mint(ColumnNo::from_host(0));
}
