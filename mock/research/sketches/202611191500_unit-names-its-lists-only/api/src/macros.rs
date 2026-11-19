//! List-shape macros for `WorkUnit::Read` / `WorkUnit::Write` and Kit
//! `Units` / `Owned`, and `rec!` for what a value function returns.
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

/// The values a value function returns, one per member of the unit's write
/// list and in the list's sequence.
///
/// `rec![a, b]` builds the `WriteVals` of a two-member write list. `rec![]`
/// is the product of the empty one.
#[macro_export]
macro_rules! rec {
    () => { $crate::values::Vals::<$crate::Empty>($crate::values::PNil) };
    ($($v:expr),+ $(,)?) => { $crate::values::Vals($crate::rec!(@cells $($v),+)) };
    (@cells $head:expr $(, $rest:expr)*) => {
        $crate::values::PCons { head: $head, tail: $crate::rec!(@cells $($rest),*) }
    };
    (@cells) => { $crate::values::PNil };
}
