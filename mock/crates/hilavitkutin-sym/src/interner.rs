//! `Interner`: the dedup-and-resolve producer trait.

use notko::Maybe;

use crate::domain::InterningDomain;
use crate::handle::Sym;

/// A producer that folds a value into a stable handle and resolves it back.
///
/// `intern` returns a handle tagged `D::KIND`; equal values give equal handles
/// (a within-domain flag may still distinguish origins, as strings do). A
/// producer with no room left for a value it does not hold answers
/// `Maybe::Isnt`, as `Generator::mint` answers its exhaustion; a value already
/// held is never refused.
/// `resolve` returns the value for a handle of this domain, or `Maybe::Isnt`
/// for a handle the producer does not know.
pub trait Interner<D: InterningDomain> {
    /// Fold `value` into a stable handle, or `Maybe::Isnt` when there is no
    /// room for a value not held.
    fn intern(&self, value: &D::Value) -> Maybe<Sym<D::Shape>>;

    /// Recover the value a handle stands in for, if this producer knows it.
    fn resolve(&self, sym: Sym<D::Shape>) -> Maybe<&D::Value>;
}
