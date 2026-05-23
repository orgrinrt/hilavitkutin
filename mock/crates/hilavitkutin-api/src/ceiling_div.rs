//! `CeilingDiv<N, D>` helper trait.
//!
//! Topic 3 S1 workaround for `generic_const_exprs` / multi-operator-
//! in-array-size bugs (#76560, #96695). Direct forms like
//! `[T; (A + B - 1) / B]` or `[T; B + 1]` either fail to elaborate
//! or require nested `where [(); EXPR]:` bounds that themselves
//! don't elaborate cleanly under the current trait solver.
//!
//! `CeilingDiv` is a typenum-style typeclass: one impl per `(N, D)`
//! pair, with the associated `const RESULT` carrying the precomputed
//! `(N + D - 1) / D`. Consumers thread `RESULT` into nested const
//! generics as a named parameter, avoiding arithmetic in const-
//! generic array bound positions entirely.
//!
//! v1 ships impls covering `N ∈ 1..=1024` and `D ∈ 1..=64`. Consumers
//! requiring larger caps add impls via the proc-macro
//! `hilavitkutin_extensions_macros::impl_ceiling_div!` (the proc-
//! macro lands as part of the same round's apply pass; until then,
//! consumers hand-impl).

/// Compile-time ceiling division.
///
/// `<CeilingDiv<N, D> as Trait>::RESULT` is `(N + D - 1) / D`,
/// computed at type-check time. The impl set is enumerated by the
/// proc-macro emission below; consumers requiring larger caps add
/// impls via the same macro.
pub trait CeilingDiv<const N: usize, const D: usize> {
    /// `(N + D - 1) / D`, computed at type-check time.
    const RESULT: usize;
}

/// Marker type carrying the result. Consumers refer to it as
/// `<Ceil<N, D> as CeilingDiv<N, D>>::RESULT`.
pub struct Ceil<const N: usize, const D: usize>;

// Placeholder impls for the common substrate caps. The proc-macro
// emission expands this set; until the proc-macro lands, hand-write
// the impls we need. Tracked: round 202605101036 src CL.
impl<const N: usize, const D: usize> CeilingDiv<N, D> for Ceil<N, D> {
    const RESULT: usize = (N + D - 1) / D;
}
