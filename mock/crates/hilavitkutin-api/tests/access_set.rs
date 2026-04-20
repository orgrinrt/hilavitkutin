//! AccessSet::LEN compile-time assertions for arities 0..=6.

#![no_std]

use hilavitkutin_api::AccessSet;

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;

#[test]
fn len_arity_0() {
    assert_eq!(<() as AccessSet>::LEN, 0);
}

#[test]
fn len_arity_1() {
    assert_eq!(<(A,) as AccessSet>::LEN, 1);
}

#[test]
fn len_arity_2() {
    assert_eq!(<(A, B) as AccessSet>::LEN, 2);
}

#[test]
fn len_arity_3() {
    assert_eq!(<(A, B, C) as AccessSet>::LEN, 3);
}

#[test]
fn len_arity_4() {
    assert_eq!(<(A, B, C, D) as AccessSet>::LEN, 4);
}

#[test]
fn len_arity_5() {
    assert_eq!(<(A, B, C, D, E) as AccessSet>::LEN, 5);
}

#[test]
fn len_arity_6() {
    assert_eq!(<(A, B, C, D, E, F) as AccessSet>::LEN, 6);
}
