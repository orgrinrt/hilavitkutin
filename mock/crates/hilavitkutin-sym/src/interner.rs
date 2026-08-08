//! `Interner`: the dedup-and-resolve producer trait.

use notko::Maybe;

use crate::domain::InterningDomain;
use crate::handle::Sym;

/// A producer that folds a value into a stable handle and resolves it back.
///
/// `intern` returns a handle tagged `D::KIND`; equal values give equal handles
/// (a within-domain flag may still distinguish origins, as strings do).
/// `resolve` returns the value for a handle of this domain, or `Maybe::Isnt`
/// for a handle the producer does not know.
pub trait Interner<D: InterningDomain> {
    /// Fold `value` into a stable handle.
    fn intern(&self, value: &D::Value) -> Sym<D::Shape>;

    /// Recover the value a handle stands in for, if this producer knows it.
    fn resolve(&self, sym: Sym<D::Shape>) -> Maybe<&D::Value>;
}
