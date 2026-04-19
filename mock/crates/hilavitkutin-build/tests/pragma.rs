//! `PragmaSet` with / without / contains + iter-order coverage.

use hilavitkutin_build::{Pragma, PragmaSet};

#[test]
fn empty_contains_nothing() {
    let s = PragmaSet::new();
    assert!(!s.contains(Pragma::FastMath));
    assert!(!s.contains(Pragma::LoopOptimization));
    assert!(!s.contains(Pragma::MimallocAllocator));
}

#[test]
fn with_adds_membership() {
    let s = PragmaSet::new()
        .with(Pragma::FastMath)
        .with(Pragma::LoopOptimization);

    assert!(s.contains(Pragma::FastMath));
    assert!(s.contains(Pragma::LoopOptimization));
    assert!(!s.contains(Pragma::Polly));
}

#[test]
fn without_removes_membership() {
    let s = PragmaSet::new()
        .with(Pragma::FastMath)
        .with(Pragma::LoopOptimization)
        .without(Pragma::FastMath);

    assert!(!s.contains(Pragma::FastMath));
    assert!(s.contains(Pragma::LoopOptimization));
}

#[test]
fn parallel_codegen_stores_units() {
    let s = PragmaSet::new().with(Pragma::ParallelCodegen(8));

    assert!(s.contains(Pragma::ParallelCodegen(0)));
    assert_eq!(s.parallel_codegen_units(), Some(8));
}

#[test]
fn parallel_codegen_without_clears_units() {
    let s = PragmaSet::new()
        .with(Pragma::ParallelCodegen(4))
        .without(Pragma::ParallelCodegen(0));

    assert!(!s.contains(Pragma::ParallelCodegen(0)));
    assert_eq!(s.parallel_codegen_units(), None);
}

#[test]
fn with_overwrites_parallel_units() {
    let s = PragmaSet::new()
        .with(Pragma::ParallelCodegen(4))
        .with(Pragma::ParallelCodegen(16));

    assert_eq!(s.parallel_codegen_units(), Some(16));
}

#[test]
fn iter_yields_in_bit_index_order() {
    // LoopOptimization = bit 0, FastMath = bit 3, Pgo = bit 5.
    let s = PragmaSet::new()
        .with(Pragma::Pgo)
        .with(Pragma::LoopOptimization)
        .with(Pragma::FastMath);

    let collected: Vec<Pragma> = s.iter().collect();
    assert_eq!(
        collected,
        vec![Pragma::LoopOptimization, Pragma::FastMath, Pragma::Pgo]
    );
}

#[test]
fn iter_includes_parallel_codegen_with_units() {
    let s = PragmaSet::new()
        .with(Pragma::LoopOptimization)
        .with(Pragma::ParallelCodegen(12));

    let collected: Vec<Pragma> = s.iter().collect();
    assert_eq!(
        collected,
        vec![Pragma::LoopOptimization, Pragma::ParallelCodegen(12)]
    );
}

#[test]
fn iter_empty_set_yields_nothing() {
    let s = PragmaSet::new();
    assert_eq!(s.iter().count(), 0);
}
