//! Strategy selection threshold tests.

use hilavitkutin::strategy::{DefaultSelector, Strategy, StrategySelector};

#[test]
fn below_10k_records_selects_sequential() {
    let s = DefaultSelector;
    assert_eq!(s.select(5_000, 4, 8, 2), Strategy::Sequential);
    assert_eq!(s.select(9_999, 10, 20, 5), Strategy::Sequential);
}

#[test]
fn deep_pipeline_selects_sequential() {
    let s = DefaultSelector;
    // depth > fibers/2 + roots ≤ 2
    assert_eq!(s.select(100_000, 10, 8, 2), Strategy::Sequential);
}

#[test]
fn wide_pipeline_selects_adaptive() {
    let s = DefaultSelector;
    // roots > depth/2
    assert_eq!(s.select(100_000, 4, 20, 5), Strategy::Adaptive);
}

#[test]
fn mixed_selects_phased() {
    let s = DefaultSelector;
    // not below-10K, not deep, not wide → Phased
    assert_eq!(s.select(100_000, 10, 20, 3), Strategy::Phased);
}
