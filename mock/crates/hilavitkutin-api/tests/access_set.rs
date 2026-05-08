//! AccessSet::LEN compile-time assertions for arities 0..=6 (macro form).

#![no_std]

use hilavitkutin_api::{AccessSet, read};

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;

#[test]
fn len_arity_0() {
    assert_eq!(<read![]>::LEN.0, 0);
}

#[test]
fn len_arity_1() {
    assert_eq!(<read![A]>::LEN.0, 1);
}

#[test]
fn len_arity_2() {
    assert_eq!(<read![A, B]>::LEN.0, 2);
}

#[test]
fn len_arity_3() {
    assert_eq!(<read![A, B, C]>::LEN.0, 3);
}

#[test]
fn len_arity_4() {
    assert_eq!(<read![A, B, C, D]>::LEN.0, 4);
}

#[test]
fn len_arity_5() {
    assert_eq!(<read![A, B, C, D, E]>::LEN.0, 5);
}

#[test]
fn len_arity_6() {
    assert_eq!(<read![A, B, C, D, E, F]>::LEN.0, 6);
}
