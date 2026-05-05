//! M2 attempt B: Concat-dedup operator that skips elements of R already
//! present in L during concatenation.
//!
//! Build: `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!     attempt_b_skip_concat.rs --emit=metadata`
//!
//! Expected: COMPILE FAILURE on the actually-discriminating shape, same
//! coherence problem as attempt A. The trivial no-op shape compiles but
//! does not dedup.

#![allow(unused)]
#![no_std]
#![feature(marker_trait_attr)]

use core::marker::PhantomData;

pub struct Empty;
pub struct Cons<H, T>(PhantomData<(H, T)>);

pub trait AccessSet {}
impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

#[marker]
pub trait Contains<X>: AccessSet {}
impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

// ConcatDedup<R>::Out: walk R; for each H in R, if L contains H, skip;
// otherwise prepend H to the accumulator. The accumulator IS L; we
// extend L by R-minus-(R-cap-L).
pub trait ConcatDedup<R> {
    type Out;
}

// Base: nothing to add.
impl<L: AccessSet> ConcatDedup<Empty> for L {
    type Out = L;
}

// Trivial recursive shape: always prepend H to L and recurse on T. This
// compiles but is plain Concat (no dedup).
impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
where
    Cons<H, L>: ConcatDedup<T>,
{
    type Out = <Cons<H, L> as ConcatDedup<T>>::Out;
}

// Demonstrate the trivial shape compiles. It is plain Concat.
pub type Demo1 = <Cons<u8, Empty> as ConcatDedup<Cons<u8, Cons<u16, Empty>>>>::Out;
// Demo1 evaluates to `Cons<u16, Cons<u8, Cons<u8, Empty>>>` (u8 still
// duplicated). Not a dedup.

// The actually-discriminating shape (commented out; same E0119 path).
//
// // Skip-if-present:
// impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
// where
//     L: Contains<H>,
//     L: ConcatDedup<T>,
// {
//     type Out = <L as ConcatDedup<T>>::Out;
// }
// // Prepend-if-absent:
// impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
// where
//     L: NotContains<H>,
//     Cons<H, L>: ConcatDedup<T>,
// {
//     type Out = <Cons<H, L> as ConcatDedup<T>>::Out;
// }
//
// Two impls of `ConcatDedup<Cons<H, T>> for L` overlap, distinguished
// by Contains-vs-NotContains. NotContains is the round-3 NotIn shape;
// it cannot be encoded soundly under coherence.
