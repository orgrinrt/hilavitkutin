//! M2 attempt A: type-level set-difference operator.
//!
//! Walks L for each element of R, removing matches. Recursive,
//! structurally similar to ContainsAll. Two recursive impls branch on
//! whether L contains the head of R; the branch needs a NotIn-style
//! disequality predicate.
//!
//! Build: `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!     attempt_a_set_difference.rs --emit=metadata`
//!
//! Expected: COMPILE FAILURE with E0119/E0751 family. The two recursive
//! impls overlap on Cons<H, T>; the trait solver cannot distinguish the
//! Contains case from the !Contains case without a NotIn predicate, and
//! NotIn was shown unsound in round-3 (see
//! `../registrable-not-in-202605051200/FINDINGS.md`).

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

// Difference<L, R>::Out = L \ R (set difference; remove from L every
// element of R).
pub trait Difference<R> {
    type Out;
}

// Base: any L minus Empty is L.
impl<L: AccessSet> Difference<Empty> for L {
    type Out = L;
}

// Recursive: L minus Cons<H, T> = (L minus T) if L contains H. We also
// need the case where L does NOT contain H, where the result is the
// same. So we write the recursive step uniformly: skip H either way and
// continue with T.
impl<L: AccessSet, H, T> Difference<Cons<H, T>> for L
where
    L: Difference<T>,
{
    type Out = <L as Difference<T>>::Out;
}

// Wait: this trivial form just always returns L (regardless of R) since
// it never removes anything. That is not set difference; it is a no-op.
// To actually remove elements, we need to discriminate on whether
// L contains the head and remove it from L. The shape that DOES remove:
//
//   impl Difference<Cons<H, T>> for Cons<H, R>  -- match: drop head
//   impl Difference<Cons<H, T>> for Cons<X, R>  -- mismatch: keep
//
// And this is exactly the round-3 NotIn problem: two impls overlap on
// Cons<_, _> via the X-vs-H dichotomy, and Rust has no type-level
// disequality predicate. The trivial shape above type-checks but does
// nothing; the actually-discriminating shape does not type-check.

// Demonstrate the trivial-but-no-op shape compiles (this file's actual
// success surface, just to make the audit-trail reproducible). It is
// *not* a working set-difference operator.
pub type Demo1 = <Cons<u8, Cons<u16, Empty>> as Difference<Cons<u8, Empty>>>::Out;
// Demo1 evaluates to `Cons<u8, Cons<u16, Empty>>` (unchanged), proving
// the operator is a no-op.

// The actually-discriminating shape (commented out; uncomment to see the
// E0119 coherence overlap):
//
// impl<H, T, R> Difference<Cons<H, T>> for Cons<H, R>
// where
//     R: Difference<Cons<H, T>>,
//     R: Difference<T>,
// {
//     type Out = <R as Difference<T>>::Out;  // drop matched head
// }
// impl<H, T, X, R> Difference<Cons<H, T>> for Cons<X, R>
// where
//     R: Difference<Cons<H, T>>,
// {
//     type Out = Cons<X, <R as Difference<Cons<H, T>>>::Out>;
// }
//
// Both impls match `Cons<_, R> as Difference<Cons<H, T>>` when X = H.
// E0119 fires.
