//! Cons-list-shape macros for `WorkUnit::Read` / `WorkUnit::Write`.
//!
//! `read![T0, T1, T2]` expands to `(T0, (T1, (T2, ())))`. The
//! cons-list shape lets `WuSatisfied` reduce by recursion at any
//! depth, removing the per-arity cap that flat-tuple-shaped
//! declarations imposed.
//!
//! Empty `read![]` yields `()`. Single-store `read![T]` yields
//! `(T, ())`. The `write!` macro has identical shape and applies to
//! `WorkUnit::Write` declarations.

#[macro_export]
macro_rules! read {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, $crate::read!($($rest),+)) };
}

#[macro_export]
macro_rules! write {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, $crate::write!($($rest),+)) };
}
