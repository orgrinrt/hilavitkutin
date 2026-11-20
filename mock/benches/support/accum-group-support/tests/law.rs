//! The law and the inputs, with no split: the law is not associative, and on
//! the bench's inputs three groupings give three results, so an equality
//! across worker counts reflects one grouping and not inputs the law forgave.

use accum_group_support::data;
use accum_group_support::law::{Sat, left_fold};

fn s(v: i16) -> Sat {
    Sat::from_raw(v)
}

#[test]
fn signed_saturating_addition_is_not_associative() {
    let left = (s(100) + s(100)) + s(-100);
    let right = s(100) + (s(100) + s(-100));
    assert_eq!(left.to_raw(), 27);
    assert_eq!(right.to_raw(), 100);
}

#[test]
fn the_law_clamps_at_both_bounds() {
    assert_eq!((s(127) + s(1)).to_raw(), 127);
    assert_eq!((s(-128) + s(-1)).to_raw(), -128);
}

#[test]
fn three_groupings_of_the_bench_inputs_give_three_results() {
    let col = data::column(0x5EED_0000_0000_0001, 1001);
    let none = left_fold(&col).to_raw();
    let by = |g: usize| {
        let parts: Vec<Sat> = col.chunks(g).map(left_fold).collect();
        left_fold(&parts).to_raw()
    };
    let (g8, g7) = (by(8), by(7));
    assert_ne!(none, g8);
    assert_ne!(none, g7);
    assert_ne!(g8, g7);
}

#[test]
fn a_seed_gives_the_same_column_and_another_seed_another() {
    let a = data::column(1, 64);
    let b = data::column(1, 64);
    let c = data::column(2, 64);
    assert!(a.iter().zip(&b).all(|(x, y)| x.to_raw() == y.to_raw()));
    assert!(a.iter().zip(&c).any(|(x, y)| x.to_raw() != y.to_raw()));
}
