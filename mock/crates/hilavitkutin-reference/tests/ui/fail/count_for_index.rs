//! A count where a position goes: the two are distinct types.
use hilavitkutin_contracts::num::{Count, Index};

fn main() {
    let _ = Index::FIRST.since(Count::ONE);
}
