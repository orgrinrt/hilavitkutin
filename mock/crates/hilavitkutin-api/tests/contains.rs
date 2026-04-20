//! Compile-time Contains<S> membership for arities 1..=6.
//!
//! The test is that this file compiles: each `fn requires_...`
//! forces the trait bound at monomorphisation.

#![no_std]

use hilavitkutin_api::{AccessSet, Contains};

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;

fn requires<S, Ts: AccessSet + Contains<S>>() {}

#[test]
fn arity_1() {
    requires::<A, (A,)>();
}

#[test]
fn arity_2() {
    requires::<A, (A, B)>();
    requires::<B, (A, B)>();
}

#[test]
fn arity_3() {
    requires::<A, (A, B, C)>();
    requires::<B, (A, B, C)>();
    requires::<C, (A, B, C)>();
}

#[test]
fn arity_4() {
    requires::<A, (A, B, C, D)>();
    requires::<B, (A, B, C, D)>();
    requires::<C, (A, B, C, D)>();
    requires::<D, (A, B, C, D)>();
}

#[test]
fn arity_5() {
    requires::<A, (A, B, C, D, E)>();
    requires::<B, (A, B, C, D, E)>();
    requires::<C, (A, B, C, D, E)>();
    requires::<D, (A, B, C, D, E)>();
    requires::<E, (A, B, C, D, E)>();
}

#[test]
fn arity_6() {
    requires::<A, (A, B, C, D, E, F)>();
    requires::<B, (A, B, C, D, E, F)>();
    requires::<C, (A, B, C, D, E, F)>();
    requires::<D, (A, B, C, D, E, F)>();
    requires::<E, (A, B, C, D, E, F)>();
    requires::<F, (A, B, C, D, E, F)>();
}
