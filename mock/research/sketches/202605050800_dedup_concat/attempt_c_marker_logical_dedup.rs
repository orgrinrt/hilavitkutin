//! M2 attempt C: logical dedup via `#[marker] Contains`.
//!
//! Observation: physical dedup is blocked, but the typestate proof
//! (`Stores: ContainsAll<AccumRead>`) does not require physical dedup.
//! `#[marker]` Contains's overlapping impls let the trait solver pick
//! any matching occurrence and stop. A duplicate-laden Cons chain
//! resolves Contains in the same time as a deduplicated one for the
//! head match; the cost shows up only in chain traversal length.
//!
//! Build: `rustup run nightly rustc --crate-type=lib --edition=2024 \
//!     attempt_c_marker_logical_dedup.rs --emit=metadata`
//!
//! Expected: WORKS. Demonstrates that `Cons<X, Cons<X, Cons<Y, Empty>>>`
//! satisfies `Contains<X>`, `Contains<Y>`, and ContainsAll over a list
//! that includes both X and Y. The duplicate X is structurally there
//! but does not impede or duplicate the proof.

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

#[marker]
pub trait ContainsAll<L>: AccessSet {}
impl<S: AccessSet> ContainsAll<Empty> for S {}
impl<S: AccessSet, H, T> ContainsAll<Cons<H, T>> for S
where
    S: Contains<H> + ContainsAll<T>,
{
}

pub struct X;
pub struct Y;

// A duplicate-laden chain.
type DupChain = Cons<X, Cons<X, Cons<Y, Cons<X, Empty>>>>;

// Compile-time witnesses: the chain satisfies Contains for X and Y, and
// ContainsAll over a request list that mentions each only once.
fn witness_contains_x<S: Contains<X>>(_: &S) {}
fn witness_contains_y<S: Contains<Y>>(_: &S) {}
fn witness_contains_all<S: ContainsAll<Cons<X, Cons<Y, Empty>>>>(_: &S) {}

pub fn proofs(c: &DupChain) {
    witness_contains_x(c);
    witness_contains_y(c);
    witness_contains_all(c);
}

// Compile-time witness: the chain ALSO satisfies ContainsAll over a
// request list that itself has duplicates. This is what happens in the
// depth-5 sketch (S1b) where AccumRead has Clock four times.
fn witness_contains_all_dup<S: ContainsAll<Cons<X, Cons<X, Cons<Y, Empty>>>>>(_: &S) {}

pub fn proofs_dup(c: &DupChain) {
    witness_contains_all_dup(c);
}

// Implication: physical dedup of AccumRead/AccumWrite is unnecessary for
// correctness. The marker traits' overlap-permitted resolution turns
// duplicates into "redundant proof candidates", not "broken proofs".
// Cost is in trait-solver traversal length, not in additional resolution
// work per element.
