//! Twin: the count of records since a position is asked of a position.
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::num::{Count, Index};

fn main() {
    assert!(
        matches!(Index::from_host(3).since(Index::FIRST), Maybe::Is(n) if n == Count::from_host(3))
    );
}
