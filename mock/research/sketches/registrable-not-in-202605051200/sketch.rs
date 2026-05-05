//! Sketch: NotIn<H> for compile-time diamond-resolution policy
//! on the Registrable<B> bridge.
//!
//! Validates whether `feature(negative_impls)` admits a sound
//! `NotIn<H>` proof over a cons-list `Stores` such that the engine's
//! Registrable<SchedulerBuilder<...>> impl on `Resource<T>` can
//! refuse to register a duplicate marker at compile time.
//!
//! Build with:
//!   rustup run nightly rustc --crate-type=lib --edition=2024 \
//!     sketch.rs --emit=metadata
//!
//! Outcome categories:
//!   - WORKS: rustc accepts the negative impl + positive blanket
//!     and rejects duplicate registrations as compile errors.
//!   - FAILS WITH <error>: rustc rejects either the impl set or
//!     the consumer test.
//!   - INCONCLUSIVE: rustc accepts the impls but doesn't
//!     produce the expected disequality at consumer sites.

#![no_std]
#![feature(negative_impls)]
#![feature(with_negative_coherence)]
#![feature(marker_trait_attr)]
#![recursion_limit = "512"]
#![allow(dead_code, incomplete_features)]

use core::marker::PhantomData;

// -----------------------------------------------------------------------
// Markers (mirror hilavitkutin-api/src/store.rs after refactor).
// Resource<T> is value-carrying; Column<T>/Virtual<T> stay ZST.
// -----------------------------------------------------------------------

#[repr(transparent)]
pub struct Resource<T>(pub T);

#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);

#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);

// -----------------------------------------------------------------------
// Approach A: negative_impls on (H, R) to encode "H IS in (H, R)".
//
// Goal: define `NotIn<H>` such that:
//   - () : NotIn<H>           (vacuously true, base case)
//   - (K, R) : NotIn<H> when K != H AND R : NotIn<H>
// without a "K != H" predicate (Rust has no generic disequality).
//
// Strategy: positive blanket on (K, R) saying "NotIn<H> if R: NotIn<H>",
// negative impl on (H, R) saying "(H, R) is NOT NotIn<H>" — i.e. it
// narrows the positive blanket's coverage at the H == K case.
// -----------------------------------------------------------------------

#[marker]
pub trait NotIn<H> {}

// Base case: empty list contains nothing.
impl<H> NotIn<H> for () {}

// Step case: (K, R) is NotIn<H> if R is NotIn<H>.
// Note: this DOES NOT require K != H; it relies on the negative
// impl below to carve out the case where K == H.
impl<H, K, R> NotIn<H> for (K, R) where R: NotIn<H> {}

// Negative impl: (H, R) is explicitly NOT NotIn<H>.
// This narrows the positive blanket's coverage, producing a
// coherence-driven disequality.
impl<H, R> !NotIn<H> for (H, R) {}

// -----------------------------------------------------------------------
// Compile-time test cases.
//
// Each `assert_not_in!(H, Stores)` body forces a NotIn<H> bound;
// it should compile when H is NOT in Stores and fail when it IS.
// -----------------------------------------------------------------------

// Type aliases for clarity.
type S0 = ();
type S1 = (Resource<u8>, ());
type S2 = (Resource<u8>, (Column<u16>, ()));
type S3 = (Resource<u8>, (Column<u16>, (Virtual<u32>, ())));

// Positive cases — these should compile.
fn _r_u8_not_in_empty() where (): NotIn<Resource<u8>> {}
fn _r_u8_not_in_s2_with_different_inner()
where (Column<u16>, (Virtual<u32>, ())): NotIn<Resource<u8>> {}
fn _c_u16_not_in_s1() where S1: NotIn<Column<u16>> {}

// Negative cases — these SHOULD FAIL TO COMPILE.
// Uncomment one at a time to verify.
//
// fn _r_u8_in_s1_should_fail() where S1: NotIn<Resource<u8>> {}
// fn _c_u16_in_s2_should_fail() where S2: NotIn<Column<u16>> {}
// fn _v_u32_in_s3_should_fail() where S3: NotIn<Virtual<u32>> {}

