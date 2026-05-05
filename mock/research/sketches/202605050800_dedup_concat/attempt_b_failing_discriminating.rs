//! M2 attempt B (failing variant): the actually-discriminating
//! ConcatDedup shape. Captures the E0119 coherence overlap that prevents
//! physical dedup from being expressed at the type level under workspace
//! constraints.
//!
//! Build (expected to FAIL):
//!   `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!     attempt_b_failing_discriminating.rs --emit=metadata`

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

// Hypothetical NotContains marker (the round-3 NotIn shape, here without
// negative_impls so we use it as a where-clause guard only).
pub trait NotContains<X> {}

pub trait ConcatDedup<R> {
    type Out;
}

impl<L: AccessSet> ConcatDedup<Empty> for L {
    type Out = L;
}

// Skip-if-present.
impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
where
    L: Contains<H>,
    L: ConcatDedup<T>,
{
    type Out = <L as ConcatDedup<T>>::Out;
}

// Prepend-if-absent.
impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
where
    L: NotContains<H>,
    Cons<H, L>: ConcatDedup<T>,
{
    type Out = <Cons<H, L> as ConcatDedup<T>>::Out;
}
