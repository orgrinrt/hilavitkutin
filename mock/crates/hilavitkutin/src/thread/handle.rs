//! Thread handle: pool thread index (domain 20).
//!
//! `#[repr(transparent)]` newtype over u16. u16 is plenty — even
//! a 512-core plan rarely approaches the addressable range.

/// Thread index in the pool.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ThreadHandle(pub u16);
