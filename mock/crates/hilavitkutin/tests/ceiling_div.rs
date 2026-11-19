//! `CeilingDiv` gives the ceiling of `N` over `D` as a constant, at the
//! edges that a plain truncating division gets wrong.

use hilavitkutin::plan::{Ceil, CeilingDiv};

#[test]
fn an_exact_quotient_is_not_rounded_up() {
    assert_eq!(<Ceil<9, 3> as CeilingDiv<9, 3>>::RESULT, 3);
    assert_eq!(<Ceil<1024, 64> as CeilingDiv<1024, 64>>::RESULT, 16);
}

#[test]
fn a_remainder_rounds_the_quotient_up() {
    assert_eq!(<Ceil<10, 3> as CeilingDiv<10, 3>>::RESULT, 4);
    assert_eq!(<Ceil<1025, 64> as CeilingDiv<1025, 64>>::RESULT, 17);
}

#[test]
fn a_numerator_below_the_divisor_is_one_and_zero_is_zero() {
    assert_eq!(<Ceil<1, 64> as CeilingDiv<1, 64>>::RESULT, 1);
    assert_eq!(<Ceil<0, 4> as CeilingDiv<0, 4>>::RESULT, 0);
}

#[test]
fn a_divisor_of_one_is_the_numerator() {
    assert_eq!(<Ceil<7, 1> as CeilingDiv<7, 1>>::RESULT, 7);
}
