//! `CeilingDiv<N, D>`: the ceiling of one capacity over another, as a
//! constant.
//!
//! A derived plan dimension, such as units per fiber, is the
//! consumer's own associated type on `PlanDims`, worst case by
//! default. Where a consumer wants the ceiling of one capacity over
//! another as its pick, it reads `<Ceil<N, D> as CeilingDiv<N, D>>::RESULT`
//! and names the value as a const generic, so the division sits in an
//! impl and never in an array-length position, where
//! `[T; A.div_ceil(B)]` needs `generic_const_exprs` (#76560, #96695).
//!
//! The engine reads none of it: it is a heuristic offered beside the
//! plan dimensions it would feed.

/// Compile-time ceiling division.
///
/// `<Ceil<N, D> as CeilingDiv<N, D>>::RESULT` is `N.div_ceil(D)`,
/// computed at type-check time, without the overflow `(N + D - 1) / D`
/// meets near the top of the range. A zero `D` fails the build where
/// the constant is read.
#[rustfmt::skip] // keeps each allow on the line it governs
pub trait CeilingDiv<const N: usize, const D: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic parameters; rust grammar requires usize; tracked: #121
    /// `N.div_ceil(D)`, computed at type-check time.
    const RESULT: usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the value is named as a const generic, and rust grammar requires usize there; tracked: #121
}

/// The one implementer: `Ceil<N, D>` carries the ceiling of `N` over
/// `D`.
pub struct Ceil<const N: usize, const D: usize>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic parameters; rust grammar requires usize; tracked: #121

#[rustfmt::skip] // keeps each allow on the line it governs
impl<const N: usize, const D: usize> CeilingDiv<N, D> for Ceil<N, D> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic parameters; rust grammar requires usize; tracked: #121
    const RESULT: usize = N.div_ceil(D); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the value is named as a const generic, and rust grammar requires usize there; tracked: #121
}
