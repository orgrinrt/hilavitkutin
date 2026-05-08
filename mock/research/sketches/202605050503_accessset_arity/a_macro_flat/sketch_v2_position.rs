//! S2 candidate A, v2: macro-flat with explicit position witness.
//!
//! v1 (`sketch.rs`) hit coherence overlap because `Contains<T0> for (T0, T1)`
//! and `Contains<T1> for (T0, T1)` collide when T0 = T1. This variant adds a
//! position witness Pos (P0, P1, ...) so each impl is distinguished by its
//! second type parameter. The user-facing wrapper `ContainsAny<X>` uses HRTB
//! over the position to hide this.
//!
//! Build: `rustc --crate-type=lib --edition=2024 sketch_v2_position.rs --emit=metadata`

#![allow(unused)]
#![no_std]

// Markers.
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

// Position witnesses, one per slot.
pub struct P0;
pub struct P1;
pub struct P2;
pub struct P3;
pub struct P4;
pub struct P5;
pub struct P6;
pub struct P7;
pub struct P8;
pub struct P9;
pub struct P10;
pub struct P11;

pub trait AccessSet {}
pub trait ContainsAt<X, P>: AccessSet {}

// Empty.
impl AccessSet for () {}

// Arity 2.
impl<T0, T1> AccessSet for (T0, T1) {}
impl<T0, T1> ContainsAt<T0, P0> for (T0, T1) {}
impl<T0, T1> ContainsAt<T1, P1> for (T0, T1) {}

// Arity 3.
impl<T0, T1, T2> AccessSet for (T0, T1, T2) {}
impl<T0, T1, T2> ContainsAt<T0, P0> for (T0, T1, T2) {}
impl<T0, T1, T2> ContainsAt<T1, P1> for (T0, T1, T2) {}
impl<T0, T1, T2> ContainsAt<T2, P2> for (T0, T1, T2) {}

// Arity 4.
impl<T0, T1, T2, T3> AccessSet for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> ContainsAt<T0, P0> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> ContainsAt<T1, P1> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> ContainsAt<T2, P2> for (T0, T1, T2, T3) {}
impl<T0, T1, T2, T3> ContainsAt<T3, P3> for (T0, T1, T2, T3) {}

// Arity 12.
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AccessSet
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T0, P0>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T1, P1>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T2, P2>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T3, P3>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T4, P4>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T5, P5>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T6, P6>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T7, P7>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T8, P8>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T9, P9>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T10, P10>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ContainsAt<T11, P11>
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}

// Demonstration: caller names the position. This is the main UX cost.
fn require_contains_at<S: ContainsAt<X, P>, X, P>() {}

pub fn demo_arity_4() {
    require_contains_at::<(M0, M1, M2, M3), M2, P2>();
}

pub fn demo_arity_12() {
    require_contains_at::<
        (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11),
        M9,
        P9,
    >();
}

#[cfg(feature = "show_missing_error")]
pub fn demo_missing() {
    require_contains_at::<(M0, M1, M2, M3), M9, P0>();
}
