//! AccessSet and Contains — compile-time store membership.
//!
//! `AccessSet` is sealed on tuples of arities 0..=12. `Contains<S>`
//! is sealed on (tuple, member) pairs: a WU that reads `Column<X>`
//! constrains its `Read` set to implement `Contains<Column<X>>`.
//! Violating that at a call site fails to compile.

use arvo::USize;
use arvo::strategy::Identity;

use crate::sealed;

/// Sealed marker on tuple read/write sets.
///
/// Implemented for arities 0 through 12. Arity cap 12 allows up to 8
/// declared columns per WU plus headroom (DESIGN guideline).
#[allow(private_bounds)]
pub trait AccessSet: sealed::Sealed + 'static {
    /// Number of member types in the set.
    const LEN: USize;
}

/// Sealed membership witness: the implementing `AccessSet` contains
/// `S` as one of its tuple members.
///
/// Provider API where-clauses reference `Contains<Column<T>>` or
/// `Contains<Resource<T>>` on the WU-declared set. Accessing a store
/// that was not declared is a compile error.
///
/// Marker trait (`#[marker]`) so overlapping position impls
/// coexist: a tuple with two equal member types still implements
/// `Contains<T>` once per position without coherence conflict.
#[marker]
#[diagnostic::on_unimplemented(
    message = "store `{Self}` does not contain `{S}`",
    note = "Register it with `.resource::<T>(initial)`, `.column::<T>()`, `.add_virtual::<T>()`, or install a Kit that registers it."
)]
pub trait Contains<S>: AccessSet {}

// Arity 0.
impl sealed::Sealed for () {}
impl AccessSet for () {
    const LEN: USize = USize::ZERO;
}

// Declarative macro: emit AccessSet for one arity plus Contains at
// every position within that arity.
macro_rules! impl_access_set {
    // Entry: arity N with type param list (T0, T1, ..., TN-1).
    // Expects exactly N - 1 commas in the tuple literal.
    ($len:expr; $($T:ident),+ $(,)?) => {
        impl<$($T: 'static),+> sealed::Sealed for ($($T,)+) {}
        impl<$($T: 'static),+> AccessSet for ($($T,)+) {
            const LEN: USize = USize($len);
        }
    };
}

// Emit one `Contains` impl per position within an arity.
// Each macro call is ONE impl for ONE position.
macro_rules! impl_contains {
    // Arguments: (tuple-type-list) (pos-type)
    // position-type must appear inside the tuple-type-list.
    ( ($($T:ident),+), $P:ident ) => {
        impl<$($T: 'static),+> Contains<$P> for ($($T,)+) {}
    };
}

// -----------------------------------------------------------------
// Arity 1.
impl_access_set!(1; T0);
impl_contains!((T0), T0);

// Arity 2.
impl_access_set!(2; T0, T1);
impl_contains!((T0, T1), T0);
impl_contains!((T0, T1), T1);

// Arity 3.
impl_access_set!(3; T0, T1, T2);
impl_contains!((T0, T1, T2), T0);
impl_contains!((T0, T1, T2), T1);
impl_contains!((T0, T1, T2), T2);

// Arity 4.
impl_access_set!(4; T0, T1, T2, T3);
impl_contains!((T0, T1, T2, T3), T0);
impl_contains!((T0, T1, T2, T3), T1);
impl_contains!((T0, T1, T2, T3), T2);
impl_contains!((T0, T1, T2, T3), T3);

// Arity 5.
impl_access_set!(5; T0, T1, T2, T3, T4);
impl_contains!((T0, T1, T2, T3, T4), T0);
impl_contains!((T0, T1, T2, T3, T4), T1);
impl_contains!((T0, T1, T2, T3, T4), T2);
impl_contains!((T0, T1, T2, T3, T4), T3);
impl_contains!((T0, T1, T2, T3, T4), T4);

// Arity 6.
impl_access_set!(6; T0, T1, T2, T3, T4, T5);
impl_contains!((T0, T1, T2, T3, T4, T5), T0);
impl_contains!((T0, T1, T2, T3, T4, T5), T1);
impl_contains!((T0, T1, T2, T3, T4, T5), T2);
impl_contains!((T0, T1, T2, T3, T4, T5), T3);
impl_contains!((T0, T1, T2, T3, T4, T5), T4);
impl_contains!((T0, T1, T2, T3, T4, T5), T5);

// Arity 7.
impl_access_set!(7; T0, T1, T2, T3, T4, T5, T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6), T6);

// Arity 8.
impl_access_set!(8; T0, T1, T2, T3, T4, T5, T6, T7);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7), T7);

// Arity 9.
impl_access_set!(9; T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T7);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8), T8);

// Arity 10.
impl_access_set!(10; T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T7);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T8);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T9);

// Arity 11.
impl_access_set!(11; T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T7);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T8);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T9);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T10);

// Arity 12.
impl_access_set!(12; T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T0);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T1);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T2);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T3);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T4);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T5);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T6);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T7);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T8);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T9);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T10);
impl_contains!((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T11);

// Cons-list recursion. The scheduler builder accumulates Stores
// as (H, R) at every step, where R is itself a cons-list. The
// arity-2 `Contains<T0> for (T0, T1)` impl above covers head
// matches. This recursive impl propagates membership down the
// tail. `#[marker]` on `Contains` permits the overlap.
impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R) where R: Contains<T> {}
