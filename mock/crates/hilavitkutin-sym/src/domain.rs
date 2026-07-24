//! Domain marker traits.
//!
//! A domain is a marker type carrying its `kind` as an associated constant,
//! refined into one of two modes: an [`InterningDomain`] deduplicates a value
//! into a stable handle, a [`GenerativeDomain`] mints a fresh handle with no
//! value. The refinement is what the producers bind against, so building the
//! wrong producer for a domain is a type error.

use crate::kind::SymKind;

/// A domain of interned identities. Carries the domain's tag.
pub trait Domain {
    /// The tag every handle of this domain carries.
    const KIND: SymKind;
}

/// A domain whose handles are deduplicated from a value and resolve back to it.
pub trait InterningDomain: Domain {
    /// The value a handle of this domain stands in for.
    type Value: ?Sized;
}

/// A domain whose handles are minted fresh, with no backing value.
pub trait GenerativeDomain: Domain {}
