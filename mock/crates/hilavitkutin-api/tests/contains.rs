//! Compile-time Contains<S> membership for arities 1..=6.
//!
//! Post round 202605082142 the legacy flat-tuple `AccessSet`/`Contains`
//! impls are gone; the cons-list shape is the sole substrate. Each
//! `requires_*` invocation forces the trait bound at monomorphisation
//! against an `read!`-built cons-list. The test is that this file
//! compiles.

#![no_std]

use hilavitkutin_api::access::{AccessSet, Contains};
use hilavitkutin_api::read;

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;

fn requires<S, Ts: AccessSet + Contains<S>>() {}

#[test]
fn arity_1() {
    requires::<A, read![A]>();
}

#[test]
fn arity_2() {
    requires::<A, read![A, B]>();
    requires::<B, read![A, B]>();
}

#[test]
fn arity_3() {
    requires::<A, read![A, B, C]>();
    requires::<B, read![A, B, C]>();
    requires::<C, read![A, B, C]>();
}

#[test]
fn arity_4() {
    requires::<A, read![A, B, C, D]>();
    requires::<B, read![A, B, C, D]>();
    requires::<C, read![A, B, C, D]>();
    requires::<D, read![A, B, C, D]>();
}

#[test]
fn arity_5() {
    requires::<A, read![A, B, C, D, E]>();
    requires::<B, read![A, B, C, D, E]>();
    requires::<C, read![A, B, C, D, E]>();
    requires::<D, read![A, B, C, D, E]>();
    requires::<E, read![A, B, C, D, E]>();
}

#[test]
fn arity_6() {
    requires::<A, read![A, B, C, D, E, F]>();
    requires::<B, read![A, B, C, D, E, F]>();
    requires::<C, read![A, B, C, D, E, F]>();
    requires::<D, read![A, B, C, D, E, F]>();
    requires::<E, read![A, B, C, D, E, F]>();
    requires::<F, read![A, B, C, D, E, F]>();
}
