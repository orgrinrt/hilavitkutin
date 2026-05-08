//! Cons-list-shape macros for `WorkUnit::Read` / `WorkUnit::Write`
//! and Kit `Units` / `Owned`.
//!
//! `read![T0, T1, T2]` expands to `Cons<T0, Cons<T1, Cons<T2, Empty>>>`.
//! Empty `read![]` yields `Empty`. Single-element `read![T]` yields
//! `Cons<T, Empty>`. `write!` is identical in shape and applies to
//! `WorkUnit::Write` declarations.

#[macro_export]
macro_rules! read {
    () => { $crate::Empty };
    ($T:ty $(,)?) => { $crate::Cons<$T, $crate::Empty> };
    ($T:ty, $($rest:ty),+ $(,)?) => { $crate::Cons<$T, $crate::read!($($rest),+)> };
}

#[macro_export]
macro_rules! write {
    () => { $crate::Empty };
    ($T:ty $(,)?) => { $crate::Cons<$T, $crate::Empty> };
    ($T:ty, $($rest:ty),+ $(,)?) => { $crate::Cons<$T, $crate::write!($($rest),+)> };
}