// -----------------------------------------------------------------------
// Subtle case: (Resource<u8>, (Resource<u8>, ()))
//
// Two distinct cons-list positions both holding Resource<u8>.
// `NotIn<Resource<u8>>` should fail at the FIRST position via the
// negative impl. Verifies that the negative impl fires at the head,
// not at a deeper match.
// -----------------------------------------------------------------------

// fn _double_resource_should_fail()
// where (Resource<u8>, (Resource<u8>, ())): NotIn<Resource<u8>> {}

// -----------------------------------------------------------------------
// Trait-solver complexity probe.
//
// At Stores depth N with NotIn<H> bound, the solver walks the cons
// list O(N). At each step, it must resolve the negative impl for
// (H, R) vs the positive blanket for (K, R). This is one branch
// per cons-list cell.
//
// The probe below tests Stores depth = 16 with NotIn<H> bound; if
// it compiles in reasonable time, the depth scales linearly.
// -----------------------------------------------------------------------

type S8 = (
    Resource<u8>,
    (Resource<u16>,
    (Column<u8>,
    (Column<u16>,
    (Virtual<u8>,
    (Virtual<u16>,
    (Resource<i8>,
    (Resource<i16>, ())))))))
);

fn _depth_8_not_in() where S8: NotIn<Column<u128>> {}

// -----------------------------------------------------------------------
// Coherence stress test.
//
// Add the engine's leaf Registrable impl shape (mocked) carrying
// the NotIn<...> bound. Verify it compiles alongside the cons-list
// blanket impls above without overlap errors.
// -----------------------------------------------------------------------

mod registrable_shape {
    use super::*;

    pub struct SchedulerBuilder<Stores>(PhantomData<Stores>);

    pub trait Registrable<B>: Sized {
        type Output;
        fn apply(self, b: B) -> Self::Output;
    }

    impl<Stores, T: 'static> Registrable<SchedulerBuilder<Stores>> for Resource<T>
    where
        Stores: NotIn<Resource<T>>,
    {
        type Output = SchedulerBuilder<(Resource<T>, Stores)>;
        fn apply(self, _: SchedulerBuilder<Stores>) -> Self::Output {
            SchedulerBuilder(PhantomData)
        }
    }

    impl<Stores, T: 'static> Registrable<SchedulerBuilder<Stores>> for Column<T>
    where
        Stores: NotIn<Column<T>>,
    {
        type Output = SchedulerBuilder<(Column<T>, Stores)>;
        fn apply(self, _: SchedulerBuilder<Stores>) -> Self::Output {
            SchedulerBuilder(PhantomData)
        }
    }

    impl<Stores, T: 'static> Registrable<SchedulerBuilder<Stores>> for Virtual<T>
    where
        Stores: NotIn<Virtual<T>>,
    {
        type Output = SchedulerBuilder<(Virtual<T>, Stores)>;
        fn apply(self, _: SchedulerBuilder<Stores>) -> Self::Output {
            SchedulerBuilder(PhantomData)
        }
    }

    // Tuple recursion (cons-list base + step).
    impl<B> Registrable<B> for () {
        type Output = B;
        fn apply(self, b: B) -> B { b }
    }

    impl<B, H, R> Registrable<B> for (H, R)
    where
        H: Registrable<B>,
        R: Registrable<<H as Registrable<B>>::Output>,
    {
        type Output = <R as Registrable<<H as Registrable<B>>::Output>>::Output;
        fn apply(self, b: B) -> Self::Output {
            let (h, r) = self;
            r.apply(h.apply(b))
        }
    }

    // Smoke test: register Resource<u8>, then Column<u16>, then
    // Virtual<u32> on an empty SchedulerBuilder. Should compile.
    pub fn smoke_three_distinct() {
        let b = SchedulerBuilder::<()>(PhantomData);
        let bundle = (Resource(0u8), (Column::<u16>(PhantomData), (Virtual::<u32>(PhantomData), ())));
        let _ = bundle.apply(b);
    }

    // Negative test: register Resource<u8> twice. Should fail to
    // compile via the NotIn<Resource<u8>> bound.
    // Uncomment to verify the failure mode.
    //
    // pub fn smoke_duplicate_resource_should_fail() {
    //     let b = SchedulerBuilder::<()>(PhantomData);
    //     let bundle = (Resource(0u8), (Resource(0u8), ()));
    //     let _ = bundle.apply(b);
    // }
}
