//! The law the folds run under, and the inputs the engine tests use.
//!
//! The engine tests are worth something only if the law is not associative and
//! the inputs reach the cases where it is not. Both are held here, without an
//! engine.

use accumulator_grouped_by_data::{Part, Sat, Val, combine, inputs, left_fold, model_grouped};

fn s(v: i16) -> Sat {
    Sat::from_raw(v)
}

#[test]
fn the_law_saturates_at_both_logical_bounds() {
    assert_eq!((s(100) + s(100)).to_raw(), 127);
    assert_eq!((s(-100) + s(-100)).to_raw(), -128);
    assert_eq!((s(100) + s(-100)).to_raw(), 0);
}

#[test]
fn the_law_is_not_associative() {
    // The witness: the same three values grouped two ways.
    let left = (s(100) + s(100)) + s(-100);
    let right = s(100) + (s(100) + s(-100));
    assert_eq!(left.to_raw(), 27);
    assert_eq!(right.to_raw(), 100);
    assert_ne!(left.to_raw(), right.to_raw());
}

/// The inputs give a different result under a grouping of eight than under no
/// grouping at all, so a test that finds two groupings equal on them has found
/// that the groupings are the same and not that the law forgave the difference.
#[test]
fn a_grouping_changes_the_result_on_the_engine_inputs() {
    for n in [1001, 6144] {
        let v: Vec<Val> = inputs(n);
        let flat = left_fold(v.iter().map(|x| x.0));
        let grouped = combine(&model_grouped(&v, 8));
        let other: Vec<Part> = model_grouped(&v, 7);
        println!(
            "records {n}: flat {}, groups of 8 {}, groups of 7 {}",
            flat.to_raw(),
            grouped.to_raw(),
            combine(&other).to_raw(),
        );
        assert_ne!(flat.to_raw(), grouped.to_raw(), "records {n}");
        assert_ne!(grouped.to_raw(), combine(&other).to_raw(), "records {n}");
    }
}
