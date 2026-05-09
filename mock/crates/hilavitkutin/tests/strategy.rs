//! Strategy selection threshold tests.

use arvo::USize;
use hilavitkutin::strategy::{DefaultSelector, Strategy, StrategySelector};

#[test]
fn below_10k_records_selects_sequential() {
    let s = DefaultSelector;
    assert_eq!(s.select(USize(5_000), USize(4), USize(8), USize(2)), Strategy::Sequential); // lint:allow(no-bare-numeric) reason: workload-shape thresholds; tracked: #399
    assert_eq!(s.select(USize(9_999), USize(10), USize(20), USize(5)), Strategy::Sequential); // lint:allow(no-bare-numeric) reason: workload-shape thresholds; tracked: #399
}

#[test]
fn deep_pipeline_selects_sequential() {
    let s = DefaultSelector;
    // depth > fibers/2 + roots <= 2
    assert_eq!(s.select(USize(100_000), USize(10), USize(8), USize(2)), Strategy::Sequential); // lint:allow(no-bare-numeric) reason: workload-shape thresholds; tracked: #399
}

#[test]
fn wide_pipeline_selects_adaptive() {
    let s = DefaultSelector;
    // roots > depth/2
    assert_eq!(s.select(USize(100_000), USize(4), USize(20), USize(5)), Strategy::Adaptive); // lint:allow(no-bare-numeric) reason: workload-shape thresholds; tracked: #399
}

#[test]
fn mixed_selects_phased() {
    let s = DefaultSelector;
    // not below-10K, not deep, not wide -> Phased
    assert_eq!(s.select(USize(100_000), USize(10), USize(20), USize(3)), Strategy::Phased); // lint:allow(no-bare-numeric) reason: workload-shape thresholds; tracked: #399
}
