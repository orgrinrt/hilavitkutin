//! S2 candidate A: macro-flat AccessSet.
//!
//! Pattern: each tuple arity gets its own AccessSet impl, and Contains<Ti>
//! has one impl per (tuple-arity, position) pair. At arity N, Contains
//! impls grow as O(N^2) (sum over k of k impls per tuple of size k).
//!
//! This file hand-codes the impls up to arity 12 to keep the sketch
//! readable. A real production rollout uses a macro to generate the
//! impls; the structural cost analysis is identical.
//!
//! Build: `rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata`

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

use core::marker::PhantomData;

// Sixteen distinct marker types stand in for store-marker positions.
pub struct M0;
pub struct M1;
pub struct M2;
pub struct M3;
pub struct M4;
pub struct M5;
pub struct M6;
pub struct M7;
pub struct M8;
pub struct M9;
pub struct M10;
pub struct M11;
pub struct M12;
pub struct M13;
pub struct M14;
pub struct M15;

pub trait AccessSet {}

#[marker]
pub trait Contains<X>: AccessSet {}

// Empty.
impl AccessSet for () {}

// Arity 1.
impl<T0> AccessSet for (T0,) {}
impl<T0> Contains<T0> for (T0,) {}

// Arity 2.
impl<T0, T1> AccessSet for (T0, T1) {}
impl<T0, T1> Contains<T0> for (T0, T1) {}
impl<T0, T1> Contains<T1> for (T0, T1) {}

// Arity 3.
impl<T0, T1, T2> AccessSet for (T0, T1, T2) {}
impl<T0, T1, T2> Contains<T0> for (T0, T1, T2) {}
impl<T0, T1, T2> Contains<T1> for (T0, T1, T2) {}
impl<T0, T1, T2> Contains<T2> for (T0, T1, T2) {}

// Arity 4.
impl<T0, T1, T2, T3> AccessSet for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> Contains<T0> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> Contains<T1> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> Contains<T2> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> Contains<T3> for (T0, T1, T2, T3) {}

// Arity 8.
impl<T0, T1, T2, T3, T4, T5, T6, T7> AccessSet for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T0> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T1> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T2> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T3> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T4> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T5> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T6> for (T0, T1, T2, T3, T4, T5, T6, T7) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Contains<T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {}

// Arity 12.
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AccessSet
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T0>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T1>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T2>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T3>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T4>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T5>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T6>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T7>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T8>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T9>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T10>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Contains<T11>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}

// Demonstration: assert membership at arity 4 and 12.
fn require_contains<S: Contains<X>, X>() {}

pub fn demo_arity_4() {
    require_contains::<(M0, M1, M2, M3), M2>();
}

pub fn demo_arity_12() {
    require_contains::<
        (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11),
        M9,
    >();
}

// Toggle to compile a deliberately-failing membership check; comment out
// to exercise the success path.
#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    require_contains::<(M0, M1, M2, M3), M9>();
}
