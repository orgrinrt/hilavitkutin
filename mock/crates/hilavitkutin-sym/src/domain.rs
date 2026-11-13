//! Domain marker traits.
//!
//! A domain is a marker type carrying its `kind` as an associated constant,
//! refined into one of two modes: an [`InterningDomain`] deduplicates a value
//! into a stable handle, a [`GenerativeDomain`] mints a fresh handle with no
//! value. The refinement is what the producers bind against, so building the
//! wrong producer for a domain is a type error.

use crate::kind::SymKind;
use crate::shape::SymShape;

/// A domain of interned identities. Carries the domain's tag.
pub trait Domain {
    /// How this domain's handles divide their 32 bits.
    ///
    /// A domain belongs to a shape, so a domain of one shape cannot hand its
    /// tag to a producer of another: the widths would not line up and the
    /// compiler says so rather than a store discovering it later.
    ///
    /// Declared rather than defaulted: `associated_type_defaults` is forbidden
    /// in this stack, and a domain saying which shape it is costs one line.
    type Shape: SymShape;

    /// The tag every handle of this domain carries.
    const KIND: SymKind<Self::Shape>;
}

/// A domain whose handles are deduplicated from a value and resolve back to it.
pub trait InterningDomain: Domain {
    /// The value a handle of this domain stands in for.
    type Value: ?Sized;
}

/// A domain whose handles are minted fresh, with no backing value.
pub trait GenerativeDomain: Domain {}
