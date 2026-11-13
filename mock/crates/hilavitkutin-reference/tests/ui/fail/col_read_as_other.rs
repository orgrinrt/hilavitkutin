//! A handle minted for a 16-bit column read as a 32-bit one.
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::num::{Index, Int};
use hilavitkutin_contracts::store::Read;
use hilavitkutin_frozen_authors::Plane;

fn read<R: Read<Plane>>(committed: &R, col: Col<Plane, Int<16>>) -> Int<32> {
    committed.get(col, Index::FIRST)
}

fn main() {
    let _ = read::<hilavitkutin_frozen_implementors::ColumnsRead<'static, Plane, 1, 4>>;
}
